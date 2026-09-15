//! Тесты добавления компонентов модуля через `cargo noro add`.

use super::*;
use crate::new;

fn test_project(name: &str) -> (Project, std::path::PathBuf) {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/cargo-noro-tests")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);

    new::run(
        "test-mod",
        None,
        new::Ui::None,
        new::Body::Bare,
        Some(dir.clone()),
    )
    .expect("шаблон разворачивается");

    let raw = std::fs::read_to_string(dir.join("manifest.toml")).expect("манифест читается");
    let manifest = toml::from_str(&raw).expect("манифест парсится");

    (
        Project {
            root: dir.clone(),
            manifest,
        },
        dir,
    )
}

#[test]
fn add_event_generates_correct_handler() {
    let (p, dir) = test_project("add-event");

    // 1. Post-событие
    run(
        &p,
        AddCmd::Event {
            name: "player.joined".to_string(),
            priority: "normal".to_string(),
        },
    )
    .expect("событие добавляется");

    // 2. Pre-событие с приоритетом
    run(
        &p,
        AddCmd::Event {
            name: "user.pre_login".to_string(),
            priority: "high".to_string(),
        },
    )
    .expect("pre-событие добавляется");

    // 3. Web-сообщение
    run(
        &p,
        AddCmd::Event {
            name: "web.message".to_string(),
            priority: "normal".to_string(),
        },
    )
    .expect("web.message добавляется");

    let lib = std::fs::read_to_string(dir.join("src/lib.rs")).expect("lib.rs читается");

    assert!(lib.contains("#[event]"));
    assert!(lib.contains("fn on_player_joined(e: PlayerJoined) -> Result<()>"));
    assert!(lib.contains("#[event(priority = high)]"));
    assert!(lib.contains("fn on_user_pre_login(e: &mut UserPreLogin) -> Result<()>"));
    assert!(lib.contains("fn on_web_message(e: WebMessage) -> Result<()>"));
}

#[test]
fn add_route_generates_correct_endpoint() {
    let (p, dir) = test_project("add-route");

    run(
        &p,
        AddCmd::Route {
            method: "get".to_string(),
            path: "/api/stats".to_string(),
        },
    )
    .expect("маршрут добавляется");

    let lib = std::fs::read_to_string(dir.join("src/lib.rs")).expect("lib.rs читается");

    assert!(lib.contains("#[route(GET, \"/api/stats\")]"));
    assert!(lib.contains("fn get_api_stats(_req: HttpRequest) -> Result<HttpResponse>"));
}

#[test]
fn add_task_generates_scheduled_task() {
    let (p, dir) = test_project("add-task");

    run(
        &p,
        AddCmd::Task {
            name: "cleanup_cache".to_string(),
            every: "30m".to_string(),
        },
    )
    .expect("задача добавляется");

    let lib = std::fs::read_to_string(dir.join("src/lib.rs")).expect("lib.rs читается");

    assert!(lib.contains("#[task(\"30m\")]"));
    assert!(lib.contains("fn cleanup_cache() -> Result<()>"));
}

#[test]
fn add_migration_sequences_correctly() {
    let (p, dir) = test_project("add-migration");

    run(
        &p,
        AddCmd::Migration {
            name: "create_users".to_string(),
        },
    )
    .expect("первая миграция создана");

    run(
        &p,
        AddCmd::Migration {
            name: "add_roles".to_string(),
        },
    )
    .expect("вторая миграция создана");

    assert!(dir.join("migrations/0001_create_users.sql").exists());
    assert!(dir.join("migrations/0002_add_roles.sql").exists());

    let m1 = std::fs::read_to_string(dir.join("migrations/0001_create_users.sql")).unwrap();
    assert!(m1.contains("mod_test-mod_create_users"));
}

#[test]
fn ui_testbed_creates_index_html() {
    let (p, dir) = test_project("ui-testbed");

    assert!(!dir.join("index.html").exists());
    let created = crate::ui::ensure_index_html(&p).expect("index.html создан");
    assert!(created);
    assert!(dir.join("index.html").exists());

    let html = std::fs::read_to_string(dir.join("index.html")).unwrap();
    assert!(html.contains("window.__noroUi"));
    assert!(html.contains("window.__noroVue"));
}
