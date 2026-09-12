//! The hub's bank.
//!
//! Money lives in accounts, and a player can hold several — so every call here
//! names an account, not a player. [`account`] gives you the one a player would
//! call theirs; [`treasury`] gives you the server's own.
//!
//! ```ignore
//! let from = bank::treasury(server_id)?;
//! let to = bank::account(server_id, player)?.expect("player has an account");
//! bank::transfer(server_id, from.id, to.id, 100, "playtime reward")?;
//! ```

use noro_module_abi::entity::Account;
use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{AccountQuery, Transfer};
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

/// The account a player would call theirs — their primary card.
///
/// `None` means they have no account on this server yet. Opening one is theirs
/// to do, not a module's.
///
/// Requires `bank = ["read"]`.
pub fn account(server_id: Uuid, who: impl IntoPlayerRef) -> Result<Option<Account>, ModuleError> {
    crate::host::bank_account_call(AccountQuery {
        server_id,
        player: who.into_player_ref(),
    })
}

/// Every account a player holds on this server.
///
/// Requires `bank = ["read"]`.
pub fn accounts(server_id: Uuid, who: impl IntoPlayerRef) -> Result<Vec<Account>, ModuleError> {
    crate::host::bank_accounts_call(AccountQuery {
        server_id,
        player: who.into_player_ref(),
    })
}

/// What a player has, in the smallest unit. Zero when they have no account.
///
/// Requires `bank = ["read"]`.
pub fn balance(server_id: Uuid, who: impl IntoPlayerRef) -> Result<i64, ModuleError> {
    Ok(account(server_id, who)?.map(|a| a.balance).unwrap_or(0))
}

/// The server's own account — where rewards come from and fees go.
///
/// Requires `bank = ["read"]`.
pub fn treasury(server_id: Uuid) -> Result<Account, ModuleError> {
    crate::host::bank_treasury_call(server_id)
}

/// Moves money. Returns the transaction id.
///
/// The balance is checked by the master inside its own transaction under a row
/// lock, so a transfer cannot overdraw an account no matter how many handlers
/// run at once. `InsufficientFunds` comes back as a `Conflict` error.
///
/// Pass an `idempotency_key` through [`transfer_once`] when a repeat would be
/// worse than a failure.
///
/// Requires `bank = ["read", "transfer"]`.
pub fn transfer(
    server_id: Uuid,
    from: Uuid,
    to: Uuid,
    amount: i64,
    comment: &str,
) -> Result<i64, ModuleError> {
    send(server_id, from, to, amount, comment, None)
}

/// The same, but safe to repeat.
///
/// Send the same key again and the master returns the transfer already made
/// instead of making a second one. Use it whenever the caller might run twice —
/// a retried event handler, a task that overlapped itself.
///
/// Requires `bank = ["read", "transfer"]`.
pub fn transfer_once(
    server_id: Uuid,
    from: Uuid,
    to: Uuid,
    amount: i64,
    comment: &str,
    key: &str,
) -> Result<i64, ModuleError> {
    send(server_id, from, to, amount, comment, Some(key.to_string()))
}

fn send(
    server_id: Uuid,
    from: Uuid,
    to: Uuid,
    amount: i64,
    comment: &str,
    idempotency_key: Option<String>,
) -> Result<i64, ModuleError> {
    crate::host::bank_transfer_call(Transfer {
        server_id,
        from,
        to,
        amount,
        comment: comment.to_string(),
        idempotency_key,
    })
}
