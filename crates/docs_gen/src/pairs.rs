//! Сторож полноты перевода.
//!
//! Starlight страницу без перевода не прячет: отдаёт её на языке по умолчанию и
//! помечает. Это верное поведение — ненайденная страница выглядела бы как
//! отсутствующая возможность, — но означает, что забытый перевод ничего не
//! ломает и потому не находится.
//!
//! Поэтому сборка падает на нём. Не «когда-нибудь допереведём», а сейчас: на
//! полпути документация врёт про то, какие языки у неё есть.

use std::collections::BTreeSet;
use std::path::Path;

/// Страницы, которые не переводятся вовсе. Пусто: пока таких нет, но список
/// нужен — иначе исключение пришлось бы прятать в условие.
const UNTRANSLATED: &[&str] = &[];

/// Страницы английской половины, у которых нет русской пары.
pub fn missing(docs: &Path, locale: &str) -> Vec<String> {
    let root = docs.join("src/content/docs");
    let english = pages(&root, Some(locale));
    let translated: BTreeSet<String> = pages(&root.join(locale), None).into_iter().collect();

    english
        .into_iter()
        .filter(|p| !UNTRANSLATED.contains(&p.as_str()))
        .filter(|p| !translated.contains(p))
        .collect()
}

/// Пути страниц относительно корня, с расширением.
///
/// `skip` — каталог локали: обходя английскую половину, её надо пропустить, а
/// обходя саму локаль — нет.
fn pages(dir: &Path, skip: Option<&str>) -> Vec<String> {
    let mut out = Vec::new();
    walk(dir, dir, skip, &mut out);
    out.sort();
    out
}

fn walk(root: &Path, dir: &Path, skip: Option<&str>, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();

        if path.is_dir() {
            if skip.is_some_and(|s| s == name) {
                continue;
            }
            walk(root, &path, skip, out);
            continue;
        }
        let ext = path.extension().unwrap_or_default().to_string_lossy();
        if ext == "md" || ext == "mdx" {
            if let Ok(rel) = path.strip_prefix(root) {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
    }
}
