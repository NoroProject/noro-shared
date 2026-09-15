---
title: "instance"
description: "Instance settings."
---

Instance settings.

| Call | What it does | Needs |
|---|---|---|
| `all() -> Result<BTreeMap<String, Value>, ModuleError>` | Every setting, by key. | `instance = ["read"]` |
| `get<T: DeserializeOwned>(key: &str) -> Result<Option<T>, ModuleError>` | One setting, parsed into the type you expect. | `instance = ["read"]` |
| `set<T: Serialize>(key: &str, value: &T) -> Result<(), ModuleError>` | Writes a setting. | `instance = ["read", "write"]` |
