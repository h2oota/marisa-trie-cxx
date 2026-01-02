//! marisa-trie-cxx
//!
//! Rust バインディングを通じて C++ 実装の Marisa トライ（MARISA trie）を利用するためのラッパーライブラリです。
//!
//! 主な目的:
//! - C++ の Marisa ライブラリと安全にやりとりするための薄いラッパーを提供する。
//! - Rust 側で Trie（辞書）を操作するための型・トレイトを公開する。
//!
//! 例（簡単な使用例）:
//! ```no_run
//! use marisa_trie_cxx::Trie;
//!
//! // Trie の生成やキー追加、検索などの操作を行います。
//! // 実際の使用法は各型（Key / Query / Keyset / Agent / Trie）のメソッドを参照してください。
//! ```
//!
//! ノート:
//! - このクレートは内部で C/C++ の FFI を使っています。生ポインタや unsafe ブロックが含まれる箇所があるため、
///!   使用時は API ドキュメントに記載の安全���条件（例: ライフタイム、同時アクセス制約など）を確認してください。

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/// 再エクスポート: Key オブジェクト (キーを表す型)
///
/// `Key` は単一のキー（バイト列）を表す型です。`Key` は `KQTrait` を実装しており、
/// バイト列や ID の取得・設定などの操作を提供します。
///
/// 例:
/// ```no_run
/// use marisa_trie_cxx::Key;
/// // Key の生成や操作については Key のメソッドを参照
/// ```
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
