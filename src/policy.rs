//! Strategy system for configurable display width behavior.
//!
//! This module defines [`WidthPolicy`], a runtime struct that allows customizing
//! the width treatment of graphemes by category:
//!
//! - Emoji (e.g. 😄, 🧑‍🤝‍🧑)
//! - CJK ideographs (e.g. 汉字), Kana, Hangul
//! - Fullwidth symbols and punctuation (e.g. Ａ, 、)
//! - Fallback for unknown graphemes
//!
//! ## Built-in Policies
//!
//! - [`WidthPolicy::terminal()`] — terminal-style (emoji = 2, CJK = 2)
//! - [`WidthPolicy::markdown()`] — Markdown-style (emoji = 1, CJK = 2)
//! - [`WidthPolicy::compact()`] — minimal width (everything = 1)
//!
//! ## Usage
//!
//! Used with policy-aware functions such as:
//!
//! - [`display_width_with_policy`](crate::grapheme::policy_ext::display_width_with_policy)
//! - [`WithPolicy`](crate::with_policy::WithPolicy) for fluent API
//!
//! > **Note:** This module is only available when the `policy` feature is enabled.

/// Defines per-category width behavior for grapheme display.
///
/// This struct allows customizing how wide each category of character
/// (emoji, CJK, fullwidth symbols) is treated at runtime.
///
/// Requires enabling the `policy` feature.
#[derive(Debug, Clone)]
pub struct WidthPolicy {
    /// Width for emoji graphemes (e.g., 😄, 🧑‍🤝‍🧑)
    pub emoji: usize,

    /// Width for CJK ideographs (e.g., 漢字), kana, and hangul
    pub cjk: usize,

    /// Width for fullwidth symbol variants and East Asian punctuations (e.g., Ａ, 、)
    pub variant: usize,

    /// Fallback width for unknown or uncategorized graphemes
    pub fallback: usize,
}

impl WidthPolicy {
    /// Standard terminal policy (emoji = 2, CJK = 2, variant = 2, fallback = 1).
    ///
    /// Recommended for monospaced environments like terminals and TUI apps.
    pub fn terminal() -> Self {
        Self {
            emoji: 2,
            cjk: 2,
            variant: 2,
            fallback: 1,
        }
    }

    /// Markdown-friendly policy (emoji = 1, CJK = 2).
    ///
    /// Optimized for Markdown tables and web text rendering where emoji occupy 1 cell.
    pub fn markdown() -> Self {
        Self {
            emoji: 1,
            cjk: 2,
            variant: 2,
            fallback: 1,
        }
    }

    /// Compact layout policy (everything = 1).
    ///
    /// Useful for logs, command-line tables, or space-constrained TUI components.
    pub fn compact() -> Self {
        Self {
            emoji: 1,
            cjk: 1,
            variant: 1,
            fallback: 1,
        }
    }

    /// Returns a tuple that uniquely identifies this policy's behavior.
    ///
    /// This is used for internal comparison only, such as determining
    /// whether a policy matches one of the built-in presets.
    ///
    /// ⚠️ Not intended for semantic equality.
    pub fn as_tuple(&self) -> (usize, usize, usize, usize) {
        (self.emoji, self.cjk, self.variant, self.fallback)
    }
    
    /// (Optional extension) Override width for a specific character.
    ///
    /// Currently, a placeholder for future per-character adjustments.
    pub fn override_char(self, _ch: char, _w: usize) -> Self {
        // optional: implement override logic later
        self
    }
}
