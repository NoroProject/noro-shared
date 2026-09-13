//! Ключ автора и подпись пакета.
//!
//! Подпись здесь не про «мастер доверяет незнакомцу»: модуль ставит владелец
//! инстанса, и файл, который он сам положил, ему и так знаком. Она про
//! **обновление**: пакет с тем же идентификатором, но другим ключом — это
//! другой автор, и мастер обязан спросить, а не принять молча.
//!
//! Отсюда и хранение ключа: рядом с проектом ему нельзя — он уедет в git с
//! первым же `git add .`, и на этом подпись кончится.

use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use ed25519_dalek::{Signer, SigningKey};
use noro_module_abi::signing::{self, Signature};

use crate::project::Project;

/// Где лежит ключ. Один на модуль: разные модули одного автора обычно и
/// выпускаются порознь, а общий ключ делает компрометацию общей бедой.
pub fn key_path(id: &str) -> Result<PathBuf> {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .context("домашний каталог не найден: где хранить ключ, неизвестно")?;
    Ok(PathBuf::from(home)
        .join(".config/noro/keys")
        .join(format!("{id}.key")))
}

/// Заводит ключ, если его ещё нет. Существующий не трогает: перезаписать его
/// значит потерять возможность выпустить обновление.
pub fn create(id: &str) -> Result<()> {
    let path = key_path(id)?;
    if path.exists() {
        bail!(
            "ключ уже есть: {}\nудалите его сами, если действительно хотите сменить автора",
            path.display()
        );
    }
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }

    let key = SigningKey::generate(&mut rand_core::OsRng);
    std::fs::write(&path, signing::hex_encode(key.as_bytes()))?;
    restrict(&path);

    println!(
        "ключ создан: {}\nоткрытая часть: {}\n\n\
         Сохраните файл отдельно от проекта. Без него обновление этого модуля \n\
         придётся ставить с подтверждением смены автора.",
        path.display(),
        signing::hex_encode(key.verifying_key().as_bytes())
    );
    Ok(())
}

/// Показывает открытый ключ — его публикуют рядом с модулем.
pub fn show(id: &str) -> Result<()> {
    println!(
        "{}",
        signing::hex_encode(load(id)?.verifying_key().as_bytes())
    );
    Ok(())
}

/// Подписывает записи пакета.
pub fn sign(p: &Project, entries: &[(String, Vec<u8>)]) -> Result<Signature> {
    let key = load(&p.manifest.module.id)?;
    let digest = signing::digest(entries);
    Ok(Signature {
        key: signing::hex_encode(key.verifying_key().as_bytes()),
        sig: signing::hex_encode(&key.sign(&digest).to_bytes()),
    })
}

fn load(id: &str) -> Result<SigningKey> {
    let path = key_path(id)?;
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("ключа нет: {}\nсоздать: cargo noro key new", path.display()))?;
    let bytes = hex(raw.trim()).context("ключ повреждён: ожидался hex из 64 символов")?;
    let bytes: [u8; 32] = bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("ключ повреждён: нужно ровно 32 байта"))?;
    Ok(SigningKey::from_bytes(&bytes))
}

fn hex(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

/// Права на файл ключа. На Windows аналога нет, и там он остаётся обычным —
/// молчать об этом хуже, чем сказать один раз.
#[cfg(unix)]
fn restrict(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
}

#[cfg(not(unix))]
fn restrict(_path: &std::path::Path) {
    println!("· права на файл ключа не выставлены: на этой системе их нет");
}
