use super::*;

#[test]
fn wildcard_matching() {
    assert!(permission_matches("*", "anything.here"));
    assert!(permission_matches("noro.server.*", "noro.server.hitech"));
    assert!(permission_matches(
        "noro.server.*",
        "noro.server.hitech.join"
    ));
    assert!(permission_matches(
        "noro.server.hitech.join",
        "noro.server.hitech.join"
    ));
    assert!(!permission_matches(
        "noro.server.*",
        "noro.admin.users.view"
    ));
    assert!(!permission_matches(
        "noro.server.hitech",
        "noro.server.hitech2"
    ));
}

/// Право на один сервер не должно открывать соседний: именно это отделяет
/// закрытую сборку от общедоступной.
#[test]
fn server_grant_does_not_leak_to_another_server() {
    let perms = ["noro.server.hitech.join"];
    assert!(any_permission_matches(perms, "noro.server.hitech.join"));
    assert!(!any_permission_matches(perms, "noro.server.vanilla.join"));
}

/// Шаблон обрывается на границе сегмента, а не по префиксу строки: иначе
/// `noro.admin.*` выдал бы права на всё, что просто начинается так же.
#[test]
fn wildcard_stops_at_the_segment_boundary() {
    assert!(!permission_matches("noro.admin.*", "noro.adminx.users"));
    assert!(!permission_matches("noro.server.*", "noro.servers.list"));
    assert!(permission_matches("noro.admin.*", "noro.admin.users.view"));
}

/// `*` поддерживается только как одиночный шаблон и как суффикс `.*`.
#[test]
fn other_star_placements_match_nothing() {
    assert!(!permission_matches("noro.*.users", "noro.admin.users"));
    assert!(!permission_matches("*.users", "noro.admin.users"));
    assert!(!permission_matches("noro.admin*", "noro.admin.users"));
}

#[test]
fn superadmin_covers_every_permission() {
    let perms = [PERM_SUPERADMIN];
    assert!(any_permission_matches(perms, PERM_WRAPPER_FILES));
    assert!(any_permission_matches(perms, &perm_server_join("hitech")));
}

#[test]
fn empty_permission_set_grants_nothing() {
    let empty: [&str; 0] = [];
    assert!(!any_permission_matches(empty, PERM_USERS_VIEW));
}

/// Управление игровой машиной — отдельная ветка, и админ серверов её не
/// получает: запись файла плюс рестарт это фактически рут на машине.
#[test]
fn server_admin_does_not_imply_machine_access() {
    let perms = [PERM_SERVERS_EDIT];
    assert!(!any_permission_matches(perms, PERM_WRAPPER_FILES));
    assert!(any_permission_matches([PERM_ADMIN_ALL], PERM_WRAPPER_FILES));
}

/// Ветка игрока выдаётся целиком одной строкой — ради этого узлы и разложены
/// по общему префиксу.
#[test]
fn a_branch_can_be_granted_at_once() {
    let perms = ["noro.admin.users.*"];
    assert!(any_permission_matches(perms, PERM_USERS_VIEW));
    assert!(any_permission_matches(perms, PERM_USERS_NOTES_WRITE));
    assert!(!any_permission_matches(perms, PERM_PUNISH_BAN));
}

/// Хелпер выдаёт мут, но не бан: виды наказаний — разные узлы.
#[test]
fn punishment_kinds_are_separate_nodes() {
    let helper = [PERM_PUNISH_MUTE, PERM_PUNISH_WARN];
    assert!(any_permission_matches(helper, &perm_punish("mute")));
    assert!(!any_permission_matches(helper, &perm_punish("ban")));
    assert!(!any_permission_matches(helper, PERM_PUNISH_BYPASS));
}

/// Три уровня выдачи для сборок: всё, все сборки сервера, одна версия.
#[test]
fn build_access_supports_three_wildcard_levels() {
    let node = perm_build_access("srv1", "b7");
    assert_eq!(node, "noro.build.srv1.b7");

    assert!(any_permission_matches(["noro.build.*"], &node));
    assert!(any_permission_matches(["noro.build.srv1.*"], &node));
    assert!(any_permission_matches([node.as_str()], &node));
    assert!(any_permission_matches([PERM_SUPERADMIN], &node));
}

/// Выдача на одну сборку не должна открывать соседнюю — ради этого всё и
/// затевалось: тестер получает только ту версию, что проверяет.
#[test]
fn build_access_does_not_leak_between_builds_or_servers() {
    let granted = perm_build_access("srv1", "b7");

    assert!(!any_permission_matches(
        [granted.as_str()],
        &perm_build_access("srv1", "b8")
    ));
    assert!(!any_permission_matches(
        ["noro.build.srv1.*"],
        &perm_build_access("srv2", "b7")
    ));
}

/// Два уровня делегирования ролей: одна роль сборки и все её роли.
#[test]
fn role_grant_supports_two_wildcard_levels() {
    let node = perm_grant_role("srv1", "banker");
    assert_eq!(node, "noro.admin.users.roles.srv1.banker");

    assert!(any_permission_matches(
        ["noro.admin.users.roles.srv1.*"],
        &node
    ));
    assert!(any_permission_matches([node.as_str()], &node));
    assert!(!any_permission_matches(
        ["noro.admin.users.roles.srv2.*"],
        &node
    ));
}

/// Условие безопасности, ради которого узел вообще так назван: делегировать
/// роли сборки можно, а глобальные — нет, они остаются за целой веткой.
#[test]
fn role_grant_on_a_server_is_not_the_global_one() {
    let delegated = ["noro.admin.users.roles.srv1.*"];
    assert!(!any_permission_matches(delegated, PERM_USERS_ROLES));

    // А выдача ветки целиком по-прежнему включает и то, и другое: так её и
    // читают, когда отдают человеку все роли разом.
    let whole = ["noro.admin.users.roles.*"];
    assert!(any_permission_matches(whole, PERM_USERS_ROLES));
    assert!(any_permission_matches(
        whole,
        &perm_grant_role("srv1", "banker")
    ));
}

/// Три уровня выдачи для подсайта: все подсайты, один целиком, одно действие.
#[test]
fn hub_moderation_supports_three_wildcard_levels() {
    let node = perm_hub_moderate("srv1");
    assert_eq!(node, "noro.hub.srv1.moderate");

    assert!(any_permission_matches(["noro.hub.*"], &node));
    assert!(any_permission_matches(["noro.hub.srv1.*"], &node));
    assert!(any_permission_matches([node.as_str()], &node));
    assert!(any_permission_matches([PERM_SUPERADMIN], &node));
}

/// То, ради чего подсайт вообще привязан к серверу: модератор одного сообщества
/// не хозяйничает в соседнем.
#[test]
fn hub_moderation_does_not_leak_to_another_server() {
    let granted = [perm_hub_moderate("srv1")];
    let granted = [granted[0].as_str()];

    assert!(!any_permission_matches(granted, &perm_hub_moderate("srv2")));
    assert!(!any_permission_matches(
        ["noro.hub.srv1.*"],
        &perm_hub_pin("srv2")
    ));
}

/// Закрепление и уборка — разные доверия, поэтому и узлы разные.
#[test]
fn pinning_is_not_implied_by_moderation() {
    let moderator = [perm_hub_moderate("srv1")];
    let moderator = [moderator[0].as_str()];
    assert!(!any_permission_matches(moderator, &perm_hub_pin("srv1")));
}

/// Настройка подсайта живёт в админке и ветку `noro.hub.*` не открывает: раздать
/// сообществу его собственную модерацию можно, не пуская никого в панель.
#[test]
fn hub_admin_nodes_are_separate_from_the_hub_branch() {
    assert!(!any_permission_matches(["noro.hub.*"], PERM_HUB_EDIT));
    assert!(!any_permission_matches(
        [PERM_HUB_EDIT],
        &perm_hub_moderate("srv1")
    ));
    assert!(any_permission_matches([PERM_ADMIN_ALL], PERM_HUB_VIEW));
}

/// Реестр — единственный источник узлов, и дубль в нём означал бы две разные
/// подсказки на одно и то же право.
#[test]
fn the_registry_has_no_duplicates() {
    let mut seen = std::collections::HashSet::new();
    for node in ALL_NODES {
        assert!(seen.insert(node.name), "duplicate node: {}", node.name);
        assert!(node.name.starts_with("noro."), "odd node: {}", node.name);
    }
}

/// Полный админский шаблон обязан покрывать каждый админский узел: он и
/// выдаётся как «полный доступ».
#[test]
fn admin_wildcard_covers_every_admin_node() {
    for node in ALL_NODES {
        if node.name.starts_with("noro.admin.") {
            assert!(
                permission_matches(PERM_ADMIN_ALL, node.name),
                "{} is not covered",
                node.name
            );
        }
    }
}

/// Банкир одной сборки не распоряжается деньгами соседней — ради этого счета
/// и привязаны к серверу.
#[test]
fn bank_rights_do_not_cross_servers() {
    let here = perm_hub_bank("srv1", "withdraw");
    assert!(any_permission_matches([here.as_str()], &here));
    assert!(!any_permission_matches(
        [here.as_str()],
        &perm_hub_bank("srv2", "withdraw")
    ));
}

/// Внести и снять — разные права: кассир на входе и тот, кто уносит со счёта,
/// это разные люди.
#[test]
fn depositing_does_not_imply_withdrawing() {
    let deposit = perm_hub_bank("srv1", "deposit");
    assert!(!any_permission_matches(
        [deposit.as_str()],
        &perm_hub_bank("srv1", "withdraw")
    ));
}

/// Ветка банка выдаётся целиком одной строкой — так её и отдают банкиру.
#[test]
fn the_whole_bank_branch_can_be_granted_at_once() {
    let all = ["noro.hub.srv1.bank.*"];
    assert!(any_permission_matches(
        all,
        &perm_hub_bank("srv1", "deposit")
    ));
    assert!(any_permission_matches(
        all,
        &perm_hub_bank("srv1", "withdraw")
    ));
    // Но модерацию ленты она не открывает: это другое доверие.
    assert!(!any_permission_matches(all, &perm_hub_moderate("srv1")));
}

/// Доступ к официальному счёту выдаётся по коду счёта, и подстановка работает
/// на каждом уровне: все счета сервера, один счёт целиком, одно действие.
#[test]
fn account_access_supports_three_wildcard_levels() {
    let node = perm_hub_account("srv1", "mayor", ACCOUNT_SPEND);
    assert_eq!(node, "noro.hub.srv1.account.mayor.spend");

    assert!(any_permission_matches(["noro.hub.srv1.account.*"], &node));
    assert!(any_permission_matches(
        ["noro.hub.srv1.account.mayor.*"],
        &node
    ));
    assert!(any_permission_matches([node.as_str()], &node));
}

/// Смотреть кассу и брать из неё — разные доверия: аудитор не расплачивается
/// деньгами мэрии, и один узел не подразумевает другой.
#[test]
fn seeing_an_account_does_not_allow_spending_from_it() {
    let auditor = [perm_hub_account("srv1", "mayor", ACCOUNT_VIEW)];
    let auditor = [auditor[0].as_str()];

    assert!(any_permission_matches(
        auditor,
        &perm_hub_account("srv1", "mayor", ACCOUNT_VIEW)
    ));
    assert!(!any_permission_matches(
        auditor,
        &perm_hub_account("srv1", "mayor", ACCOUNT_SPEND)
    ));
    assert!(!any_permission_matches(
        auditor,
        &perm_hub_account("srv1", "mayor", ACCOUNT_MANAGE)
    ));
}

/// Казначей мэрии не распоряжается счётом полиции — и тем более счётом того же
/// имени на соседнем сервере.
#[test]
fn account_access_does_not_leak_to_another_account_or_server() {
    let mayor = [perm_hub_account("srv1", "mayor", ACCOUNT_SPEND)];
    let mayor = [mayor[0].as_str()];

    assert!(!any_permission_matches(
        mayor,
        &perm_hub_account("srv1", "police", ACCOUNT_SPEND)
    ));
    assert!(!any_permission_matches(
        mayor,
        &perm_hub_account("srv2", "mayor", ACCOUNT_SPEND)
    ));
}

/// Банкир двигает деньги между казной и картами, казначей — по своему счёту.
/// Одно не подразумевает другого, иначе «выдать доступ к счёту мэрии» открыло
/// бы кассу банка.
#[test]
fn account_access_is_not_the_same_as_being_a_banker() {
    let banker = [perm_hub_bank("srv1", "deposit")];
    let banker = [banker[0].as_str()];
    assert!(!any_permission_matches(
        banker,
        &perm_hub_account("srv1", "mayor", ACCOUNT_SPEND)
    ));

    let treasurer = [perm_hub_account("srv1", "mayor", ACCOUNT_SPEND)];
    let treasurer = [treasurer[0].as_str()];
    assert!(!any_permission_matches(
        treasurer,
        &perm_hub_bank("srv1", "deposit")
    ));
}

/// Управление списком счетов и распоряжение деньгами одного из них — разные
/// доверия: казначей мэрии не должен заводить себе новые структуры.
#[test]
fn managing_accounts_is_not_spending_from_one() {
    let treasurer = [perm_hub_account("srv1", "mayor", ACCOUNT_SPEND)];
    let treasurer = [treasurer[0].as_str()];
    assert!(!any_permission_matches(
        treasurer,
        &perm_hub_accounts("srv1", ACCOUNTS_CREATE)
    ));

    let manager = [perm_hub_accounts("srv1", ACCOUNTS_CREATE)];
    let manager = [manager[0].as_str()];
    assert!(!any_permission_matches(
        manager,
        &perm_hub_account("srv1", "mayor", ACCOUNT_SPEND)
    ));
}

/// Завести счёт, переименовать и закрыть — три разных решения, и последнее
/// самое опасное: выдав «создавать», сервер не выдаёт «удалять».
#[test]
fn account_management_actions_are_separate() {
    let creator = [perm_hub_accounts("srv1", ACCOUNTS_CREATE)];
    let creator = [creator[0].as_str()];
    assert!(!any_permission_matches(
        creator,
        &perm_hub_accounts("srv1", ACCOUNTS_DELETE)
    ));
    assert!(any_permission_matches(
        ["noro.hub.srv1.accounts.*"],
        &perm_hub_accounts("srv1", ACCOUNTS_DELETE)
    ));
}
