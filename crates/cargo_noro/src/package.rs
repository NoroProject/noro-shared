//! Упаковка в `.noromod`.
//!
//! Zip пишется своим кодом, а не вызовом `zip`: тот есть не везде, а на macOS
//! ещё и кладёт в архив служебные `__MACOSX`, из-за которых мастер видит в
//! пакете файлы, которых автор не клал.

use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use zip::write::SimpleFileOptions;

use crate::project::Project;

/// Что уезжает в пакет целыми каталогами.
const DIRS: [&str; 3] = ["web", "locales", "migrations"];

pub fn run(p: &Project, debug: bool, signed: bool) -> Result<()> {
    let id = &p.manifest.module.id;
    let dist = p.path("dist");
    std::fs::create_dir_all(&dist).context("не создаётся dist/")?;

    // Сначала собираем записи, потом пишем архив: подпись считается по
    // содержимому, и знать его надо целиком до первой записи в файл.
    let mut entries: Vec<(String, Vec<u8>)> = vec![
        (
            "manifest.toml".to_string(),
            std::fs::read(p.path("manifest.toml"))?,
        ),
        ("module.wasm".to_string(), std::fs::read(p.wasm(debug)?)?),
    ];
    if p.has("icon.png") {
        entries.push(("icon.png".to_string(), std::fs::read(p.path("icon.png"))?));
    }
    for dir in DIRS {
        collect(&p.path(dir), dir, &mut entries)?;
    }
    // Порядок фиксируется: иначе две сборки одного и того же дают разный
    // sha256, и мастер считает пакет новым там, где ничего не менялось.
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    if signed {
        let signature = crate::sign::sign(p, &entries)?;
        entries.push((
            noro_module_abi::signing::SIGNATURE_ENTRY.to_string(),
            toml::to_string_pretty(&signature)?.into_bytes(),
        ));
    }

    let out = dist.join(format!("{id}.noromod"));
    let file =
        std::fs::File::create(&out).with_context(|| format!("{} не пишется", out.display()))?;
    let mut zip = zip::ZipWriter::new(file);
    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for (name, body) in &entries {
        zip.start_file(name, opts)?;
        zip.write_all(body)?;
    }

    zip.finish().context("архив не закрывается")?;

    let bytes = std::fs::read(&out)?;
    println!(
        "готово: {} ({}, sha256 {}){}",
        out.display(),
        human(bytes.len()),
        &hex(&Sha256::digest(&bytes))[..12],
        if signed { ", подписан" } else { "" }
    );
    Ok(())
}

/// Собирает каталог рекурсивно, сохраняя пути такими, какими они лягут в пакет.
fn collect(dir: &Path, prefix: &str, out: &mut Vec<(String, Vec<u8>)>) -> Result<()> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        // Прячем то, что кладёт система, а не автор.
        if name.starts_with('.') {
            continue;
        }
        let inner = format!("{prefix}/{name}");
        if path.is_dir() {
            collect(&path, &inner, out)?;
        } else {
            out.push((inner, std::fs::read(&path)?));
        }
    }
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn human(bytes: usize) -> String {
    match bytes {
        n if n >= 1 << 20 => format!("{:.1} МБ", n as f64 / (1 << 20) as f64),
        n => format!("{} КБ", n / 1024),
    }
}
