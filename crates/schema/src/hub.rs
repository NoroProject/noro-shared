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
    pub ads: AdSettings,
    pub currency: CurrencySettings,
    pub bank: BankSettings,
    pub fines: FineSettings,
    pub petitions: PetitionSettings,
    pub courts: CourtSettings,
    pub towns: TownSettings,
    pub map: MapSettings,
    pub communities: CommunitySettings,
    pub market: MarketSettings,
    pub delivery: DeliverySettings,
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

    /// Цена часа купленного закрепа. Ноль — закреп даром.
    ///
    /// Купленный закреп не тот же, что `hub_posts.pinned`: тот бессрочный и
    /// раздаётся правом — это голос сервера, и он всегда выше купленного.
    pub pin_price_per_hour: i64,
    /// Потолок одной покупки.
    pub pin_max_hours: i32,
    /// Сколько купленных закрепов держится на сервере одновременно.
    ///
    /// Ноль — закреп не продаётся. Потолок нужен не ради денег: без него верх
    /// ленты выкупается целиком, и лента перестаёт быть лентой.
    pub pin_slots: i32,
}

impl Default for FeedSettings {
    fn default() -> Self {
        Self {
            post_max_chars: 4000,
            images_max: 4,
            cooldown_seconds: 30,
            pin_price_per_hour: 0,
            pin_max_hours: 24,
            pin_slots: 0,
        }
    }
}

/// Рекламная карусель над лентой.
///
/// Реклама — обычная запись игрока, которую автор оплатил: своего вида записей
/// с отдельным сочинением текста нет, иначе рядом с лентой появилась бы вторая
/// лента со своей модерацией.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AdSettings {
    /// Цена дня показа. Ноль — показ даром.
    pub price_per_day: i64,
    pub max_days: i32,
    /// Сколько карточек показывает карусель за раз.
    ///
    /// Пул бывает больше: лишние не отбрасываются, а вращаются — выбор
    /// случайный при каждой загрузке. Иначе заплативший шестым не показался бы
    /// ни разу.
    pub slots: i32,
    /// Код официального счёта, куда идут деньги. Пусто — в казну.
    pub payee: String,
}

impl Default for AdSettings {
    fn default() -> Self {
        Self {
            price_per_day: 0,
            max_days: 14,
            slots: 5,
            payee: String::new(),
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
    /// Сколько часов надо наиграть, чтобы подписать петицию.
    ///
    /// Ноль — подписывает любой участник. Порог нужен не против злого умысла, а
    /// против пустых аккаунтов: подпись должна что-то стоить, иначе счётчик
    /// перестаёт значить «столько людей этого хотят».
    pub min_playtime_hours: i32,
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
            min_playtime_hours: 0,
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
    /// Может ли мэр переименовать свой город.
    ///
    /// Выключено по умолчанию: имя города — то, по чему его знают на сервере, и
    /// свободное переименование превращает список городов в движущуюся цель.
    /// Там, где это неважно, сервер разрешает — и мэрия правит имя сама, не
    /// беспокоя минюст.
    pub mayor_can_rename: bool,
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
            mayor_can_rename: false,
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

/// Как на сервере заводят сообщества. Те же три режима, что у городов, и
/// значения совпадают намеренно: оператор настраивает их рядом, и «free» в
/// одном месте обязан значить то же, что «free» в другом.
pub const COMMUNITIES_FREE: &str = "free";
pub const COMMUNITIES_APPLICATION: &str = "application";
pub const COMMUNITIES_STAFF: &str = "staff";

pub const COMMUNITY_MODES: &[&str] =
    &[COMMUNITIES_FREE, COMMUNITIES_APPLICATION, COMMUNITIES_STAFF];

/// Сообщества подсайта: новостные каналы, городские паблики, магазины, кланы.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct CommunitySettings {
    /// `free` — сообщество появляется сразу; `application` — заявка ждёт штаба;
    /// `staff` — заводит только сотрудник по праву.
    ///
    /// Строка, а не перечисление: настройки живут в JSONB, и незнакомое
    /// значение из будущей версии не должно ронять разбор всего подсайта.
    pub registration_mode: String,
    /// Цена основания. Ноль — бесплатно.
    pub founding_price: i64,
    /// Код официального счёта, куда идёт плата. Пусто — в казну.
    pub payee: String,
    /// Сколько сообществ держит один человек.
    ///
    /// Потолок нужен не против злого умысла, а против застолблённых названий:
    /// без него каталог за неделю зарастает пустыми пабликами, заведёнными
    /// впрок. Ноль — без предела.
    pub per_user_max: i32,
    /// Сколько часов надо наиграть на сервере, чтобы завести сообщество.
    ///
    /// Считается из `player_sessions`, как и порог основания города: второго
    /// счётчика времени в проекте нет и заводить его нельзя — он разойдётся с
    /// первым.
    pub min_playtime_hours: i32,
    pub name_max: i32,
    pub description_max: i32,
}

impl Default for CommunitySettings {
    fn default() -> Self {
        Self {
            registration_mode: COMMUNITIES_FREE.into(),
            founding_price: 0,
            payee: String::new(),
            per_user_max: 3,
            min_playtime_hours: 0,
            name_max: 48,
            description_max: 2000,
        }
    }
}

/// Товар лежит в бочках, которые построили игроки.
pub const STORAGE_VANILLA: &str = "vanilla";
/// Товар изымается у продавца и живёт в виртуальном хранилище.
pub const STORAGE_VAULT: &str = "vault";

pub const STORAGE_MODES: &[&str] = &[STORAGE_VANILLA, STORAGE_VAULT];

/// Цвета стен хранения — те самые шестнадцать красителей игры.
///
/// Не произвольный оттенок из палитры сайта: стену выкладывают в мире цветными
/// блоками, и цвет, которого в игре нет, выложить нечем. Расхождение между
/// чертежом и стеной хуже, чем отсутствие цвета вовсе.
pub const DYE_COLOURS: &[&str] = &[
    "white",
    "orange",
    "magenta",
    "light_blue",
    "yellow",
    "lime",
    "pink",
    "gray",
    "light_gray",
    "cyan",
    "purple",
    "blue",
    "brown",
    "green",
    "red",
    "black",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct MarketSettings {
    /// `vanilla` — пункты хранения из бочек; `vault` — виртуальное хранилище.
    ///
    /// Строка, а не перечисление, по той же причине, что и режим регистрации
    /// сообществ: настройки живут в JSONB, и значение из будущей версии не
    /// должно ронять разбор всего подсайта.
    pub storage_mode: String,
    /// Пошлина за выставление лота. Ноль — даром.
    pub listing_price: i64,
    /// Доля сервиса с продажи, в процентах.
    pub fee_percent: i32,
    /// Сколько часов даётся продавцу разложить товар по бочкам.
    pub stock_hours: i32,
    /// Сколько дней лот висит на витрине.
    pub lot_days: i32,
    /// Сколько дней у покупателя, чтобы забрать оплаченное.
    ///
    /// Срок нужен не покупателю, а бочке: пока товар ждёт, ячейка занята, и
    /// продавец не может ей распорядиться. Вышел срок — деньги обратно, товар
    /// снова в продажу.
    pub pickup_days: i32,
    /// Сколько лотов держит один продавец. Ноль — без предела.
    pub max_lots_per_seller: i32,
    /// Потолок порций одного продавца на один пункт.
    ///
    /// Он и заставляет товар растекаться по карте: без него весь склад
    /// активного продавца собирается в пункте на спавне, и остальные пункты
    /// стоят пустыми.
    pub max_packs_per_point: i32,
    /// Насколько занятость пункта отталкивает раскладку, в блоках расстояния
    /// за каждую занятую ячейку.
    pub occupancy_penalty: i32,
    /// За какой срок считаются страйки.
    pub strike_window_days: i32,
    /// Сколько страйков за этот срок закрывают торговлю. Ноль — не закрывают.
    pub strikes_to_ban: i32,
    pub trade_ban_days: i32,
}

impl Default for MarketSettings {
    fn default() -> Self {
        Self {
            storage_mode: STORAGE_VANILLA.into(),
            listing_price: 0,
            fee_percent: 0,
            stock_hours: 24,
            lot_days: 30,
            pickup_days: 7,
            max_lots_per_seller: 20,
            max_packs_per_point: 8,
            occupancy_penalty: 50,
            strike_window_days: 30,
            strikes_to_ban: 3,
            trade_ban_days: 7,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeliverySettings {
    /// Плата за сам факт рейса, независимо от расстояния.
    pub base_price: i64,
    /// Сколько добавляется за каждый отрезок пути длиной `step_blocks`.
    ///
    /// Считается от дальней бочки заказа до адреса. Разные миры дают только
    /// базу: дорога через портал не выражается в блоках, и придумывать курс
    /// значило бы придумывать число.
    pub price_per_step: i64,
    /// Длина этого отрезка в блоках.
    ///
    /// Отдельным числом, а не жёсткой сотней. Деньги целые, и «монета за триста
    /// блоков» через цену за сотню невыразима вовсе: треть монеты округляется в
    /// ноль, и дальний рейс оказывается бесплатным. Пара «сколько за сколько»
    /// выражает и это, и «две монеты за тысячу».
    pub step_blocks: i32,

    /// Сколько сервис берёт с рейса сверх процента.
    ///
    /// Нужно ровно по той же причине, что и шаг расстояния: плата за рейс — это
    /// единицы монет, и процент на ней ходит скачками по десять. Постоянная
    /// часть даёт мелкий шаг там, где процент его не даёт.
    pub fee_flat: i64,
    /// Доля сервиса с платы курьеру, в процентах.
    ///
    /// Своя, а не общая с рынком: продажи и рейсы — разные экономики. Сервер
    /// вправе брать долю с торговли и не брать с курьеров, пока служба не
    /// разошлась, — одним числом на двоих это не выражается.
    pub fee_percent: i32,
    /// Код официального счёта, на который капает эта доля.
    ///
    /// Пусто — туда же, куда все прочие комиссии банка. Отдельный счёт нужен,
    /// когда службу доставки держит не сервер, а город или гильдия: тогда её
    /// выручка не должна смешиваться с доходом банка.
    pub payee: String,

    /// Дальше этого покупатель видит курьера с точностью до 256 блоков.
    pub far_blocks: i32,
    /// Ближе этого — точные координаты: курьер уже на месте встречи.
    pub near_blocks: i32,

    /// Сколько минут связка рейсов одного оформления висит на доске неделимой.
    ///
    /// Есть кому увезти всё разом — увозит; никто не взялся, и связка сама
    /// распадается на отдельные рейсы, которые разбирают по одному.
    pub bundle_minutes: i32,
    /// Сколько часов у курьера довезти после взятия.
    pub take_hours: i32,
    /// Сколько рейсов курьер держит одновременно. Ноль — без предела.
    pub max_active_per_courier: i32,
}

impl Default for DeliverySettings {
    fn default() -> Self {
        Self {
            base_price: 0,
            price_per_step: 0,
            step_blocks: 100,
            fee_flat: 0,
            fee_percent: 0,
            payee: String::new(),
            far_blocks: 500,
            near_blocks: 100,
            bundle_minutes: 10,
            take_hours: 24,
            max_active_per_courier: 5,
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
        if self.feed.pin_price_per_hour < 0 {
            bad.push("feed.pin_price_per_hour");
        }
        // Месяц сверху: закреп на год — это уже не закреп, а вторая шапка
        // подсайта, и делается она настройками сервера, а не покупкой.
        if !(1..=720).contains(&self.feed.pin_max_hours) {
            bad.push("feed.pin_max_hours");
        }
        if !(0..=20).contains(&self.feed.pin_slots) {
            bad.push("feed.pin_slots");
        }

        if self.ads.price_per_day < 0 {
            bad.push("ads.price_per_day");
        }
        if !(1..=365).contains(&self.ads.max_days) {
            bad.push("ads.max_days");
        }
        // Ноль запрещён намеренно: включённый раздел с пустой каруселью — это
        // оплаченный показ, которого никто не видит.
        if !(1..=10).contains(&self.ads.slots) {
            bad.push("ads.slots");
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
        if !(0..=10_000).contains(&self.petitions.min_playtime_hours) {
            bad.push("petitions.min_playtime_hours");
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

        if !COMMUNITY_MODES.contains(&self.communities.registration_mode.as_str()) {
            bad.push("communities.registration_mode");
        }
        if self.communities.founding_price < 0 {
            bad.push("communities.founding_price");
        }
        if !(0..=100).contains(&self.communities.per_user_max) {
            bad.push("communities.per_user_max");
        }
        if !(0..=10_000).contains(&self.communities.min_playtime_hours) {
            bad.push("communities.min_playtime_hours");
        }
        if !(1..=128).contains(&self.communities.name_max) {
            bad.push("communities.name_max");
        }
        if !(1..=20_000).contains(&self.communities.description_max) {
            bad.push("communities.description_max");
        }

        if !STORAGE_MODES.contains(&self.market.storage_mode.as_str()) {
            bad.push("market.storage_mode");
        }
        if self.market.listing_price < 0 {
            bad.push("market.listing_price");
        }
        if !(0..=100).contains(&self.market.fee_percent) {
            bad.push("market.fee_percent");
        }
        // Час снизу: меньше — и продавец не успеет дойти до пункта. Месяц
        // сверху: лот, который месяц ждёт закладки, занимает бочки впустую.
        if !(1..=720).contains(&self.market.stock_hours) {
            bad.push("market.stock_hours");
        }
        if !(1..=365).contains(&self.market.lot_days) {
            bad.push("market.lot_days");
        }
        if !(1..=90).contains(&self.market.pickup_days) {
            bad.push("market.pickup_days");
        }
        if !(0..=1000).contains(&self.market.max_lots_per_seller) {
            bad.push("market.max_lots_per_seller");
        }
        // Ноль здесь означал бы, что разложить нельзя никуда, — это не «без
        // предела», а неработающий маркет.
        if !(1..=1000).contains(&self.market.max_packs_per_point) {
            bad.push("market.max_packs_per_point");
        }
        if !(0..=100_000).contains(&self.market.occupancy_penalty) {
            bad.push("market.occupancy_penalty");
        }
        if !(1..=365).contains(&self.market.strike_window_days) {
            bad.push("market.strike_window_days");
        }
        if !(0..=100).contains(&self.market.strikes_to_ban) {
            bad.push("market.strikes_to_ban");
        }
        if !(0..=365).contains(&self.market.trade_ban_days) {
            bad.push("market.trade_ban_days");
        }

        if self.delivery.base_price < 0 {
            bad.push("delivery.base_price");
        }
        if self.delivery.price_per_step < 0 {
            bad.push("delivery.price_per_step");
        }
        // Ноль здесь — деление на ноль в расчёте платы, а не «бесплатно»:
        // бесплатно выражается нулевой ценой за отрезок.
        if !(1..=100_000).contains(&self.delivery.step_blocks) {
            bad.push("delivery.step_blocks");
        }
        if self.delivery.fee_flat < 0 {
            bad.push("delivery.fee_flat");
        }
        if !(0..=100).contains(&self.delivery.fee_percent) {
            bad.push("delivery.fee_percent");
        }
        // Ступени независимы намеренно: «ближняя дальше дальней» не запрещается
        // здесь, потому что здесь проверяется одно поле, а не пара. Разбор
        // ступеней написан так, что переставленные границы дают более грубую
        // точность, а не более точную, — испортить приватность настройкой нельзя.
        if !(0..=100_000).contains(&self.delivery.far_blocks) {
            bad.push("delivery.far_blocks");
        }
        if !(0..=100_000).contains(&self.delivery.near_blocks) {
            bad.push("delivery.near_blocks");
        }
        if !(0..=1440).contains(&self.delivery.bundle_minutes) {
            bad.push("delivery.bundle_minutes");
        }
        // Час снизу: меньше — и рейс просрочен раньше, чем курьер дошёл до
        // первой бочки.
        if !(1..=720).contains(&self.delivery.take_hours) {
            bad.push("delivery.take_hours");
        }
        if !(0..=100).contains(&self.delivery.max_active_per_courier) {
            bad.push("delivery.max_active_per_courier");
        }

        bad
    }

    /// Цена по её имени из `PAID_SETTINGS`.
    ///
    /// `None` — путь неизвестен: правило о платных настройках описано именами,
    /// и опечатка в имени сняла бы проверку молча. Тест не даёт разойтись.
    pub fn price(&self, path: &str) -> Option<i64> {
        Some(match path {
            "feed.pin_price_per_hour" => self.feed.pin_price_per_hour,
            "courts.claim_price" => self.courts.claim_price,
            "petitions.filing_price" => self.petitions.filing_price,
            "towns.founding_price" => self.towns.founding_price,
            "towns.chunk_price" => self.towns.chunk_price,
            "communities.founding_price" => self.communities.founding_price,
            "market.listing_price" => self.market.listing_price,
            "delivery.base_price" => self.delivery.base_price,
            "delivery.price_per_step" => self.delivery.price_per_step,
            _ => return None,
        })
    }
}

#[cfg(test)]
#[path = "hub_tests.rs"]
mod tests;
