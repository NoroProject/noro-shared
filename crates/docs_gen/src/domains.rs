//! Справочник доменов SDK, собранный из исходников самого SDK.
//!
//! Список функций, переписанный в Markdown руками, отстаёт молча: домен
//! получает новый вызов, документация о нём не узнаёт, и не падает ничего.
//! Поэтому страница строится по `crates/sdk/src/*.rs` — по тем же файлам, из
//! которых rustdoc делает справочник.
//!
//! Разбор строковый, а не через `syn`: нужны сигнатура, первая строка
//! документации и строка «Requires», то есть ровно то, что видно глазом. Полное
//! дерево разбора ради трёх полей — это зависимость, которую пришлось бы
//! обновлять вместе с языком.

use std::fmt::Write as _;
use std::path::Path;

/// Домен целиком.
pub struct Domain {
    pub name: String,
    /// Первая строка `//!` — чем домен занимается.
    pub summary: String,
    pub calls: Vec<Call>,
}

pub struct Call {
    /// Сигнатура без `pub fn`, как её видит автор.
    pub signature: String,
    pub summary: String,
    /// Что нужно выдать, из строки «Requires …».
    pub requires: Option<String>,
}

/// Файлы, которые в справочник не идут: у них нет домена в смысле возможностей.
const SKIP: &[&str] = &["lib", "host", "log"];

pub fn read(sdk_src: &Path) -> Vec<Domain> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(sdk_src) else {
        return out;
    };
    let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    paths.sort();

    for path in paths {
        let stem = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if path.extension().is_none_or(|e| e != "rs") || SKIP.contains(&stem.as_str()) {
            continue;
        }
        let Ok(src) = std::fs::read_to_string(&path) else {
            continue;
        };
        let calls = calls(&src);
        if calls.is_empty() {
            continue;
        }
        out.push(Domain {
            name: stem,
            summary: module_summary(&src),
            calls,
        });
    }
    out
}

/// Первая содержательная строка `//!`.
fn module_summary(src: &str) -> String {
    src.lines()
        .take_while(|l| l.starts_with("//!") || l.trim().is_empty())
        .filter_map(|l| l.strip_prefix("//!"))
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or_default()
        .to_string()
}

fn calls(src: &str) -> Vec<Call> {
    let mut out = Vec::new();
    let lines: Vec<&str> = src.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let Some(rest) = line.strip_prefix("pub fn ") else {
            continue;
        };
        let Some((name, tail)) = rest.split_once('(') else {
            continue;
        };

        // Документация — подряд идущие `///` над объявлением.
        let mut doc: Vec<&str> = Vec::new();
        for prev in lines[..i].iter().rev() {
            match prev.trim_start().strip_prefix("///") {
                Some(text) => doc.push(text.trim()),
                None => break,
            }
        }
        doc.reverse();

        out.push(Call {
            signature: signature(name, tail, &lines[i..]),
            summary: doc
                .iter()
                .find(|l| !l.is_empty())
                .unwrap_or(&"")
                .to_string(),
            requires: doc
                .iter()
                .find_map(|l| l.strip_prefix("Requires "))
                .map(|r| r.trim_end_matches('.').to_string()),
        });
    }
    out
}

/// Сигнатура одной строкой: многострочные аргументы схлопываются.
fn signature(name: &str, first_tail: &str, rest: &[&str]) -> String {
    let mut text = format!("{name}({first_tail}");
    if text.contains(')') {
        return tidy(&text);
    }
    for line in rest.iter().skip(1) {
        text.push(' ');
        text.push_str(line.trim());
        if line.contains(')') {
            break;
        }
    }
    tidy(&text)
}

fn tidy(text: &str) -> String {
    let body = text.split_once(" {").map(|(s, _)| s).unwrap_or(text);
    let one_line = body.split_whitespace().collect::<Vec<_>>().join(" ");
    // Следы переноса: у многострочного объявления после `(` и перед `)`
    // остаётся пробел, а перед `)` ещё и запятая. В таблице это читается как
    // опечатка автора SDK.
    one_line
        .replace("( ", "(")
        .replace(", )", ")")
        .replace(" )", ")")
}

pub fn page(domains: &[Domain], lang: crate::Lang) -> String {
    let mut page = String::from(match lang {
        crate::Lang::En => {
            "---\n\
         title: SDK domains\n\
         description: Every call the SDK offers, with what it needs granted.\n\
         ---\n\n\
         :::note\n\
         Generated from the SDK's own sources when the site is built. It cannot fall\n\
         behind the code, and editing it by hand has no effect.\n\
         :::\n\n\
         Each domain is one module of `noro_sdk`, and one line in `[capabilities]`. An\n\
         action the operator withheld answers `CapabilityDenied` naming what is missing;\n\
         it does not crash your module.\n\n\
         Full signatures, types and the longer explanations are in the\n\
         [rustdoc](/noro-shared/api/noro_sdk/).\n"
        }
        crate::Lang::Ru => {
            "---\n\
         title: Домены SDK\n\
         description: Все вызовы SDK и то, что для каждого нужно выдать.\n\
         ---\n\n\
         :::note\n\
         Собирается из исходников самого SDK при сборке сайта. Отстать от кода не может,\n\
         а править руками бесполезно.\n\
         :::\n\n\
         Домен — это один модуль `noro_sdk` и одна строка в `[capabilities]`. Действие,\n\
         которое оператор не выдал, отвечает `CapabilityDenied` с именем недостающего —\n\
         модуль от этого не падает.\n\n\
         Полные сигнатуры, типы и подробности — в\n\
         [rustdoc](/noro-shared/api/noro_sdk/). Описания там на английском: они живут\n\
         вместе с кодом, а он публичный.\n"
        }
    });

    for d in domains {
        let _ = write!(page, "\n## `{}`\n\n", d.name);
        if !d.summary.is_empty() {
            let _ = writeln!(page, "{}\n", d.summary);
        }
        let head = match lang {
            crate::Lang::En => "| Call | What it does | Needs |",
            crate::Lang::Ru => "| Вызов | Что делает | Нужно |",
        };
        let _ = write!(page, "{head}\n|---|---|---|\n");
        for c in &d.calls {
            let _ = writeln!(
                page,
                "| `{}` | {} | {} |",
                c.signature,
                escape(&c.summary),
                c.requires.as_deref().unwrap_or("—")
            );
        }
    }
    page
}

/// Вертикальная черта в описании разорвала бы строку таблицы.
fn escape(text: &str) -> String {
    text.replace('|', "\\|")
}
