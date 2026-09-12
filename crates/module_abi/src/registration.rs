//! What a module declares about itself in code.
//!
//! Subscriptions, endpoints and tasks live next to their handlers rather than in
//! the manifest: the function name used to be duplicated there as a string, and
//! a typo in it produced a silently dead handler. More importantly, the event
//! name is derived from the argument type (`Event::NAME`), so subscribing to
//! one event while accepting another's struct is no longer possible: it will
//! not compile.
//!
//! The manifest is left with only what has to be known before any code runs:
//! the identifier, the version, the required ABI, the capabilities being asked
//! for, the permission nodes. Learning the capabilities from code would mean
//! running that code before the operator decided it was allowed to.

use serde::{Deserialize, Serialize};

use crate::manifest::{Auth, Priority, SettingDecl, SettingKind};

/// The answer from the `noro_register` export.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Registration {
    #[serde(default)]
    pub events: Vec<EventReg>,
    #[serde(default)]
    pub routes: Vec<RouteReg>,
    #[serde(default)]
    pub tasks: Vec<TaskReg>,
    /// The fields of the settings form.
    ///
    /// Also from code rather than the manifest: a setting is almost always
    /// introduced together with the code that reads it, and keeping them in
    /// separate files means one day deleting one and forgetting the other.
    #[serde(default)]
    pub settings: Vec<SettingDecl>,
}

/// A subscription to an event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReg {
    /// A name from the catalog. Filled in by the macro from `Event::NAME`;
    /// never written by hand.
    pub name: String,
    /// The module export to call.
    pub handler: String,
    #[serde(default)]
    pub priority: Priority,
}

/// An endpoint under `/api/modules/<id>/…`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteReg {
    pub method: String,
    pub path: String,
    pub handler: String,
    #[serde(default)]
    pub auth: Auth,
}

/// A background task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskReg {
    pub handler: String,
    /// The interval: `30s`, `5m`, `1h`, `24h`.
    pub every: String,
}

impl Registration {
    /// What is wrong with the declaration. An empty list means the module can
    /// be started.
    ///
    /// The master checks this after calling `noro_register`: the declaration
    /// comes from somebody else's code, and taking it at its word is not on
    /// even with complete trust in the author — a typo stays a typo.
    pub fn violations(&self) -> Vec<String> {
        let mut out = Vec::new();

        for e in &self.events {
            if crate::events::find(&e.name).is_none() {
                out.push(format!("the event `{}` does not exist", e.name));
            }
            if e.handler.is_empty() {
                out.push(format!(
                    "the subscription to `{}` has an empty handler",
                    e.name
                ));
            }
        }

        for r in &self.routes {
            if !r.path.starts_with('/') || r.path.contains("..") {
                out.push(format!("the endpoint path `{}` is not allowed", r.path));
            }
            let method = r.method.to_ascii_uppercase();
            if !["GET", "POST", "PUT", "PATCH", "DELETE"].contains(&method.as_str()) {
                out.push(format!("the method `{}` is not supported", r.method));
            }
            if let Auth::Permission(node) | Auth::Admin(node) = &r.auth {
                if node.is_empty() {
                    out.push(format!(
                        "the endpoint `{}` has an empty permission node",
                        r.path
                    ));
                }
            }
        }

        // Two endpoints on one method and path raise the question of which one
        // fires, and there is no good answer to it.
        let mut seen = Vec::new();
        for r in &self.routes {
            let key = (r.method.to_ascii_uppercase(), r.path.clone());
            if seen.contains(&key) {
                out.push(format!(
                    "the endpoint {} {} is declared twice",
                    key.0, key.1
                ));
            }
            seen.push(key);
        }

        for s in &self.settings {
            if s.key.is_empty() {
                out.push("a setting has an empty key".to_string());
            }
            if let (Some(min), Some(max)) = (s.min, s.max) {
                if min > max {
                    out.push(format!(
                        "the setting `{}` has a minimum above its maximum",
                        s.key
                    ));
                }
            }
        }

        for t in &self.tasks {
            if crate::validate::parse_every(&t.every).is_none() {
                out.push(format!(
                    "the task interval `{}` did not parse: expected 30s, 5m, 1h",
                    t.every
                ));
            }
        }

        out
    }

    /// The endpoint answering this request.
    pub fn route(&self, method: &str, path: &str) -> Option<&RouteReg> {
        self.routes
            .iter()
            .find(|r| r.path == path && r.method.eq_ignore_ascii_case(method))
    }

    /// Introduces a settings field and returns it for refinement.
    ///
    /// ```ignore
    /// reg.setting("points_per_hour", Number, "mod-shop-per-hour")
    ///    .default(100)
    ///    .range(0, 10_000);
    /// ```
    pub fn setting(
        &mut self,
        key: impl Into<String>,
        kind: SettingKind,
        label: impl Into<String>,
    ) -> &mut SettingDecl {
        self.settings.push(SettingDecl {
            key: key.into(),
            kind,
            label: label.into(),
            hint: None,
            min: None,
            max: None,
            options: Vec::new(),
            default: None,
        });
        self.settings.last_mut().expect("just pushed")
    }

    /// The subscriptions to an event, in priority order.
    pub fn subscriptions(&self, event: &str) -> Vec<&EventReg> {
        let mut out: Vec<&EventReg> = self.events.iter().filter(|e| e.name == event).collect();
        out.sort_by_key(|e| e.priority);
        out
    }
}

#[cfg(test)]
#[path = "registration_tests.rs"]
mod tests;
