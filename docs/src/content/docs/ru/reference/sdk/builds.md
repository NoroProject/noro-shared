---
title: "builds"
description: "Client builds and the files inside them."
---

Client builds and the files inside them.

| Вызов | Что делает | Нужно |
|---|---|---|
| `of(server_id: Uuid) -> Result<Vec<Build>, ModuleError>` | The client builds of a server. | `builds = ["read"]` |
| `published(server_id: Uuid) -> Result<Option<Build>, ModuleError>` | The build players are currently getting, if there is one. | `builds = ["read"]` |
| `unpublish(build_id: Uuid) -> Result<(), ModuleError>` | Takes a build out of publication, so the launcher stops handing it out. | `builds = ["read", "publish"]` |
| `files(build_id: Uuid) -> Result<Vec<BuildFile>, ModuleError>` | Everything inside a build, with the hash the launcher downloads by. | `builds = ["read"]` |
| `read(build_id: Uuid, path: &str) -> Result<Option<String>, ModuleError>` | The text of one file, by its path in the build. | `builds = ["read"]` |
| `write(build_id: Uuid, path: &str, text: &str) -> Result<BuildFile, ModuleError>` | Writes a text file into the build, replacing whatever was at that path. | `builds = ["files"]` |
| `write_for(build_id: Uuid, path: &str, text: &str, side: &str) -> Result<BuildFile, ModuleError>` | The same, for one side only: `client`, `server` or `both`. | `builds = ["files"]` |
| `attach(build_id: Uuid, path: &str, sha1: &str) -> Result<BuildFile, ModuleError>` | Puts an already-stored file into the build under a path. | `builds = ["files"]` and `files = ["read"]` |
| `remove(build_id: Uuid, path: &str) -> Result<bool, ModuleError>` | Removes a file from the build. `false` means there was nothing at that path. | `builds = ["files"]` |
