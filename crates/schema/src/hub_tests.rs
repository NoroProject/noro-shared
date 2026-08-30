use super::*;

/// Главный инвариант: в базе у существующих серверов лежит пустой `{}`, и он
/// обязан читаться в рабочие настройки, а не в нули, при которых нельзя
/// написать ни одной записи.
#[test]
fn an_empty_object_reads_as_working_defaults() {
    let settings: HubSettings = serde_json::from_str("{}").expect("пустой объект должен читаться");

    assert_eq!(settings, HubSettings::default());
    assert!(settings.feed.post_max_chars > 0);
    assert!(settings.violations().is_empty());
}

/// Раздел, которого ещё нет в JSON, подставляется целиком: новая волна добавляет
/// секцию, и старые строки в базе не требуют бэкфилла.
#[test]
fn a_missing_section_falls_back_to_its_defaults() {
    let json = r#"{"feed":{"post_max_chars":100}}"#;
    let settings: HubSettings = serde_json::from_str(json).unwrap();

    assert_eq!(settings.feed.post_max_chars, 100);
    assert_eq!(settings.feed.images_max, FeedSettings::default().images_max);
    assert_eq!(settings.petitions, PetitionSettings::default());
}

/// Поле из будущей версии не должно ронять старый мастер: он читает то, что
/// понимает, и работает дальше.
#[test]
fn an_unknown_field_does_not_break_parsing() {
    let json = r#"{"feed":{"post_max_chars":500,"future_knob":true},"aliens":{"x":1}}"#;
    let settings: HubSettings = serde_json::from_str(json).unwrap();

    assert_eq!(settings.feed.post_max_chars, 500);
}

/// Значения ходят в базу и обратно без потерь — иначе сохранение формы тихо
/// меняло бы то, чего никто не трогал.
#[test]
fn settings_survive_a_round_trip() {
    let mut settings = HubSettings::default();
    settings.currency.name = "Ruble".into();
    settings.currency.symbol = "₽".into();
    settings.bank.card_price = 2500;
    settings.petitions.votes_needed = 42;

    let json = serde_json::to_string(&settings).unwrap();
    let back: HubSettings = serde_json::from_str(&json).unwrap();

    assert_eq!(back, settings);
}

/// Границы проверяются по полям, а не «валидно/невалидно»: форме нужно
/// подсветить именно ту строку, которую человек испортил.
#[test]
fn violations_name_the_offending_field() {
    let mut settings = HubSettings::default();
    settings.feed.cooldown_seconds = -1;
    settings.bank.transfer_fee_percent = 150;

    let bad = settings.violations();

    assert!(bad.contains(&"feed.cooldown_seconds"));
    assert!(bad.contains(&"bank.transfer_fee_percent"));
    assert!(!bad.contains(&"feed.post_max_chars"));
}

/// Настройка не может поднять предел картинок выше жёсткого: это защита
/// читателей ленты, а не предпочтение владельца сервера.
#[test]
fn the_image_limit_cannot_exceed_the_hard_cap() {
    let mut settings = HubSettings::default();
    settings.feed.images_max = IMAGES_HARD_MAX + 1;

    assert!(settings.violations().contains(&"feed.images_max"));

    settings.feed.images_max = IMAGES_HARD_MAX;
    assert!(settings.violations().is_empty());
}

/// Деньги отрицательными не бывают: отрицательная цена — это выплата за
/// действие, и она обрушила бы любую экономику ещё до её появления.
#[test]
fn prices_cannot_be_negative() {
    let mut settings = HubSettings::default();
    settings.courts.claim_price = -1;
    settings.towns.founding_price = -100;

    let bad = settings.violations();
    assert!(bad.contains(&"courts.claim_price"));
    assert!(bad.contains(&"towns.founding_price"));
}

/// Валюта без названия оставила бы в интерфейсе «у вас 5 » — пустая строка тут
/// не «значение по умолчанию», а поломка показа.
#[test]
fn the_currency_needs_a_name() {
    let mut settings = HubSettings::default();
    settings.currency.name = "   ".into();

    assert!(settings.violations().contains(&"currency.name"));
}

/// Ноль карт означал бы банк, в котором нельзя завести счёт.
#[test]
fn a_player_must_be_allowed_at_least_one_card() {
    let mut settings = HubSettings::default();
    settings.bank.max_cards = 0;
    assert!(settings.violations().contains(&"bank.max_cards"));

    settings.bank.max_cards = 1;
    assert!(settings.violations().is_empty());
}

/// Выдержку можно отключить нулём — на приватном сервере перебирать номера
/// некому, — но отрицательной она не бывает.
#[test]
fn the_card_age_may_be_zero_but_not_negative() {
    let mut settings = HubSettings::default();
    settings.bank.card_min_age_days = 0;
    assert!(settings.violations().is_empty());

    settings.bank.card_min_age_days = -1;
    assert!(settings.violations().contains(&"bank.card_min_age_days"));
}

/// Умолчания банка обязаны быть рабочими: пустой {} в базе — обычное дело.
#[test]
fn bank_defaults_are_sane() {
    let bank = HubSettings::default().bank;
    assert!(bank.max_cards >= 1);
    assert_eq!(bank.card_min_age_days, 30);
}

/// Срок оплаты можно отключить нулём, отрицательным он не бывает.
#[test]
fn the_fine_due_period_may_be_zero() {
    let mut settings = HubSettings::default();
    settings.fines.due_days = 0;
    assert!(settings.violations().is_empty());

    settings.fines.due_days = -1;
    assert!(settings.violations().contains(&"fines.due_days"));
}

/// Предел суммы штрафа — защита от лишнего нуля в поле, а не от злого умысла.
#[test]
fn a_fine_limit_cannot_be_negative() {
    let mut settings = HubSettings::default();
    settings.fines.max_amount = -1;
    assert!(settings.violations().contains(&"fines.max_amount"));
}

/// Правило о платных настройках описано именами, а имена — повод для опечатки.
/// Незнакомое имя означало бы цену без присмотра, и заметить это должен тест.
#[test]
fn every_paid_setting_has_a_price_to_read() {
    let s = HubSettings::default();
    for (path, _) in crate::PAID_SETTINGS {
        assert!(s.price(path).is_some(), "no price behind {path}");
    }
    assert_eq!(s.price("courts.review_days"), None);
}

/// Бесплатный суд на сервере без банка — обычное состояние, а не поломка.
#[test]
fn defaults_are_free_and_pass_without_a_bank() {
    let s = HubSettings::default();
    assert!(crate::unmet_prices(|_| false, |p| s.price(p).unwrap_or(0)).is_empty());
}
