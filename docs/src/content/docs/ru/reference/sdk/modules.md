---
title: "modules"
description: "Межмодульные вызовы RPC."
---

Межмодульные вызовы RPC.

| Вызов | Что делает | Нужно |
|---|---|---|
| `call<R: DeserializeOwned, P: Serialize>(target: &str, method: &str, payload: &P) -> Result<R, ModuleError>` | Вызывает метод RPC другого установленного модуля. | `modules = ["call"]` |
