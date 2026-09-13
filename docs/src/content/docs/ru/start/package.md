---
title: Внутри пакета
description: Что лежит в .noromod и где проходит граница между манифестом и кодом.
---

`.noromod` — это zip. Собирает его `cargo noro package`, а внутри оказывается вот что.

```
manifest.toml          обязателен — кто вы и что просите
module.wasm            обязателен — ваши скомпилированные обработчики
web/                   файлы мини-аппа, если он есть
locales/{en,ru}.ftl    ваши тексты, ключи с префиксом mod-<id>-
migrations/NNNN_*.sql  миграции вашей схемы Postgres
icon.png               128×128
```

## Манифест — это паспорт

В нём только то, что нужно знать **до** того, как запустится ваш код:

```toml
[module]
id = "playtime-rewards"
name = "Playtime Rewards"
version = "1.2.0"
api = "1.0"                    # требуемая версия ABI; мастер сверяет мажор
scope = "instance"             # instance | server

[capabilities]
players = ["read"]
store = true

[[apps]]
placement = "cabinet"          # admin | hub | cabinet | widget
kind = "vue"                   # vue | page
entry = "app.js"               # относительно web/ внутри пакета
title = "mod-playtime-rewards-title"
icon = "i-lucide-gift"

[[permissions]]
node = "noro.module.playtime-rewards.view"
label = "mod-playtime-rewards-perm-view"
```

Возможности не могли бы приехать из кода даже в принципе: чтобы их прочитать, пришлось бы
запустить модуль до того, как оператор решил, что ему это позволено.

## Всё остальное — код

Подписки, ручки, задачи и форма настроек объявляются атрибутами, а `#[noro::module]`
собирает их в декларацию, которую мастер спрашивает при установке:

```rust
#[noro::module]
impl Shop {
    #[register]
    fn setup(reg: &mut Registration) {
        reg.setting("tax", SettingKind::Number, "mod-shop-tax")
            .default(5)
            .range(0, 100);
    }

    #[init]
    fn start() -> Result<()> {
        log::info("магазин поднялся");
        Ok(())
    }

    #[event(priority = high)]
    fn on_transfer(e: BankPreTransfer) -> Result<()> { Ok(()) }

    #[route(POST, "/buy", auth = permission("noro.module.shop.buy"))]
    fn buy(req: HttpRequest) -> Result<u64> { Ok(0) }
}
```

Раньше это жило в манифесте, и было в этом две беды. Имя обработчика повторялось строкой,
и опечатка давала обработчик, который молча никогда не звали. И имя события было строкой
тоже — ничто не мешало подписаться на `player.joined`, принимая `PlayerLeft`. Теперь имя
берётся из типа аргумента, и такая ошибка не компилируется.

| Атрибут | Сигнатура | Что делает |
|---|---|---|
| `#[event]` | `fn(E) -> Result<()>` | подписка; имя берётся из `E` |
| `#[event(priority = high)]` | та же | `lowest` · `low` · `normal` · `high` · `highest` · `monitor` |
| `#[route(GET, "/path")]` | `fn(HttpRequest) -> Result<T>` | ручка под `/api/modules/<id>/path` |
| `#[route(POST, "/p", auth = public)]` | та же | `public` · `user` · `permission("node")` · `admin("node")` |
| `#[task("1h")]` | `fn() -> Result<()>` | по расписанию: `30s` · `5m` · `1h` · `2d` — см. [задачи](../../guides/tasks/) |
| `#[register]` | `fn(&mut Registration)` | добавляет поля настроек |
| `#[init]` | `fn() -> Result<()>` | разовая работа при включении |

Приоритет по умолчанию `normal`, доступ к ручке — `user`. Публичную ручку надо попросить;
она никогда не получается из забытого аргумента.

:::note[Внутри атрибутов автодополнения нет]
Редактор не знает грамматику чужого макроса, пока тот не развернулся, — это верно и для
`#[serde(…)]`, и для `#[clap(…)]`. Наведение на `#[noro::module]` показывает всю таблицу
выше, а опечатка внутри атрибута даёт ошибку со списком допустимых значений.
:::

## Локали

Каждый ваш ключ обязан начинаться с `mod-<id>-`. Мастер проверяет это при установке,
поэтому столкнуться с ключом панели или другого модуля ваши не могут. Правка оператора
для того же ключа побеждает вашу.
