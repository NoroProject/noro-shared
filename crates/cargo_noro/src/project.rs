//! Где лежит модуль и что о нём написано.

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use noro_module_abi::manifest::Manifest;

pub struct Project {
    pub root: PathBuf,
    pub manifest: Manifest,
}

impl Project {
    pub fn path(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }

    pub fn has(&self, rel: &str) -> bool {
        self.path(rel).exists()
    }

    /// Каталог сборки wasm.
    pub fn wasm_dir(&self, debug: bool) -> PathBuf {
        self.root
            .join("target/wasm32-unknown-unknown")
            .join(if debug { "debug" } else { "release" })
    }

    /// Сам `.wasm`, если он уже собран.
    pub fn wasm(&self, debug: bool) -> Result<PathBuf> {
        let dir = self.wasm_dir(debug);
        let found = std::fs::read_dir(&dir)
            .with_context(|| format!("каталог сборки не читается: {}", dir.display()))?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .find(|p| p.extension().is_some_and(|e| e == "wasm"));
        found.ok_or_else(|| anyhow::anyhow!("в {} нет ни одного .wasm", dir.display()))
    }
}

/// Ищет корень модуля от текущего каталога вверх.
///
/// Вверх — потому что собирать хочется и из `src/`, как `cargo build`. Граница
/// поиска — корень файловой системы: подниматься выше некуда, а `manifest.toml`
/// чужого проекта по дороге не встретится, он бывает только у модуля.
pub fn find() -> Result<Project> {
    let start = std::env::current_dir().context("текущий каталог недоступен")?;
    let mut dir: &Path = &start;

    loop {
        let file = dir.join("manifest.toml");
        if file.is_file() {
            let raw = std::fs::read_to_string(&file)
                .with_context(|| format!("{} не читается", file.display()))?;
            let manifest: Manifest = toml::from_str(&raw)
                .with_context(|| format!("{} не разбирается", file.display()))?;
            return Ok(Project {
                root: dir.to_path_buf(),
                manifest,
            });
        }
        match dir.parent() {
            Some(up) => dir = up,
            None => bail!(
                "здесь нет модуля: `manifest.toml` не найден ни в {}, ни выше\n\
                 создать новый: cargo noro new <id>",
                start.display()
            ),
        }
    }
}
