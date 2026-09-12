//! Своё хранилище модуля.
//!
//! Ключ-значение с областями: запись принадлежит либо всему инстансу, либо
//! сборке, либо игроку. Область — часть ключа, а не фильтр, поэтому `points`
//! игрока и `points` сборки не пересекаются и склеивать идентификатор в строку
//! руками не нужно.
//!
//! Данные сносятся вместе с модулем. Для таблиц, выборок и сортировок есть своя
//! схема Postgres — здесь их нет.

use noro_module_abi::error::ModuleError;
use noro_module_abi::store::{StoreGet, StoreIncr, StoreList, StoreScope, StoreSet};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

/// Хранилище в выбранной области.
pub struct Store(StoreScope);

/// Общее на весь инстанс: настройки модуля, счётчики, отметки.
pub fn instance() -> Store {
    Store(StoreScope::Instance)
}

/// Своё для каждой сборки.
pub fn server(id: Uuid) -> Store {
    Store(StoreScope::server(id))
}

/// Своё для каждого игрока. Сносится вместе с игроком.
pub fn user(id: Uuid) -> Store {
    Store(StoreScope::user(id))
}

impl Store {
    /// Читает значение и разбирает его в нужный тип.
    ///
    /// `None` — ключа нет. Если значение есть, но не разбирается в `T`, это
    /// ошибка: молчаливое `None` здесь означало бы потерю данных, которую
    /// заметят через неделю.
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, ModuleError> {
        let raw: Option<Value> = crate::host::store_get_call(StoreGet {
            scope: self.0,
            key: key.to_string(),
        })?;
        match raw {
            None => Ok(None),
            Some(v) => serde_json::from_value(v).map(Some).map_err(|e| {
                ModuleError::invalid(format!("значение под ключом «{key}» не разобрано: {e}"))
            }),
        }
    }

    /// Читает значение, подставляя умолчание, если ключа нет.
    pub fn get_or<T: DeserializeOwned + Default>(&self, key: &str) -> Result<T, ModuleError> {
        Ok(self.get(key)?.unwrap_or_default())
    }

    pub fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<(), ModuleError> {
        let value = serde_json::to_value(value)
            .map_err(|e| ModuleError::invalid(format!("значение не упаковано: {e}")))?;
        crate::host::store_set_call(StoreSet {
            scope: self.0,
            key: key.to_string(),
            value,
        })
    }

    /// Удаляет ключ. `true` — он был.
    pub fn delete(&self, key: &str) -> Result<bool, ModuleError> {
        crate::host::store_delete_call(StoreGet {
            scope: self.0,
            key: key.to_string(),
        })
    }

    /// Прибавляет к числу и возвращает результат.
    ///
    /// Одной операцией, а не чтением с записью: обработчики событий вполне
    /// могут выполняться одновременно, и пара «прочитал, прибавил, записал»
    /// теряла бы начисления.
    ///
    /// Если под ключом лежит не число — отказ, а не перезапись.
    pub fn incr(&self, key: &str, delta: i64) -> Result<i64, ModuleError> {
        crate::host::store_incr_call(StoreIncr {
            scope: self.0,
            key: key.to_string(),
            delta,
        })
    }

    /// Ключи и значения по префиксу, страницами.
    pub fn list(
        &self,
        prefix: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<(String, Value)>, ModuleError> {
        crate::host::store_list_call(StoreList {
            scope: self.0,
            prefix: prefix.to_string(),
            limit,
            offset,
        })
    }
}
