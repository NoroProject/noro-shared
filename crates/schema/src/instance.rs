//! What the instance has switched on as a whole.
//!
//! Not per user and not per build: an operator turns a whole subsystem off
//! because the community does not use it, and every client has to stop
//! showing it — the site, the launcher, the in-game panel.
//!
//! Every field defaults to "on". A client that talks to a master from before
//! the switch existed gets no such frame at all and keeps behaving as it
//! always did; a master that adds a switch tomorrow does not turn anything off
//! in a client that has never heard of it.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstanceFeatures {
    /// Direct messages between players through the master: the site, the
    /// launcher and the in-game panel. The block list stays either way — the
    /// in-game `/ignore` is stored in it.
    #[serde(default = "on")]
    pub messenger: bool,
}

impl Default for InstanceFeatures {
    fn default() -> Self {
        Self { messenger: true }
    }
}

fn on() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_switch_reads_as_on() {
        let parsed: InstanceFeatures = serde_json::from_str("{}").expect("parses");
        assert_eq!(parsed, InstanceFeatures::default());
        assert!(parsed.messenger);
    }

    #[test]
    fn the_frames_carry_the_switch() {
        let off = InstanceFeatures { messenger: false };
        let launcher = crate::ServerWsMsg::InstanceFeatures { features: off }.to_json();
        assert_eq!(
            launcher,
            r#"{"t":"InstanceFeatures","d":{"features":{"messenger":false}}}"#
        );
        let site = crate::PlayerWsMsg::FeaturesChanged { features: off }.to_json();
        assert_eq!(
            site,
            r#"{"t":"FeaturesChanged","d":{"features":{"messenger":false}}}"#
        );
    }
}
