//! The instance's news.
//!
//! ```ignore
//! let id = news::publish("Server update", "We moved to 1.21.", None)?;
//! news::edit(id, "Server update", "We moved to 1.21.1, actually.", None, true)?;
//! ```

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{NewsDraft, NewsItem};
use uuid::Uuid;

/// A page of news, pinned items first and then newest first.
///
/// Requires `news = ["read"]`.
pub fn list(page: i64, per_page: i64) -> Result<Vec<NewsItem>, ModuleError> {
    crate::host::news_list_call(serde_json::json!({ "page": page, "per_page": per_page }))
}

/// One item by identifier.
///
/// Requires `news = ["read"]`.
pub fn get(id: Uuid) -> Result<Option<NewsItem>, ModuleError> {
    crate::host::news_get_call(id)
}

/// Publishes an item and returns its identifier.
///
/// The body is markdown, the same as the panel's editor writes. `pinned` keeps
/// it at the top of the list regardless of date.
///
/// Requires `news = ["read", "publish"]`.
pub fn publish(title: &str, body: &str, preview_url: Option<&str>) -> Result<Uuid, ModuleError> {
    crate::host::news_save_call(NewsDraft {
        id: None,
        title: title.to_string(),
        body: body.to_string(),
        preview_url: preview_url.map(str::to_string),
        pinned: false,
    })
}

/// Publishes an item pinned to the top.
///
/// Requires `news = ["read", "publish"]`.
pub fn publish_pinned(
    title: &str,
    body: &str,
    preview_url: Option<&str>,
) -> Result<Uuid, ModuleError> {
    crate::host::news_save_call(NewsDraft {
        id: None,
        title: title.to_string(),
        body: body.to_string(),
        preview_url: preview_url.map(str::to_string),
        pinned: true,
    })
}

/// Rewrites an existing item.
///
/// Every field is replaced, not merged: a partial edit would mean guessing
/// which of the absent fields you meant to clear.
///
/// Requires `news = ["read", "edit"]`.
pub fn edit(
    id: Uuid,
    title: &str,
    body: &str,
    preview_url: Option<&str>,
    pinned: bool,
) -> Result<(), ModuleError> {
    crate::host::news_save_call(NewsDraft {
        id: Some(id),
        title: title.to_string(),
        body: body.to_string(),
        preview_url: preview_url.map(str::to_string),
        pinned,
    })
    .map(|_: Uuid| ())
}

/// Deletes an item.
///
/// Requires `news = ["read", "edit"]`.
pub fn delete(id: Uuid) -> Result<(), ModuleError> {
    crate::host::news_delete_call(id)
}
