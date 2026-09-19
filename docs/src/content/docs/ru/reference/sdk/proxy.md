---
title: "proxy"
description: "Маршрутизация через Velocity, перенос игроков, эвакуация серверов и заголовки."
---

Операции с прокси Velocity: перемещение игроков между подсерверами, эвакуация серверов и сетевые заголовки.

| Вызов | Что делает | Нужно |
|---|---|---|
| `transfer(who, target_server_id) -> Result<bool, ModuleError>` | Перемещает игрока на другой подсервер по идентификатору игрового сервера. | `proxy = ["transfer"]` |
| `transfer_named(who, target_server_name) -> Result<bool, ModuleError>` | Перемещает игрока на другой подсервер по его зарегистрированному имени. | `proxy = ["transfer"]` |
| `evacuate(from_server, to_server, title) -> Result<u32, ModuleError>` | Эвакуирует всех игроков с одного подсервера на другой (например, в Limbo / Lobby). | `proxy = ["manage"]` |
| `title(who, title_text, subtitle_text) -> Result<(), ModuleError>` | Отправляет заголовок Title игроку или всем игрокам через прокси. | `proxy = ["title"]` |
| `subservers(proxy_id) -> Result<Vec<GameServer>, ModuleError>` | Возвращает список подсерверов, зарегистрированных за этим прокси. | `proxy = ["read"]` |
