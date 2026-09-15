// This file exceeds 150 lines because it serves as the central CLI definition and dispatcher.
//! `cargo noro` — создание, сборка и проверка модулей.
//!
//! Инструмент существует ради одной вещи, которой не может дать скрипт: он
//! знает манифест теми же типами, что и мастер. Автор узнаёт об опечатке в
//! возможности до загрузки пакета, а не при первом вызове в проде.
//!
//! Остальное — сборка, упаковка, цикл разработки — раньше было тремя bash-ами в
//! шаблоне. Они работали, но жили в шаблоне: исправление доезжало только до тех,
//! кто создал проект заново.

mod add;
mod build;
mod check;
mod dev;
mod new;
mod package;
mod project;
mod sign;
mod ui;

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
enum KeyCmd {
    /// Создать ключ для этого модуля.
    New,
    /// Показать открытую часть — её публикуют рядом с модулем.
    Show,
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
        /// Содержимое: `example` — по образцу каждого объявления, `bare` — пусто.
        #[arg(long, default_value = "example")]
        body: new::Body,
        /// То же, что `--body bare`.
        #[arg(long, conflicts_with = "body")]
        bare: bool,
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
        /// Подписать ключом автора. Без него мастер примет пакет, но обновить
        /// подписанный модуль неподписанным уже не даст.
        #[arg(long)]
        sign: bool,
    },
    /// Ключ автора, которым подписывают пакет.
    Key {
        #[command(subcommand)]
        cmd: KeyCmd,
    },
    /// Пересобирать на каждое сохранение — для dev-режима мастера.
    Dev,
    /// Добавить обработчик события, маршрут, задачу или миграцию.
    Add {
        #[command(subcommand)]
        cmd: add::AddCmd,
    },
    /// Запустить локальную среду разработки UI мини-аппа с моками Noro.
    Ui,
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
        Cmd::New {
            id,
            name,
            ui,
            body,
            bare,
            path,
        } => {
            let body = if bare { new::Body::Bare } else { body };
            new::run(&id, name.as_deref(), ui, body, path)
        }
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
        Cmd::Package { debug, sign } => {
            let p = project::find()?;
            check::run(&p, false)?;
            build::run(&p, debug)?;
            // Второй раз уже строго: теперь файлы мини-аппа обязаны быть.
            check::run(&p, true)?;
            package::run(&p, debug, sign)
        }
        Cmd::Key { cmd } => {
            let id = project::find()?.manifest.module.id;
            match cmd {
                KeyCmd::New => sign::create(&id),
                KeyCmd::Show => sign::show(&id),
            }
        }
        Cmd::Dev => {
            let p = project::find()?;
            check::run(&p, false)?;
            dev::run(&p)
        }
        Cmd::Add { cmd } => {
            let p = project::find()?;
            add::run(&p, cmd)
        }
        Cmd::Ui => {
            let p = project::find()?;
            ui::run(&p)
        }
    }
}
