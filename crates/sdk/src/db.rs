//! Your own tables.
//!
//! Ship `migrations/0001_init.sql` in the package and the master creates a
//! schema `mod_<your_id>`, applies your migrations and tracks what it applied.
//! Everything here runs inside that schema: unqualified names resolve to your
//! tables, and nothing else is reachable.
//!
//! ```ignore
//! db::execute(Query::new("INSERT INTO orders (player_id, total) VALUES ($1, $2)")
//!     .bind(player.to_string())
//!     .bind(500))?;
//!
//! let rows = db::query(Query::new("SELECT * FROM orders WHERE total > $1").bind(100))?;
//! for row in rows {
//!     let total = row["total"].as_i64().unwrap_or(0);
//! }
//! ```
//!
//! # What you cannot reach
//!
//! Queries run under a restricted Postgres role with no rights outside your
//! schema, so `SELECT * FROM users` is refused by the database itself rather
//! than by a check somebody has to remember to write. Platform data comes from
//! the typed domains — [`crate::players`], [`crate::roles`] and the rest —
//! where capabilities are checked and the projection is deliberate.
//!
//! # Values are bound, never spliced
//!
//! `$1`, `$2`, … and [`Query::bind`]. Assembling a statement with `format!`
//! works right up to the first username with an apostrophe in it.

use noro_module_abi::error::ModuleError;
use noro_module_abi::sql::{Query, Rows};

/// Runs a query and returns its rows.
///
/// Rows are objects keyed by column name, so `row["total"]` reads the column
/// rather than a position that shifts when the query grows.
///
/// Requires `db = true`.
pub fn query(q: Query) -> Result<Rows, ModuleError> {
    crate::host::db_query_call(q)
}

/// Runs a statement and returns how many rows it touched.
///
/// For `INSERT`, `UPDATE`, `DELETE` and DDL. A statement that returns rows
/// works here too — the rows are simply discarded.
///
/// Requires `db = true`.
pub fn execute(q: Query) -> Result<u64, ModuleError> {
    crate::host::db_execute_call(q)
}

/// The first row, or `None`.
///
/// Requires `db = true`.
pub fn one(q: Query) -> Result<Option<serde_json::Map<String, serde_json::Value>>, ModuleError> {
    Ok(query(q)?.into_iter().next())
}

/// The first column of the first row, parsed into `T`.
///
/// For `SELECT count(*)` and the like.
///
/// Requires `db = true`.
pub fn scalar<T: serde::de::DeserializeOwned>(q: Query) -> Result<Option<T>, ModuleError> {
    let Some(row) = one(q)? else {
        return Ok(None);
    };
    let Some((_, value)) = row.into_iter().next() else {
        return Ok(None);
    };
    serde_json::from_value(value)
        .map(Some)
        .map_err(|e| ModuleError::invalid(format!("value did not parse: {e}")))
}
