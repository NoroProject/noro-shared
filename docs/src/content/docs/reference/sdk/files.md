---
title: "files"
description: "The shared file store."
---

The shared file store.

| Call | What it does | Needs |
|---|---|---|
| `put(bytes: &[u8]) -> Result<StoredFile, ModuleError>` | Stores bytes and returns where they landed. | `files = ["write"]` |
| `read(sha1: &str) -> Result<Option<Vec<u8>>, ModuleError>` | Reads a file back. `None` — nothing is stored under that hash. | `files = ["read"]` |
| `exists(sha1: &str) -> Result<bool, ModuleError>` | Whether anything is stored under that hash. | `files = ["read"]` |
| `url(sha1: &str) -> Result<String, ModuleError>` | Where a stored file is served from. | `files = ["read"]` |
