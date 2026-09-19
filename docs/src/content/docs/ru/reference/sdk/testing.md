---
title: "testing"
description: "Помощники для юнит-тестирования функций модуля без запуска мастер-сервера."
---

Помощники для юнит-тестирования функций модуля без запуска мастер-сервера.

| Вызов | Что делает | Нужно |
|---|---|---|
| `mock_player(name: &str) -> Player` | Создаёт тестового игрока [`Player`]. | — |
| `mock_context() -> EventCtx` | Создаёт тестовый контекст события [`EventCtx`] (источник Web или System). | — |
| `mock_web_message<T: Serialize>(name: &str, payload: &T) -> WebMessage` | Создаёт тестовое сообщение [`WebMessage`] с сериализованной полезной нагрузкой. | — |
| `mock_request(method: &str, path: &str, body: Option<serde_json::Value>, user: Option<Uuid>) -> HttpRequest` | Создаёт тестовый HTTP-запрос [`HttpRequest`]. | — |
