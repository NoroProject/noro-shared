---
title: "cache"
description: "In-memory TTL cache for modules."
---

In-memory TTL cache for modules.

| Call | What it does | Needs |
|---|---|---|
| `get<T: DeserializeOwned>(key: &str) -> Result<Option<T>, ModuleError>` | Reads a value from the cache. | — |
| `get_or<T: DeserializeOwned + Default>(key: &str) -> Result<T, ModuleError>` | Reads a value, substituting default if the key is absent or expired. | — |
| `set<T: Serialize>(key: &str, value: &T, ttl_secs: u64) -> Result<(), ModuleError>` | Stores a value in the cache with a specified TTL in seconds. | — |
| `delete(key: &str) -> Result<bool, ModuleError>` | Deletes a key from the cache. Returns `true` if the key was present. | — |
