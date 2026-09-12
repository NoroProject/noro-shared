//! Outbound HTTP: what a module sends, and what comes back.
//!
//! Separate from [`crate::http`], which is the *incoming* side — a request to
//! one of the module's own endpoints. These two travel in opposite directions
//! and have nothing in common but the protocol.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A request to somewhere outside.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpCall {
    /// `GET`, `POST`, `PUT`, `PATCH`, `DELETE`.
    pub method: String,
    /// The full URL. Its host must be in the manifest's allow-list, and `https`
    /// is the only accepted scheme — a module's outbound traffic crosses the
    /// open internet.
    pub url: String,
    /// Headers to send. `Host` and the length are set by the master.
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    /// The body, if any.
    #[serde(default)]
    pub body: Option<String>,
}

impl HttpCall {
    pub fn get(url: impl Into<String>) -> Self {
        HttpCall {
            method: "GET".to_string(),
            url: url.into(),
            headers: BTreeMap::new(),
            body: None,
        }
    }

    pub fn post(url: impl Into<String>, body: impl Into<String>) -> Self {
        HttpCall {
            method: "POST".to_string(),
            url: url.into(),
            headers: BTreeMap::new(),
            body: Some(body.into()),
        }
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }
}

/// What came back.
///
/// A non-2xx status is **not** an error: it is an answer, and whether a 404
/// matters is the module's call, not the master's.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpReply {
    pub status: u16,
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    pub body: String,
}

impl HttpReply {
    /// Whether the status is 2xx.
    pub fn ok(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// The body parsed as JSON.
    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Result<T, crate::error::ModuleError> {
        serde_json::from_str(&self.body)
            .map_err(|e| crate::error::ModuleError::invalid(format!("answer is not JSON: {e}")))
    }
}
