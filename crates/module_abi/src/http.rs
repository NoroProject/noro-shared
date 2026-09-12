//! Запрос к ручке модуля.
//!
//! Мастер разбирает HTTP сам и передаёт модулю уже готовое: разбирать заголовки
//! и куки в песочнице незачем, а решения по доступу принимаются до вызова.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// То, что модуль получает при вызове своей ручки.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    /// `GET`, `POST`, … — как объявлено в манифесте.
    pub method: String,
    /// Путь внутри модуля, начиная со слэша.
    pub path: String,
    /// Параметры строки запроса одним объектом.
    #[serde(default)]
    pub query: Value,
    /// Тело, если оно было.
    #[serde(default)]
    pub body: Option<Value>,
    /// Кто зовёт. Пусто у публичной ручки без входа.
    ///
    /// Проверять права заново не нужно: мастер уже сверил их с тем, что
    /// объявлено в манифесте, и до модуля чужой запрос не доходит.
    #[serde(default)]
    pub user: Option<Uuid>,
}

impl HttpRequest {
    /// Тело, разобранное в нужный тип.
    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Option<T> {
        serde_json::from_value(self.body.clone()?).ok()
    }

    /// Строковый параметр запроса.
    pub fn param(&self, name: &str) -> Option<&str> {
        self.query.get(name)?.as_str()
    }

    /// Кто зовёт, если ручка требует входа.
    pub fn require_user(&self) -> Result<Uuid, crate::error::ModuleError> {
        self.user
            .ok_or_else(|| crate::error::ModuleError::invalid("ручка вызвана без входа"))
    }
}
