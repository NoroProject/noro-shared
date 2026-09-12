//! Сборка: мини-апп, wasm, `wasm-opt`.

use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::project::Project;

pub fn run(p: &Project, debug: bool) -> Result<()> {
    if p.has("package.json") {
        frontend(p)?;
    }
    wasm(p, debug)?;
    optimise(p, debug);
    Ok(())
}

/// Фронтенд собирается первым: его ошибки читаются легче, а wasm дольше.
fn frontend(p: &Project) -> Result<()> {
    let pm = if which("bun") { "bun" } else { "npm" };
    if !p.has("node_modules") {
        println!("==> {pm} install");
        step(p, pm, &["install"])?;
    }
    println!("==> мини-апп");
    step(p, pm, &["run", "build"])
}

fn wasm(p: &Project, debug: bool) -> Result<()> {
    println!("==> wasm");
    let mut args = vec!["build", "--target", "wasm32-unknown-unknown"];
    if !debug {
        args.push("--release");
    }
    step(p, "cargo", &args).context(
        "цель wasm32-unknown-unknown не установлена? \
         rustup target add wasm32-unknown-unknown",
    )
}

/// `wasm-opt` режет размер примерно втрое и стоит в системе не у всех.
///
/// Поэтому его отсутствие — не ошибка, а строка в выводе: пакет соберётся и
/// без него, просто тяжелее.
fn optimise(p: &Project, debug: bool) {
    if debug {
        return;
    }
    if !which("wasm-opt") {
        println!("==> wasm-opt не найден, пропускаю — пакет будет больше");
        return;
    }
    let Ok(wasm) = p.wasm(false) else { return };
    let tmp = wasm.with_extension("opt.wasm");

    println!("==> wasm-opt");
    let ok = Command::new("wasm-opt")
        .arg("-Oz")
        .arg(&wasm)
        .arg("-o")
        .arg(&tmp)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if ok {
        let _ = std::fs::rename(&tmp, &wasm);
    } else {
        let _ = std::fs::remove_file(&tmp);
        println!("   wasm-opt не отработал — беру как есть");
    }
}

fn step(p: &Project, program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .current_dir(&p.root)
        .status()
        .with_context(|| format!("`{program}` не запускается — он установлен?"))?;
    if !status.success() {
        bail!("`{program} {}` завершился с ошибкой", args.join(" "));
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
