---
title: "http"
description: "Calling out to the internet."
---

:::note
Generated from the SDK's own sources. Editing it by hand has no effect.
:::

Calling out to the internet.

| Call | What it does | Needs |
|---|---|---|
| `send(call: HttpCall) -> Result<HttpReply, ModuleError>` | Sends a request and waits for the answer. | the host in `http = [...]` |
| `get_json<T: for<'de> serde::Deserialize<'de>>(url: &str) -> Result<T, ModuleError>` | A `GET` whose answer is parsed as JSON. | the host in `http = [...]` |
