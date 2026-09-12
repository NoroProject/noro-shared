//! Conversations with players.
//!
//! One thread per conversation, whether it started in the cabinet, in game
//! with `/support`, or out of a moderation case. A module can read the queue,
//! answer, and open a thread of its own.
//!
//! ```ignore
//! let id = tickets::open(player, "Your appeal", "We have looked at it again.")?;
//! tickets::reply(id, "Lifted. Sorry about that.")?;
//! ```

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{Ticket, TicketDraft, TicketMessage, TicketReply};
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

/// The staff queue: everything not closed, newest activity first.
///
/// Requires `tickets = ["read"]`.
pub fn queue(page: i64, per_page: i64) -> Result<Vec<Ticket>, ModuleError> {
    crate::host::tickets_queue_call(serde_json::json!({ "page": page, "per_page": per_page }))
}

/// One conversation.
///
/// Requires `tickets = ["read"]`.
pub fn get(id: Uuid) -> Result<Option<Ticket>, ModuleError> {
    crate::host::ticket_get_call(id)
}

/// The messages of a conversation, oldest first.
///
/// Requires `tickets = ["read"]`.
pub fn messages(id: Uuid) -> Result<Vec<TicketMessage>, ModuleError> {
    crate::host::ticket_messages_call(id)
}

/// Writes a reply, signed with your module.
///
/// It is signed with the module rather than with a person because there is no
/// person: attributing it to whoever enabled the module would put words in
/// their mouth, and the player is going to read those words.
///
/// The player is notified the way any staff reply notifies them — in the
/// cabinet, and in game if they are online.
///
/// Requires `tickets = ["read", "reply"]`.
pub fn reply(id: Uuid, content: &str) -> Result<(), ModuleError> {
    crate::host::ticket_reply_call(TicketReply {
        ticket_id: id,
        content: content.to_string(),
    })
}

/// Opens a conversation with a player and returns its identifier.
///
/// If one is already open with them, that one is used: two threads about the
/// same thing is how an answer gets lost.
///
/// Requires `tickets = ["read", "reply"]`.
pub fn open(who: impl IntoPlayerRef, subject: &str, content: &str) -> Result<Uuid, ModuleError> {
    crate::host::ticket_open_call(TicketDraft {
        player: who.into_player_ref(),
        subject: subject.to_string(),
        content: content.to_string(),
    })
}

/// Closes a conversation.
///
/// Requires `tickets = ["read", "reply"]`.
pub fn close(id: Uuid) -> Result<(), ModuleError> {
    crate::host::ticket_close_call(id)
}
