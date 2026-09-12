//! The shapes of host calls that take more than one argument.
//!
//! These live in the ABI because both sides have to agree on them: the SDK
//! serializes, the master parses. A struct with named fields rather than a
//! tuple, because a call carrying two `Uuid`s in a row is one worth being
//! unable to get backwards.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::player::{Player, PlayerRef};

/// How to name a role in a call.
///
/// Machine name or identifier, the same two ways the panel names one. Struct
/// variants rather than tuples: on an internally tagged enum serde requires the
/// content to be an object, and a newtype variant fails only at runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "by", rename_all = "snake_case")]
pub enum RoleRef {
    /// `vip`, `moderator` — what you write in code.
    Name {
        name: String,
    },
    Id {
        id: Uuid,
    },
}

impl RoleRef {
    pub fn name(name: impl Into<String>) -> Self {
        RoleRef::Name { name: name.into() }
    }

    pub fn id(id: Uuid) -> Self {
        RoleRef::Id { id }
    }
}

/// Everything a role can be named by, so `roles::get("vip")` works.
pub trait IntoRoleRef {
    fn into_role_ref(self) -> RoleRef;
}

impl IntoRoleRef for RoleRef {
    fn into_role_ref(self) -> RoleRef {
        self
    }
}

impl IntoRoleRef for &str {
    fn into_role_ref(self) -> RoleRef {
        RoleRef::name(self)
    }
}

impl IntoRoleRef for String {
    fn into_role_ref(self) -> RoleRef {
        RoleRef::Name { name: self }
    }
}

impl IntoRoleRef for Uuid {
    fn into_role_ref(self) -> RoleRef {
        RoleRef::Id { id: self }
    }
}

/// Granting or revoking a role.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleGrant {
    pub player: PlayerRef,
    pub role: RoleRef,
}

/// A permission on a player, in one context.
///
/// `server_id = None` means everywhere. It is not a filter but part of the
/// identity of the grant: revoking "on this server" must not quietly take away
/// the global one, which is why the master matches it exactly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionOn {
    pub player: PlayerRef,
    pub node: String,
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

/// Which player, and where to look at their permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionQuery {
    pub player: PlayerRef,
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

/// Access to a server build — the right to join it at all.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerAccess {
    pub player: PlayerRef,
    pub server_id: Uuid,
}

/// Access to one client build of a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildAccess {
    pub player: PlayerRef,
    pub build_id: Uuid,
}

/// Turning maintenance mode on or off for a game server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maintenance {
    pub game_server_id: Uuid,
    pub enabled: bool,
}

/// Banning a player by the account flag.
///
/// This is the flag, not a punishment: it keeps the account out entirely and
/// carries no duration. Timed sanctions are `punish`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanRequest {
    pub player: PlayerRef,
    pub banned: bool,
    #[serde(default)]
    pub reason: Option<String>,
}

/// A message to one player, in game.
///
/// The text is finished text, not a Fluent key: the module's catalog is built
/// for the web and is not installed in the master's own `i18n`, so resolving a
/// key here would quietly hand the player the key itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMessage {
    pub player: PlayerRef,
    pub message: String,
}

/// A message to everyone, or to everyone on one server build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub message: String,
    /// `None` — the whole instance.
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

/// What kind of sanction to issue.
///
/// An enum rather than a string: the master stores it as text, but a typo in
/// `"mute"` would arrive as a warning and nobody would notice for a week.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PunishKind {
    /// Out of the game and the launcher both.
    Ban,
    /// Out of one server build only.
    ServerBan,
    /// Cannot speak.
    Mute,
    /// A note the player has to acknowledge.
    Warn,
}

impl PunishKind {
    /// How the master stores it.
    pub fn as_str(self) -> &'static str {
        match self {
            PunishKind::Ban => "ban",
            PunishKind::ServerBan => "server_ban",
            PunishKind::Mute => "mute",
            PunishKind::Warn => "warn",
        }
    }
}

/// A sanction to issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishRequest {
    pub player: PlayerRef,
    pub kind: PunishKind,
    /// What the player will read. Not a key — see [`PlayerMessage`].
    pub reason: String,
    /// How long it lasts. `None` — until it is lifted by hand.
    #[serde(default)]
    pub seconds: Option<i64>,
    /// Which server build it applies to. `None` — all of them.
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

/// Which account, on which server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountQuery {
    pub server_id: Uuid,
    pub player: PlayerRef,
}

/// Moving money between two accounts.
///
/// Accounts, not players: a player can hold several, and "their money" is not a
/// well-defined place to take it from. Take the account you mean from
/// [`AccountQuery`] or from the treasury.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub server_id: Uuid,
    pub from: Uuid,
    pub to: Uuid,
    /// In the smallest unit. Must be above zero — a transfer of nothing is a
    /// mistake, and a negative one is a transfer the other way written wrong.
    pub amount: i64,
    /// Shown in the bank's ledger to both sides.
    pub comment: String,
    /// Repeat protection. Send the same key again and the master returns the
    /// transfer already made instead of making a second one — which is what you
    /// want when your handler ran twice.
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

/// A linked login: how a player signs in besides the game itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// `discord`, `twitch`, `google` — the machine name of the platform.
    pub provider: String,
    /// Their identifier on that platform.
    pub provider_user_id: String,
    #[serde(default)]
    pub username: Option<String>,
    /// The platform the player registered through. Their Minecraft UUID is
    /// derived from it, so this one cannot be unlinked.
    #[serde(default)]
    pub primary: bool,
}

/// Linking a login to a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkRequest {
    pub player: PlayerRef,
    pub provider: String,
    pub provider_user_id: String,
    /// The name on that platform, for staff to recognise.
    #[serde(default)]
    pub username: Option<String>,
}

/// Which platform to look at, on which player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderQuery {
    pub player: PlayerRef,
    pub provider: String,
}

/// Renaming a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameRequest {
    pub player: PlayerRef,
    /// The new Minecraft username.
    pub username: String,
}

/// Setting a skin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinRequest {
    pub player: PlayerRef,
    /// The file. `None` clears the skin back to the default.
    #[serde(default)]
    pub url: Option<String>,
    /// The slim model — Alex arms. Part of the same decision as the file: a
    /// slim texture on classic arms reads as broken.
    #[serde(default)]
    pub slim: bool,
}

/// Setting a cape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapeRequest {
    pub player: PlayerRef,
    /// A cape from the instance's set. `None` takes the cape off.
    #[serde(default)]
    pub cape_id: Option<Uuid>,
}

/// A saved skin, under a name the player gave it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinPreset {
    pub id: Uuid,
    pub name: String,
    pub skin_url: String,
    pub slim: bool,
}

/// Saving a skin under a name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavePreset {
    pub player: PlayerRef,
    pub name: String,
    pub skin_url: String,
    /// `None` — take the geometry the player is wearing now.
    #[serde(default)]
    pub slim: Option<bool>,
}

/// Which preset of which player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetRef {
    pub player: PlayerRef,
    pub preset_id: Uuid,
}

/// Publishing or unpublishing a client build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRequest {
    pub build_id: Uuid,
    pub published: bool,
}

/// A cape available on the instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cape {
    pub id: Uuid,
    pub name: String,
    pub url: String,
}

/// A news item as a module sees it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub preview_url: Option<String>,
    /// Kept at the top of the list regardless of date.
    pub pinned: bool,
    pub published_at: chrono::DateTime<chrono::Utc>,
}

/// A news item to publish, or the new content of one.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsDraft {
    /// Set when editing; absent when publishing something new.
    #[serde(default)]
    pub id: Option<Uuid>,
    pub title: String,
    /// Markdown, the same as the panel's editor writes.
    pub body: String,
    #[serde(default)]
    pub preview_url: Option<String>,
    #[serde(default)]
    pub pinned: bool,
}

/// An instance setting to write.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingWrite {
    pub key: String,
    /// Whatever the setting holds. The master stores settings as JSON and does
    /// not type them: the set grows, and freezing it in the ABI would mean a
    /// new setting needing an ABI release.
    pub value: serde_json::Value,
}

/// A player's session — one signed-in launcher or browser.
///
/// There is no address and no user agent here, because the master does not keep
/// them: a session is a token with a scope and a lifetime, and inventing fields
/// for the ABI that the database has never held would be worse than their
/// absence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    /// What the token is allowed to do: `launcher`, `web`, an OAuth scope.
    pub scope: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Opened by staff acting as this player, rather than by the player.
    #[serde(default)]
    pub impersonated: bool,
}

/// Which session of which player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRef {
    pub player: PlayerRef,
    /// Absent — every session they have.
    #[serde(default)]
    pub session_id: Option<Uuid>,
}

/// A restart schedule of one game server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestartSchedule {
    pub id: Uuid,
    pub game_server_id: Uuid,
    /// A cron expression, when that is how it was set.
    #[serde(default)]
    pub cron: Option<String>,
    /// Times of day, when it was set that way instead.
    #[serde(default)]
    pub at_times: Vec<String>,
    #[serde(default)]
    pub interval_minutes: Option<i32>,
    /// How long players are warned for.
    pub notice_minutes: i32,
    /// What to do when people are still playing: `wait`, `force`, `skip`.
    pub online_policy: String,
    pub active: bool,
    #[serde(default)]
    pub next_run_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A restart schedule to create.
///
/// Exactly one of `cron`, `at_times` and `interval_minutes` says *when* — the
/// master refuses a draft that names none or several, because "every two hours
/// and also at 04:00" has no single answer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScheduleDraft {
    pub game_server_id: Uuid,
    #[serde(default)]
    pub cron: Option<String>,
    #[serde(default)]
    pub at_times: Vec<String>,
    #[serde(default)]
    pub interval_minutes: Option<i32>,
    /// Minutes of warning. Zero means no warning at all.
    #[serde(default)]
    pub notice_minutes: i32,
    /// `wait` by default — a restart that kicks people mid-fight is rarely what
    /// was meant.
    #[serde(default)]
    pub online_policy: Option<String>,
    #[serde(default)]
    pub max_defer_minutes: Option<i32>,
}

/// A file in the shared store.
///
/// The store is addressed by content hash, so putting the same bytes twice
/// stores them once — and the hash you get back is the address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredFile {
    /// The sha1 of the content. Its identity and its address both.
    pub sha1: String,
    pub size: i64,
    /// Where it is served from, ready to put in a page.
    pub url: String,
}

/// Bytes to store.
///
/// Base64, because JSON is what crosses the wasm boundary and raw bytes do not
/// survive it. That is also why files are capped: the payload passes through
/// the sandbox's memory twice, encoded and decoded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWrite {
    pub base64: String,
}

/// A player in game right now, and where.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlinePlayer {
    pub player: Player,
    pub game_server_id: Uuid,
    /// Hidden from other players by staff tooling.
    #[serde(default)]
    pub vanished: bool,
}

/// One load measurement of a game server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    pub game_server_id: Uuid,
    /// Ticks per second. Twenty is healthy.
    #[serde(default)]
    pub tps: Option<f64>,
    /// Milliseconds per tick. Rises before tps falls, so it notices trouble
    /// first.
    #[serde(default)]
    pub mspt: Option<f64>,
    #[serde(default)]
    pub heap_used_mb: Option<i32>,
    #[serde(default)]
    pub heap_max_mb: Option<i32>,
    #[serde(default)]
    pub online_players: Option<i32>,
    pub at: chrono::DateTime<chrono::Utc>,
}

/// A role to create or rewrite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDraft {
    /// Set when editing an existing role.
    #[serde(default)]
    pub id: Option<Uuid>,
    /// The machine name: `vip`, `moderator`.
    pub name: String,
    pub display_name: String,
    /// A hex colour, `#5865F2`.
    #[serde(default)]
    pub color: Option<String>,
    /// Given to every new player automatically.
    #[serde(default)]
    pub is_default: bool,
    /// The server the role belongs to. `None` — it applies everywhere.
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

/// Access to one optional mod of a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionalMod {
    pub player: PlayerRef,
    pub server_id: Uuid,
    /// The mod's name as the build lists it.
    pub mod_name: String,
}

/// A conversation with a player — from the panel, from the game, or opened out
/// of a moderation case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Uuid,
    pub player_id: Uuid,
    pub player_name: String,
    pub subject: String,
    /// `open`, `answered`, `closed`.
    pub status: String,
    /// The case it came out of, when it did.
    #[serde(default)]
    pub case_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_message_at: chrono::DateTime<chrono::Utc>,
    /// Messages from the player that staff have not read.
    pub unread: i64,
}

/// One message in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketMessage {
    pub id: Uuid,
    /// `player`, `staff` or `system`.
    pub side: String,
    /// The name as it was when the message was sent: names change, history
    /// does not.
    pub author_name: String,
    #[serde(default)]
    pub author_role: Option<String>,
    pub content: String,
    pub at: chrono::DateTime<chrono::Utc>,
}

/// A reply to write into a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketReply {
    pub ticket_id: Uuid,
    pub content: String,
}

/// Opening a conversation with a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketDraft {
    pub player: PlayerRef,
    pub subject: String,
    /// The first message. Without one the player sees a subject and nothing
    /// else, which reads as a mistake.
    pub content: String,
}

/// A moderation case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Case {
    pub id: Uuid,
    /// The number staff quote to each other.
    pub number: i64,
    pub target_id: Uuid,
    /// `open`, `claimed`, `resolved`, `rejected`.
    pub status: String,
    /// The game server it was opened on, when it was opened from in game.
    #[serde(default)]
    pub game_server_id: Option<Uuid>,
    /// The moderator it is claimed by, if any.
    #[serde(default)]
    pub claimed_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// One entry in a case's timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseEvent {
    pub id: Uuid,
    /// What happened: `claim`, `punishment`, `note`, `chat`.
    pub kind: String,
    #[serde(default)]
    pub actor_name: Option<String>,
    /// The details, shape depending on `kind`.
    #[serde(default)]
    pub details: serde_json::Value,
    pub at: chrono::DateTime<chrono::Utc>,
}

/// Closing a case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseVerdict {
    pub case_id: Uuid,
    /// `confirmed` — the report held up; anything else rejects it.
    pub verdict: String,
    /// What staff will read later, when the player asks.
    pub resolution: String,
    /// The rule it was decided under, when there was one.
    #[serde(default)]
    pub rule_code: Option<String>,
}

/// A page of hub documents.
///
/// The documents come through as JSON rather than as typed structs, and that is
/// deliberate. The hub's model is large and still moving — posts, towns, lots,
/// claims, petitions, each with a dozen fields and their own history. Freezing
/// it here would make every one of those fields a public contract and every
/// change to the hub a breaking ABI release, which is exactly what this crate
/// exists to avoid.
///
/// Read what you need out of the object and treat a missing field as possible.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubPage {
    pub items: Vec<serde_json::Value>,
    pub total: i64,
}

/// Which hub, and how much of it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubQuery {
    pub server_id: Uuid,
    #[serde(default)]
    pub page: i64,
    #[serde(default)]
    pub per_page: i64,
    /// Filters the list where the list supports it.
    #[serde(default)]
    pub status: Option<String>,
    /// Free-text search, where the list supports it.
    #[serde(default)]
    pub search: Option<String>,
}

/// One document of a hub.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubItem {
    pub server_id: Uuid,
    pub id: Uuid,
}

/// A post to write into the feed, on behalf of a player.
///
/// The author is named because a post is something a *player* did. A module has
/// no person behind it, and a feed entry from nobody is not a thing the hub can
/// represent — nor something a reader could reply to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubPost {
    pub server_id: Uuid,
    pub author: PlayerRef,
    pub body: String,
}

/// A fine, issued by a named member of staff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FineDraft {
    pub server_id: Uuid,
    /// Who is issuing it. Their name goes on the fine, and they answer for it.
    pub issuer: PlayerRef,
    pub target: PlayerRef,
    /// In the hub's minor units.
    pub amount: i64,
    pub reason: String,
    /// Days to pay. Zero — no deadline.
    #[serde(default)]
    pub due_days: i32,
}

/// Founding a town on somebody's behalf.
///
/// The founding fee comes off the player's primary card. A module does not
/// choose the card: the fee is the player's money, and picking which of their
/// cards it leaves is their decision, not an automation's. Without a card, and
/// with a fee to pay, the call fails rather than finding a way.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownDraft {
    pub server_id: Uuid,
    /// Who founds it, and becomes its mayor.
    pub founder: PlayerRef,
    pub name: String,
    /// The address the town lives at: `[a-z0-9-]`, 26 characters at most.
    pub slug: String,
}

/// Listing a lot on the market on somebody's behalf.
///
/// Only the vanilla mode is reachable: there the goods stay in the seller's
/// barrels and the lot carries their addresses. The vault mode takes the item
/// itself, encoded by the platform the agent runs on, and a module has no item
/// in hand to give.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LotDraft {
    pub server_id: Uuid,
    /// Whose lot it is, and who gets paid.
    pub seller: PlayerRef,
    /// The item key: `minecraft:cobblestone`.
    pub item: String,
    /// What it is called in the game. Empty — derived from the key.
    #[serde(default)]
    pub name: String,
    /// The seller's own name for the stack: "a shulker of food".
    #[serde(default)]
    pub custom_name: String,
    #[serde(default)]
    pub description: String,
    /// Items in one pack.
    pub pack_size: i32,
    /// The price of one pack, in the hub's minor units.
    pub pack_price: i64,
    /// How many packs are on offer.
    pub packs: i32,
    /// Repeating the same key returns the same lot instead of a second one.
    #[serde(default)]
    pub idempotency_key: String,
}

/// Filing a claim in court on somebody's behalf.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimDraft {
    pub server_id: Uuid,
    /// Who files it, and answers for it.
    pub plaintiff: PlayerRef,
    /// The kind of claim the hub's court recognises.
    pub kind: String,
    pub title: String,
    pub body: String,
    /// The defendant's game name. Empty when suing a town or an account.
    #[serde(default)]
    pub defendant: String,
    /// The defendant town's address.
    #[serde(default)]
    pub defendant_town: String,
    /// The defendant account's code.
    #[serde(default)]
    pub defendant_account: String,
    /// What is being claimed: chunks, a fine number, an amount.
    #[serde(default)]
    pub subject: serde_json::Value,
}

/// Signing or unsigning a petition, as a named player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetitionVote {
    pub server_id: Uuid,
    pub petition_id: Uuid,
    pub player: PlayerRef,
}

/// An event a module publishes for other modules to handle.
///
/// The name must start with `mod.<your-id>.` — checked on the way out, the same
/// way locale keys are. Without that rule a module could publish
/// `player.banned` and every handler of the real event would believe it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleEvent {
    /// `mod.shop.purchase`, for instance.
    pub name: String,
    /// Whatever the event carries. Its shape is your contract with whoever
    /// subscribes, and the master does not look inside.
    pub payload: serde_json::Value,
    /// The server it concerns, when it concerns one. Modules scoped to a server
    /// only hear events carrying theirs.
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

#[cfg(test)]
#[path = "ops_tests.rs"]
mod tests;
