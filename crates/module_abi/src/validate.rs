//! Проверка манифеста.
//!
//! Живёт в публичном крейте, потому что проверяют двое и по одним правилам:
//! `cargo-noro` при сборке пакета и мастер при установке. Разъехаться они не
//! могут — иначе автор собирает пакет, который мастер молча не принимает.
//!
//! Здесь только манифест. Подписки, ручки и задачи объявляются кодом, и
//! проверяет их [`crate::Registration::violations`] — уже после того, как
//! модуль их вернул.

use crate::manifest::{Manifest, Placement};

/// Что не так с манифестом. Пустой список — можно ставить.
pub fn violations(m: &Manifest) -> Vec<String> {
    let mut out = Vec::new();
    let id = &m.module.id;

    if id.is_empty()
        || !id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        out.push(format!(
            "идентификатор «{id}»: допустимы только строчные латинские буквы, цифры и дефис"
        ));
    }
    if id.starts_with('-') || id.ends_with('-') {
        out.push(format!(
            "идентификатор «{id}» не может начинаться или кончаться дефисом"
        ));
    }
    // Имя схемы Postgres ограничено 63 байтами, и туда же едет префикс `mod_`.
    if id.len() > 58 {
        out.push(format!("идентификатор «{id}» длиннее 58 символов"));
    }
    if m.module.version.is_empty() {
        out.push("версия не указана".to_string());
    }
    if m.module.api.is_empty() {
        out.push("не указана требуемая версия ABI (api)".to_string());
    }

    for a in &m.apps {
        if a.entry.is_empty() || a.entry.starts_with('/') || a.entry.contains("..") {
            out.push(format!(
                "точка входа мини-аппа «{}» должна быть относительным путём внутри web/",
                a.entry
            ));
        }
        if a.placement == Placement::Widget && a.slot.is_none() {
            out.push("виджету нужен слот (slot)".to_string());
        }
    }

    // Узлы прав обязаны жить в своей ветке: иначе модуль заявит `noro.admin.*`
    // и получит подсказку на чужое право в редакторе ролей.
    let prefix = format!("noro.module.{id}.");
    for p in &m.permissions {
        if !p.node.starts_with(&prefix) {
            out.push(format!(
                "узел прав «{}» должен начинаться с «{prefix}»",
                p.node
            ));
        }
    }

    for host in &m.capabilities.http {
        if host.contains('/') || host.contains(':') {
            out.push(format!("в allowlist ожидается хост, а не адрес: «{host}»"));
        }
    }

    out
}

/// Разбирает интервал задачи в секунды.
pub fn parse_every(s: &str) -> Option<u64> {
    let s = s.trim();
    let (num, mult) = match s.chars().last()? {
        's' => (&s[..s.len() - 1], 1),
        'm' => (&s[..s.len() - 1], 60),
        'h' => (&s[..s.len() - 1], 3600),
        'd' => (&s[..s.len() - 1], 86_400),
        _ => return None,
    };
    let n: u64 = num.parse().ok()?;
    if n == 0 {
        return None;
    }
    Some(n * mult)
}

/// Совместим ли модуль с этой версией ABI. Сверяется мажор.
pub fn api_compatible(required: &str, current: &str) -> bool {
    let major = |v: &str| v.split('.').next().unwrap_or("").to_string();
    !major(required).is_empty() && major(required) == major(current)
}

#[cfg(test)]
#[path = "validate_tests.rs"]
mod tests;
