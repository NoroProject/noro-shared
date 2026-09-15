---
title: "hub"
description: "A server's hub: the feed, its members, towns, market, court, petitions and"
---

A server's hub: the feed, its members, towns, market, court, petitions and

| Call | What it does | Needs |
|---|---|---|
| `feed(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | The feed, newest and pinned first. | `hub = ["read"]` |
| `members(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | The hub's members. | `hub = ["read"]` |
| `playtime(server_id: Uuid, who: impl IntoPlayerRef) -> Result<serde_json::Value, ModuleError>` | How long a player has played on this server, and when they were last seen. | `hub = ["read"]` |
| `post(server_id: Uuid, author: impl IntoPlayerRef, body: &str) -> Result<serde_json::Value, ModuleError>` | Writes a post into the feed on behalf of a player. | `hub = ["read", "post"]` |
| `towns(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | The towns of a server. | `towns = ["read"]` |
| `town(server_id: Uuid, id: Uuid) -> Result<Option<serde_json::Value>, ModuleError>` | One town. | `towns = ["read"]` |
| `market(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | Lots currently on sale. | `market = ["read"]` |
| `lot(server_id: Uuid, id: Uuid) -> Result<Option<serde_json::Value>, ModuleError>` | One lot. | `market = ["read"]` |
| `court(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | Court cases. | `court = ["read"]` |
| `court_case(server_id: Uuid, id: Uuid) -> Result<Option<serde_json::Value>, ModuleError>` | One court case. | `court = ["read"]` |
| `petitions(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | Petitions and how far along they are. | `petitions = ["read"]` |
| `sign(server_id: Uuid, petition_id: Uuid, who: impl IntoPlayerRef) -> Result<bool, ModuleError>` | Signs a petition on behalf of a player. `false` — they had already signed. | `petitions = ["read", "sign"]` |
| `unsign(server_id: Uuid, petition_id: Uuid, who: impl IntoPlayerRef) -> Result<bool, ModuleError>` | Takes a signature back. `false` — there was none. | `petitions = ["read", "sign"]` |
| `fines(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | The fines of a server. | `fines = ["read"]` |
| `fines_of(server_id: Uuid, who: impl IntoPlayerRef) -> Result<HubPage, ModuleError>` | The fines of one player. | `fines = ["read"]` |
| `fine(draft: FineDraft) -> Result<serde_json::Value, ModuleError>` | Issues a fine, in the name of a staff member. | `fines = ["read", "issue"]` |
| `found_town(draft: TownDraft) -> Result<serde_json::Value, ModuleError>` | Founds a town, in the name of the player who becomes its mayor. | `towns = ["read", "manage"]` |
| `list_lot(draft: LotDraft) -> Result<serde_json::Value, ModuleError>` | Lists a lot on the market, in the seller's name. | `market = ["read", "sell"]` |
| `file_claim(draft: ClaimDraft) -> Result<serde_json::Value, ModuleError>` | Files a claim in court, in the plaintiff's name. | `court = ["read", "file"]` |
| `start_petition(draft: PetitionDraft) -> Result<serde_json::Value, ModuleError>` | Starts a petition, in the name of its author. | `petitions = ["read", "create"]` |
