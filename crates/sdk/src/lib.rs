//! SDK для написания модулей Noro.
//!
//! Модуль собирается в `wasm32-unknown-unknown` и ставится в мастер одним
//! файлом. Внутри мастера он исполняется в песочнице, но вызовы отсюда — не
//! сетевые: каждая функция этого SDK попадает в тот же код, который обслуживает
//! панель администратора.
//!
//! # Как выглядит модуль
//!
//! ```ignore
//! use noro_sdk::prelude::*;
//!
//! pub struct Greeter;
//!
//! #[noro::module]
//! impl Greeter {
//!     /// Здоровается с теми, кто зашёл впервые.
//!     #[event]
//!     fn on_join(e: PlayerJoined) -> Result<()> {
//!         if e.first_join {
//!             log::info(format!("{} впервые на {}", e.player.label(), e.ctx.server_name()));
//!         }
//!         Ok(())
//!     }
//!
//!     /// Своя ручка: `GET /api/modules/<id>/me`.
//!     #[route(GET, "/me")]
//!     fn me(req: HttpRequest) -> Result<u64> {
//!         Ok(store::user(req.require_user()?).get("points")?.unwrap_or(0))
//!     }
//! }
//! ```
//!
//! Ничего из этого не дублируется в манифесте. Имя события мастер выводит из
//! типа аргумента, поэтому подписаться на одно событие, а принять структуру
//! другого нельзя — не соберётся. В манифесте остаётся только то, что нужно
//! знать **до** запуска кода: идентификатор, версия, требуемый ABI и
//! запрашиваемые возможности.
//!
//! # Отказы
//!
//! Каждый вызов может вернуть [`ModuleError`]. Самая частая причина —
//! возможность, которую оператор не выдал модулю при установке: тогда придёт
//! `CapabilityDenied` с названием того, чего не хватает.
//!
//! # Сторонние библиотеки
//!
//! Подключаются обычным `cargo add` и вкомпилируются прямо в `.wasm`. Ни
//! shading, ни relocation, ни объявления зависимостей в манифесте, как в
//! плагинах Paper, не нужно: у каждого модуля свой `.wasm` со своей копией,
//! поэтому два модуля с разными версиями одной библиотеки не конфликтуют в
//! принципе. Пакет получается самодостаточным и не зависит от того, доступен
//! ли чей-то репозиторий в момент установки.
//!
//! Ограничение одно: библиотека должна собираться под
//! `wasm32-unknown-unknown`. Чистые вычисления идут как есть — `regex`,
//! `rust_decimal`, `serde`, `sha2`, `base64`. А вот всё, что лезет в
//! окружение, там не работает и работать не может:
//!
//! | Что нужно библиотеке | Почему не выйдет | Чем заменить |
//! |---|---|---|
//! | Сеть (`reqwest`, `tokio`) | у песочницы нет сокетов | host-функция HTTP по allowlist из манифеста |
//! | Системные часы (`chrono` с `clock`) | часов у wasm нет, вернётся эпоха | [`now()`] |
//! | Случайность (`uuid/v4`, `rand`) | нет источника энтропии | идентификаторы приходят от мастера |
//! | Файлы, потоки, процессы | песочница их не даёт | [`store`] и своя схема Postgres |
//!
//! Зависимости с такими фичами обычно достаточно подключить с
//! `default-features = false` — именно так сделано с `uuid` и `chrono` в самом
//! SDK.
//!
//! Второе, о чём стоит помнить, — размер: всё подключённое едет внутри `.wasm`
//! и занимает память инстанса.

pub mod host;
pub mod log;
pub mod players;
pub mod store;

pub use noro_module_abi as abi;

/// Слой песочницы. Реэкспортирован, потому что `#[plugin_fn]` раскрывается в
/// пути вида `extism_pdk::…` — без этого имени в области видимости макрос не
/// собирается, и каждому модулю пришлось бы подключать extism-pdk отдельной
/// зависимостью, зная про него.
pub use extism_pdk;

/// Тот же serde_json, что и у макросов: они раскрываются в `noro_sdk::serde_json::…`.
pub use serde_json;

/// Разметка обработчиков: `#[noro::module]`, `#[event]`, `#[route]`, `#[task]`.
pub use noro_sdk_macros as noro;

/// Чем кончился вызов. Ошибка одна на весь SDK — [`abi::error::ModuleError`].
pub type Result<T> = core::result::Result<T, abi::error::ModuleError>;

/// Всё, что нужно обычному модулю, одним `use`.
pub mod prelude {
    pub use crate::abi::error::{ErrorKind, ModuleError};
    pub use crate::abi::events::*;
    pub use crate::abi::manifest::{Priority, SettingKind};
    pub use crate::abi::HttpRequest;
    pub use crate::abi::Registration;
    pub use crate::abi::{
        Account, ActorRef, Build, EventCtx, GameServer, IntoPlayerRef, Origin, Player, PlayerRef,
        Punishment, Role, Server,
    };
    pub use crate::{log, noro, now, players, store, Result};

    // Сам крейт, а не только его имена: макрос `plugin_fn` раскрывается в
    // `extism_pdk::…`, и без этого импорта модуль не собрался бы.
    pub use crate::extism_pdk;
    pub use extism_pdk::{plugin_fn, FnResult, Json};
    pub use serde::{Deserialize, Serialize};
    pub use uuid::Uuid;
}

/// Текущее время мастера.
///
/// Часы берутся у хоста: у wasm своих нет, а `SystemTime::now()` в этой цели
/// либо не собирается, либо возвращает эпоху.
pub fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::from_timestamp(host::now_secs(), 0).unwrap_or_default()
}
