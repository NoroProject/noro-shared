//! A Noro module, ready to be made into yours.
//!
//! Everything below is a working example of one thing each: a settings field, a
//! one-off setup step, an event handler and an endpoint. Delete what you do not
//! need — none of it is required for the module to load.
//!
//! Documentation: <https://noroproject.github.io/noro-shared/>

use noro_sdk::prelude::*;
use serde::Serialize;

pub struct Module;

/// What the endpoint answers with. Any `Serialize` will do.
#[derive(Serialize)]
pub struct Stats {
    joins: i64,
    greeted: bool,
}

#[noro::module]
impl Module {
    /// The settings form the operator fills in, declared once at install time.
    ///
    /// Settings live here rather than in the manifest because a setting is
    /// almost always introduced together with the code that reads it.
    #[register]
    fn setup(reg: &mut Registration) {
        reg.setting("greet", SettingKind::Toggle, "mod-{{id}}-greet")
            .default(true)
            .hint("mod-{{id}}-greet-hint");
    }

    /// Runs once when the module is enabled, with capabilities already granted.
    ///
    /// Unlike the declaration above, this can reach the store — so it is the
    /// place for anything that has to exist before the first event arrives.
    #[init]
    fn start() -> Result<()> {
        log::info("template module enabled");
        Ok(())
    }

    /// The event name comes from the argument type, never from a string.
    ///
    /// Swap `PlayerJoined` for any other event and the subscription follows;
    /// the catalog is at
    /// <https://noroproject.github.io/noro-shared/reference/events/>.
    #[event]
    fn on_join(e: PlayerJoined) -> Result<()> {
        let joins = store::user(e.player.id).incr("joins", 1)?;

        if e.first_join {
            log::info(format!("{} is new here", e.player.label()));
        } else {
            log::debug(format!("{} has joined {joins} times", e.player.label()));
        }

        Ok(())
    }

    /// Reachable at `GET /api/modules/{{id}}/me`.
    ///
    /// `auth` defaults to any signed-in player, so `require_user` cannot fail
    /// here — but it returns a `Result` so that turning the endpoint public
    /// later does not silently hand you a missing user.
    #[route(GET, "/me")]
    fn me(req: HttpRequest) -> Result<Stats> {
        let id = req.require_user()?;
        Ok(Stats {
            joins: store::user(id).get_or("joins")?,
            greeted: store::instance().get_or("greet")?,
        })
    }
}
