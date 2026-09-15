//! Inter-module RPC wire requests.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCall {
    pub target: String,
    pub method: String,
    pub payload: Value,
}
