//! Manifest validation.
//!
//! It lives in the public crate because two parties validate, by the same
//! rules: `cargo-noro` when building the package and the master when installing
//! it. They cannot drift apart — otherwise the author builds a package the
//! master silently refuses.
//!
//! Only the manifest is here. Subscriptions, endpoints and tasks are declared
//! in code, and [`crate::Registration::violations`] checks those — after the
//! module has returned them.

use crate::manifest::{Manifest, Placement};

/// What is wrong with the manifest. An empty list means it can be installed.
pub fn violations(m: &Manifest) -> Vec<String> {
    let mut out = Vec::new();
    let id = &m.module.id;

    if id.is_empty()
        || !id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        out.push(format!(
            "the identifier `{id}`: only lowercase latin letters, digits and hyphens are allowed"
        ));
    }
    if id.starts_with('-') || id.ends_with('-') {
        out.push(format!(
            "the identifier `{id}` cannot start or end with a hyphen"
        ));
    }
    // A Postgres schema name is capped at 63 bytes, and the `mod_` prefix goes in there too.
    if id.len() > 58 {
        out.push(format!(
            "the identifier `{id}` is longer than 58 characters"
        ));
    }
    if m.module.version.is_empty() {
        out.push("no version given".to_string());
    }
    if m.module.api.is_empty() {
        out.push("no required ABI version given (api)".to_string());
    }

    for a in &m.apps {
        if a.entry.is_empty() || a.entry.starts_with('/') || a.entry.contains("..") {
            out.push(format!(
                "the mini-app entry point `{}` has to be a relative path inside web/",
                a.entry
            ));
        }
        if a.placement == Placement::Widget && a.slot.is_none() {
            out.push("a widget needs a slot".to_string());
        }
    }

    // Permission nodes have to live in their own branch: otherwise a module
    // claims `noro.admin.*` and shows up as a hint for somebody else's
    // permission in the role editor.
    let prefix = format!("noro.module.{id}.");
    for p in &m.permissions {
        if !p.node.starts_with(&prefix) {
            out.push(format!(
                "the permission node `{}` has to start with `{prefix}`",
                p.node
            ));
        }
    }

    for host in &m.capabilities.http {
        if host.contains('/') || host.contains(':') {
            out.push(format!(
                "the allow-list expects a host, not an address: `{host}`"
            ));
        }
    }

    out
}

/// Parses a task interval into seconds.
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

/// Whether a module is compatible with this ABI version. The major is compared.
pub fn api_compatible(required: &str, current: &str) -> bool {
    let major = |v: &str| v.split('.').next().unwrap_or("").to_string();
    !major(required).is_empty() && major(required) == major(current)
}

#[cfg(test)]
#[path = "validate_tests.rs"]
mod tests;
