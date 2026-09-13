//! Private messages between players.
//!
//! One conversation per pair, the same one whether it is read on the site or in
//! game. A module can look into it, write on a player's behalf, and see where
//! that player is right now.
//!
//! ```ignore
//! // An answering machine for someone who is away.
//! if dm::presence(&recipient)?.kind == "offline" {
//!     dm::send(&recipient, &sender, "I am away until Monday.")?;
//! }
//! ```
//!
//! **`read` is the heaviest thing an operator can hand you.** It opens messages
//! two people hold to be between them; the instance's privacy policy promises
//! that staff do not read them. Ask for it when the job needs it — filtering,
//! auto-replies — and not to have it just in case. The operator sees what you
//! asked for and decides.

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{DmDraft, DmMessage, DmPresence, DmThread};
use noro_module_abi::player::IntoPlayerRef;

/// The conversations a player has, most recent first.
///
/// Requires `dm = ["read"]`.
pub fn threads(
    who: impl IntoPlayerRef,
    page: i64,
    per_page: i64,
) -> Result<Vec<DmThread>, ModuleError> {
    crate::host::dm_threads_call(serde_json::json!({
        "who": who.into_player_ref(),
        "page": page,
        "per_page": per_page,
    }))
}

/// The messages between two players, newest first.
///
/// An empty list when they have never written to each other — that is not an
/// error, just an empty conversation.
///
/// Requires `dm = ["read"]`.
pub fn history(
    a: impl IntoPlayerRef,
    b: impl IntoPlayerRef,
    page: i64,
    per_page: i64,
) -> Result<Vec<DmMessage>, ModuleError> {
    crate::host::dm_history_call(serde_json::json!({
        "a": a.into_player_ref(),
        "b": b.into_player_ref(),
        "page": page,
        "per_page": per_page,
    }))
}

/// Writes a message on a player's behalf. Returns its id.
///
/// The sender is named rather than left as the module, for the same reason a
/// ticket reply is signed with the module and a fine is issued by a person: the
/// recipient is going to answer, and a message from nobody leaves them with
/// nobody to answer to. The history records which module wrote it, and the
/// conversation shows that too.
///
/// Refused the same way as any other message would be: a block, an empty body,
/// one over the length limit. A refusal does not say which — for the recipient
/// of that refusal, "blocked" and "no such conversation" must look alike.
///
/// Requires `dm = ["send"]`.
pub fn send(
    from: impl IntoPlayerRef,
    to: impl IntoPlayerRef,
    body: &str,
) -> Result<uuid::Uuid, ModuleError> {
    crate::host::dm_send_call(DmDraft {
        from: from.into_player_ref(),
        to: to.into_player_ref(),
        body: body.to_string(),
    })
}

/// Where a player is: `in_game`, `on_site`, `in_launcher` or `offline`.
///
/// A player hidden by vanish reads as offline. They left the online list on
/// purpose, and this must not be the thing that gives them away.
///
/// Requires `dm = ["read"]`.
pub fn presence(who: impl IntoPlayerRef) -> Result<DmPresence, ModuleError> {
    crate::host::dm_presence_call(who.into_player_ref())
}

/// The conversations of one player, reached as `player.dm()`.
///
/// A handle rather than four methods hanging off `Player`: everything here is
/// about one person's correspondence, and grouping says so — the same way
/// `player.store()` groups their storage.
#[derive(Debug, Clone, Copy)]
pub struct Conversations(uuid::Uuid);

/// Their conversations by id. Prefer `player.dm()`.
pub fn of(who: uuid::Uuid) -> Conversations {
    Conversations(who)
}

impl Conversations {
    /// Their conversations, most recent first. See [`threads`].
    pub fn threads(&self, page: i64, per_page: i64) -> Result<Vec<DmThread>, ModuleError> {
        threads(self.0, page, per_page)
    }

    /// What they and one other person wrote. See [`history`].
    pub fn with(
        &self,
        peer: impl IntoPlayerRef,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<DmMessage>, ModuleError> {
        history(self.0, peer, page, per_page)
    }

    /// Writes to someone **as this player**. See [`send`].
    pub fn send_to(
        &self,
        peer: impl IntoPlayerRef,
        body: &str,
    ) -> Result<uuid::Uuid, ModuleError> {
        send(self.0, peer, body)
    }

    /// Writes **to this player**, signed with whoever is named.
    pub fn from(
        &self,
        sender: impl IntoPlayerRef,
        body: &str,
    ) -> Result<uuid::Uuid, ModuleError> {
        send(sender, self.0, body)
    }
}
