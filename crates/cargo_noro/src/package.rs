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

pub fn run(p: &Project, debug: bool) -> Result<()> {
    let id = &p.manifest.module.id;
    let dist = p.path("dist");
    std::fs::create_dir_all(&dist).context("не создаётся dist/")?;

    let out = dist.join(format!("{id}.noromod"));
    let file =
        std::fs::File::create(&out).with_context(|| format!("{} не пишется", out.display()))?;
    let mut zip = zip::ZipWriter::new(file);
    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    zip.start_file("manifest.toml", opts)?;
    zip.write_all(&std::fs::read(p.path("manifest.toml"))?)?;

    zip.start_file("module.wasm", opts)?;
    zip.write_all(&std::fs::read(p.wasm(debug)?)?)?;

    if p.has("icon.png") {
        zip.start_file("icon.png", opts)?;
        zip.write_all(&std::fs::read(p.path("icon.png"))?)?;
    }

    for dir in DIRS {
        add_dir(&mut zip, &p.path(dir), dir, opts)?;
    }

    zip.finish().context("архив не закрывается")?;

    let bytes = std::fs::read(&out)?;
    println!(
        "готово: {} ({}, sha256 {})",
        out.display(),
        human(bytes.len()),
        &hex(&Sha256::digest(&bytes))[..12]
    );
    Ok(())
}

/// Кладёт каталог рекурсивно, сохраняя пути внутри архива.
fn add_dir<W: Write + std::io::Seek>(
    zip: &mut zip::ZipWriter<W>,
    dir: &Path,
    prefix: &str,
    opts: SimpleFileOptions,
) -> Result<()> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Ok(());
    };
    // Порядок фиксируется: иначе две сборки одного и того же дают разный
    // sha256, и мастер считает пакет новым там, где ничего не менялось.
    let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
    paths.sort();

    for path in paths {
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        // Прячем то, что кладёт система, а не автор.
        if name.starts_with('.') {
            continue;
        }
        let inner = format!("{prefix}/{name}");
        if path.is_dir() {
            add_dir(zip, &path, &inner, opts)?;
        } else {
            zip.start_file(&inner, opts)?;
            zip.write_all(&std::fs::read(&path)?)?;
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
