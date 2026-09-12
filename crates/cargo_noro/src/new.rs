//! Создание модуля из встроенного шаблона.
//!
//! Шаблон вшит в двоичный файл, а не скачивается: команда обязана работать без
//! сети и давать ровно то, что обещала её версия. `cargo noro new` — не клон
//! репозитория, а слепок того, что этот инструмент умеет проверять и собирать.
//!
//! Подстановка — три точных токена, а не движок шаблонов. В `App.vue` полно
//! вставок Vue вида `{{ data.joins }}`, и любой движок подавился бы ими.

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

#[derive(Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Ui {
    /// Компонент, который панель монтирует внутри себя.
    Vue,
    /// Без экрана: события, ручки, задачи.
    None,
}

/// Файлы шаблона: путь в проекте → содержимое.
const CORE: &[(&str, &str)] = &[
    ("manifest.toml", include_str!("../template/manifest.toml")),
    ("Cargo.toml", include_str!("../template/Cargo.toml")),
    ("src/lib.rs", include_str!("../template/src/lib.rs")),
    ("locales/en.ftl", include_str!("../template/locales/en.ftl")),
    ("locales/ru.ftl", include_str!("../template/locales/ru.ftl")),
    ("README.md", include_str!("../template/README.md")),
    (
        "rust-toolchain.toml",
        include_str!("../template/rust-toolchain.toml"),
    ),
    // Имя без точки: пакет с `.gitignore` внутри собственного исходника cargo
    // при публикации выкидывает.
    (".gitignore", include_str!("../template/gitignore")),
];

const VUE: &[(&str, &str)] = &[
    ("package.json", include_str!("../template/package.json")),
    (
        "vite.config.mjs",
        include_str!("../template/vite.config.mjs"),
    ),
    ("ui/app.js", include_str!("../template/ui/app.js")),
    ("ui/App.vue", include_str!("../template/ui/App.vue")),
];

pub fn run(id: &str, name: Option<&str>, ui: Ui, path: Option<PathBuf>) -> Result<()> {
    check_id(id)?;
    let name = name.map(str::to_string).unwrap_or_else(|| title(id));
    let root = path.unwrap_or_else(|| PathBuf::from(id));

    if root.exists() && std::fs::read_dir(&root).is_ok_and(|mut d| d.next().is_some()) {
        bail!("{} существует и не пуст", root.display());
    }

    let mut files: Vec<&(&str, &str)> = CORE.iter().collect();
    if ui == Ui::Vue {
        files.extend(VUE.iter());
    }

    for (rel, body) in files {
        let mut body = body
            .replace("{{id}}", id)
            .replace("{{name}}", &name)
            .replace("{{pascal}}", &pascal(id));
        // Без мини-аппа в манифесте нечего объявлять: раздел `[[apps]]`
        // ссылался бы на файл, которого не будет.
        if ui == Ui::None && *rel == "manifest.toml" {
            body = strip_apps(&body);
        }
        write(&root.join(rel), &body)?;
    }

    println!(
        "создан {} в {}\n\n  cd {}\n  cargo noro package\n",
        name,
        root.display(),
        root.display()
    );
    Ok(())
}

/// Те же правила, что у мастера при установке. Проверяются здесь, а не после
/// первой сборки: идентификатор уезжает в имя ящика и в префикс ключей, и
/// менять его потом — работа в четырёх местах.
fn check_id(id: &str) -> Result<()> {
    let ok = !id.is_empty()
        && id.len() <= 58
        && !id.starts_with('-')
        && !id.ends_with('-')
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    if !ok {
        bail!(
            "`{id}` не годится в идентификаторы: строчные латинские буквы, \
             цифры и дефис, не длиннее 58 символов, не с дефиса по краям"
        );
    }
    Ok(())
}

fn write(path: &Path, body: &str) -> Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).with_context(|| format!("не создаётся {}", dir.display()))?;
    }
    std::fs::write(path, body).with_context(|| format!("не пишется {}", path.display()))
}

/// Убирает раздел `[[apps]]` вместе с объясняющим его комментарием.
///
/// Комментарий приходится забирать откатом назад: в TOML он стоит **перед**
/// разделом, и к моменту, когда становится ясно, что раздел лишний, он уже
/// выведен. Оставленный без раздела, он объяснял бы то, чего в файле нет.
fn strip_apps(manifest: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    let mut skipping = false;

    for line in manifest.lines() {
        let head = line.trim_start();
        if head.starts_with("[[apps]]") {
            skipping = true;
            while out
                .last()
                .is_some_and(|l| l.trim_start().starts_with('#') || l.trim().is_empty())
            {
                out.pop();
            }
            continue;
        }
        if skipping && head.starts_with('[') {
            skipping = false;
        }
        if !skipping {
            out.push(line);
        }
    }

    let mut text = out.join("\n");
    text.push('\n');
    text
}

/// `playtime-rewards` → `Playtime Rewards`.
fn title(id: &str) -> String {
    id.split('-').map(capitalise).collect::<Vec<_>>().join(" ")
}

/// `playtime-rewards` → `PlaytimeRewards`.
fn pascal(id: &str) -> String {
    id.split('-').map(capitalise).collect()
}

fn capitalise(word: &str) -> String {
    let mut c = word.chars();
    match c.next() {
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
#[path = "new_tests.rs"]
mod tests;
