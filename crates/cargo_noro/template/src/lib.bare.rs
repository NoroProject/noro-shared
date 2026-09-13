//! A Noro module.
//!
//! Documentation: <https://noroproject.github.io/noro-shared/>

use noro_sdk::prelude::*;

pub struct Module;

// Everything a module does is declared here with an attribute — the manifest
// repeats none of it, and no handler name is written as a string anywhere.
//
//   #[event]   fn on_join(e: PlayerJoined) -> Result<()>   the name comes from the type
//   #[route]   #[route(GET, "/top")]                        /api/modules/{{id}}/top
//   #[task]    #[task(every = "1h")]                        on a schedule
//   #[init]    fn start() -> Result<()>                     once, when enabled
//   #[register] fn setup(reg: &mut Registration)            settings and permissions
//
// The event catalog:
// <https://noroproject.github.io/noro-shared/reference/events/>
#[noro::module]
impl Module {}
