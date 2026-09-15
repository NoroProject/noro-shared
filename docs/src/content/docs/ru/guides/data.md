---
title: Хранение данных
description: Ключ-значение по областям и своя схема Postgres.
---

## Ключ-значение

У модуля есть хранилище с тремя областями. Область — часть ключа, а не фильтр: `points`
игрока и `points` сборки не столкнутся никогда, и склеивать идентификаторы в строки руками
не придётся.

```rust
store::instance().set("last_payout", &now())?;
store::server(server_id).set("cfg", &ShopCfg { tax: 5 })?;
store::user(player_id).incr("points", 10)?;

let cfg: Option<ShopCfg> = store::server(server_id).get("cfg")?;
let points = store::user(player_id).get_or::<i64>("points")?;
```

Значением может быть всё, что умеет `serde`. Нужно `store = true`.

### Как дотянуться от сущности

```rust
#[event]
fn on_join(e: PlayerJoined) -> Result<()> {
    e.player.store().incr("joins", 1)?;
    Ok(())
}
```

`player.store()` — это `store::user(player.id)`, а `server.store()` —
`store::server(server.id)`: та же область, записанная без крюка через идентификатор.

У билда и у игрового сервера `store()` нет. Областей три — инстанс, сборка, игрок, — и ни
одна из этих двух сущностей не является областью. `store()` у игрового сервера пришлось бы
отдать областью его сборки, и два игровых сервера одной сборки молча делили бы ключ,
который читается как их собственный.

### get возвращает None только когда ключа нет

Если значение есть, но не разбирается в запрошенный тип — это ошибка, а не `None`. Тихий
`None` был бы потерей данных, которую замечают через неделю.

### incr — одна операция

```rust
let total = store::user(id).incr("points", 10)?;
```

Не чтение с последующей записью. Обработчики событий могут идти одновременно, и тройка
«прочитал — сложил — записал» теряла бы начисления. Если под ключом лежит не число, вызов
падает, а не затирает его.

### Перебор

```rust
for (key, value) in store::instance().list("payout:", 100, 0)? {
    // …
}
```

По префиксу, страницами, не больше 500 за вызов. Чтения без страниц здесь нет — ни здесь,
ни где-либо ещё на этой платформе.

### Время жизни

Данные под `user` уходят вместе с игроком, под `server` — вместе со сборкой. Всё уходит
при удалении модуля.

## Своя схема Postgres

Положите в пакет `migrations/0001_init.sql`, и мастер заведёт схему `mod_<ваш_id>`,
накатит миграции и запомнит накатанное. При удалении схема сносится вместе со всем, что в
ней лежит.

```sql
-- migrations/0001_init.sql
CREATE TABLE orders (
    id          bigserial PRIMARY KEY,
    player_id   uuid NOT NULL,
    total       bigint NOT NULL,
    paid_at     timestamptz
);
```

Дальше — запросы:

```rust
db::execute(query!(
    "INSERT INTO orders (player_id, total) VALUES ($1, $2)",
    player.to_string(),
    500
))?;

// Строгая типизация строк таблицы:
let orders: Vec<Order> = db::query_as(query!(
    "SELECT * FROM orders WHERE paid_at > $1",
    since
))?;

// Получение одной строки или скаляра:
let latest: Option<Order> = db::one_as(query!("SELECT * FROM orders ORDER BY id DESC LIMIT 1"))?;
let count: Option<i64> = db::scalar(query!("SELECT count(*) FROM orders"))?;
```

Строки приходят объектами с именами колонок, а не позиционными массивами: запрос, у
которого прибавилась колонка, не должен молча сдвинуть все индексы в читающем его коде.

### Значения привязываются, а не склеиваются

`$1`, `$2`, … и `.bind(…)`. Собранный через `format!` запрос работает ровно до первого
ника с апострофом, и это ещё хороший случай.

Типы JSON ложатся в Postgres ожидаемо: строка в `text`, целое в `bigint`, дробное в
`double precision`, булево в `boolean`, null в NULL, а объект или массив — в `jsonb`.

### До чего не дотянуться

Запросы идут с путём поиска, прибитым к вашей схеме, и под ограниченной ролью Postgres,
у которой больше нигде прав нет. `SELECT * FROM users` отвергает **сама база**, а не
проверка, которую кто-то должен не забыть написать:

```text
ERROR: permission denied for table users
```

Данные платформы берутся из типизированных доменов, где проверяются возможности и проекция
сделана осознанно. Если роль завести не удалось — некоторые управляемые Postgres этого не
дают, — мастер говорит об этом на старте, и границей остаётся только путь поиска. Делать
вид, что это одно и то же, он не станет.

### Пределы

Запрос, возвращающий больше десяти тысяч строк, отвергается, а не отдаётся: строки едут в
память песочницы, и `SELECT *` по таблице на миллион унёс бы модуль с собой. Добавьте
`LIMIT`.

## Миграции сверяются по контрольной сумме

Правка уже накатанной миграции — это способ остановить загрузку модуля. Добавьте вместо неё
новый файл. То же правило и по той же причине действует для миграций самого мастера.

## Настройки модуля с `#[derive(Settings)]`

Для удобного объявления и чтения настроек модуля, редактируемых оператором в панели управления:

```rust
#[derive(Settings, Serialize, Deserialize, Default)]
pub struct ModuleConfig {
    #[setting(label = "mod-rewards-interval", type = "number", min = 1, max = 3600)]
    pub reward_interval_secs: i64,

    #[setting(label = "mod-rewards-enabled", type = "toggle")]
    pub enabled: bool,
}
```

Вызов `ModuleConfig::load()?` в обработчиках событий, ручках или задачах загружает актуальные значения напрямую из хранилища.

## In-memory TTL-кеш

Когда модулю требуется временное, энергозависимое хранилище с автоочисткой по истечении срока (токены подтверждения, состояние сессий, предварительно вычисленные результаты):

```rust
use noro_sdk::prelude::*;

// Кеширование сериализуемой структуры с временем жизни (TTL) в секундах:
cache::set("verify_token:12345", &VerifyData { user_id, attempts: 0 }, 300)?; // 5 минут

// Получение значения:
let cached: Option<VerifyData> = cache::get("verify_token:12345")?;

// Получение со значением по умолчанию:
let count: u64 = cache::get_or("global_counter")?;

// Удаление ключа досрочно:
cache::delete("verify_token:12345")?;
```

Требует разрешения:
```toml
[capabilities]
cache = ["read", "write"]
```

## Межмодульный RPC (`#[rpc]` и `modules::call`)

Модули могут предоставлять API друг другу и вызывать функции соседних модулей.

### Объявление RPC-метода

Пометьте метод атрибутом `#[rpc("имя_метода")]` (или просто `#[rpc]` для имени функции):

```rust
#[derive(Serialize, Deserialize)]
pub struct DiscountReq {
    pub player_id: Uuid,
    pub price: u64,
}

#[noro::module]
impl ShopService {
    #[rpc("calc_discount")]
    fn calc_discount(Json(req): Json<DiscountReq>) -> Result<u64> {
        let discount = req.price / 10;
        Ok(req.price - discount)
    }
}
```

### Вызов из другого модуля

Используйте `modules::call`:

```rust
let final_price: u64 = modules::call(
    "shop_service",
    "calc_discount",
    &DiscountReq { player_id, price: 100 },
)?;
```

Требует разрешения:
```toml
[capabilities]
modules = ["call"]
```

## Модульное тестирование с `noro_sdk::testing`

Пишите локальные юнит-тесты на обработчики событий, WebSocket-действия и HTTP-ручки без необходимости запускать процесс мастера:

```rust
#[cfg(test)]
mod tests {
    use noro_sdk::testing::*;
    use noro_sdk::prelude::*;

    #[test]
    fn test_my_handler() {
        let msg = mock_web_message("Steve", &json!({ "action": "bid", "amount": 100 }));
        assert_eq!(msg.player.label(), "Steve");

        let req = mock_request("GET", "/status", None, Some(msg.player.id));
        assert_eq!(req.method, "GET");
    }
}
```
