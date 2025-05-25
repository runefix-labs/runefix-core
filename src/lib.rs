//! # runefix-core
//!
//! Unicode-aware display width engine for terminals, TUI, and Markdown rendering.
//!
//! This crate provides precise width computation for multilingual text, including:
//!
//! - East Asian fullwidth characters (CJK)
//! - Emoji (including ZWJ sequences)
//! - Fullwidth punctuation and symbol variants
//! - Grapheme-aware string truncation and wrapping
//!
//! It ensures text alignment is consistent across platforms and fonts by working at
//! the grapheme cluster level and consulting curated Unicode-derived datasets.
//!
//! ## Features
//!
//! - `get_display_width`: Width of a single grapheme or string
//! - `split_graphemes`: Unicode-aware character segmentation
//! - `truncate_by_width`: Safe truncation without splitting emoji/CJK
//! - `split_by_width`: Line wrapping by column width
//!
//! ## Example
//!
//! ```rust
//! use runefix_core::{RuneDisplayWidth, grapheme_widths};
//!
//! assert_eq!('你'.rune_width(), 2);
//! assert_eq!(
//!     grapheme_widths("Hi，世界"),
//!     vec![("H", 1), ("i", 1), ("，", 2), ("世", 2), ("界", 2)]
//! );
//! ```
//!
//! See [README](https://github.com/pokeyaro/runefix-rs/tree/main/crates/core) for dataset source and usage details.

/// Re-exports: Primary public API
pub use grapheme::{
    display_width,
    display_widths,
    grapheme_widths,
    split_graphemes,
    truncate_by_width,
    split_by_width
};
pub use ext::RuneDisplayWidth;
pub use version::get_char_table_version;

// Internal modules (not re-exported directly)
mod consts;
mod version;
mod rules;
mod width;
mod grapheme;
mod ext;
