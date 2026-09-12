//! Контекст события: где оно случилось и кто его вызвал.
//!
//! Без контекста событие бесполезно на инстансе с несколькими сборками: модуль
//! видит «игрок вошёл», но не видит куда, и вынужден догадываться по данным
//! самого события. Поэтому [`EventCtx`] едет с каждым событием, а не только с
//! теми, где сборка кажется важной.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Откуда пришло действие, породившее событие.
///
/// Различать источник приходится постоянно: модуль, выдающий награду за вход,
/// не должен срабатывать на техническое обновление профиля из CLI, а модуль
/// антифрода — наоборот, интересуется только веб-действиями.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Origin {
    /// Игровой сервер через агента.
    Game,
    /// Сайт, личный кабинет или админка.
    Web,
    /// Десктопный лаунчер.
    Launcher,
    /// `noro-admin` или админ-токен.
    Cli,
    /// Другой модуль. Позволяет не реагировать на собственное эхо.
    Module { id: String },
    /// Сам мастер: планировщик, миграция, истечение срока наказания.
    System,
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Origin::Game => f.write_str("game"),
            Origin::Web => f.write_str("web"),
            Origin::Launcher => f.write_str("launcher"),
            Origin::Cli => f.write_str("cli"),
            Origin::Module { id } => write!(f, "module:{id}"),
            Origin::System => f.write_str("system"),
        }
    }
}

/// Кто инициировал действие.
///
/// Отдельно от [`Origin`]: источник отвечает на «через что», актор — на «кто».
/// Бан может прийти из веба рукой модератора и из CLI тем же модератором.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ActorRef {
    User { id: Uuid, username: String },
    Module { id: String },
    Token { name: String },
    System,
}

impl ActorRef {
    /// Идентификатор пользователя, если действие совершил человек.
    pub fn user_id(&self) -> Option<Uuid> {
        match self {
            ActorRef::User { id, .. } => Some(*id),
            _ => None,
        }
    }
}

/// Обстоятельства события.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCtx {
    pub origin: Origin,
    pub actor: ActorRef,
    /// Сборка, к которой относится событие. `None` у глобальных: регистрация
    /// пользователя ни к одной сборке не привязана.
    #[serde(default)]
    pub server_id: Option<Uuid>,
    /// Слаг сборки — чтобы не ходить за ним отдельным вызовом ради строки лога.
    #[serde(default)]
    pub server_slug: Option<String>,
    /// Конкретный игровой сервер, если событие пришло из игры.
    #[serde(default)]
    pub game_server_id: Option<Uuid>,
    pub at: DateTime<Utc>,
    /// Глубина вложенности публикаций.
    ///
    /// Действие модуля порождает новые события, те будят другие модули, и без
    /// счётчика это замыкается в кольцо. Мастер перестаёт публиковать, когда
    /// глубина упирается в потолок.
    #[serde(default)]
    pub depth: u8,
}

impl EventCtx {
    /// Пришло ли событие из игры.
    pub fn from_game(&self) -> bool {
        matches!(self.origin, Origin::Game)
    }

    /// Породил ли это событие сам модуль с таким идентификатором.
    ///
    /// Обязательная проверка в обработчиках, которые сами меняют те же данные:
    /// иначе модуль реагирует на собственную запись и уходит в цикл.
    pub fn caused_by(&self, module_id: &str) -> bool {
        matches!(&self.origin, Origin::Module { id } if id == module_id)
    }

    /// Подпись сборки для логов: слаг, иначе идентификатор, иначе «глобально».
    pub fn server_name(&self) -> String {
        self.server_slug
            .clone()
            .or_else(|| self.server_id.map(|id| id.to_string()))
            .unwrap_or_else(|| "global".to_string())
    }
}
