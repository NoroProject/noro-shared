//! Who may talk to whom inside one node.

use serde::{Deserialize, Serialize};

/// Servers that reach each other directly on the node.
///
/// Everything else on the machine is closed to a server: the cloud metadata
/// service, the host's own ports, the other servers. A proxy still has to
/// reach the backends behind it, and the master is the only side that knows
/// which those are.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkGroup {
    /// Stable id of the group — the proxy's panel server.
    pub id: uuid::Uuid,
    pub servers: Vec<uuid::Uuid>,
}
