---
title: "testing"
description: "Helpers for unit testing module functions without running the master server."
---

Helpers for unit testing module functions without running the master server.

| Call | What it does | Needs |
|---|---|---|
| `mock_player(name: &str) -> Player` | Constructs a mock [`Player`] for tests. | — |
| `mock_context() -> EventCtx` | Constructs a mock [`EventCtx`] originated from web or system. | — |
| `mock_web_message<T: Serialize>(name: &str, payload: &T) -> WebMessage` | Constructs a mock [`WebMessage`] with serialized payload. | — |
| `mock_request(method: &str, path: &str, body: Option<serde_json::Value>, user: Option<Uuid>) -> HttpRequest` | Constructs a mock [`HttpRequest`]. | — |
