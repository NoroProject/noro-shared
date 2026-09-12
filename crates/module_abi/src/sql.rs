//! Queries against a module's own Postgres schema.
//!
//! Values travel as JSON and are **bound**, never spliced into the statement.
//! That is not a style preference: a module assembling SQL by formatting its own
//! strings would be one `'` in a player's username away from a broken query, and
//! one deliberate one away from something worse.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A statement with its parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// One statement. `$1`, `$2`, … refer to `params`.
    pub sql: String,
    /// Bound values, in order. JSON types map to Postgres: a string to `text`,
    /// an integer to `bigint`, a float to `double precision`, a bool to
    /// `boolean`, null to NULL, and anything else — an object or an array — to
    /// `jsonb`.
    #[serde(default)]
    pub params: Vec<Value>,
}

impl Query {
    pub fn new(sql: impl Into<String>) -> Self {
        Query {
            sql: sql.into(),
            params: Vec::new(),
        }
    }

    /// Adds a parameter. `$1` is the first one added.
    pub fn bind(mut self, value: impl Into<Value>) -> Self {
        self.params.push(value.into());
        self
    }
}

/// What a `SELECT` returned: rows as objects, keyed by column name.
///
/// Objects rather than positional arrays, because a query that grows a column
/// should not silently shift every index in the code that reads it.
pub type Rows = Vec<serde_json::Map<String, Value>>;
