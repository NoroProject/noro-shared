//! Request extractors for declarative `#[route]` handlers.

use std::ops::Deref;

use noro_module_abi::error::ModuleError;
use noro_module_abi::http::HttpRequest;
use serde::de::DeserializeOwned;
use uuid::Uuid;

/// Types that can be extracted from an incoming [`HttpRequest`].
pub trait FromRequest: Sized {
    fn from_request(req: &HttpRequest) -> Result<Self, ModuleError>;
}

impl FromRequest for HttpRequest {
    fn from_request(req: &HttpRequest) -> Result<Self, ModuleError> {
        Ok(req.clone())
    }
}

/// Extractor for an authenticated player's UUID.
///
/// Returns an error (401 Unauthorized) if the request has no caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthUser(pub Uuid);

impl Deref for AuthUser {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for AuthUser {
    fn from_request(req: &HttpRequest) -> Result<Self, ModuleError> {
        req.require_user().map(AuthUser)
    }
}

/// Extractor for an optional caller UUID (present if signed in).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptionalUser(pub Option<Uuid>);

impl FromRequest for OptionalUser {
    fn from_request(req: &HttpRequest) -> Result<Self, ModuleError> {
        Ok(OptionalUser(req.user))
    }
}

/// Extractor for deserializing JSON request body into `T`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Json<T>(pub T);

impl<T> Deref for Json<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: DeserializeOwned> FromRequest for Json<T> {
    fn from_request(req: &HttpRequest) -> Result<Self, ModuleError> {
        req.json::<T>()
            .map(Json)
            .ok_or_else(|| ModuleError::invalid("missing or invalid JSON body"))
    }
}

impl<T: DeserializeOwned> FromRequest for extism_pdk::Json<T> {
    fn from_request(req: &HttpRequest) -> Result<Self, ModuleError> {
        req.json::<T>()
            .map(extism_pdk::Json)
            .ok_or_else(|| ModuleError::invalid("missing or invalid JSON body"))
    }
}

/// Extractor for deserializing query string parameters into `T`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryParams<T>(pub T);

impl<T> Deref for QueryParams<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: DeserializeOwned> FromRequest for QueryParams<T> {
    fn from_request(req: &HttpRequest) -> Result<Self, ModuleError> {
        serde_json::from_value(req.query.clone())
            .map(QueryParams)
            .map_err(|e| ModuleError::invalid(format!("invalid query parameters: {e}")))
    }
}

/// Extractor for the raw query parameter JSON value.
#[derive(Debug, Clone)]
pub struct RawParams(pub serde_json::Value);

impl FromRequest for RawParams {
    fn from_request(req: &HttpRequest) -> Result<Self, ModuleError> {
        Ok(RawParams(req.query.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct Payload {
        count: i32,
    }

    #[test]
    fn auth_user_extractor() {
        let uid = Uuid::from_u128(1);
        let mut req = HttpRequest {
            method: "POST".into(),
            path: "/test".into(),
            query: json!({}),
            body: None,
            user: Some(uid),
        };
        assert_eq!(*AuthUser::from_request(&req).unwrap(), uid);
        assert_eq!(OptionalUser::from_request(&req).unwrap().0, Some(uid));

        req.user = None;
        assert!(AuthUser::from_request(&req).is_err());
        assert_eq!(OptionalUser::from_request(&req).unwrap().0, None);
    }

    #[test]
    fn json_and_query_extractors() {
        let req = HttpRequest {
            method: "POST".into(),
            path: "/test".into(),
            query: json!({ "count": 42 }),
            body: Some(json!({ "count": 100 })),
            user: None,
        };
        let body: Json<Payload> = Json::from_request(&req).unwrap();
        assert_eq!(body.count, 100);

        let query: QueryParams<Payload> = QueryParams::from_request(&req).unwrap();
        assert_eq!(query.count, 42);
    }
}
