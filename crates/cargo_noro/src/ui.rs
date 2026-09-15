//! Локальная среда разработки UI мини-аппа с эмуляцией Noro Web Panel.
//!
//! Запускает Vite dev-сервер с моком `window.__noroUi`, эмуляцией WebSocket
//! (`noro.ws`) и поддержкой HMR без необходимости поднимать мастер-сервер.

use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::project::Project;

pub fn run(p: &Project) -> Result<()> {
    if !p.has("ui") && !p.has("package.json") {
        bail!("в этом модуле нет UI (каталог ui/ или package.json не найден)");
    }

    ensure_index_html(p)?;

    let pm = if which("bun") { "bun" } else { "npm" };
    if !p.has("node_modules") {
        println!("==> установка зависимостей: {pm} install");
        let status = Command::new(pm)
            .arg("install")
            .current_dir(&p.root)
            .status()
            .with_context(|| format!("ошибка запуска `{pm} install`"))?;
        if !status.success() {
            bail!("`{pm} install` завершился с ошибкой");
        }
    }

    println!("==> запуск локальной среды разработки UI...");
    println!("    локальный сервер: http://localhost:5173/");
    println!("    тестовое окружение Noro с mock window.__noroUi и HMR");

    let status = if which("bun") {
        Command::new("bun")
            .args(["x", "vite", "--open"])
            .current_dir(&p.root)
            .status()
            .context("не удалось запустить `bun x vite`")?
    } else {
        Command::new("npx")
            .args(["vite", "--open"])
            .current_dir(&p.root)
            .status()
            .context("не удалось запустить `npx vite`")?
    };

    if !status.success() {
        bail!("сервер разработки Vite завершил работу с ошибкой");
    }
    Ok(())
}

fn which(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

pub fn ensure_index_html(p: &Project) -> Result<bool> {
    let index_file = p.path("index.html");
    if !index_file.exists() {
        let template = include_str!("../template/index.dev.html")
            .replace("{{id}}", &p.manifest.module.id)
            .replace("{{name}}", &p.manifest.module.name);
        std::fs::write(&index_file, template)
            .with_context(|| format!("не удалось создать {}", index_file.display()))?;
        println!("✓ создан тестовый index.html для локальной разработки UI");
        return Ok(true);
    }
    Ok(false)
}
