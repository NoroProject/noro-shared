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

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct TownSettings {
    pub founding_price: i64,
    /// Код официального счёта, куда идёт плата. Пусто — в казну.
    pub payee: String,
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
            _ => return None,
        })
    }
}

#[cfg(test)]
#[path = "hub_tests.rs"]
mod tests;
