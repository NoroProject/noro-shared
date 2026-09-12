//! Цикл разработки: пересборка на каждое сохранение.
//!
//! Опрос раз в секунду, а не подписка на события файловой системы. Наблюдатели
//! ведут себя по-разному на macOS и Linux, и оба спотыкаются об атомарное
//! переименование, которым компилятор пишет результат: приходит уведомление о
//! файле, которого в этот момент нет. Опрос скучнее и не врёт.
//!
//! Мастер в dev-режиме читает wasm, мини-апп и локали прямо с диска и
//! перезагружает модуль сам, как только видит новую сборку.

use std::path::Path;
use std::time::{Duration, SystemTime};

use anyhow::Result;

use crate::build;
use crate::project::Project;

/// За чем следим. Вывод сборки сюда не входит намеренно: он меняется от самой
/// сборки, и цикл гонялся бы за собственным хвостом.
const WATCH: [&str; 5] = ["src", "ui", "locales", "manifest.toml", "Cargo.toml"];

pub fn run(p: &Project) -> Result<()> {
    println!("слежу за {} — Ctrl-C чтобы выйти", WATCH.join(", "));
    let mut last = SystemTime::UNIX_EPOCH;

    loop {
        let now = newest(p);
        if now > last {
            last = now;
            // Отказ сборки не прерывает цикл: человек правит код и сохраняет
            // ещё раз, а выход из-за опечатки заставлял бы запускать заново.
            match build::run(p, false) {
                Ok(()) => println!("== пересобрано {}", stamp()),
                Err(e) => eprintln!("!! {e:#}"),
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}

/// Самая свежая правка среди отслеживаемого.
fn newest(p: &Project) -> SystemTime {
    WATCH
        .iter()
        .map(|rel| newest_in(&p.path(rel)))
        .max()
        .unwrap_or(SystemTime::UNIX_EPOCH)
}

fn newest_in(path: &Path) -> SystemTime {
    let Ok(meta) = std::fs::metadata(path) else {
        return SystemTime::UNIX_EPOCH;
    };
    if meta.is_file() {
        return meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    }
    std::fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| newest_in(&e.path()))
                .max()
                .unwrap_or(SystemTime::UNIX_EPOCH)
        })
        .unwrap_or(SystemTime::UNIX_EPOCH)
}

/// Время в выводе — чтобы было видно, что цикл жив, а не завис.
fn stamp() -> String {
    let secs = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!(
        "{:02}:{:02}:{:02}",
        (secs / 3600) % 24,
        (secs / 60) % 60,
        secs % 60
    )
}
