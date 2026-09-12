//! Награды за время в игре — пример модуля Noro.
//!
//! Считает, сколько игрок провёл на серверах, и начисляет за это очки.
//! Показывает три вещи, из которых состоит почти любой модуль: подписку на
//! события, работу со своим хранилищем и собственную ручку для мини-аппа.
//!
//! Ничего из этого не объявляется в манифесте: `#[event]` и `#[route]` стоят
//! прямо над обработчиками, а имя события мастер выводит из типа аргумента.

use noro_sdk::prelude::*;

/// Сколько очков за час игры, пока настройки не заданы.
const DEFAULT_PER_HOUR: i64 = 100;

pub struct PlaytimeRewards;

#[noro::module]
impl PlaytimeRewards {
    /// Что модуль объявляет о себе сверх того, что видно по атрибутам.
    ///
    /// Вызывается при установке и при включении — в песочнице, где модулю ещё
    /// ничего не выдано. Сюда идёт то, что не привязано к конкретному
    /// обработчику: поля формы настроек.
    #[register]
    fn setup(reg: &mut Registration) {
        reg.setting(
            "points_per_hour",
            SettingKind::Number,
            "mod-playtime-rewards-per-hour",
        )
        .default(DEFAULT_PER_HOUR)
        .range(0, 10_000);
    }

    /// Разовая подготовка при включении.
    ///
    /// В отличие от `#[register]`, здесь у модуля уже есть выданные
    /// возможности: можно читать хранилище и ходить наружу.
    #[init]
    fn start() -> Result<()> {
        let per_hour: i64 = store::instance()
            .get("points_per_hour")?
            .unwrap_or(DEFAULT_PER_HOUR);
        log::info(format!("награды включены: {per_hour} очков за час"));
        Ok(())
    }

    /// Игрок вошёл: запоминаем момент, чтобы на выходе было от чего считать.
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
        Ok(())
    }

    /// Игрок вышел: начисляем за наигранное.
    #[event(priority = normal)]
    fn on_left(e: PlayerLeft) -> Result<()> {
        // Длительность считает мастер: она у него уже есть из закрытой сессии,
        // а разница с нашей отметкой врала бы при перезапуске сервера.
        let seconds = e.session_secs.max(0);
        if seconds == 0 {
            return Ok(());
        }

        let per_hour: i64 = store::instance()
            .get("points_per_hour")?
            .unwrap_or(DEFAULT_PER_HOUR);
        let earned = seconds * per_hour / 3600;

        let me = store::user(e.player.id);
        me.incr("seconds_played", seconds)?;
        if earned > 0 {
            let total = me.incr("points", earned)?;
            log::info(format!(
                "{} получает {earned} очков за {} мин, всего {total}",
                e.player.label(),
                seconds / 60
            ));
        }
        Ok(())
    }

    /// Сколько очков у того, кто открыл мини-апп.
    ///
    /// Права проверил мастер: до модуля запрос без входа не доходит, поэтому
    /// здесь достаточно взять идентификатор из запроса.
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

#[derive(Serialize, Deserialize)]
pub struct Points {
    pub player: String,
    pub points: i64,
    pub seconds_played: i64,
}
