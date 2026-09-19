---
title: Межмодульный RPC
description: Предоставление методов для других модулей и прямой вызов процедур между модулями.
---

Модули в Noro выполняются в изолированных песочницах WebAssembly с собственным хранилищем и схемами базы данных. Когда нескольким модулям необходимо взаимодействовать друг с другом — например, модуль квестов проверяет наличие предмета в модуле инвентаря или система аукциона запрашивает баланс игрока, — межмодульный RPC обеспечивает прямой строго типизированный вызов методов между ними.

Вызовы происходят внутри процесса мастер-сервера без накладных расходов HTTP, сетевых задержек и связывания схем баз данных.

## Объявление RPC-метода

Чтобы метод стал доступен другим модулям, пометьте его атрибутом `#[rpc]` внутри блока `#[noro::module]`:

```rust
use noro_sdk::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct DiscountReq {
    pub player_id: Uuid,
    pub original_price: u64,
}

#[derive(Serialize, Deserialize)]
pub struct DiscountRes {
    pub final_price: u64,
    pub discount_applied: u64,
}

#[noro::module]
impl LoyaltyModule {
    #[rpc("calculate_discount")]
    fn calculate_discount(Json(req): Json<DiscountReq>) -> Result<DiscountRes> {
        let discount = req.original_price / 10; // Скидка 10%
        Ok(DiscountRes {
            final_price: req.original_price - discount,
            discount_applied: discount,
        })
    }
}
```

### Именование методов

- `#[rpc("пользовательское_имя")]`: Явно задаёт имя вызываемого метода.
- `#[rpc]`: Использует имя функции Rust в качестве имени метода.

### Параметры и возвращаемые значения

- Параметры извлекаются с помощью декларативного экстрактора `Json<T>`.
- Возвращаемым типом может быть любая сериализуемая структура, обёрнутая в `Result<T>` или `Result<T, ModuleError>`.

## Вызов RPC-метода из другого модуля

Используйте функцию `modules::call`:

```rust
use noro_sdk::prelude::*;

let result: DiscountRes = modules::call(
    "loyalty",
    "calculate_discount",
    &DiscountReq {
        player_id: player.id,
        original_price: 1000,
    },
)?;

log::info(format!("Итоговая цена со скидкой: {}", result.final_price));
```

Параметры вызова:
1. `target: &str` — идентификатор целевого модуля из его `manifest.toml`.
2. `method: &str` — имя объявленного RPC-метода.
3. `payload: &P` — ссылка на сериализуемую полезную нагрузку запроса.

Функция `modules::call` возвращает `Result<R, ModuleError>`, автоматически десериализуя ответ в запрошенный тип `R`.

## Требуемые разрешения

Вызов других модулей требует объявления разрешения `modules` в `manifest.toml`:

```toml
[capabilities]
modules = ["call"]
```

Без этого разрешения попытка вызова `modules::call` будет отклонена мастер-сервером с ошибкой доступа.

## Обработка ошибок

| Ситуация | Результат |
|---|---|
| Целевой модуль не установлен или отключён | `Err(ModuleError::not_found("module not found"))` |
| Метод не найден в целевом модуле | `Err(ModuleError::not_found("route not found"))` |
| У вызывающего модуля нет разрешения `modules = ["call"]` | `Err(ModuleError::permission_denied(...))` |
| Обработчик вернул ошибку `Err(ModuleError)` | Ошибка передаётся вызывающему коду |
| Обработчик запаниковал | Возвращается ошибка и регистрируется сбой целевого модуля |

Поскольку зависимый модуль может отсутствовать на конкретном сервере, вызывающий код должен корректно обрабатывать `ModuleError::not_found`, если интеграция является опциональной.
