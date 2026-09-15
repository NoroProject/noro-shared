---
title: Веб-сокеты и Realtime
description: Двусторонний обмен сообщениями в реальном времени между вкладками браузера и модулем.
---

Когда модуль управляет интерактивным веб-интерфейсом — живым аукционом, казино, динамическим мониторингом серверов или мгновенными оповещениями игроков — опрашивать HTTP-ручки каждые несколько секунд неэффективно и создаёт задержку.

Noro Master держит единое постоянное WebSocket-соединение (`/api/player/ws`) с каждым игроком, авторизованным в веб-кабинете или админ-панели. Ваш модуль подключается напрямую к этому транспорту в обоих направлениях.

## Декларативные действия с `#[ws_action]`

Когда вкладка браузера игрока отправляет JSON-сообщение с полем `"action"` или `"type"`, его можно направить напрямую в специализированный метод-обработчик с помощью атрибута `#[ws_action("имя_действия")]`:

```rust
use noro_sdk::prelude::*;

#[derive(Deserialize)]
struct BetPayload {
    amount: u64,
}

#[noro::module]
impl Casino {
    #[ws_action("place_bet")]
    fn on_bet(player: Player, Json(bet): Json<BetPayload>) -> Result<()> {
        log::info(format!("{} сделал ставку: {}", player.label(), bet.amount));

        // Мгновенный ответ игроку через веб-сокет:
        web_ws::send(player.id, json!({
            "type": "bet_confirmed",
            "amount": bet.amount
        }))?;
        Ok(())
    }
}
```

### Поддерживаемые экстракторы для `#[ws_action]`

Параметры инжектируются автоматически так же, как в `#[route]`:
- `Player` — данные авторизованного игрока
- `AuthUser` — UUID игрока (или ошибка авторизации)
- `OptionalUser` — опциональный UUID игрока
- `Json<T>` — тело сообщения, автоматически десериализованное в структуру `T`
- `RawParams` / `Value` — сырой JSON полезной нагрузки (`serde_json::Value`)
- `EventCtx` — контекст события (время, происхождение, инициатор)
- `WebMessage` — полное событие со всеми полями

### Ручная обработка через `#[event]`

Если вы хотите обрабатывать все входящие WebSocket-сообщения в единой функции вручную:

```rust
#[event]
fn on_web_message(e: WebMessage) -> Result<()> {
    log::info(format!("{} прислал: {:?}", e.player.label(), e.payload));
    Ok(())
}
```

Для приёма сообщений требуется разрешение `web_ws = ["read"]` в `noro.toml`:

```toml
[capabilities]
web_ws = ["read", "notify"]
```

## Подписка и отправка в Web UI (Фронтенд)

В вашем мини-аппе (компоненте Vue) используйте объект `noro.ws`, предоставляемый пакетом `@noroproject/module-ui`:

```ts
import { onUnmounted } from 'vue'
import { useNoro } from '@noroproject/module-ui'

const noro = useNoro()

// 1. Подписка на входящие сообщения, отправленные модулем:
const unsubscribe = noro.ws.on<{ type: string; balance?: number }>((data) => {
    if (data.type === 'balance_updated') {
        currentBalance.value = data.balance ?? 0
    }
})

// Отписка при размонтировании компонента:
onUnmounted(() => unsubscribe())

// 2. Отправка действия обратно в модуль по WebSocket:
function sendAction() {
    noro.ws.send({
        action: 'spin_wheel',
        bet: 50
    })
}
```

Никакой ручной настройки подключения, передачи токенов авторизации или логики реконнекта не нужно: `@noroproject/module-ui` использует уже открытое и авторизованное соединение мастера.

## Отправка сообщений из модуля (Push в браузер)

Чтобы отправлять данные из Rust во вкладки браузера игроков без ожидания входящего HTTP-запроса:

```rust
use noro_sdk::web_ws;

// Отправка конкретному игроку во все его открытые вкладки:
web_ws::send(player_id, json!({
    "type": "balance_updated",
    "balance": 1250
}))?;

// Широковещательная рассылка всем игрокам, у которых сейчас открыт сайт:
web_ws::broadcast(json!({
    "type": "announcement",
    "text": "Событие на сервере началось!"
}))?;
```

### Проверка онлайн-статуса

Вы можете проверить, открыт ли у игрока сайт в данный момент:

```rust
if web_ws::is_online(player_id)? {
    // Игрок сейчас прямо на сайте
}

let total_viewers = web_ws::online_count()?;
```

Для отправки сообщений и проверки статуса требуются разрешения `web_ws = ["notify"]` и `web_ws = ["read"]` в `[capabilities]`.
