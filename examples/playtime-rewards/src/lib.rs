// Модуль наград за игровое время: начисление очков, роли, кэш, WebSocket и RPC.

use noro_sdk::prelude::*;

const DEFAULT_PER_HOUR: i64 = 100;
const DEFAULT_HOURS_FOR_ROLE: i64 = 10;

pub struct PlaytimeRewards;

#[derive(Serialize, Deserialize)]
pub struct Points {
    pub player: String,
    pub points: i64,
    pub seconds_played: i64,
}

#[derive(Deserialize)]
pub struct GetPointsQuery {
    pub player_id: Uuid,
}

#[noro::module]
impl PlaytimeRewards {
    #[register]
    fn setup(reg: &mut Registration) {
        reg.setting(
            "points_per_hour",
            SettingKind::Number,
            "mod-playtime-rewards-per-hour",
        )
        .default(DEFAULT_PER_HOUR)
        .range(0, 10_000);

        reg.setting(
            "role_after_hours",
            SettingKind::Number,
            "mod-playtime-rewards-role-after",
        )
        .default(DEFAULT_HOURS_FOR_ROLE)
        .range(0, 10_000)
        .hint("mod-playtime-rewards-role-after-hint");

        reg.setting(
            "role_name",
            SettingKind::Text,
            "mod-playtime-rewards-role-name",
        )
        .default("veteran");
    }

    #[init]
    fn start() -> Result<()> {
        let per_hour: i64 = store::instance()
            .get("points_per_hour")?
            .unwrap_or(DEFAULT_PER_HOUR);
        log::info(format!("награды включены: {per_hour} очков за час"));
        Ok(())
    }

    #[event]
    fn on_join(e: PlayerJoined) -> Result<()> {
        if e.first_join {
            log::info(format!(
                "{} впервые заходит на {}",
                e.player.label(),
                e.ctx.server_name()
            ));
        }
        store::user(e.player.id).set("last_join", &noro_sdk::now().timestamp())?;
        // Кэшируем онлайн-статус в памяти
        cache::set(&format!("active:{}", e.player.id), &true, 300)?;
        Ok(())
    }

    #[event(priority = normal)]
    fn on_left(e: PlayerLeft) -> Result<()> {
        let seconds = e.session_secs.max(0);
        let _ = cache::delete(&format!("active:{}", e.player.id));
        if seconds == 0 {
            return Ok(());
        }

        let per_hour: i64 = store::instance()
            .get("points_per_hour")?
            .unwrap_or(DEFAULT_PER_HOUR);
        let earned = seconds * per_hour / 3600;

        let me = store::user(e.player.id);
        let played = me.incr("seconds_played", seconds)?;
        if earned > 0 {
            let total = me.incr("points", earned)?;
            log::info(format!(
                "{} получает {earned} очков за {} мин, всего {total}",
                e.player.label(),
                seconds / 60
            ));
        }

        Self::maybe_promote(&e.player, played)?;
        Ok(())
    }

    fn maybe_promote(player: &Player, played_secs: i64) -> Result<()> {
        let hours: i64 = store::instance()
            .get("role_after_hours")?
            .unwrap_or(DEFAULT_HOURS_FOR_ROLE);
        if hours <= 0 || played_secs < hours * 3600 {
            return Ok(());
        }

        let role: String = store::instance()
            .get("role_name")?
            .unwrap_or_else(|| "veteran".to_string());
        if player.has_role(&role) {
            return Ok(());
        }

        match roles::grant(player.id, role.as_str()) {
            Ok(()) => log::info(format!("{} получает роль «{role}»", player.label())),
            Err(e) => log::warn(format!("роль «{role}» не выдана: {e}")),
        }
        Ok(())
    }

    /// WebSocket действие: запрос бонуса из браузера
    #[ws_action("claim_bonus")]
    fn on_claim_bonus(player: Player) -> Result<()> {
        let key = format!("bonus_cooldown:{}", player.id);
        if cache::get::<bool>(&key)?.unwrap_or(false) {
            web_ws::send(
                player.id,
                json!({
                    "type": "bonus_status",
                    "status": "cooldown",
                    "message": "Бонус уже получен недавно"
                }),
            )?;
            return Ok(());
        }

        // Начисляем бонусные 10 очков и ставим кулдаун на 60 сек в кэш
        let me = store::user(player.id);
        let points = me.incr("points", 10)?;
        cache::set(&key, &true, 60)?;

        web_ws::send(
            player.id,
            json!({
                "type": "bonus_status",
                "status": "claimed",
                "added": 10,
                "total_points": points
            }),
        )?;
        Ok(())
    }

    /// RPC-метод для других модулей (например, магазина или боевого пропуска):
    /// `modules::call::<Points>("playtime-rewards", "get_points", &GetPointsQuery { player_id })`
    #[rpc("get_points")]
    fn rpc_get_points(Json(query): Json<GetPointsQuery>) -> Result<Points> {
        let me = store::user(query.player_id);
        let player = players::require(query.player_id)?;
        Ok(Points {
            player: player.label(),
            points: me.get("points")?.unwrap_or(0),
            seconds_played: me.get("seconds_played")?.unwrap_or(0),
        })
    }

    #[route(POST, "/reset", auth = permission("noro.module.playtime-rewards.reset"))]
    fn route_reset(req: HttpRequest) -> Result<Points> {
        let user_id = req.require_user()?;
        let me = store::user(user_id);
        me.set("points", &0)?;
        log::info(format!("очки сброшены у {user_id}"));
        Ok(Points {
            player: players::require(user_id)?.label(),
            points: 0,
            seconds_played: me.get("seconds_played")?.unwrap_or(0),
        })
    }

    #[route(GET, "/me")]
    fn route_me(req: HttpRequest) -> Result<Points> {
        let user_id = req.require_user()?;
        let player = players::require(user_id)?;
        let me = store::user(user_id);
        Ok(Points {
            player: player.label(),
            points: me.get("points")?.unwrap_or(0),
            seconds_played: me.get("seconds_played")?.unwrap_or(0),
        })
    }
}

#[cfg(test)]
mod tests {
    use noro_sdk::prelude::*;
    use noro_sdk::testing::*;

    #[test]
    fn mock_web_message_has_valid_player() {
        let msg = mock_web_message("Steve", &json!({ "action": "claim_bonus" }));
        assert_eq!(msg.player.label(), "Steve");
    }

    #[test]
    fn mock_request_has_correct_method() {
        let req = mock_request("GET", "/me", None, Some(mock_player("Steve").id));
        assert_eq!(req.method, "GET");
        assert!(req.user.is_some());
    }
}
