use super::*;

/// Штраф без банка — требование заплатить туда, где нет счетов.
#[test]
fn fines_need_a_bank() {
    assert_eq!(requires(FEATURE_FINES), Some(FEATURE_BANK));
}

/// Суд разбирает спор, а петиция собирает подписи и без экономики. Денег
/// требует только пошлина — а это уже настройка, а не раздел.
#[test]
fn courts_and_petitions_work_without_a_bank() {
    assert_eq!(requires(FEATURE_COURTS), None);
    assert_eq!(requires(FEATURE_PETITIONS), None);
    assert!(unmet(|f| f == FEATURE_COURTS || f == FEATURE_PETITIONS).is_empty());
}

/// Зато платными без банка они быть не могут.
#[test]
fn a_price_without_a_bank_is_reported() {
    let bank_off = |f: &str| f == FEATURE_COURTS;
    let priced = |p: &str| if p == "courts.claim_price" { 500 } else { 0 };

    assert_eq!(
        unmet_prices(bank_off, priced),
        vec![("courts.claim_price", FEATURE_BANK)]
    );
    // Бесплатный суд без банка — законное состояние.
    assert!(unmet_prices(bank_off, |_| 0).is_empty());
    // С банком цена ничему не противоречит.
    assert!(unmet_prices(|_| true, priced).is_empty());
}

/// Лента и профили самодостаточны: сервер может включить только их.
#[test]
fn the_basic_sections_depend_on_nothing() {
    assert_eq!(requires(FEATURE_FEED), None);
    assert_eq!(requires(FEATURE_PROFILES), None);
    assert_eq!(requires(FEATURE_BANK), None);
}

/// Выключая банк, оператор должен видеть, что погаснет вместе с ним.
#[test]
fn turning_off_a_base_names_what_falls_with_it() {
    assert_eq!(dependents(FEATURE_BANK), vec![FEATURE_FINES]);
    assert!(dependents(FEATURE_FINES).is_empty());
}

/// Опечатка в пути параметра оставила бы цену без присмотра.
#[test]
fn every_paid_setting_points_at_a_known_feature() {
    for (path, needs) in PAID_SETTINGS {
        assert!(ALL_FEATURES.contains(needs), "unknown base: {needs}");
        assert!(path.contains('.'), "path must be section.field: {path}");
        assert_eq!(price_requires(path), Some(*needs));
    }
    assert_eq!(price_requires("feed.post_max_chars"), None);
}

/// Ради чего всё и заведено: включённый раздел без основы — состояние, которое
/// мастер обязан отвергнуть.
#[test]
fn an_enabled_section_without_its_base_is_reported() {
    let broken = unmet(|f| f == FEATURE_FINES);
    assert_eq!(broken, vec![(FEATURE_FINES, FEATURE_BANK)]);

    let fine = unmet(|f| f == FEATURE_FINES || f == FEATURE_BANK);
    assert!(fine.is_empty());
}

/// Выключенный раздел никого не держит: банк можно выключить, если штрафы тоже
/// выключены.
#[test]
fn a_disabled_section_does_not_complain() {
    assert!(unmet(|_| false).is_empty());
    assert!(unmet(|f| f == FEATURE_BANK).is_empty());
}

/// Опечатка в ключе зависимости сделала бы проверку молча бесполезной.
#[test]
fn every_dependency_is_a_known_feature() {
    for (what, needs) in FEATURE_DEPENDENCIES {
        assert!(ALL_FEATURES.contains(what), "unknown feature: {what}");
        assert!(ALL_FEATURES.contains(needs), "unknown base: {needs}");
        assert_ne!(what, needs, "{what} depends on itself");
        // Цепочки глубже одной проверка не разворачивает — если такая появится,
        // здесь и станет видно, что `unmet` пора делать транзитивным.
        assert!(
            requires(needs).is_none(),
            "{needs} is itself dependent: make unmet() transitive"
        );
    }
}
