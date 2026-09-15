//! A request to a module's endpoint.
//!
//! The master parses the HTTP itself and hands the module the finished result:
//! there is no reason to parse headers and cookies inside the sandbox, and
//! access decisions are made before the call.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// What a module receives when its endpoint is called.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    /// `GET`, `POST`, … — as declared in the code.
    pub method: String,
    /// The path inside the module, starting with a slash.
    pub path: String,
    /// The query-string parameters as a single object.
    #[serde(default)]
    pub query: Value,
    /// The body, if there was one.
    #[serde(default)]
    pub body: Option<Value>,
    /// Who is calling. Empty on a public endpoint with no sign-in.
    ///
    /// There is no need to re-check permissions: the master has already
    /// compared them against what was declared, and a request that fails never
    /// reaches the module.
    #[serde(default)]
    pub user: Option<Uuid>,
}

impl HttpRequest {
    /// The body, parsed into the requested type.
    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Option<T> {
        serde_json::from_value(self.body.clone()?).ok()
    }

    /// A string query parameter.
    pub fn param(&self, name: &str) -> Option<&str> {
        self.query.get(name)?.as_str()
    }

    /// Who is calling, when the endpoint requires a sign-in.
    pub fn require_user(&self) -> Result<Uuid, crate::error::ModuleError> {
        self.user.ok_or_else(|| {
            crate::error::ModuleError::invalid("the endpoint was called without a sign-in")
        })
    }
}

/// A response from a module endpoint.
///
/// If returned from a route handler, the master unpacks status, headers, and body.
/// If a handler returns plain data, the master defaults to 200 OK application/json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    #[serde(default = "default_marker")]
    pub __noro_http_response: bool,
    pub status: u16,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    #[serde(default)]
    pub body: Value,
}

fn default_marker() -> bool {
    true
}

impl HttpResponse {
    pub fn new(status: u16, body: impl Serialize) -> Self {
        let body = serde_json::to_value(body).unwrap_or(Value::Null);
        Self {
            __noro_http_response: true,
            status,
            headers: Vec::new(),
            body,
        }
    }

    pub fn ok(body: impl Serialize) -> Self {
        Self::new(200, body)
    }

    pub fn not_found(body: impl Serialize) -> Self {
        Self::new(404, body)
    }

    pub fn bad_request(body: impl Serialize) -> Self {
        Self::new(400, body)
    }

    pub fn unauthorized(body: impl Serialize) -> Self {
        Self::new(401, body)
    }

    pub fn forbidden(body: impl Serialize) -> Self {
        Self::new(403, body)
    }

    pub fn redirect(location: impl Into<String>) -> Self {
        let mut r = Self::new(302, Value::Null);
        r.headers.push(("Location".to_string(), location.into()));
        r
    }

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}
