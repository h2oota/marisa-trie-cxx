//! marisa module
//!
//! このモジュールは marisa の C/C++ バインディングを薄くラップした公開要素（定数、列挙型、型エイリアスなど）を提供します。
//!
//! 公開される定数や列挙型は、元の C++ MARISA 実装の同等の意味を持ちます。
//! ここで公開される値を使って、Trie の構成（キャッシュレベル、テールモード、ノード順序など）を指定します。

#![allow(unused_imports)]
// #![feature(specialization)]

// pub use create::agent::*;
// pub use key::*;
// pub use keyset::*;
// pub use trie::*;

// mod agent;
// mod key;
// mod keyset;
// mod trie;

//use libc::size_t;


//use std::os::raw::{c_char, c_ulong};

/// 最小試行数 (MARISA_MIN_NUM_TRIES)
///
/// Trie 構築時の num_tries の下限を表します。通常は marisa のデフォルト定数を参照してください。
pub use marisa_wrapper::marisa_num_tries_MARISA_MIN_NUM_TRIES as MARISA_MIN_NUM_TRIES;
/// 最大試行数 (MARISA_MAX_NUM_TRIES)
pub use marisa_wrapper::marisa_num_tries_MARISA_MAX_NUM_TRIES as MARISA_MAX_NUM_TRIES;
/// デフォルトの試行数 (MARISA_DEFAULT_NUM_TRIES)
pub use marisa_wrapper::marisa_num_tries_MARISA_DEFAULT_NUM_TRIES as MARISA_DEFAULT_NUM_TRIES;

/// キャッシュレベル: 巨大
pub use marisa_wrapper::marisa_cache_level_MARISA_HUGE_CACHE as MARISA_HUGE_CACHE;
/// キャッシュレベル: 大
pub use marisa_wrapper::marisa_cache_level_MARISA_LARGE_CACHE as MARISA_LARGE_CACHE;
/// キャッシュレベル: 普通
pub use marisa_wrapper::marisa_cache_level_MARISA_NORMAL_CACHE as MARISA_NORMAL_CACHE;
/// キャッシュレベル: 小
pub use marisa_wrapper::marisa_cache_level_MARISA_SMALL_CACHE as MARISA_SMALL_CACHE;
/// キャッシュレベル: 非常に小さい
pub use marisa_wrapper::marisa_cache_level_MARISA_TINY_CACHE as MARISA_TINY_CACHE;
/// デフォルトキャッシュレベル
pub use marisa_wrapper::marisa_cache_level_MARISA_DEFAULT_CACHE as MARISA_DEFAULT_CACHE;

/// テールモード: テキスト
pub use marisa_wrapper::marisa_tail_mode_MARISA_TEXT_TAIL as MARISA_TEXT_TAIL;
/// テールモード: バイナリ
pub use marisa_wrapper::marisa_tail_mode_MARISA_BINARY_TAIL as MARISA_BINARY_TAIL;
/// デフォルトテールモード
pub use marisa_wrapper::marisa_tail_mode_MARISA_DEFAULT_TAIL as MARISA_DEFAULT_TAIL;

/// ノード順序: ラベル順
pub use marisa_wrapper::marisa_node_order_MARISA_LABEL_ORDER as MARISA_LABEL_ORDER;
/// ノード順序: 重み順
pub use marisa_wrapper::marisa_node_order_MARISA_WEIGHT_ORDER as MARISA_WEIGHT_ORDER;
/// デフォルトのノード順序
pub use marisa_wrapper::marisa_node_order_MARISA_DEFAULT_ORDER as MARISA_DEFAULT_ORDER;

/// 設定マスク: TRIES マスク
pub use marisa_wrapper::marisa_config_mask_MARISA_NUM_TRIES_MASK as MARISA_NUM_TRIES_MASK;
/// 設定マスク: CACHE レベル
pub use marisa_wrapper::marisa_config_mask_MARISA_CACHE_LEVEL_MASK as MARISA_CACHE_LEVEL_MASK;
/// 設定マスク: TAIL モード
pub use marisa_wrapper::marisa_config_mask_MARISA_TAIL_MODE_MASK as MARISA_TAIL_MODE_MASK;
/// 設定マスク: NODE ORDER
pub use marisa_wrapper::marisa_config_mask_MARISA_NODE_ORDER_MASK as MARISA_NODE_ORDER_MASK;
/// 設定マスク: CONFIG 全体
pub use marisa_wrapper::marisa_config_mask_MARISA_CONFIG_MASK as MARISA_CONFIG_MASK;
/// テールモードの列挙型
pub use marisa_wrapper::marisa_TailMode as TailMode;
/// ノード順序の列挙型
pub use marisa_wrapper::marisa_NodeOrder as NodeOrder;


/*
pub use marisa_wrapper::marisa_error_code_MARISA_OK as MARISA_OK;
pub use marisa_wrapper::marisa_error_code_MARISA_STATE_ERROR as MARISA_STATE_ERROR;
pub use marisa_wrapper::marisa_error_code_MARISA_NULL_ERROR as MARISA_NULL_ERROR;
pub use marisa_wrapper::marisa_error_code_MARISA_BOUND_ERROR as MARISA_BOUND_ERROR;
pub use marisa_wrapper::marisa_error_code_MARISA_RANGE_ERROR as MARISA_RANGE_ERROR;
pub use marisa_wrapper::marisa_error_code_MARISA_CODE_ERROR as MARISA_CODE_ERROR;
pub use marisa_wrapper::marisa_error_code_MARISA_RESET_ERROR as MARISA_RESET_ERROR;
pub use marisa_wrapper::marisa_error_code_MARISA_SIZE_ERROR as MARISA_SIZE_ERROR;
pub use marisa_wrapper::marisa_error_code_MARISA_MEMORY_ERROR as MARISA_MEMORY_ERROR;
*/
