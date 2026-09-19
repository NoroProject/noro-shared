//! Grants on a single hosted server, held by its owner and its subusers.
//!
//! A deliberately separate namespace from `noro.*`: these mean nothing outside
//! one server. Putting them in the global registry would offer an operator a
//! role checkbox that grants nothing anywhere — and the role editor is exactly
//! where that lie would be believed.
//!
//! Matching reuses [`crate::permissions::permission_matches`], so `file.*` and
//! a bare `*` work the same way they do for staff permissions. An owner
//! resolves to `["*"]`.

/// One grant: name, group for the UI, and an i18n key explaining it.
pub struct PanelNode {
    pub name: &'static str,
    pub group: &'static str,
    pub title: &'static str,
}

macro_rules! panel_nodes {
    ($($konst:ident = $name:literal, $group:literal, $title:literal;)*) => {
        $(pub const $konst: &str = $name;)*

        /// Every grant — for the subuser editor.
        pub const PANEL_ALL_NODES: &[&PanelNode] = &[$(&PanelNode {
            name: $name, group: $group, title: $title,
        }),*];
    };
}

panel_nodes! {
    // --- power and console ----------------------------------------------------
    CONTROL_CONSOLE    = "control.console", "panel-perm-group-control", "panel-perm-control-console";
    CONTROL_COMMAND    = "control.command", "panel-perm-group-control", "panel-perm-control-command";
    CONTROL_START      = "control.start", "panel-perm-group-control", "panel-perm-control-start";
    CONTROL_STOP       = "control.stop", "panel-perm-group-control", "panel-perm-control-stop";
    CONTROL_RESTART    = "control.restart", "panel-perm-group-control", "panel-perm-control-restart";
    CONTROL_KILL       = "control.kill", "panel-perm-group-control", "panel-perm-control-kill";

    // --- files ----------------------------------------------------------------
    FILE_READ          = "file.read", "panel-perm-group-files", "panel-perm-file-read";
    FILE_WRITE         = "file.write", "panel-perm-group-files", "panel-perm-file-write";
    FILE_DELETE        = "file.delete", "panel-perm-group-files", "panel-perm-file-delete";
    FILE_ARCHIVE       = "file.archive", "panel-perm-group-files", "panel-perm-file-archive";
    FILE_SFTP          = "file.sftp", "panel-perm-group-files", "panel-perm-file-sftp";

    // --- backups --------------------------------------------------------------
    BACKUP_READ        = "backup.read", "panel-perm-group-backups", "panel-perm-backup-read";
    BACKUP_CREATE      = "backup.create", "panel-perm-group-backups", "panel-perm-backup-create";
    BACKUP_RESTORE     = "backup.restore", "panel-perm-group-backups", "panel-perm-backup-restore";
    BACKUP_DELETE      = "backup.delete", "panel-perm-group-backups", "panel-perm-backup-delete";
    BACKUP_DOWNLOAD    = "backup.download", "panel-perm-group-backups", "panel-perm-backup-download";

    // --- startup and network --------------------------------------------------
    STARTUP_READ       = "startup.read", "panel-perm-group-startup", "panel-perm-startup-read";
    STARTUP_UPDATE     = "startup.update", "panel-perm-group-startup", "panel-perm-startup-update";
    ALLOCATION_READ    = "allocation.read", "panel-perm-group-startup", "panel-perm-allocation-read";
    ALLOCATION_UPDATE  = "allocation.update", "panel-perm-group-startup", "panel-perm-allocation-update";

    // --- schedules ------------------------------------------------------------
    SCHEDULE_READ      = "schedule.read", "panel-perm-group-schedules", "panel-perm-schedule-read";
    SCHEDULE_UPDATE    = "schedule.update", "panel-perm-group-schedules", "panel-perm-schedule-update";
    SCHEDULE_DELETE    = "schedule.delete", "panel-perm-group-schedules", "panel-perm-schedule-delete";

    // --- subusers -------------------------------------------------------------
    USER_READ          = "user.read", "panel-perm-group-users", "panel-perm-user-read";
    USER_CREATE        = "user.create", "panel-perm-group-users", "panel-perm-user-create";
    USER_UPDATE        = "user.update", "panel-perm-group-users", "panel-perm-user-update";
    USER_DELETE        = "user.delete", "panel-perm-group-users", "panel-perm-user-delete";

    // --- the server itself ----------------------------------------------------
    BUILD_SYNC         = "build.sync", "panel-perm-group-server", "panel-perm-build-sync";
    SETTINGS_RENAME    = "settings.rename", "panel-perm-group-server", "panel-perm-settings-rename";
    SETTINGS_REINSTALL = "settings.reinstall", "panel-perm-group-server", "panel-perm-settings-reinstall";
    SETTINGS_LISTING   = "settings.listing", "panel-perm-group-server", "panel-perm-settings-listing";
    ACTIVITY_READ      = "activity.read", "panel-perm-group-server", "panel-perm-activity-read";
}

/// What an owner holds. Not a list of every node: a grant added later has to
/// reach the owner without a migration.
pub const OWNER: &str = "*";

/// Does this set of grants cover the required one?
pub fn has(granted: &[String], required: &str) -> bool {
    crate::permissions::any_permission_matches(granted.iter().map(String::as_str), required)
}

/// Grants that only read. A suspended server keeps answering these so the owner
/// can see *why* it is suspended instead of a wall of refusals.
pub fn is_read_only(node: &str) -> bool {
    matches!(
        node,
        CONTROL_CONSOLE
            | FILE_READ
            | BACKUP_READ
            | STARTUP_READ
            | ALLOCATION_READ
            | SCHEDULE_READ
            | USER_READ
            | ACTIVITY_READ
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_covers_everything() {
        let owner = vec![OWNER.to_string()];
        for node in PANEL_ALL_NODES {
            assert!(has(&owner, node.name), "owner must hold {}", node.name);
        }
    }

    #[test]
    fn group_wildcard_covers_its_group() {
        let granted = vec!["file.*".to_string()];
        assert!(has(&granted, FILE_READ));
        assert!(has(&granted, FILE_WRITE));
        assert!(!has(&granted, CONTROL_KILL));
    }

    #[test]
    fn unrelated_grant_does_not_leak() {
        let granted = vec![BACKUP_READ.to_string()];
        assert!(has(&granted, BACKUP_READ));
        assert!(!has(&granted, BACKUP_DELETE));
        assert!(!has(&granted, FILE_READ));
    }

    /// Names are stored in `panel_server_access.permissions`; renaming one
    /// silently drops that grant for everyone who already had it.
    #[test]
    fn names_are_unique_and_grouped() {
        let mut seen = std::collections::BTreeSet::new();
        for node in PANEL_ALL_NODES {
            assert!(seen.insert(node.name), "duplicate node {}", node.name);
            assert!(
                node.name.contains('.'),
                "{} must be group.action",
                node.name
            );
        }
    }
}
