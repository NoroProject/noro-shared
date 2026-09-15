---
title: "modules"
description: "Inter-module RPC calls."
---

Inter-module RPC calls.

| Вызов | Что делает | Нужно |
|---|---|---|
| `call<R: DeserializeOwned, P: Serialize>(target: &str, method: &str, payload: &P) -> Result<R, ModuleError>` | Calls an RPC method exposed by another module. | — |
