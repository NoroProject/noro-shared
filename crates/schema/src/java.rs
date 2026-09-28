//! Java runtimes the master hands out.
//!
//! Mojang publishes its own JREs as named components — `jre-legacy`,
//! `java-runtime-gamma` and so on — one per platform, each with a file manifest
//! and hashes. The master already downloads the one a build asks for; this is
//! the same list, opened up so a player can pick a different one.
//!
//! The reason is mods. A build is tested on the runtime Mojang recommends for
//! its Minecraft version, and that is the right default for everyone. But a mod
//! the player added themselves can need a newer JVM than that, and the only
//! honest alternatives are "pick another runtime" or "the game closes and
//! nobody knows why".
//!
//! Only Mojang's own runtimes, never a path on the player's disk. A JDK found
//! on a machine is an unknown build of an unknown version, and a crash on it
//! costs an evening to tell apart from a broken mod. These come with hashes,
//! are verified like every other file, and are the same bytes for everybody.

use serde::{Deserialize, Serialize};

/// One runtime a player may choose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JavaRuntimeOption {
    /// Mojang's component name, e.g. `java-runtime-gamma`.
    pub component: String,
    /// Full version as Mojang states it, e.g. `21.0.3`.
    pub version: String,
    /// Major number — what a mod's requirements are written in.
    pub major: u32,
    /// The one this build was built against.
    pub recommended: bool,
}

/// Major version out of Mojang's version string.
///
/// `1.8.0_412` is Java 8, not Java 1: before Java 9 the major number is the
/// second component, and a mod asking for 17 must not be told that 8 satisfies
/// it because "1" is smaller.
pub fn major_of(version: &str) -> u32 {
    let mut parts = version.split(['.', '_', '-', '+']);
    match parts.next().and_then(|p| p.parse::<u32>().ok()) {
        Some(1) => parts.next().and_then(|p| p.parse().ok()).unwrap_or(1),
        Some(n) => n,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::major_of;

    #[test]
    fn modern_versions_use_the_first_number() {
        assert_eq!(major_of("21.0.3"), 21);
        assert_eq!(major_of("17"), 17);
    }

    #[test]
    fn java_8_is_8() {
        assert_eq!(major_of("1.8.0_412"), 8);
    }

    #[test]
    fn nonsense_is_zero_rather_than_a_panic() {
        assert_eq!(major_of(""), 0);
        assert_eq!(major_of("alpha"), 0);
    }
}
