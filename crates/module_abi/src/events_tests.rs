//! Что в каталоге событий нельзя проверить компилятором.

use super::ALL_EVENTS;

/// Группа события — имя Rust-модуля, в котором лежит его payload.
///
/// Связь неочевидная и ничем не удерживаемая: в каталоге группа пишется
/// строкой, и ошибиться в ней ничего не мешает. Цена ошибки — не поломка, а
/// тихо битые ссылки: документация строит путь
/// `events/<group>/struct.<payload>.html`, и раздел, названный чужой группой,
/// целиком ведёт в никуда. Замечают такое через месяцы.
#[test]
fn every_group_is_a_real_module() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/events");

    for e in ALL_EVENTS {
        let file = dir.join(format!("{}.rs", e.group));
        assert!(
            file.is_file(),
            "{}: группы «{}» нет среди модулей событий — ссылки этого раздела \
             будут вести в несуществующую страницу",
            e.name,
            e.group
        );

        let src = std::fs::read_to_string(&file).expect("модуль событий читается");
        assert!(
            src.contains(&format!("pub struct {}", e.payload)),
            "{}: payload «{}» объявлен не в events/{}.rs — ссылка на него \
             указывает не туда",
            e.name,
            e.payload,
            e.group
        );
    }
}
