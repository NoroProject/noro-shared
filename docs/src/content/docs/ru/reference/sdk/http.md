---
title: "http"
description: "Calling out to the internet."
---

:::note
Собирается из исходников самого SDK. Править руками бесполезно.
:::

Calling out to the internet.

| Вызов | Что делает | Нужно |
|---|---|---|
| `send(call: HttpCall) -> Result<HttpReply, ModuleError>` | Sends a request and waits for the answer. | the host in `http = [...]` |
| `get_json<T: for<'de> serde::Deserialize<'de>>(url: &str) -> Result<T, ModuleError>` | A `GET` whose answer is parsed as JSON. | the host in `http = [...]` |
