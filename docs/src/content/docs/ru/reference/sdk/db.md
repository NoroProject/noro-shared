---
title: "db"
description: "Your own tables."
---

:::note
Собирается из исходников самого SDK. Править руками бесполезно.
:::

Your own tables.

| Вызов | Что делает | Нужно |
|---|---|---|
| `query(q: Query) -> Result<Rows, ModuleError>` | Runs a query and returns its rows. | `db = true` |
| `execute(q: Query) -> Result<u64, ModuleError>` | Runs a statement and returns how many rows it touched. | `db = true` |
| `one(q: Query) -> Result<Option<serde_json::Map<String, serde_json::Value>>, ModuleError>` | The first row, or `None`. | `db = true` |
| `scalar<T: serde::de::DeserializeOwned>(q: Query) -> Result<Option<T>, ModuleError>` | The first column of the first row, parsed into `T`. | `db = true` |
