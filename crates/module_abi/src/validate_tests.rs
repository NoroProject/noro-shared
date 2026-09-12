use super::*;
use crate::manifest::{Capabilities, ModuleMeta, PermissionDecl, Placement, Scope};

fn manifest(id: &str) -> Manifest {
    Manifest {
        module: ModuleMeta {
            id: id.to_string(),
            name: "Тест".to_string(),
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

/// Идентификатор едет в имя схемы Postgres и в URL. Пропустить сюда точку или
/// заглавную букву — значит сломаться позже и в чужом месте.
#[test]
fn the_id_is_restricted_to_schema_safe_characters() {
    for bad in ["Shop", "my.shop", "my shop", "-shop", "shop-"] {
        assert!(
            !violations(&manifest(bad)).is_empty(),
            "идентификатор «{bad}» обязан быть отвергнут"
        );
    }
}

/// Чужая ветка прав дала бы модулю подсказку на право, которого он не заводил.
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

/// Виджет без слота некуда поставить.
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

/// Выход за пределы `web/` — единственный способ вытащить из пакета чужой файл.
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
    // Ноль означал бы задачу, которая будит модуль непрерывно.
    assert_eq!(parse_every("0s"), None);
    assert_eq!(parse_every("часик"), None);
    assert_eq!(parse_every("5"), None);
}

#[test]
fn only_the_major_of_the_abi_has_to_match() {
    assert!(api_compatible("1.0", "1.4"));
    assert!(api_compatible("1.7", "1.0"));
    assert!(!api_compatible("2.0", "1.0"));
    assert!(!api_compatible("", "1.0"));
}

/// Пустой набор возможностей закрыт целиком: забытая строка в манифесте не
/// должна незаметно открывать домен.
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
    // Сам суффикс без поддомена — не совпадение: иначе `*.example.org`
    // открывал бы и `example.org`, который автор не называл.
    assert!(!caps.allows_host("example.org"));
    assert!(!caps.allows_host("evil-discord.com"));
}
