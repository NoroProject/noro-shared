//! Проверки, которые иначе достались бы мастеру.
//!
//! Ради них инструмент и существует. Опечатка в возможности, ключ локали без
//! префикса, мини-апп без файла — всё это мастер находит при установке, то есть
//! после сборки, загрузки и клика. Здесь это видно за секунду.
//!
//! Правила манифеста не дублируются: их знает `noro_module_abi::validate`, тот
//! же код, которым проверяет мастер. Здесь только то, для чего нужен диск.

use std::collections::BTreeSet;

use anyhow::{bail, Result};
use noro_module_abi::manifest::ALL_CAPABILITIES;
use noro_module_abi::{validate, ABI_VERSION};

use crate::project::Project;

/// `strict` — упаковка. До неё мини-апп ещё не собран, и требовать его файлы
/// значило бы ронять проверку на только что созданном проекте; в пакете же их
/// отсутствие — настоящая поломка, мастер такой манифест отвергнет.
pub fn run(p: &Project, strict: bool) -> Result<()> {
    let mut bad = validate::violations(&p.manifest);
    bad.extend(capabilities(p));
    bad.extend(api(p));
    bad.extend(locales(p));
    bad.extend(migrations(p));

    let missing = apps(p);
    if strict {
        bad.extend(missing);
    } else {
        for note in missing {
            println!("· {note}");
        }
    }

    if bad.is_empty() {
        return Ok(());
    }
    bail!(
        "манифест не пройдёт установку:\n{}",
        bad.iter()
            .map(|b| format!("  · {b}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Домен есть, действия нет. Молчаливый отказ на первом же вызове.
fn capabilities(p: &Project) -> Vec<String> {
    let asked = serde_json::to_value(&p.manifest.capabilities).unwrap_or_default();
    let mut out = Vec::new();

    for (domain, actions) in ALL_CAPABILITIES {
        let Some(list) = asked.get(domain).and_then(|v| v.as_array()) else {
            continue;
        };
        for action in list.iter().filter_map(|v| v.as_str()) {
            if !actions.contains(&action) {
                out.push(format!(
                    "возможность `{domain} = [\"{action}\"]`: такого действия нет. Есть: {}",
                    actions.join(", ")
                ));
            }
        }
    }
    out
}

/// Требуемая версия ABI против той, с которой собран сам инструмент.
fn api(p: &Project) -> Vec<String> {
    if validate::api_compatible(&p.manifest.module.api, ABI_VERSION) {
        return Vec::new();
    }
    vec![format!(
        "манифест требует ABI {}, а этот `cargo noro` собран с {ABI_VERSION}: \
         мастер той же версии модуль не примет",
        p.manifest.module.api
    )]
}

/// Ключи локалей: префикс обязателен, и то, на что ссылается манифест, должно
/// существовать хотя бы в одном языке.
fn locales(p: &Project) -> Vec<String> {
    let prefix = format!("mod-{}-", p.manifest.module.id);
    let mut out = Vec::new();
    let mut defined = BTreeSet::new();

    let dir = p.path("locales");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return out;
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "ftl") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let file = path.file_name().unwrap_or_default().to_string_lossy();
        for line in text.lines() {
            // Ключ — начало строки до `=`; продолжения и комментарии отступлены
            // или начинаются с `#`.
            let Some((key, _)) = line.split_once('=') else {
                continue;
            };
            if line.starts_with([' ', '\t', '#', '[', '.', '*']) {
                continue;
            }
            let key = key.trim();
            if key.is_empty() {
                continue;
            }
            defined.insert(key.to_string());
            if !key.starts_with(&prefix) {
                out.push(format!(
                    "{file}: ключ `{key}` обязан начинаться с `{prefix}` — \
                     иначе он столкнётся с ключом панели или другого модуля"
                ));
            }
        }
    }

    // Ссылки из манифеста. Пустой каталог локалей не проверяем: модуль без
    // своих текстов — нормальный случай.
    if !defined.is_empty() {
        for app in &p.manifest.apps {
            if !defined.contains(&app.title) {
                out.push(format!(
                    "мини-апп ссылается на ключ `{}`, которого нет в locales/",
                    app.title
                ));
            }
        }
        for perm in &p.manifest.permissions {
            if !defined.contains(&perm.label) {
                out.push(format!(
                    "право `{}` ссылается на ключ `{}`, которого нет в locales/",
                    perm.node, perm.label
                ));
            }
        }
    }
    out
}

/// Точка входа мини-аппа должна существовать после сборки.
fn apps(p: &Project) -> Vec<String> {
    p.manifest
        .apps
        .iter()
        .filter(|a| !p.has(&format!("web/{}", a.entry)))
        .map(|a| {
            format!(
                "мини-апп `{}`: нет web/{} — он появляется при сборке",
                a.title, a.entry
            )
        })
        .collect()
}

/// Миграции: номер в начале имени и без повторов.
fn migrations(p: &Project) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(p.path("migrations")) else {
        return Vec::new();
    };
    let mut seen: BTreeSet<i64> = BTreeSet::new();
    let mut out = Vec::new();

    for entry in entries.filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".sql") {
            continue;
        }
        let digits: String = name.chars().take_while(|c| c.is_ascii_digit()).collect();
        match digits.parse::<i64>() {
            Ok(n) if !seen.insert(n) => out.push(format!(
                "две миграции с номером {n}: порядок между ними ничем не задан"
            )),
            Ok(_) => {}
            Err(_) => out.push(format!(
                "миграция `{name}` без номера в начале имени: \
                 порядок накатывания берётся из него"
            )),
        }
    }
    out
}
