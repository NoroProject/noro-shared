use super::*;
use crate::manifest::{Capabilities, ModuleMeta, PermissionDecl, Placement, Scope};

fn manifest(id: &str) -> Manifest {
    Manifest {
        module: ModuleMeta {
            id: id.to_string(),
            name: "Test".to_string(),
            version: "1.0.0".to_string(),
            api: "1.0".to_string(),
            scope: Scope::Instance,
            author: None,
            description: None,
            homepage: None,
        },
        capabilities: Capabilities::default(),
        apps: Vec::new(),
        permissions: Vec::new(),
    }
}

#[test]
fn a_plain_manifest_passes() {
    assert!(violations(&manifest("shop")).is_empty());
}

/// The identifier travels into a Postgres schema name and into URLs. Letting a
/// dot or a capital letter through here means breaking later, somewhere else.
#[test]
fn the_id_is_restricted_to_schema_safe_characters() {
    for bad in ["Shop", "my.shop", "my shop", "-shop", "shop-"] {
        assert!(
            !violations(&manifest(bad)).is_empty(),
            "the identifier `{bad}` must be rejected"
        );
    }
}

/// Somebody else's permission branch would hint a module at a permission it never introduced.
#[test]
fn permission_nodes_stay_in_the_modules_own_branch() {
    let mut m = manifest("shop");
    m.permissions.push(PermissionDecl {
        node: "noro.admin.users.ban".to_string(),
        label: "mod-shop-perm".to_string(),
    });
    assert_eq!(violations(&m).len(), 1);

    m.permissions[0].node = "noro.module.shop.view".to_string();
    assert!(violations(&m).is_empty());
}

/// A widget with no slot has nowhere to go.
#[test]
fn a_widget_needs_a_slot() {
    let mut m = manifest("shop");
    m.apps.push(crate::manifest::AppDecl {
        placement: Placement::Widget,
        kind: crate::manifest::AppKind::Page,
        entry: "widget.html".to_string(),
        title: "mod-shop-title".to_string(),
        icon: None,
        permission: None,
        slot: None,
    });
    assert_eq!(violations(&m).len(), 1);
}

/// Escaping `web/` is the only way to pull a foreign file out of the package.
#[test]
fn mini_app_entries_cannot_escape_the_package() {
    let mut m = manifest("shop");
    m.apps.push(crate::manifest::AppDecl {
        placement: Placement::Admin,
        kind: crate::manifest::AppKind::Page,
        entry: "../../etc/passwd".to_string(),
        title: "t".to_string(),
        icon: None,
        permission: None,
        slot: None,
    });
    assert_eq!(violations(&m).len(), 1);
}

#[test]
fn intervals_are_parsed() {
    assert_eq!(parse_every("30s"), Some(30));
    assert_eq!(parse_every("5m"), Some(300));
    assert_eq!(parse_every("1h"), Some(3600));
    assert_eq!(parse_every("2d"), Some(172_800));
    // Zero would mean a task that wakes the module without pause.
    assert_eq!(parse_every("0s"), None);
    assert_eq!(parse_every("an hour or so"), None);
    assert_eq!(parse_every("5"), None);
}

#[test]
fn only_the_major_of_the_abi_has_to_match() {
    assert!(api_compatible("1.0", "1.4"));
    assert!(api_compatible("1.7", "1.0"));
    assert!(!api_compatible("2.0", "1.0"));
    assert!(!api_compatible("", "1.0"));
}

/// An empty capability set is closed entirely: a forgotten line in the
/// manifest must not quietly open a domain.
#[test]
fn capabilities_default_to_closed() {
    let caps = Capabilities::default();
    assert!(!caps.allows("players", "read"));
    assert!(!caps.allows("store", ""));
    assert!(!caps.allows_host("discord.com"));

    let caps = Capabilities {
        players: vec!["read".to_string()],
        store: true,
        http: vec!["discord.com".to_string(), "*.example.org".to_string()],
        ..Default::default()
    };
    assert!(caps.allows("players", "read"));
    assert!(!caps.allows("players", "ban"));
    assert!(caps.allows("store", ""));
    assert!(caps.allows_host("discord.com"));
    assert!(caps.allows_host("api.example.org"));
    // The suffix alone, with no subdomain, is not a match: otherwise
    // `*.example.org` would also open `example.org`, which the author never
    // named.
    assert!(!caps.allows_host("example.org"));
    assert!(!caps.allows_host("evil-discord.com"));
}
