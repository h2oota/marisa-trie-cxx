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


    MARISA_MIN_NUM_TRIES,
    MARISA_MAX_NUM_TRIES,
    MARISA_DEFAULT_NUM_TRIES,
    MARISA_HUGE_CACHE,
    MARISA_LARGE_CACHE,
    MARISA_NORMAL_CACHE,
    MARISA_SMALL_CACHE,
    MARISA_TINY_CACHE,
    MARISA_DEFAULT_CACHE,
    MARISA_TEXT_TAIL,
    MARISA_BINARY_TAIL,
    MARISA_DEFAULT_TAIL,
    MARISA_LABEL_ORDER,
    MARISA_WEIGHT_ORDER,
    MARISA_DEFAULT_ORDER,
    MARISA_NUM_TRIES_MASK,
    MARISA_CACHE_LEVEL_MASK,
    MARISA_TAIL_MODE_MASK,
    MARISA_NODE_ORDER_MASK,
    MARISA_CONFIG_MASK,
    TailMode,
    NodeOrder,
};
