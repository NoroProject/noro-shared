---
title: "players"
description: "Players."
---

Players.

| Call | What it does | Needs |
|---|---|---|
| `get(who: impl IntoPlayerRef) -> Result<Option<Player>, ModuleError>` | Finds a player by any of the ways they can be named. | the `players = ["read"]` capability |
| `require(who: impl IntoPlayerRef) -> Result<Player, ModuleError>` | The same lookup, except a missing player is an error. | — |
| `ban(who: impl IntoPlayerRef, reason: Option<&str>) -> Result<(), ModuleError>` | Bans an account outright. | `players = ["read", "ban"]` |
| `unban(who: impl IntoPlayerRef) -> Result<(), ModuleError>` | Lifts an account ban. Unbanning someone who is not banned is not an error. | `players = ["read", "ban"]` |
| `rename(who: impl IntoPlayerRef, username: &str) -> Result<(), ModuleError>` | Changes a player's Minecraft username. | `players = ["read", "rename"]` |
| `set_skin(who: impl IntoPlayerRef, url: Option<&str>, slim: bool) -> Result<(), ModuleError>` | Sets a player's skin. `url = None` clears it back to the default. | `players = ["read", "skin"]` |
| `set_cape(who: impl IntoPlayerRef, cape_id: Option<Uuid>) -> Result<(), ModuleError>` | Puts a cape from the instance's set on a player. `None` takes it off. | `players = ["read", "skin"]` |
| `capes() -> Result<Vec<Cape>, ModuleError>` | Every cape the instance has. | `players = ["read"]` |
| `presets(who: impl IntoPlayerRef) -> Result<Vec<SkinPreset>, ModuleError>` | The skins a player has saved, newest first. | `players = ["read"]` |
| `save_preset(who: impl IntoPlayerRef, name: &str, skin_url: &str, slim: Option<bool>) -> Result<SkinPreset, ModuleError>` | Saves a skin under a name. `slim = None` takes the geometry they wear now. | `players = ["read", "skin"]` |
| `delete_preset(who: impl IntoPlayerRef, preset_id: Uuid) -> Result<bool, ModuleError>` | Deletes a saved skin. `false` — it was not there, or not theirs. | `players = ["read", "skin"]` |
