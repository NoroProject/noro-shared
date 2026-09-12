//! Что модуль объявляет о себе кодом.
//!
//! Подписки, ручки и задачи живут рядом с обработчиками, а не в манифесте: имя
//! функции там дублировалось строкой, и опечатка в нём давала молча мёртвый
//! обработчик. Главное же — имя события выводится из типа аргумента
//! (`Event::NAME`), поэтому подписаться на одно событие, а принять структуру
//! другого больше нельзя: не соберётся.
//!
//! В манифесте остаётся только то, что нужно знать до того, как код
//! исполнится: идентификатор, версия, требуемый ABI, запрашиваемые
//! возможности, узлы прав. Узнать возможности из кода значило бы выполнить код
//! до того, как оператор решил, что ему можно.

use serde::{Deserialize, Serialize};

use crate::manifest::{Auth, Priority, SettingDecl, SettingKind};

/// Ответ экспорта `noro_register`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Registration {
    #[serde(default)]
    pub events: Vec<EventReg>,
    #[serde(default)]
    pub routes: Vec<RouteReg>,
    #[serde(default)]
    pub tasks: Vec<TaskReg>,
    /// Поля формы настроек.
    ///
    /// Тоже из кода, а не из манифеста: настройка почти всегда заводится
    /// вместе с кодом, который её читает, и держать их в разных файлах значит
    /// однажды удалить одно и забыть другое.
    #[serde(default)]
    pub settings: Vec<SettingDecl>,
}

/// Подписка на событие.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReg {
    /// Имя из каталога. Заполняется макросом из `Event::NAME`, руками не
    /// пишется.
    pub name: String,
    /// Экспорт модуля, который надо позвать.
    pub handler: String,
    #[serde(default)]
    pub priority: Priority,
}

/// Ручка под `/api/modules/<id>/…`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteReg {
    pub method: String,
    pub path: String,
    pub handler: String,
    #[serde(default)]
    pub auth: Auth,
}

/// Фоновая задача.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskReg {
    pub handler: String,
    /// Интервал: `30s`, `5m`, `1h`, `24h`.
    pub every: String,
}

impl Registration {
    /// Что не так с декларацией. Пустой список — можно поднимать модуль.
    ///
    /// Проверяет мастер после вызова `noro_register`: декларация приходит из
    /// чужого кода, и доверять ей на слово нельзя даже при полном доверии
    /// автору — опечатка остаётся опечаткой.
    pub fn violations(&self) -> Vec<String> {
        let mut out = Vec::new();

        for e in &self.events {
            if crate::events::find(&e.name).is_none() {
                out.push(format!("события «{}» не существует", e.name));
            }
            if e.handler.is_empty() {
                out.push(format!("у подписки на «{}» пустой обработчик", e.name));
            }
        }

        for r in &self.routes {
            if !r.path.starts_with('/') || r.path.contains("..") {
                out.push(format!("путь ручки «{}» недопустим", r.path));
            }
            let method = r.method.to_ascii_uppercase();
            if !["GET", "POST", "PUT", "PATCH", "DELETE"].contains(&method.as_str()) {
                out.push(format!("метод «{}» не поддерживается", r.method));
            }
            if let Auth::Permission(node) | Auth::Admin(node) = &r.auth {
                if node.is_empty() {
                    out.push(format!("у ручки «{}» пустой узел прав", r.path));
                }
            }
        }

        // Две ручки на один метод и путь — это вопрос «какая из них
        // сработает», на который нет хорошего ответа.
        let mut seen = Vec::new();
        for r in &self.routes {
            let key = (r.method.to_ascii_uppercase(), r.path.clone());
            if seen.contains(&key) {
                out.push(format!("ручка {} {} объявлена дважды", key.0, key.1));
            }
            seen.push(key);
        }

        for s in &self.settings {
            if s.key.is_empty() {
                out.push("у настройки пустой ключ".to_string());
            }
            if let (Some(min), Some(max)) = (s.min, s.max) {
                if min > max {
                    out.push(format!("у настройки «{}» минимум больше максимума", s.key));
                }
            }
        }

        for t in &self.tasks {
            if crate::validate::parse_every(&t.every).is_none() {
                out.push(format!(
                    "интервал задачи «{}» не разобран: ожидается 30s, 5m, 1h",
                    t.every
                ));
            }
        }

        out
    }

    /// Ручка, отвечающая этому запросу.
    pub fn route(&self, method: &str, path: &str) -> Option<&RouteReg> {
        self.routes
            .iter()
            .find(|r| r.path == path && r.method.eq_ignore_ascii_case(method))
    }

    /// Заводит поле настроек и отдаёт его для уточнений.
    ///
    /// ```ignore
    /// reg.setting("points_per_hour", Number, "mod-shop-per-hour")
    ///    .default(100)
    ///    .range(0, 10_000);
    /// ```
    pub fn setting(
        &mut self,
        key: impl Into<String>,
        kind: SettingKind,
        label: impl Into<String>,
    ) -> &mut SettingDecl {
        self.settings.push(SettingDecl {
            key: key.into(),
            kind,
            label: label.into(),
            hint: None,
            min: None,
            max: None,
            options: Vec::new(),
            default: None,
        });
        self.settings.last_mut().expect("только что добавили")
    }

    /// Подписки на событие, в порядке приоритета.
    pub fn subscriptions(&self, event: &str) -> Vec<&EventReg> {
        let mut out: Vec<&EventReg> = self.events.iter().filter(|e| e.name == event).collect();
        out.sort_by_key(|e| e.priority);
        out
    }
}

#[cfg(test)]
#[path = "registration_tests.rs"]
mod tests;
