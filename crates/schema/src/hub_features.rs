//! Разделы подсайта и их зависимости.
//!
//! Зависимости двух видов, и путать их нельзя.
//!
//! **Раздел от раздела** — когда без основы не остаётся ничего: штраф это
//! требование заплатить, и без счетов платить нечем и некуда.
//!
//! **Настройка от раздела** — когда раздел работает, а отдельный его параметр
//! нет. Суд разбирает спор, а петиция собирает подписи и без всякой экономики;
//! деньги нужны только пошлине. Поэтому суды и петиции включаются при
//! выключенном банке — просто бесплатными.
//!
//! Объявлено здесь одной строкой, а проверяют и мастер, и форма в админке. Два
//! списка разошлись бы: интерфейс предлагал бы то, что сервер отвергнет, — и
//! это выглядело бы поломкой, а не правилом.

/// Ключи разделов. Совпадают с колонками `<key>_enabled` в `server_hubs`.
pub const FEATURE_FEED: &str = "feed";
pub const FEATURE_PROFILES: &str = "profiles";
pub const FEATURE_BANK: &str = "bank";
pub const FEATURE_FINES: &str = "fines";
pub const FEATURE_COURTS: &str = "courts";
pub const FEATURE_PETITIONS: &str = "petitions";
pub const FEATURE_TOWNS: &str = "towns";

pub const ALL_FEATURES: &[&str] = &[
    FEATURE_FEED,
    FEATURE_PROFILES,
    FEATURE_BANK,
    FEATURE_FINES,
    FEATURE_COURTS,
    FEATURE_PETITIONS,
    FEATURE_TOWNS,
];

/// Что без чего не работает: `(раздел, его основа)`.
///
/// Список плоский, а не дерево: цепочек глубже одной пока нет, и разворачивать
/// их рекурсивно — усложнение под задачу, которой не существует. Если появится
/// раздел, зависящий от штрафов, проверку придётся сделать транзитивной, и это
/// будет видно по тесту `every_dependency_is_a_known_feature`.
pub const FEATURE_DEPENDENCIES: &[(&str, &str)] = &[
    // Штраф — требование заплатить: без счетов платить нечем и некуда, и
    // раздел выродился бы в список долгов, которые невозможно закрыть.
    (FEATURE_FINES, FEATURE_BANK),
];

/// Платные настройки: `(путь параметра, раздел, без которого он не работает)`.
///
/// Путь — тот же `секция.поле`, что возвращает `HubSettings::violations`,
/// поэтому в интерфейс имя параметра приходит одним способом.
///
/// Правило одно: **ненулевая цена требует включённого банка**. Проверяется
/// итоговое состояние, а не присланное поле, — «выключить банк, оставив
/// пошлину» ломает то же самое с другой стороны. Отказ здесь громкий
/// намеренно: молчаливое обнуление превратило бы платный раздел в бесплатный
/// так, что никто бы не заметил.
pub const PAID_SETTINGS: &[(&str, &str)] = &[
    ("courts.claim_price", FEATURE_BANK),
    ("petitions.filing_price", FEATURE_BANK),
    ("towns.founding_price", FEATURE_BANK),
    // Цены самого банка сюда не входят: они лежат в его же секции, которая без
    // банка не показывается и не применяется. Требовать их обнуления при
    // выключении банка значило бы терять настроенное на ровном месте.
];

/// Чего не хватает разделу.
pub fn requires(feature: &str) -> Option<&'static str> {
    FEATURE_DEPENDENCIES
        .iter()
        .find(|(what, _)| *what == feature)
        .map(|(_, needs)| *needs)
}

/// Разделы, которые погаснут вместе с этим.
pub fn dependents(feature: &str) -> Vec<&'static str> {
    FEATURE_DEPENDENCIES
        .iter()
        .filter(|(_, needs)| *needs == feature)
        .map(|(what, _)| *what)
        .collect()
}

/// Включённые разделы, у которых не включена основа.
///
/// Принимает функцию, а не структуру: набор разделов живёт колонками в базе, и
/// собирать ради проверки ещё один тип — лишний слой, который придётся
/// синхронизировать при каждом новом разделе.
pub fn unmet(is_on: impl Fn(&str) -> bool) -> Vec<(&'static str, &'static str)> {
    FEATURE_DEPENDENCIES
        .iter()
        .filter(|(what, needs)| is_on(what) && !is_on(needs))
        .map(|(what, needs)| (*what, *needs))
        .collect()
}

/// Что нужно включить, чтобы этот параметр можно было сделать платным.
pub fn price_requires(path: &str) -> Option<&'static str> {
    PAID_SETTINGS
        .iter()
        .find(|(what, _)| *what == path)
        .map(|(_, needs)| *needs)
}
/// Ненулевые цены, у которых выключена основа.
///
/// Значения берутся функцией, а не из `HubSettings`: этот модуль описывает
/// правила, а не структуру настроек, и знать про её поля ему незачем.
pub fn unmet_prices(
    is_on: impl Fn(&str) -> bool,
    price_of: impl Fn(&str) -> i64,
) -> Vec<(&'static str, &'static str)> {
    PAID_SETTINGS
        .iter()
        .filter(|(what, needs)| price_of(what) > 0 && !is_on(needs))
        .map(|(what, needs)| (*what, *needs))
        .collect()
}

#[cfg(test)]
#[path = "hub_features_tests.rs"]
mod tests;
