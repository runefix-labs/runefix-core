//! # runefix-core
//!
//! Unicode-aware display width engine for terminals, TUI, and Markdown rendering.
//!
//! `runefix-core` provides precise width computation for multilingual text,
//! with support for:
//!
//! - East Asian fullwidth characters (CJK)
//! - Emoji (including multi-codepoint ZWJ sequences)
//! - Fullwidth punctuation and symbol variants
//! - Grapheme-aware string truncation and wrapping
//!
//! It ensures consistent text alignment across platforms and fonts
//! by working at the grapheme cluster level, consulting curated Unicode-derived datasets.
//!
//! ## Features
//!
//! - [`get_display_width`] – Width of a single grapheme
//! - [`display_widths`] – Widths of multiple graphemes (Vec<(str, usize)>)
//! - [`split_graphemes`] – Unicode-aware segmentation
//! - [`truncate_by_width`] – Safe truncation without splitting CJK/emoji
//! - [`split_by_width`] – Line wrapping by terminal width
//! - [`RuneDisplayWidth`] – Trait for `.rune_width()` extension
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
//! ## Resources
//!
//! - [📖 Dataset source & CLI](https://github.com/pokeyaro/runefix-rs/tree/main/crates/core)
//! - [📦 Crates.io](https://crates.io/crates/runefix-core)
//! - [🧪 docs.rs Documentation](https://docs.rs/runefix-core)
//!
//! > **Note:** Enable the `policy` feature to use configurable width strategies
//! > such as `terminal()`, `markdown()`, or `compact()`.

/// Public API: Core utilities for grapheme width and segmentation.
pub use grapheme::{
    display_width,
    display_widths,
    grapheme_widths,
    split_graphemes,
    truncate_by_width,
    split_by_width
};

/// Public API: Trait extension for `.rune_width()`.
pub use ext::RuneDisplayWidth;

/// Public API: Unicode database version used by this build.
pub use consts::UNICODE_VERSION;

/// Feature-gated: Width policy system for dynamic customization.
#[cfg(feature = "policy")]
pub use policy::WidthPolicy;

#[cfg(feature = "policy")]
pub use width::display_width_with_policy;

// ─────────────────────────────────────────────────────────────
// Internal modules (not directly re-exported)
// ─────────────────────────────────────────────────────────────

mod consts;
mod rules;
mod width;
mod grapheme;
mod ext;

#[cfg(feature = "policy")]
pub mod policy;
