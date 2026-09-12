//! Манифест модуля — `manifest.toml` внутри пакета.
//!
//! Файл длиннее ста пятидесяти строк намеренно: это одна декларация, и разносить
//! её половины по разным файлам значило бы искать поля манифеста в двух местах.
//!
//! Манифест разбирают двое: мастер при установке и `cargo-noro` при сборке
//! пакета. Поэтому типы живут здесь, а не в приватном коде мастера — иначе
//! автор модуля не смог бы проверить свой манифест, не поставив его.
//!
//! Здесь только то, что нужно знать **до** того, как код модуля исполнится:
//! кто он, какой ABI ему нужен, что он просит и какие узлы прав заводит. Что
//! он делает — события, ручки, задачи — объявляется кодом и приезжает
//! [`crate::Registration`]: имя обработчика там не дублируется строкой, а имя
//! события выводится из типа.

use serde::{Deserialize, Serialize};

/// Разобранный `manifest.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub module: ModuleMeta,
    #[serde(default)]
    pub capabilities: Capabilities,
    #[serde(default)]
    pub apps: Vec<AppDecl>,
    #[serde(default)]
    pub permissions: Vec<PermissionDecl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMeta {
    /// Только `[a-z0-9-]`: идентификатор едет в имя схемы Postgres, в префикс
    /// ключей локали и в URL, и экранировать его в трёх местах никто не станет.
    pub id: String,
    pub name: String,
    pub version: String,
    /// Требуемая версия ABI, например `"1.0"`. Мастер сверяет мажор.
    pub api: String,
    #[serde(default)]
    pub scope: Scope,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub homepage: Option<String>,
}

/// Где работает модуль.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// На весь инстанс сразу. Видит и глобальные события вроде регистрации.
    #[default]
    Instance,
    /// Включается на выбранных сборках. Событий без привязки к сборке не
    /// получает вовсе: адресовать их было бы некуда.
    Server,
}

/// Что модулю разрешено трогать.
///
/// Пустой набор — запрет. Оператор видит этот список при установке целиком,
/// поэтому названия полей должны читаться человеком, а не быть битовой маской.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Capabilities {
    /// `read`, `ban`, `rename`.
    #[serde(default)]
    pub players: Vec<String>,
    /// `read`, `link`.
    #[serde(default)]
    pub identities: Vec<String>,
    /// `read`, `grant`.
    #[serde(default)]
    pub roles: Vec<String>,
    /// `read`, `grant`.
    #[serde(default)]
    pub permissions: Vec<String>,
    /// `grant` — выдача доступа к сборкам и билдам.
    #[serde(default)]
    pub access: Vec<String>,
    /// `read`.
    #[serde(default)]
    pub servers: Vec<String>,
    /// `read`, `maintenance`.
    #[serde(default)]
    pub gameservers: Vec<String>,
    /// `read`, `publish`.
    #[serde(default)]
    pub builds: Vec<String>,
    /// `read`, `transfer`.
    #[serde(default)]
    pub bank: Vec<String>,
    /// `issue`, `revoke`.
    #[serde(default)]
    pub punish: Vec<String>,
    /// `tell`, `announce`, `kick`.
    #[serde(default)]
    pub agent: Vec<String>,
    /// Своё KV-хранилище.
    #[serde(default)]
    pub store: bool,
    /// Своя схема Postgres.
    #[serde(default)]
    pub db: bool,
    /// Хосты, куда модулю позволено ходить. Пусто — наружу нельзя.
    #[serde(default)]
    pub http: Vec<String>,
}

impl Capabilities {
    /// Разрешено ли действие в домене. Домен без действий закрыт целиком.
    pub fn allows(&self, domain: &str, action: &str) -> bool {
        let list = match domain {
            "players" => &self.players,
            "identities" => &self.identities,
            "roles" => &self.roles,
            "permissions" => &self.permissions,
            "access" => &self.access,
            "servers" => &self.servers,
            "gameservers" => &self.gameservers,
            "builds" => &self.builds,
            "bank" => &self.bank,
            "punish" => &self.punish,
            "agent" => &self.agent,
            "store" => return self.store,
            "db" => return self.db,
            _ => return false,
        };
        list.iter().any(|a| a == action)
    }

    /// Позволено ли ходить на этот хост.
    pub fn allows_host(&self, host: &str) -> bool {
        self.http.iter().any(|h| {
            h == host
                || h.strip_prefix("*.")
                    .is_some_and(|suffix| host.ends_with(suffix) && host.len() > suffix.len())
        })
    }
}

/// Порядок обработчиков одного события.
///
/// Повторяет привычную из Bukkit лестницу: тот, кто решает, идёт позже тех, кто
/// смотрит. `Monitor` — для наблюдателей, его решение об отмене игнорируется.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Lowest,
    Low,
    #[default]
    Normal,
    High,
    Highest,
    Monitor,
}

/// Кого пускать в ручку.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Auth {
    /// Любой, даже неавторизованный. Для приёма вебхуков извне.
    Public,
    /// Любой вошедший игрок.
    #[default]
    User,
    /// Игрок с правом.
    Permission(String),
    /// Только админ-токен или персонал с правом.
    Admin(String),
}

/// Мини-апп и место, куда он встраивается.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDecl {
    pub placement: Placement,
    /// Чем это открывается: страницей в песочнице или компонентом панели.
    #[serde(default)]
    pub kind: AppKind,
    /// Файл внутри `web/` пакета.
    pub entry: String,
    /// Ключ Fluent с названием пункта.
    pub title: String,
    /// Имя иконки `i-lucide-*`.
    #[serde(default)]
    pub icon: Option<String>,
    /// Право, без которого пункт не показывается.
    #[serde(default)]
    pub permission: Option<String>,
    /// Слот, если `placement = "widget"`.
    #[serde(default)]
    pub slot: Option<String>,
}

/// Как мини-апп попадает на экран.
///
/// Выбор не про удобство, а про изоляцию. `Page` — отдельная страница в
/// песочнице: своя область, ничего чужого не видит, общается через мост.
/// `Vue` — компонент, который панель монтирует у себя: полный Vue, её атомы и
/// её же вид, но и её окружение целиком, включая возможность сломать страницу.
///
/// Второе годится потому, что модули ставит сам владелец инстанса. Для чужого
/// кода остаётся первое.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppKind {
    /// Страница в iframe. Значение по умолчанию: изоляция не должна теряться
    /// из-за забытого поля.
    #[default]
    Page,
    /// Компонент Vue, собранный модулем и смонтированный панелью.
    Vue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Placement {
    /// Раздел в админке.
    Admin,
    /// Раздел в подсайте сборки.
    Hub,
    /// Страница в личном кабинете.
    Cabinet,
    /// Виджет в чужой странице, место задаёт `slot`.
    Widget,
}

/// Поле формы настроек. Повторяет модель полей настроек подсайта, чтобы
/// админка рисовала их тем же кодом.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingDecl {
    pub key: String,
    #[serde(rename = "type")]
    pub kind: SettingKind,
    /// Ключ Fluent с подписью поля.
    pub label: String,
    #[serde(default)]
    pub hint: Option<String>,
    #[serde(default)]
    pub min: Option<i64>,
    #[serde(default)]
    pub max: Option<i64>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

impl SettingDecl {
    /// Значение, которое подставляется, пока оператор ничего не выбрал.
    pub fn default(&mut self, value: impl Into<serde_json::Value>) -> &mut Self {
        self.default = Some(value.into());
        self
    }

    /// Границы для числа. Их проверяет админка, а не модуль.
    pub fn range(&mut self, min: i64, max: i64) -> &mut Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// Пояснение под полем. Ключ Fluent, как и подпись.
    pub fn hint(&mut self, key: impl Into<String>) -> &mut Self {
        self.hint = Some(key.into());
        self
    }

    /// Варианты для `select`.
    pub fn options(&mut self, values: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.options = values.into_iter().map(Into::into).collect();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingKind {
    Number,
    Text,
    Toggle,
    Select,
    /// Список строк через запятую.
    List,
}

/// Узел прав, который заводит модуль.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionDecl {
    /// Обязан начинаться с `noro.module.<id>.`.
    pub node: String,
    /// Ключ Fluent с пояснением.
    pub label: String,
}
