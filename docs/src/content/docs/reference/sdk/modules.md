---
title: "modules"
description: "Inter-module RPC calls."
---

Inter-module RPC calls.

| Call | What it does | Needs |
|---|---|---|
| `call<R: DeserializeOwned, P: Serialize>(target: &str, method: &str, payload: &P) -> Result<R, ModuleError>` | Calls an RPC method exposed by another module. | — |
