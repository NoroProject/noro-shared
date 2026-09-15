---
title: "news"
description: "The instance's news."
---

The instance's news.

| Вызов | Что делает | Нужно |
|---|---|---|
| `list(page: i64, per_page: i64) -> Result<Vec<NewsItem>, ModuleError>` | A page of news, pinned items first and then newest first. | `news = ["read"]` |
| `get(id: Uuid) -> Result<Option<NewsItem>, ModuleError>` | One item by identifier. | `news = ["read"]` |
| `publish(title: &str, body: &str, preview_url: Option<&str>) -> Result<Uuid, ModuleError>` | Publishes an item and returns its identifier. | `news = ["read", "publish"]` |
| `publish_pinned(title: &str, body: &str, preview_url: Option<&str>) -> Result<Uuid, ModuleError>` | Publishes an item pinned to the top. | `news = ["read", "publish"]` |
| `edit(id: Uuid, title: &str, body: &str, preview_url: Option<&str>, pinned: bool) -> Result<(), ModuleError>` | Rewrites an existing item. | `news = ["read", "edit"]` |
| `delete(id: Uuid) -> Result<(), ModuleError>` | Deletes an item. | `news = ["read", "edit"]` |
