---
title: Тестирование модулей
description: Написание быстрых изолированных юнит-тестов для WASM-модулей с noro_sdk::testing.
---

Тестирование бизнес-логики модулей не должно требовать запуска полного мастер-сервера, базы данных PostgreSQL или игрового клиента Minecraft.

Модуль `noro_sdk::testing` предоставляет фикстуры и генераторы мок-объектов. Вы можете тестировать обработчики событий, HTTP-ручки и действия WebSocket локально с помощью стандартных юнит-тестов Rust.

## Доступные мок-помощники

Импортируйте модуль тестирования в тестовый блок:

```rust
use noro_sdk::testing::*;
```

Модуль предоставляет четыре основных конструктора:

| Функция | Возвращает | Назначение |
|---|---|---|
| `mock_player(name)` | `Player` | Создаёт тестового игрока с идентификатором, никнеймом, UUID Minecraft и стандартными ролями |
| `mock_context()` | `EventCtx` | Создаёт контекст события (`Origin::Web`, `ActorRef::System`) |
| `mock_web_message(name, payload)` | `WebMessage` | Создаёт WebSocket-сообщение от тестового игрока с сериализованной полезной нагрузкой JSON |
| `mock_request(method, path, body, user)` | `HttpRequest` | Создаёт тестовый HTTP-запрос с методом, путём, телом JSON и опциональным ID пользователя |

## Тестирование обработчиков событий

Обработчики событий принимают строго типизированные структуры. Вы можете создавать их напрямую и проверять логику:

```rust
// В коде вашего модуля
pub fn check_chat_message(text: &str) -> bool {
    !text.contains("forbidden_word")
}

#[cfg(test)]
mod tests {
    use super::*;
    use noro_sdk::testing::*;

    #[test]
    fn test_chat_filter() {
        let player = mock_player("Steve");
        assert_eq!(player.label(), "Steve");
        assert!(!player.banned);

        assert!(check_chat_message("Привет, мир!"));
        assert!(!check_chat_message("Тут есть forbidden_word в тексте"));
    }
}
```

## Тестирование HTTP-ручек

Вы можете вызывать функции-обработчики маршрутов напрямую, передавая `HttpRequest`, созданный через `mock_request`:

```rust
use noro_sdk::prelude::*;

pub fn handle_profile_status(req: HttpRequest) -> Result<String> {
    let user_id = req.require_user()?;
    Ok(format!("Статус пользователя: {user_id}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use noro_sdk::testing::*;

    #[test]
    fn test_authenticated_endpoint() {
        let player = mock_player("Alex");
        let req = mock_request("GET", "/api/modules/stats/status", None, Some(player.id));

        let res = handle_profile_status(req).expect("обработчик должен вернуть успех");
        assert!(res.contains(&player.id.to_string()));
    }

    #[test]
    fn test_unauthenticated_fails() {
        let req = mock_request("GET", "/api/modules/stats/status", None, None);
        let err = handle_profile_status(req).expect_err("гость должен быть отклонён");
        assert_eq!(err.code(), "UNAUTHORIZED");
    }
}
```

## Тестирование WebSocket-действий

Когда мини-аппы отправляют сообщения через мост, мастер оборачивает их в `WebMessage`. Вы можете симулировать эти сообщения через `mock_web_message`:

```rust
use noro_sdk::prelude::*;

pub fn handle_bid(msg: WebMessage) -> Result<u64> {
    let amount = msg.payload.get("amount")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| ModuleError::invalid("missing amount"))?;

    Ok(amount * 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use noro_sdk::testing::*;
    use serde_json::json;

    #[test]
    fn test_websocket_bid() {
        let msg = mock_web_message("Dalynkaa", &json!({ "action": "bid", "amount": 250 }));

        assert_eq!(msg.player.label(), "Dalynkaa");
        let result = handle_bid(msg).unwrap();
        assert_eq!(result, 500);
    }
}
```

## Запуск тестов

Запускайте тесты на вашей рабочей машине стандартной командой Cargo:

```bash
cargo test
```

:::note[Хостовая архитектура против WASM]
Хотя модули компилируются в цель `wasm32-unknown-unknown` для создания пакета `.noromod`, тесты выполняются под вашей нативной хостовой системой (`x86_64` или `aarch64`). Это обеспечивает максимальную скорость выполнения без необходимости поднимать окружение WASM.
:::
