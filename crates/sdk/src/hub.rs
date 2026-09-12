//! A server's hub: the feed, its members, towns, market, court, petitions and
//! fines.
//!
//! ```ignore
//! let feed = hub::feed(server_id, 1)?;
//! for post in feed.items {
//!     let body = post["body"].as_str().unwrap_or_default();
//! }
//! ```
//!
//! # Why these come back as JSON
//!
//! Everything here answers with [`HubPage`] — objects rather than typed
//! structs. The hub's model is large and still moving, and pinning it in the
//! ABI would make every field of every town and lot a public contract, and
//! every change to the hub a breaking release. Read the fields you need and
//! treat a missing one as possible.
//!
//! # Why writes name a player
//!
//! A post, a fine, a signature — these are things a **player** did. A module has
//! no person behind it, so it says on whose behalf it is acting, and that
//! person's name is what appears. Automation can act for someone; it cannot be
//! someone.

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{FineDraft, HubItem, HubPage, HubPost, HubQuery, PetitionVote};
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

fn query(server_id: Uuid, page: i64) -> HubQuery {
    HubQuery {
        server_id,
        page,
        per_page: 25,
        status: None,
        search: None,
    }
}

/// The feed, newest and pinned first.
///
/// Requires `hub = ["read"]`.
pub fn feed(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError> {
    crate::host::hub_feed_call(query(server_id, page))
}

/// The hub's members.
///
/// Requires `hub = ["read"]`.
pub fn members(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError> {
    crate::host::hub_members_call(query(server_id, page))
}

/// How long a player has played on this server, and when they were last seen.
///
/// Requires `hub = ["read"]`.
pub fn playtime(
    server_id: Uuid,
    who: impl IntoPlayerRef,
) -> Result<serde_json::Value, ModuleError> {
    crate::host::hub_playtime_call(serde_json::json!({
        "server_id": server_id,
        "player": who.into_player_ref(),
    }))
}

/// Writes a post into the feed on behalf of a player.
///
/// Requires `hub = ["read", "post"]`.
pub fn post(
    server_id: Uuid,
    author: impl IntoPlayerRef,
    body: &str,
) -> Result<serde_json::Value, ModuleError> {
    crate::host::hub_post_call(HubPost {
        server_id,
        author: author.into_player_ref(),
        body: body.to_string(),
    })
}

/// The towns of a server.
///
/// Requires `towns = ["read"]`.
pub fn towns(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError> {
    crate::host::hub_towns_call(query(server_id, page))
}

/// One town.
///
/// Requires `towns = ["read"]`.
pub fn town(server_id: Uuid, id: Uuid) -> Result<Option<serde_json::Value>, ModuleError> {
    crate::host::hub_town_call(HubItem { server_id, id })
}

/// Lots currently on sale.
///
/// Requires `market = ["read"]`.
pub fn market(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError> {
    crate::host::hub_market_call(query(server_id, page))
}

/// One lot.
///
/// Requires `market = ["read"]`.
pub fn lot(server_id: Uuid, id: Uuid) -> Result<Option<serde_json::Value>, ModuleError> {
    crate::host::hub_lot_call(HubItem { server_id, id })
}

/// Court cases.
///
/// Requires `court = ["read"]`.
pub fn court(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError> {
    crate::host::hub_court_call(query(server_id, page))
}

/// One court case.
///
/// Requires `court = ["read"]`.
pub fn court_case(server_id: Uuid, id: Uuid) -> Result<Option<serde_json::Value>, ModuleError> {
    crate::host::hub_court_case_call(HubItem { server_id, id })
}

/// Petitions and how far along they are.
///
/// Requires `petitions = ["read"]`.
pub fn petitions(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError> {
    crate::host::hub_petitions_call(query(server_id, page))
}

/// Signs a petition on behalf of a player. `false` — they had already signed.
///
/// A petition that gathers enough signatures is promoted by the master on its
/// own, so a module does not have to watch for the threshold.
///
/// Requires `petitions = ["read", "sign"]`.
pub fn sign(
    server_id: Uuid,
    petition_id: Uuid,
    who: impl IntoPlayerRef,
) -> Result<bool, ModuleError> {
    crate::host::hub_sign_call(PetitionVote {
        server_id,
        petition_id,
        player: who.into_player_ref(),
    })
}

/// Takes a signature back. `false` — there was none.
///
/// Requires `petitions = ["read", "sign"]`.
pub fn unsign(
    server_id: Uuid,
    petition_id: Uuid,
    who: impl IntoPlayerRef,
) -> Result<bool, ModuleError> {
    crate::host::hub_unsign_call(PetitionVote {
        server_id,
        petition_id,
        player: who.into_player_ref(),
    })
}

/// The fines of a server.
///
/// Requires `fines = ["read"]`.
pub fn fines(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError> {
    crate::host::hub_fines_call(query(server_id, page))
}

/// The fines of one player.
///
/// Requires `fines = ["read"]`.
pub fn fines_of(server_id: Uuid, who: impl IntoPlayerRef) -> Result<HubPage, ModuleError> {
    crate::host::hub_fines_of_call(serde_json::json!({
        "server_id": server_id,
        "player": who.into_player_ref(),
    }))
}

/// Issues a fine, in the name of a staff member.
///
/// The issuer is named and their name goes on the fine: the player will see who
/// fined them and ask that person about it.
///
/// Requires `fines = ["read", "issue"]`.
pub fn fine(draft: FineDraft) -> Result<serde_json::Value, ModuleError> {
    crate::host::hub_fine_call(draft)
}
