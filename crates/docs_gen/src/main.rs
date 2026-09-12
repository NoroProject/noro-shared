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

/// The groups, in the order they appear on the page. Named rather than derived
/// from the data so a new group has to be given a heading deliberately — an
/// unnamed one shows up under the fallback title and gets noticed.
const GROUPS: &[(&str, &str)] = &[
    ("player", "Players and signing in"),
    ("access", "Roles, permissions, access"),
    ("infra", "Servers, builds, game servers"),
    ("moderation", "Moderation"),
    ("economy", "Economy and the hub"),
];

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

    fs::create_dir_all(dir).expect("create the reference directory");
    fs::write(dir.join("events.md"), events_page()).expect("write events.md");

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

fn events_page() -> String {
    let mut page = String::from(
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
         :::caution[Pre events are not delivered yet]\n\
         The master currently publishes `Post` events only. A subscription to a `Pre` event\n\
         compiles and installs, but the handler never runs — see\n\
         [Events](../../guides/events/).\n\
         :::\n",
    );

    for (group, title) in GROUPS {
        let rows: Vec<_> = ALL_EVENTS.iter().filter(|e| e.group == *group).collect();
        if rows.is_empty() {
            continue;
        }

        let _ = write!(
            page,
            "\n## {title}\n\n| Event | Kind | Payload |\n|---|---|---|\n"
        );
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
        .filter(|e| !GROUPS.iter().any(|(g, _)| *g == e.group))
        .collect();
    if !unlisted.is_empty() {
        let _ = write!(page, "\n## Ungrouped\n\n| Event | Group |\n|---|---|\n");
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
