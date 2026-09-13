---
title: Каталог событий
description: Все события, на которые можно подписаться. Собирается из ABI.
---

:::note
Страница собирается из `ALL_EVENTS` в `noro-module-abi` при сборке сайта. Отстать
от кода она не может, а править её руками бесполезно.
:::

Обработчик не называет событие строкой. `#[event]` берёт имя из типа аргумента —
значит, столбец **Структура** и есть то, чем подписываются:

```rust
#[event]
fn on_join(e: PlayerJoined) -> Result<()> { Ok(()) }
```

`Post` приходит после того, как всё случилось и записано. `Pre` — до действия, и
существует ради того, чтобы его отменить или изменить.

:::note[Про Pre]
Обработчик `Pre` объявляется тем, что берёт событие по `&mut`: ссылка и говорит
мастеру, что ответа стоит подождать. Доставляются все; у двух есть условия, и они
описаны в [Событиях](../../guides/events/).
:::

## Игроки и вход

| Событие | Вид | Структура |
|---|---|---|
| `player.pre_chat` | `Pre` | [`PlayerPreChat`](../../api/noro_module_abi/events/player/struct.PlayerPreChat.html) |
| `player.pre_join` | `Pre` | [`PlayerPreJoin`](../../api/noro_module_abi/events/player/struct.PlayerPreJoin.html) |
| `player.joined` | `Post` | [`PlayerJoined`](../../api/noro_module_abi/events/player/struct.PlayerJoined.html) |
| `player.left` | `Post` | [`PlayerLeft`](../../api/noro_module_abi/events/player/struct.PlayerLeft.html) |
| `user.registered` | `Post` | [`UserRegistered`](../../api/noro_module_abi/events/player/struct.UserRegistered.html) |
| `user.pre_login` | `Pre` | [`UserPreLogin`](../../api/noro_module_abi/events/player/struct.UserPreLogin.html) |
| `user.logged_in` | `Post` | [`UserLoggedIn`](../../api/noro_module_abi/events/player/struct.UserLoggedIn.html) |
| `user.banned` | `Post` | [`UserBanned`](../../api/noro_module_abi/events/player/struct.UserBanned.html) |
| `user.unbanned` | `Post` | [`UserUnbanned`](../../api/noro_module_abi/events/player/struct.UserUnbanned.html) |
| `user.pre_rename` | `Pre` | [`UserPreRename`](../../api/noro_module_abi/events/player/struct.UserPreRename.html) |
| `user.renamed` | `Post` | [`UserRenamed`](../../api/noro_module_abi/events/player/struct.UserRenamed.html) |
| `user.skin_changed` | `Post` | [`UserSkinChanged`](../../api/noro_module_abi/events/player/struct.UserSkinChanged.html) |
| `user.identity_linked` | `Post` | [`IdentityLinked`](../../api/noro_module_abi/events/player/struct.IdentityLinked.html) |
| `user.identity_unlinked` | `Post` | [`IdentityUnlinked`](../../api/noro_module_abi/events/player/struct.IdentityUnlinked.html) |

## Роли, права, доступы

| Событие | Вид | Структура |
|---|---|---|
| `role.created` | `Post` | [`RoleCreated`](../../api/noro_module_abi/events/access/struct.RoleCreated.html) |
| `role.updated` | `Post` | [`RoleUpdated`](../../api/noro_module_abi/events/access/struct.RoleUpdated.html) |
| `role.deleted` | `Post` | [`RoleDeleted`](../../api/noro_module_abi/events/access/struct.RoleDeleted.html) |
| `user.pre_role_granted` | `Pre` | [`RolePreGrant`](../../api/noro_module_abi/events/access/struct.RolePreGrant.html) |
| `user.role_granted` | `Post` | [`RoleGranted`](../../api/noro_module_abi/events/access/struct.RoleGranted.html) |
| `user.role_revoked` | `Post` | [`RoleRevoked`](../../api/noro_module_abi/events/access/struct.RoleRevoked.html) |
| `user.permission_granted` | `Post` | [`PermissionGranted`](../../api/noro_module_abi/events/access/struct.PermissionGranted.html) |
| `user.permission_revoked` | `Post` | [`PermissionRevoked`](../../api/noro_module_abi/events/access/struct.PermissionRevoked.html) |

## Сборки, билды, игровые серверы

| Событие | Вид | Структура |
|---|---|---|
| `server.created` | `Post` | [`ServerCreated`](../../api/noro_module_abi/events/infra/struct.ServerCreated.html) |
| `server.updated` | `Post` | [`ServerUpdated`](../../api/noro_module_abi/events/infra/struct.ServerUpdated.html) |
| `server.deleted` | `Post` | [`ServerDeleted`](../../api/noro_module_abi/events/infra/struct.ServerDeleted.html) |
| `build.created` | `Post` | [`BuildCreated`](../../api/noro_module_abi/events/infra/struct.BuildCreated.html) |
| `build.pre_publish` | `Pre` | [`BuildPrePublish`](../../api/noro_module_abi/events/infra/struct.BuildPrePublish.html) |
| `build.published` | `Post` | [`BuildPublished`](../../api/noro_module_abi/events/infra/struct.BuildPublished.html) |
| `build.deleted` | `Post` | [`BuildDeleted`](../../api/noro_module_abi/events/infra/struct.BuildDeleted.html) |
| `gameserver.online` | `Post` | [`GameServerOnline`](../../api/noro_module_abi/events/infra/struct.GameServerOnline.html) |
| `gameserver.offline` | `Post` | [`GameServerOffline`](../../api/noro_module_abi/events/infra/struct.GameServerOffline.html) |
| `gameserver.maintenance` | `Post` | [`GameServerMaintenance`](../../api/noro_module_abi/events/infra/struct.GameServerMaintenance.html) |
| `instance.setting_changed` | `Post` | [`InstanceSettingChanged`](../../api/noro_module_abi/events/infra/struct.InstanceSettingChanged.html) |
| `module.enabled` | `Post` | [`ModuleEnabled`](../../api/noro_module_abi/events/infra/struct.ModuleEnabled.html) |
| `module.disabled` | `Post` | [`ModuleDisabled`](../../api/noro_module_abi/events/infra/struct.ModuleDisabled.html) |
| `launcher.message` | `Post` | [`LauncherMessage`](../../api/noro_module_abi/events/infra/struct.LauncherMessage.html) |

## Модерация

| Событие | Вид | Структура |
|---|---|---|
| `punishment.pre_issue` | `Pre` | [`PunishmentPreIssue`](../../api/noro_module_abi/events/moderation/struct.PunishmentPreIssue.html) |
| `punishment.issued` | `Post` | [`PunishmentIssued`](../../api/noro_module_abi/events/moderation/struct.PunishmentIssued.html) |
| `punishment.revoked` | `Post` | [`PunishmentRevoked`](../../api/noro_module_abi/events/moderation/struct.PunishmentRevoked.html) |
| `punishment.expired` | `Post` | [`PunishmentExpired`](../../api/noro_module_abi/events/moderation/struct.PunishmentExpired.html) |
| `report.created` | `Post` | [`ReportCreated`](../../api/noro_module_abi/events/moderation/struct.ReportCreated.html) |
| `case.created` | `Post` | [`CaseCreated`](../../api/noro_module_abi/events/moderation/struct.CaseCreated.html) |
| `case.resolved` | `Post` | [`CaseResolved`](../../api/noro_module_abi/events/moderation/struct.CaseResolved.html) |

## Экономика и подсайт

| Событие | Вид | Структура |
|---|---|---|
| `bank.pre_transfer` | `Pre` | [`BankPreTransfer`](../../api/noro_module_abi/events/economy/struct.BankPreTransfer.html) |
| `bank.transferred` | `Post` | [`BankTransferred`](../../api/noro_module_abi/events/economy/struct.BankTransferred.html) |
| `bank.account_opened` | `Post` | [`BankAccountOpened`](../../api/noro_module_abi/events/economy/struct.BankAccountOpened.html) |
| `hub.pre_post` | `Pre` | [`HubPrePost`](../../api/noro_module_abi/events/economy/struct.HubPrePost.html) |
| `hub.post_created` | `Post` | [`HubPostCreated`](../../api/noro_module_abi/events/economy/struct.HubPostCreated.html) |
| `hub.member_joined` | `Post` | [`HubMemberJoined`](../../api/noro_module_abi/events/economy/struct.HubMemberJoined.html) |
| `town.founded` | `Post` | [`TownFounded`](../../api/noro_module_abi/events/economy/struct.TownFounded.html) |
| `market.lot_listed` | `Post` | [`MarketLotListed`](../../api/noro_module_abi/events/economy/struct.MarketLotListed.html) |
| `market.lot_sold` | `Post` | [`MarketLotSold`](../../api/noro_module_abi/events/economy/struct.MarketLotSold.html) |
| `fine.issued` | `Post` | [`FineIssued`](../../api/noro_module_abi/events/economy/struct.FineIssued.html) |
