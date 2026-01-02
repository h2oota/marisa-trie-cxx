// ===== Feature-based build mode flags =====
/// Build mode: "use_system_marisa" or "vendored"
#[cfg(feature = "use_system_marisa")]
pub const BUILD_MODE: &str = "use_system_marisa";
#[cfg(not(feature = "use_system_marisa"))]
pub const BUILD_MODE: &str = "vendored";

// ===== Public modules =====

mod marisa;

// ===== Re-exports (public API surface) =====

pub use marisa:: {
    KeyObject as Key,
    QueryObject as Query,
    KeysetObject as Keyset,
    AgentObject as Agent,
    TrieObject as Trie,

    BaseTrait,
    KQTrait,
    KeyTrait,
    QueryTrait,
    KeysetTrait,
    AgentTrait,
    TrieTrait,

    MarisaError,
};
