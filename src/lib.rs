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
//! 🧬 **Atom API**
//! - [`atoms`] – Runefix-specific visual segmentation for layout (width-based units)
//! 
//! 🧩 **Segmentation API**
//! - [`graphemes`] – Unicode-compliant grapheme cluster splitting (UAX #29)
//!
//! 📏 **Measurement API**
//! - [`display_width`] – Total width of a string (grapheme-aware, terminal-style)
//! - [`display_widths`] – Widths of each grapheme cluster (`Vec<usize>`)
//! - [`grapheme_widths`] – Widths with original clusters (`Vec<(&str, usize)>`)
//!
//! 📐 **Layout API**
//! - [`truncate_by_width`] – Truncates text by width without splitting graphemes
//! - [`split_by_width`] – Wraps a string into lines based on terminal width
//!
//! 🍭 **Ergonomic Extensions**
//! - [`RuneDisplayWidth`] – Trait for:
//!     - `.rune_width()` on `char`
//!     - `.width()`, `.display_width()`, `.display_widths()` on `str`
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

// ───── Public APIs ─────────────────────────────────────────────

// Atom-based segmentation for layout units (runefix-specific)
pub use atom::atoms;

// Grapheme-based core processing functions (always available)
pub use grapheme::{
    display_width, display_widths, grapheme_widths, graphemes, split_by_width, truncate_by_width,
};

// Unicode-aware trait extensions for `char` and `str`
pub use ext::RuneDisplayWidth;

// Unicode data version used internally
pub use consts::UNICODE_VERSION;

// ───── Optional: Feature-gated APIs (requires `policy`) ─────────

// Configurable width strategy struct
#[cfg(feature = "policy")]
pub use policy::WidthPolicy;

// Ergonomic wrapper for applying a WidthPolicy to strings
#[cfg(feature = "policy")]
pub use with_policy::WithPolicy;

// Policy-aware versions of grapheme layout functions
#[cfg(feature = "policy")]
pub use crate::grapheme::policy_ext::{
    display_width_with_policy, display_widths_with_policy, grapheme_widths_with_policy,
    split_by_width_with_policy, truncate_by_width_with_policy,
};

// ───── Internal Modules (implementation details) ───────────────

mod atom;
mod consts;
mod ext;
mod grapheme;
mod rules;
mod width;

#[cfg(feature = "policy")]
mod policy;
#[cfg(feature = "policy")]
mod with_policy;
