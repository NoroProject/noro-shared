//! Шаблон обязан проходить собственную проверку.
//!
//! Иначе первое, что видит человек после `cargo noro new`, — отказ `cargo noro
//! check` на файлах, которых он не писал. Шаблон живёт в отдельном каталоге, и
//! компилятор его содержимое не смотрит.

use super::*;

/// Куда писать проект в тесте. `target/` уже вне репозитория и уже чистится.
fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/cargo-noro-tests")
        .join(name);
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn a_new_module_passes_its_own_check() {
    for (ui, name) in [(Ui::Vue, "with-ui"), (Ui::None, "no-ui")] {
        let dir = scratch(name);
        run("test-module", None, ui, Some(dir.clone())).expect("шаблон разворачивается");

        let manifest: noro_module_abi::manifest::Manifest =
            toml::from_str(&std::fs::read_to_string(dir.join("manifest.toml")).unwrap())
                .expect("манифест шаблона разбирается");

        let bad = noro_module_abi::validate::violations(&manifest);
        assert!(
            bad.is_empty(),
            "{name}: шаблон не проходит проверку: {bad:?}"
        );
        assert_eq!(manifest.module.id, "test-module", "{name}: подстановка id");

        // Ключ заголовка обязан существовать: на нём проверка спотыкается
        // чаще всего, потому что имя ключа собрано из идентификатора.
        let en = std::fs::read_to_string(dir.join("locales/en.ftl")).unwrap();
        assert!(
            en.contains("mod-test-module-title"),
            "{name}: ключи локалей не переименованы вместе с модулем"
        );

        // Без мини-аппа раздел должен исчезнуть целиком: оставшийся объявлял бы
        // экран, файла которого в пакете не будет.
        assert_eq!(
            manifest.apps.is_empty(),
            ui == Ui::None,
            "{name}: раздел [[apps]] не соответствует выбору"
        );
    }
}

#[test]
fn a_bad_identifier_is_refused_before_anything_is_written() {
    for id in ["Shop", "my_shop", "-shop", "shop-", ""] {
        assert!(
            run(id, None, Ui::None, Some(scratch("rejected"))).is_err(),
            "`{id}` приняли за идентификатор"
        );
    }
    assert!(
        !scratch("rejected").exists(),
        "каталог создан до проверки имени"
    );
}

#[test]
fn vue_interpolation_survives_substitution() {
    let dir = scratch("vue-braces");
    run("shop", None, Ui::Vue, Some(dir.clone())).expect("шаблон разворачивается");

    let app = std::fs::read_to_string(dir.join("ui/App.vue")).unwrap();
    assert!(
        app.contains("{{ data.joins }}"),
        "вставки Vue съедены подстановкой — в шаблоне те же фигурные скобки"
    );
}
