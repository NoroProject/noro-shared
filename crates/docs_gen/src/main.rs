//! Generates the reference pages of the documentation site from the ABI.
//!
//! The event catalog and the capability list exist in code already — as
//! `ALL_EVENTS` and as the fields of `Capabilities`. Retyping them into Markdown
//! would create a second source that silently falls behind: an event added to
//! the registry would simply be missing from the docs, and nothing would fail.
//!
//! So the pages are built here instead. The site's build runs this first, which
//! makes a stale catalog impossible rather than merely unlikely.
//!
//! ```text
//! cargo run -p noro-docs-gen -- docs/src/content/docs/reference
//! ```

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use noro_module_abi::events::{EventKind, ALL_EVENTS};

mod domains;
mod pairs;

/// The groups, in the order they appear on the page. Named rather than derived
/// from the data so a new group has to be given a heading deliberately — an
/// unnamed one shows up under the fallback title and gets noticed.
const GROUPS: &[(&str, &str, &str)] = &[
    ("player", "Players and signing in", "Игроки и вход"),
    (
        "access",
        "Roles, permissions, access",
        "Роли, права, доступы",
    ),
    (
        "infra",
        "Servers, builds, game servers",
        "Сборки, билды, игровые серверы",
    ),
    ("moderation", "Moderation", "Модерация"),
    ("economy", "Economy and the hub", "Экономика и подсайт"),
];

/// Язык страницы. Таблицы у них общие — имена событий и сигнатуры не
/// переводятся, — а различается только то, что вокруг.
#[derive(Clone, Copy, PartialEq)]
pub enum Lang {
    En,
    Ru,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let out = args.next().unwrap_or_else(|| {
        eprintln!("usage: noro-docs-gen <reference-dir> [rustdoc-root]");
        std::process::exit(2);
    });
    let dir = Path::new(&out);

    // Given the built rustdoc, every link is checked against it. A page full of
    // links into a reference nobody verified is worse than no links at all:
    // they look authoritative and lead nowhere.
    if let Some(root) = args.next() {
        let missing = check_links(Path::new(&root));
        if !missing.is_empty() {
            eprintln!(
                "{} event links point at missing rustdoc pages:",
                missing.len()
            );
            for m in &missing {
                eprintln!("  {m}");
            }
            std::process::exit(1);
        }
        println!("verified {} event links against {root}", ALL_EVENTS.len());
    }

    // Русская половина живёт в `ru/`, там же, где её ждёт Starlight. Каталог
    // создаётся здесь, а не руками: страница генерируемая, и пустой каталог в
    // репозитории только вводил бы в заблуждение.
    let ru = dir
        .parent()
        .map(|d| d.join("ru/reference"))
        .unwrap_or_else(|| dir.join("ru"));

    fs::create_dir_all(dir).expect("create the reference directory");
    fs::create_dir_all(&ru).expect("create the russian reference directory");
    fs::write(dir.join("events.md"), events_page(Lang::En)).expect("write events.md");
    fs::write(ru.join("events.md"), events_page(Lang::Ru)).expect("write ru events.md");

    // Справочник доменов — из исходников SDK. Путь относительный от корня
    // репозитория: генератор запускается оттуда же, откуда собирается сайт.
    let sdk = Path::new("crates/sdk/src");
    let found = domains::read(sdk);
    if found.is_empty() {
        eprintln!("не найдено ни одного домена в {}", sdk.display());
        std::process::exit(1);
    }
    // Полнота перевода — после генерации: сгенерированные страницы тоже
    // ложатся парами, и считать до них значило бы ругаться на них самих.
    let docs = dir
        .ancestors()
        .find(|p| p.join("astro.config.mjs").exists())
        .unwrap_or(Path::new("docs"));
    let untranslated = pairs::missing(docs, "ru");
    if !untranslated.is_empty() {
        eprintln!("{} страниц без перевода на ru:", untranslated.len());
        for p in &untranslated {
            eprintln!("  {p}");
        }
        eprintln!(
            "положите перевод в docs/src/content/docs/ru/ — иначе сайт обещает язык, \
             которого у половины страниц нет"
        );
        std::process::exit(1);
    }

    let calls: usize = found.iter().map(|d| d.calls.len()).sum();
    let facades = domains::facades(sdk);
    for (path, lang) in [
        (dir.join("sdk.md"), Lang::En),
        (ru.join("sdk.md"), Lang::Ru),
    ] {
        let page = domains::page(&found, lang) + &domains::facade_section(&facades, lang);
        fs::write(path, page).expect("write sdk.md");
    }
    println!("generated {} calls across {} domains", calls, found.len());

    println!(
        "generated {} events into {}",
        ALL_EVENTS.len(),
        dir.display()
    );
}

/// The rustdoc pages named by the catalog that do not exist.
fn check_links(root: &Path) -> Vec<String> {
    ALL_EVENTS
        .iter()
        .map(|e| {
            root.join("noro_module_abi/events")
                .join(e.group)
                .join(format!("struct.{}.html", e.payload))
        })
        .filter(|p| !p.exists())
        .map(|p| p.display().to_string())
        .collect()
}

fn events_page(lang: Lang) -> String {
    let mut page = String::from(match lang {
        Lang::En => {
            "---\n\
         title: Event catalog\n\
         description: Every event a module can subscribe to, generated from the ABI.\n\
         ---\n\n\
         :::note\n\
         This page is generated from `ALL_EVENTS` in `noro-module-abi` when the site is\n\
         built. It cannot fall behind the code, and editing it by hand has no effect.\n\
         :::\n\n\
         A handler names no event by string. `#[event]` takes the name from the type of\n\
         its argument, so the **Payload** column is also what you subscribe with:\n\n\
         ```rust\n\
         #[event]\n\
         fn on_join(e: PlayerJoined) -> Result<()> { Ok(()) }\n\
         ```\n\n\
         `Post` events run after everything has happened and been written. `Pre` events run\n\
         before the action and are meant to cancel or change it.\n\n\
         :::note[Pre events]\n\
         A `Pre` handler is declared by taking its event as `&mut`: the reference is what\n\
         tells the master your answer is worth waiting for. All of them are delivered; the\n\
         two with conditions are described in [Events](../../guides/events/).\n\
         :::\n"
        }
        Lang::Ru => {
            "---\n\
         title: Каталог событий\n\
         description: Все события, на которые можно подписаться. Собирается из ABI.\n\
         ---\n\n\
         :::note\n\
         Страница собирается из `ALL_EVENTS` в `noro-module-abi` при сборке сайта. Отстать\n\
         от кода она не может, а править её руками бесполезно.\n\
         :::\n\n\
         Обработчик не называет событие строкой. `#[event]` берёт имя из типа аргумента —\n\
         значит, столбец **Структура** и есть то, чем подписываются:\n\n\
         ```rust\n\
         #[event]\n\
         fn on_join(e: PlayerJoined) -> Result<()> { Ok(()) }\n\
         ```\n\n\
         `Post` приходит после того, как всё случилось и записано. `Pre` — до действия, и\n\
         существует ради того, чтобы его отменить или изменить.\n\n\
         :::note[Про Pre]\n\
         Обработчик `Pre` объявляется тем, что берёт событие по `&mut`: ссылка и говорит\n\
         мастеру, что ответа стоит подождать. Доставляются все; у двух есть условия, и они\n\
         описаны в [Событиях](../../guides/events/).\n\
         :::\n"
        }
    });

    for (group, title_en, title_ru) in GROUPS {
        let title = if lang == Lang::Ru { title_ru } else { title_en };
        let rows: Vec<_> = ALL_EVENTS.iter().filter(|e| e.group == *group).collect();
        if rows.is_empty() {
            continue;
        }

        let head = match lang {
            Lang::En => "| Event | Kind | Payload |",
            Lang::Ru => "| Событие | Вид | Структура |",
        };
        let _ = write!(page, "\n## {title}\n\n{head}\n|---|---|---|\n");
        for e in rows {
            let kind = match e.kind {
                EventKind::Pre => "`Pre`",
                EventKind::Post => "`Post`",
            };
            let _ = writeln!(
                page,
                "| `{}` | {kind} | [`{}`]({}) |",
                e.name,
                e.payload,
                rustdoc(e.group, e.payload)
            );
        }
    }

    let unlisted: Vec<_> = ALL_EVENTS
        .iter()
        .filter(|e| !GROUPS.iter().any(|(g, _, _)| *g == e.group))
        .collect();
    if !unlisted.is_empty() {
        let head = match lang {
            Lang::En => "## Ungrouped\n\n| Event | Group |",
            Lang::Ru => "## Без группы\n\n| Событие | Группа |",
        };
        let _ = write!(page, "\n{head}\n|---|---|\n");
        for e in unlisted {
            let _ = writeln!(page, "| `{}` | `{}` |", e.name, e.group);
        }
    }

    page
}

/// Deep link into the published rustdoc. Relative, so it survives whatever the
/// site is hosted under.
///
/// Both halves come from the catalog rather than being derived from the event
/// name: the struct name disagrees with it nine times out of fifty-one, and
/// rustdoc files the struct under the module it is declared in — which is the
/// group.
fn rustdoc(group: &str, payload: &str) -> String {
    format!("../../api/noro_module_abi/events/{group}/struct.{payload}.html")
}
