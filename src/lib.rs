
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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

mod marisa;
