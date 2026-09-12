//! Everything here goes across the wasm boundary as JSON, so what is being
//! tested is that it survives the trip.
//!
//! This is not a formality. `PlayerRef` once carried newtype variants and
//! looked perfectly fine: it compiled, and it panicked on the first real call
//! with "cannot serialize tagged newtype variant" — because an internally
//! tagged enum needs its content to be an object. Nothing but a round trip
//! catches that, and it is the exact mistake a new `*Ref` invites.

use super::*;
use crate::player::PlayerRef;
use uuid::Uuid;

/// A round trip through JSON, the way the bridge does it.
fn round<T: serde::Serialize + serde::de::DeserializeOwned>(v: &T) -> T {
    let raw = serde_json::to_string(v).expect("serialized");
    serde_json::from_str(&raw).expect("parsed back")
}

#[test]
fn role_ref_survives_json() {
    let by_name = RoleRef::name("vip");
    assert_eq!(round(&by_name), by_name);

    let id = Uuid::from_u128(7);
    let by_id = RoleRef::id(id);
    assert_eq!(round(&by_id), by_id);

    // The tag is what the master matches on, so its spelling is part of the
    // contract and not an implementation detail.
    let raw = serde_json::to_value(RoleRef::name("vip")).unwrap();
    assert_eq!(raw["by"], "name");
    assert_eq!(raw["name"], "vip");
}

#[test]
fn a_role_is_named_however_you_have_it() {
    let id = Uuid::from_u128(3);
    assert_eq!("vip".into_role_ref(), RoleRef::name("vip"));
    assert_eq!(String::from("vip").into_role_ref(), RoleRef::name("vip"));
    assert_eq!(id.into_role_ref(), RoleRef::id(id));
}

#[test]
fn requests_survive_json() {
    let player = Uuid::from_u128(1);
    let server = Uuid::from_u128(2);

    let grant = RoleGrant {
        player: PlayerRef::id(player),
        role: RoleRef::name("vip"),
    };
    let back = round(&grant);
    assert_eq!(back.player, grant.player);
    assert_eq!(back.role, grant.role);

    // `server_id: None` means "everywhere" and has to come back as None rather
    // than as a missing field the master reads as something else.
    let perm = PermissionOn {
        player: PlayerRef::name("Dalynkaa"),
        node: "noro.module.shop.buy".to_string(),
        server_id: None,
    };
    let back = round(&perm);
    assert_eq!(back.node, perm.node);
    assert_eq!(back.server_id, None);

    let perm = PermissionOn {
        server_id: Some(server),
        ..perm
    };
    assert_eq!(round(&perm).server_id, Some(server));

    let access = ServerAccess {
        player: PlayerRef::discord("123"),
        server_id: server,
    };
    assert_eq!(round(&access).server_id, server);

    let m = Maintenance {
        game_server_id: server,
        enabled: true,
    };
    let back = round(&m);
    assert!(back.enabled);
    assert_eq!(back.game_server_id, server);
}

/// A ban with no reason is a normal ban, not a malformed request.
#[test]
fn a_ban_without_a_reason_parses() {
    let raw = r#"{"player":{"by":"name","name":"Dalynkaa"},"banned":true}"#;
    let req: BanRequest = serde_json::from_str(raw).expect("parsed");
    assert!(req.banned);
    assert_eq!(req.reason, None);
}
