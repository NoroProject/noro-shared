//! Параметры подсайта сервера.
//!
//! Файл длиннее обычного намеренно: это одна таблица значений с их границами, и
//! разрезать её по секциям значило бы искать «где задаётся цена» в шести местах.
//!
//! Всё лежит в одном JSONB (`server_hubs.settings`), потому что набор параметров
//! открыт — он растёт с каждым новым разделом. Флаги самих разделов, наоборот,
//! колонки: их список закрыт и они нужны в `WHERE`.
//!
//! Каждое поле — `#[serde(default)]`, поэтому пустой `{}` в базе обязан давать
//! рабочий подсайт, а незнакомое поле из будущей версии не роняет старый мастер.
//! Деньги везде целые, в минимальных единицах (как копейки): плавающая точка в
//! балансах — обычный способ потерять монету на переводе.

use serde::{Deserialize, Serialize};

/// Настройки подсайта целиком.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct HubSettings {
    pub feed: FeedSettings,
    pub currency: CurrencySettings,
    pub bank: BankSettings,
    pub fines: FineSettings,
    pub petitions: PetitionSettings,
    pub courts: CourtSettings,
    pub towns: TownSettings,
    pub map: MapSettings,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct FeedSettings {
    /// Предел длины записи. Совпадает с пределом сообщения в тикетах — тот же
    /// порядок текста, и разные пределы объяснить игроку нечем.
    pub post_max_chars: i32,
    pub images_max: i32,
    /// Пауза между записями одного автора. Ноль — паузы нет.
    pub cooldown_seconds: i32,
}

impl Default for FeedSettings {
    fn default() -> Self {
        Self {
            post_max_chars: 4000,
            images_max: 4,
            cooldown_seconds: 30,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct CurrencySettings {
    pub name: String,
    pub symbol: String,
    /// Знаков после запятой — только для показа: в базе всегда целые единицы.
    pub precision: i32,
    pub start_balance: i64,
}

impl Default for CurrencySettings {
    fn default() -> Self {
        Self {
            name: "Coin".into(),
            symbol: String::new(),
            precision: 0,
            start_balance: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct BankSettings {
    pub card_price: i64,
    /// Комиссия перевода в процентах.
    pub transfer_fee_percent: i32,
    /// Предел одного перевода. Ноль — без предела.
    pub transfer_max: i64,
    /// Сколько карт держит один игрок.
    ///
    /// Карты видны в списках, участвуют в переводах и в сверке, и три десятка
    /// пустых карт у одного человека засоряют и то, и другое.
    pub max_cards: i32,
    /// Сколько дней карта живёт до того, как её разрешено закрыть.
    ///
    /// Без выдержки номера перебирают: закрыл, открыл, посмотрел — и так до
    /// красивого. Месяц делает перебор бессмысленным, не мешая обычной жизни:
    /// ненужную карту просто оставляют пустой.
    pub card_min_age_days: i32,
    /// Цена картинки на карточке. Ноль — оформление бесплатно.
    pub card_image_price: i64,

    /// Цена улучшения карты. Ноль — купить нельзя, только выдача банкиром.
    ///
    /// Улучшенная карта не берёт комиссию за переводы, имеет свой предел
    /// перевода и допускает свою картинку — обычной остаются готовые фоны
    /// сервера.
    pub plus_price: i64,
    /// Предел перевода с улучшенной карты. Ноль — без предела.
    pub plus_transfer_max: i64,
    /// Куда падают комиссии: код официального счёта. Пусто — служебный счёт
    /// сборов, как раньше.
    ///
    /// Комиссия — доход того, кто держит банк, и на многих серверах это
    /// конкретная структура, а не «сервер вообще».
    pub fees_account: String,
}

impl Default for BankSettings {
    fn default() -> Self {
        Self {
            card_price: 0,
            transfer_fee_percent: 0,
            transfer_max: 0,
            max_cards: 4,
            card_min_age_days: 30,
            card_image_price: 0,
            plus_price: 0,
            plus_transfer_max: 0,
            fees_account: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct FineSettings {
    /// Сколько дней даётся на оплату. Ноль — срока нет.
    pub due_days: i32,
    /// Предел одного штрафа. Ноль — без предела.
    ///
    /// Нужен не против злого умысла, а против опечатки: лишний ноль в сумме
    /// превращает штраф в приговор, и заметить это должен сервер, а не игрок.
    pub max_amount: i64,
}

impl Default for FineSettings {
    fn default() -> Self {
        Self {
            due_days: 7,
            max_amount: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PetitionSettings {
    pub filing_price: i64,
    /// Код официального счёта, куда идёт пошлина. Пусто — в казну.
    ///
    /// У штрафа получателя выбирает тот, кто его выписывает; здесь выбирать
    /// некому — платит сам подающий, — поэтому счёт задаётся один раз.
    pub payee: String,
    pub votes_needed: i32,
    pub days_open: i32,
}

impl Default for PetitionSettings {
    fn default() -> Self {
        Self {
            filing_price: 0,
            payee: String::new(),
            votes_needed: 10,
            days_open: 14,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct CourtSettings {
    pub claim_price: i64,
    /// Код официального счёта, куда идёт пошлина. Пусто — в казну.
    pub payee: String,
    pub review_days: i32,
}

impl Default for CourtSettings {
    fn default() -> Self {
        Self {
            claim_price: 0,
            payee: String::new(),
            review_days: 7,
        }
    }
}

/// Как на сервере заводят города.
pub const TOWNS_FREE: &str = "free";
pub const TOWNS_APPLICATION: &str = "application";
pub const TOWNS_MINISTRY: &str = "ministry";

pub const TOWN_MODES: &[&str] = &[TOWNS_FREE, TOWNS_APPLICATION, TOWNS_MINISTRY];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TownSettings {
    /// `free` — город появляется сразу; `application` — заявка ждёт минюста;
    /// `ministry` — заводит только сотрудник.
    ///
    /// Строка, а не перечисление: настройки живут в JSONB, и незнакомое
    /// значение из будущей версии не должно ронять разбор всего подсайта.
    /// Проверяет его `violations`, как и всё остальное здесь.
    pub registration_mode: String,
    pub founding_price: i64,
    /// Код официального счёта, куда идёт плата. Пусто — в казну.
    pub payee: String,

    /// Лимит чанков собирается из четырёх слагаемых:
    /// `base_chunks + chunks_per_resident × жители + бонус минюста + куплено`.
    ///
    /// Так «по числу жителей», «за деньги», «выдаёт минюст» и «без лимита» —
    /// это четыре набора чисел, а не четыре ветки кода. Ноль выключает
    /// слагаемое, `max_chunks = 0` снимает потолок.
    pub base_chunks: i32,
    pub chunks_per_resident: i32,
    /// Цена чанка сверх бесплатного лимита. Ноль — докупать нельзя.
    pub chunk_price: i64,
    /// Жёсткий потолок территории. Ноль — без потолка.
    pub max_chunks: i32,

    /// Сколько пустых чанков обязано лежать между городами. Ноль — можно
    /// вплотную.
    pub min_gap_chunks: i32,
    /// Разрешены ли оторванные куски территории.
    ///
    /// По умолчанию нет: «граница города» подразумевает связную область, а
    /// разбросанные по всей карте одиночные чанки — это способ застолбить
    /// места впрок, а не построить город. Серверу с портами и колониями
    /// правило мешает, поэтому оно снимается настройкой, а не переписыванием.
    pub allow_exclaves: bool,
    /// Сколько часов надо наиграть на сервере, чтобы основать город.
    ///
    /// Считается из `player_sessions`; второго счётчика времени в проекте нет
    /// и заводить его ради городов нельзя — он разойдётся с первым.
    pub min_playtime_hours: i32,
}

impl Default for TownSettings {
    fn default() -> Self {
        Self {
            registration_mode: TOWNS_FREE.into(),
            founding_price: 0,
            payee: String::new(),
            base_chunks: 16,
            chunks_per_resident: 0,
            chunk_price: 0,
            max_chunks: 0,
            min_gap_chunks: 0,
            allow_exclaves: false,
            min_playtime_hours: 0,
        }
    }
}

/// Карта сервера: что рендерить, как часто и что показывать поверх.
///
/// Живёт в секции городов не случайно — карту заводят ради границ, — но
/// секцией отдельной: тайлы рисуются и без единого города, а слои включаются
/// по одному.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MapSettings {
    /// Какие миры рендерить. Пусто — только основной мир сервера.
    ///
    /// Список, а не «всё подряд»: модовая сборка приносит десятки технических
    /// измерений, и каждое из них — это гигабайты тайлов ни для кого.
    pub worlds: Vec<String>,
    /// Как часто агент отдаёт накопленное. Реже — дешевле, но карта старее.
    pub push_interval_seconds: i32,
    /// Потолок темпа рендера. Ноль — не рендерить вовсе: так карту выключают,
    /// не теряя уже нарисованного.
    pub chunks_per_second: i32,
    /// Сколько уровней уменьшения строит мастер поверх базового.
    pub zoom_levels: i32,

    /// Слои. Каждый включается отдельно: живые точки игроков на PvP-сборке —
    /// оружие, а метки администрации нужны и там, где всё остальное выключено.
    pub show_players: bool,
    pub show_town_labels: bool,
    pub show_admin_markers: bool,
    pub show_disputes: bool,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            worlds: Vec::new(),
            push_interval_seconds: 30,
            chunks_per_second: 8,
            zoom_levels: 4,
            show_players: false,
            show_town_labels: true,
            show_admin_markers: true,
            show_disputes: true,
        }
    }
}

/// Жёсткий потолок картинок на запись.
///
/// Ограничение не вкусовое: каждая картинка это загрузка в CAS и строка в
/// ленте, и «сто картинок в посте» кладёт страницу всем читателям, а не только
/// автору. Настройка может быть меньше, больше — нет.
pub const IMAGES_HARD_MAX: i32 = 10;

/// Поле настроек, вышедшее за границы.
///
/// Возвращается именем поля в форме `секция.поле`, потому что ровно так его
/// показывает форма в админке: ошибка обязана подсветить конкретное поле, а не
/// всю страницу.
pub type SettingsViolation = &'static str;

impl HubSettings {
    /// Что в настройках выходит за границы.
    ///
    /// Проверка живёт здесь, а не в хендлере, чтобы форма в вебе и мастер
    /// пользовались одними и теми же числами: расходящиеся границы дают отказ,
    /// который на форме выглядит взявшимся из ниоткуда.
    pub fn violations(&self) -> Vec<SettingsViolation> {
        let mut bad = Vec::new();

        if !(1..=20_000).contains(&self.feed.post_max_chars) {
            bad.push("feed.post_max_chars");
        }
        if !(0..=IMAGES_HARD_MAX).contains(&self.feed.images_max) {
            bad.push("feed.images_max");
        }
        if !(0..=3600).contains(&self.feed.cooldown_seconds) {
            bad.push("feed.cooldown_seconds");
        }

        if self.currency.name.trim().is_empty() || self.currency.name.chars().count() > 32 {
            bad.push("currency.name");
        }
        if self.currency.symbol.chars().count() > 8 {
            bad.push("currency.symbol");
        }
        if !(0..=4).contains(&self.currency.precision) {
            bad.push("currency.precision");
        }
        if self.currency.start_balance < 0 {
            bad.push("currency.start_balance");
        }

        if self.bank.card_price < 0 {
            bad.push("bank.card_price");
        }
        if !(0..=100).contains(&self.bank.transfer_fee_percent) {
            bad.push("bank.transfer_fee_percent");
        }
        if self.bank.transfer_max < 0 {
            bad.push("bank.transfer_max");
        }
        if !(1..=32).contains(&self.bank.max_cards) {
            bad.push("bank.max_cards");
        }
        if !(0..=365).contains(&self.bank.card_min_age_days) {
            bad.push("bank.card_min_age_days");
        }
        if self.bank.card_image_price < 0 {
            bad.push("bank.card_image_price");
        }
        if self.bank.plus_price < 0 {
            bad.push("bank.plus_price");
        }
        if self.bank.plus_transfer_max < 0 {
            bad.push("bank.plus_transfer_max");
        }

        if !(0..=365).contains(&self.fines.due_days) {
            bad.push("fines.due_days");
        }
        if self.fines.max_amount < 0 {
            bad.push("fines.max_amount");
        }

        if self.petitions.filing_price < 0 {
            bad.push("petitions.filing_price");
        }
        if !(1..=10_000).contains(&self.petitions.votes_needed) {
            bad.push("petitions.votes_needed");
        }
        if !(1..=365).contains(&self.petitions.days_open) {
            bad.push("petitions.days_open");
        }

        if self.courts.claim_price < 0 {
            bad.push("courts.claim_price");
        }
        if !(1..=365).contains(&self.courts.review_days) {
            bad.push("courts.review_days");
        }

        if self.towns.founding_price < 0 {
            bad.push("towns.founding_price");
        }
        if !TOWN_MODES.contains(&self.towns.registration_mode.as_str()) {
            bad.push("towns.registration_mode");
        }
        if !(0..=10_000).contains(&self.towns.base_chunks) {
            bad.push("towns.base_chunks");
        }
        if !(0..=1_000).contains(&self.towns.chunks_per_resident) {
            bad.push("towns.chunks_per_resident");
        }
        if self.towns.chunk_price < 0 {
            bad.push("towns.chunk_price");
        }
        if !(0..=100_000).contains(&self.towns.max_chunks) {
            bad.push("towns.max_chunks");
        }
        if !(0..=64).contains(&self.towns.min_gap_chunks) {
            bad.push("towns.min_gap_chunks");
        }
        if !(0..=10_000).contains(&self.towns.min_playtime_hours) {
            bad.push("towns.min_playtime_hours");
        }

        if !(5..=3600).contains(&self.map.push_interval_seconds) {
            bad.push("map.push_interval_seconds");
        }
        // Ноль — законное значение: так рендер останавливают, не стирая карту.
        if !(0..=256).contains(&self.map.chunks_per_second) {
            bad.push("map.chunks_per_second");
        }
        if !(0..=8).contains(&self.map.zoom_levels) {
            bad.push("map.zoom_levels");
        }
        if self.map.worlds.len() > 32 {
            bad.push("map.worlds");
        }

        bad
    }

    /// Цена по её имени из `PAID_SETTINGS`.
    ///
    /// `None` — путь неизвестен: правило о платных настройках описано именами,
    /// и опечатка в имени сняла бы проверку молча. Тест не даёт разойтись.
    pub fn price(&self, path: &str) -> Option<i64> {
        Some(match path {
            "courts.claim_price" => self.courts.claim_price,
            "petitions.filing_price" => self.petitions.filing_price,
            "towns.founding_price" => self.towns.founding_price,
            "towns.chunk_price" => self.towns.chunk_price,
            _ => return None,
        })
    }
}

#[cfg(test)]
#[path = "hub_tests.rs"]
mod tests;
