---
title: "db"
description: "Your own tables."
---

:::note
Generated from the SDK's own sources. Editing it by hand has no effect.
:::

Your own tables.

| Call | What it does | Needs |
|---|---|---|
| `query(q: Query) -> Result<Rows, ModuleError>` | Runs a query and returns its rows. | `db = true` |
| `execute(q: Query) -> Result<u64, ModuleError>` | Runs a statement and returns how many rows it touched. | `db = true` |
| `one(q: Query) -> Result<Option<serde_json::Map<String, serde_json::Value>>, ModuleError>` | The first row, or `None`. | `db = true` |
| `scalar<T: serde::de::DeserializeOwned>(q: Query) -> Result<Option<T>, ModuleError>` | The first column of the first row, parsed into `T`. | `db = true` |
| `query_as<T: serde::de::DeserializeOwned>(q: Query) -> Result<Vec<T>, ModuleError>` | Runs a query and parses each row into `T`. | `db = true` |
| `one_as<T: serde::de::DeserializeOwned>(q: Query) -> Result<Option<T>, ModuleError>` | The first row parsed into `T`, or `None`. | `db = true` |
