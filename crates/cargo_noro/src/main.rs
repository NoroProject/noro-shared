//! `cargo noro` — создание, сборка и проверка модулей.
//!
//! Инструмент существует ради одной вещи, которой не может дать скрипт: он
//! знает манифест теми же типами, что и мастер. Автор узнаёт об опечатке в
//! возможности до загрузки пакета, а не при первом вызове в проде.
//!
//! Остальное — сборка, упаковка, цикл разработки — раньше было тремя bash-ами в
//! шаблоне. Они работали, но жили в шаблоне: исправление доезжало только до тех,
//! кто создал проект заново.

mod build;
mod check;
mod dev;
mod new;
mod package;
mod project;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "cargo-noro",
    bin_name = "cargo noro",
    version,
    about = "Модули Noro: создать, собрать, проверить"
)]
struct Cli {
    // Cargo зовёт подкоманду как `cargo-noro noro build`, подставляя своё имя
    // первым аргументом. Без этой заглушки clap принял бы его за команду.
    #[arg(hide = true, value_parser = ["noro"])]
    _cargo: Option<String>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Создать новый модуль.
    New {
        /// Идентификатор: `[a-z0-9-]`. Он же имя ящика и префикс ключей.
        id: String,
        /// Человеческое имя. По умолчанию — идентификатор словами.
        #[arg(long)]
        name: Option<String>,
        /// Мини-апп: `vue` — компонент внутри панели, `none` — без экрана.
        #[arg(long, default_value = "vue")]
        ui: new::Ui,
        /// Куда положить. По умолчанию — каталог с именем модуля.
        #[arg(long)]
        path: Option<std::path::PathBuf>,
    },
    /// Проверить манифест, локали и раскладку.
    Check,
    /// Собрать wasm и мини-апп.
    Build {
        /// Собрать отладочно: быстрее, но пакет вчетверо больше.
        #[arg(long)]
        debug: bool,
    },
    /// Собрать и упаковать в `.noromod`.
    Package {
        #[arg(long)]
        debug: bool,
    },
    /// Пересобирать на каждое сохранение — для dev-режима мастера.
    Dev,
}

fn main() {
    if let Err(e) = run() {
        // Цепочка причин целиком: «не собрался пакет» без «zip не пишется в
        // dist/» заставляет искать заново.
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::New { id, name, ui, path } => new::run(&id, name.as_deref(), ui, path),
        Cmd::Check => {
            let p = project::find()?;
            check::run(&p, false)?;
            println!("проверка пройдена");
            Ok(())
        }
        Cmd::Build { debug } => {
            let p = project::find()?;
            check::run(&p, false)?;
            build::run(&p, debug)?;
            Ok(())
        }
        Cmd::Package { debug } => {
            let p = project::find()?;
            check::run(&p, false)?;
            build::run(&p, debug)?;
            // Второй раз уже строго: теперь файлы мини-аппа обязаны быть.
            check::run(&p, true)?;
            package::run(&p, debug)
        }
        Cmd::Dev => {
            let p = project::find()?;
            check::run(&p, false)?;
            dev::run(&p)
        }
    }
}
