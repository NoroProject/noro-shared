---
title: Возможности
description: Что модуль просит, что выдаёт оператор и что за этим стоит.
---

Модуль **просит**, оператор **выдаёт**. Совпадать эти списки не обязаны: выдать меньше
запрошенного — обычное и поддержанное дело. Вызов в невыданное возвращает
`CapabilityDenied` с именем недостающего и модуль не роняет.

```toml
[capabilities]
players = ["read"]
store = true
http = ["discord.com"]
```

Пустой список — это отказ. Домен, который вы не упомянули, закрыт.

## До чего дотягивается каждая

За каждым действием ниже стоит host-функция, и это утверждает тест — см. примечание после
таблицы.

| Возможность | Действия | Что можно звать |
|---|---|---|
| `players` | `read` | `players::get`, `players::require`, `players::capes` |
| | `ban` | `players::ban`, `players::unban` |
| | `rename` | `players::rename` |
| | `skin` | `players::set_skin`, `set_cape`, `presets`, `save_preset`, `delete_preset` |
| `identities` | `read` | `identities::of`, `identities::find` |
| | `link` | `identities::link`, `identities::unlink` |
| `roles` | `read` | `roles::list`, `roles::get`, `roles::of` |
| | `grant` | `roles::grant`, `roles::revoke` |
| | `manage` | `roles::create`, `update`, `delete` |
| `permissions` | `read` | `permissions::has`, `permissions::effective` (и варианты с `_on`) |
| | `grant` | `permissions::grant`, `permissions::revoke` |
| `access` | `grant` | `access::allow_join`, `access::allow_build`, и их `revoke_*` |
| `servers` | `read` | `servers::list`, `get`, `by_slug` |
| `builds` | `read` | `builds::of`, `builds::published`, `builds::files`, `builds::read` |
| | `files` | `builds::write`, `builds::attach`, `builds::remove` |
| | `publish` | `builds::unpublish` — саму публикацию делает оператор |
| `gameservers` | `read` | `servers::game_servers`, `servers::game_server` |
| | `maintenance` | `servers::set_maintenance` |
| `punish` | `read` | `punish::active`, `punish::history` |
| | `issue` | `punish::ban`, `mute`, `warn`, `server_ban`, `issue` |
| | `revoke` | `punish::revoke` |
| `bank` | `read` | `bank::account`, `accounts`, `balance`, `treasury` |
| | `transfer` | `bank::transfer`, `bank::transfer_once` |
| `agent` | `tell` | `chat::tell` |
| | `announce` | `chat::announce`, `chat::announce_on` |
| | `kick` | `chat::kick` |
| `optional_mods` | `grant` | `access::allow_mod`, `revoke_mod` |
| `news` | `read` | `news::list`, `news::get` |
| | `publish` | `news::publish`, `publish_pinned` |
| | `edit` | `news::edit`, `news::delete` |
| `instance` | `read` | `instance::all`, `instance::get` |
| | `write` | `instance::set` |
| `sessions` | `read` | `sessions::of` |
| | `revoke` | `sessions::revoke`, `revoke_all` |
| `restarts` | `read` | `restarts::of` |
| | `manage` | `restarts::add`, `restarts::remove` |
| `roster` | `read` | `roster::online`, `locate`, `is_online` |
| `telemetry` | `read` | `telemetry::of` |
| `files` | `read` | `files::read`, `exists`, `url` |
| | `write` | `files::put` |
| `tickets` | `read` | `tickets::queue`, `get`, `messages` |
| | `reply` | `tickets::reply`, `open`, `close` |
| `cases` | `read` | `cases::get`, `events`, `open_on` |
| | `claim` | `cases::claim` |
| | `resolve` | `cases::resolve` |
| `hub` | `read` | `hub::feed`, `members`, `playtime` |
| | `post` | `hub::post` |
| `towns` | `read` | `hub::towns`, `hub::town` |
| | `manage` | `hub::found_town` |
| `market` | `read` | `hub::market`, `hub::lot` |
| | `sell` | `hub::list_lot` |
| `court` | `read` | `hub::court`, `hub::court_case` |
| | `file` | `hub::file_claim` |
| `petitions` | `read` | `hub::petitions` |
| | `sign` | `hub::sign`, `hub::unsign` |
| | `create` | `hub::start_petition` |
| `fines` | `read` | `hub::fines`, `hub::fines_of` |
| | `issue` | `hub::fine` |
| `events` | `emit` | `events::emit`, `events::emit_on` |
| `launcher` | `read` | `launcher::is_online`, `launcher::connected` |
| | `notify` | `launcher::send`, `launcher::broadcast` |
| `store` | `true` | всё хранилище ключ-значение |
| `db` | `true` | `db::query`, `execute`, `one`, `scalar`, и ваши миграции |
| `http` | список разрешённых хостов | `http::send`, `http::get_json` |

## Та, у которой длинная рука

`builds = ["files"]` пишет то, что лаунчер скачает и положит человеку в каталог игры.
Переподписывать после этого нечего: манифест собирается и подписывается на каждый запрос
лаунчера из текущих строк, поэтому записанный файл живой со следующего же обращения. Между
вызовом и игроком нет ни одной проверки.

Поэтому это отдельное действие, а не часть `read`, и поэтому оператор видит его отдельной
строкой при установке. Текст кладётся напрямую и ограничен мегабайтом — это конфиги, а не
моды. Всё, что больше, кладут один раз через `files::put` и привязывают по хешу: тащить
десятки мегабайт строкой через песочницу — не план.

Публикация билда по-прежнему за оператором: она пересобирает артефакты и дотягивает
недостающее — минуты работы, а у вызова модуля секунды.

:::note[Список и код разъехаться не могут]
Каждое действие первой таблицы проверяется тестом, который читает исходники самого мастера:
и возможность, которую никто не проверяет, и проверка возможности, которую нельзя
запросить, роняют сборку. Семнадцать доменов однажды имели host-функции и не имели поля в
манифесте вовсе — просьба о них выбрасывалась без единого слова, и любой вызов отвергался,
что бы оператор ни выдал.
:::

## Действовать за кого-то

Некоторые записи называют игрока: пост в ленту, штраф, подпись петиции, взятие дела. Это
поступки **человека**, а у модуля человека за спиной нет: запись в ленте «ни от кого»
подсайт не умеет представить, да и ответить на неё некому.

Поэтому модуль говорит, чей это поступок, и появляется именно это имя. Автоматика может
действовать за кого-то, но не может быть кем-то. В журнале остаётся и то и другое: модуль,
сделавший вызов, и игрок, за которого он действовал.

## Записи аудируются как записи персонала

Роль, выданная модулем, или игрок, забаненный им, ложатся в общий журнал **тем же
действием**, каким легли бы у оператора, и подписью `модуль «ваш-id»`. Это намеренно: бан
обязан быть в том списке, который персонал и так читает, а не в отдельной ленте. Чтения не
аудируются — у них свой журнал вызовов.

Ничто из записанного модулем не приписывается человеку. `granted_by` остаётся пустым, а не
называет того, кто модуль включил: решения он не принимал, и вешать на него каждое
последующее действие было бы ложью, которую журнал уже не заберёт назад.

## Почему это слова, а не битовая маска

Оператор читает этот список целиком, прежде чем выдать. То, что перед оценкой надо
расшифровывать, оценивать не станут.

## Список хостов для HTTP

```toml
[capabilities]
http = ["discord.com", "*.example.org"]
```

Хосты, а не URL: путь или порт здесь отвергаются при установке. `*.example.org` покрывает
поддомены, но **не** сам `example.org`: шаблон не должен молча выдавать родителя, которого
никто не называл.

Список — только первая из трёх проверок. Имя резолвится, и запрос отвергается, если любой
полученный адрес лежит внутри машины или в приватной сети, а соединение потом идёт на
проверенный адрес, а не на тот, в который имя разрешится мгновением позже. Один список
стоил бы немногого: разрешённое имя, направленное на `127.0.0.1`, сделало бы из модуля
способ достать внутренние службы мастера. См. [наружу](../../guides/platform/#наружу).
