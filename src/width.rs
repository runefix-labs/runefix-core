//! Core width resolution logic for grapheme clusters.
//!
//! This module defines low-level width computation used by all public APIs.
//!
//! It supports:
//! - Terminal-style default width (`get_display_width`)
//! - Customizable policy-based width (`get_display_width_with_policy`)
//!
//! Widths are resolved to 0, 1, or 2 columns, depending on:
//! - Control characters
//! - ASCII
//! - CJK, Hangul, Kana, fullwidth symbols
//! - Emoji (including ZWJ)
//!
//! Feature `policy` enables runtime policy customization.

use crate::rules::cjk::is_cjk;
use crate::rules::kana::is_kana;
use crate::rules::emoji::is_emoji;
use crate::rules::hangul::is_hangul;
use crate::rules::punct::is_fullwidth_punct;
use crate::rules::variants::is_fullwidth_variant;
#[cfg(feature = "policy")]
use crate::policy::WidthPolicy;

//
// ─── Public API Entrypoints ─────────────────────────────────────────
//

/// Returns the display width of a grapheme cluster for terminal environments.
///
/// This API is always available, and automatically falls back to a built-in strategy
/// if the `policy` feature is not enabled.
///
/// This function determines how many columns a grapheme cluster (i.e., a user-perceived character)
/// occupies when rendered in a monospace terminal or console. It follows Unicode-aware rules with
/// fullwidth handling for East Asian scripts and emoji combinations.
///
/// The evaluation order is carefully structured to minimize misclassification:
///
/// 1. **Control characters** (e.g. `\x01`) → width = `0`
/// 2. **ASCII characters** (<= U+007F) → width = `1`
/// 3. **Fullwidth single characters** (exact match in lookup tables):
///     - CJK Unified Ideographs
///     - Japanese Kana (Hiragana/Katakana)
///     - Hangul syllables
///     - Fullwidth symbol variants (e.g. `Ａ`, `！`)
///     - Fullwidth punctuations (e.g. `。`, `、`)
///     → width = `2`
/// 4. **Emoji** (including multi-codepoint ZWJ sequences) → width = `2`
///
/// Characters not matching any of the above are treated as width `1`.
///
/// # Arguments
///
/// * `s` - A string slice, typically one grapheme cluster (`&str`)
///
/// # Returns
///
/// A `usize` indicating the display width: `0`, `1`, or `2`.
///
/// # Example (internal usage only)
///
/// ```rust,ignore
/// // Example usage:
/// // Widths follow terminal-style behavior (CJK = 2, emoji = 2)
/// assert_eq!(get_display_width("R"), 1);
/// assert_eq!(get_display_width("語"), 2);
/// assert_eq!(get_display_width("🦀"), 2);
/// assert_eq!(get_display_width("\x00"), 0);
/// ```
///
/// # Note
///
/// If the `policy` feature is enabled, prefer using [`get_display_width_with_policy()`]  
/// instead of [`get_display_width()`] to apply environment-specific width rules.
#[cfg(not(feature = "policy"))]
pub(crate) fn get_display_width(s: &str) -> usize {
    DefaultPolicy.resolve_width(s)
}

/// Returns the display width of a grapheme cluster using [`WidthPolicy::terminal()`].
///
/// Requires `policy` feature.
#[cfg(feature = "policy")]
pub(crate) fn get_display_width(s: &str) -> usize {
    WidthPolicy::terminal().resolve_width(s)
}

/// Grapheme width lookup using a custom width policy.
///
/// Requires enabling `--features policy`.
///
/// # Parameters
/// - `s`: grapheme cluster (as `&str`)
/// - `policy`: custom `WidthPolicy`, or fallback to `.terminal()` if `None`
///
/// # Use Case
/// Supports runtime customization of width behavior,
/// enabling environment-specific layout strategies (e.g. markdown, TUI, logs).
///
/// # Returns
/// - width = `0`, `1`, or `2`
///
/// # Strategy
/// - Each rule uses `policy.cjk`, `policy.emoji`, `policy.variant`, etc.
#[cfg(feature = "policy")]
pub fn get_display_width_with_policy(s: &str, policy: Option<&WidthPolicy>) -> usize {
    policy.unwrap_or(&WidthPolicy::terminal()).resolve_width(s)
}

//
// ─── Width Resolution for Policy (Feature = "policy") ──────────────
//

#[cfg(feature = "policy")]
impl WidthPolicy {
    /// Resolves the width of a grapheme using this policy.
    ///
    /// Applies per-category width rules for emoji, CJK, variants, etc.
    pub fn resolve_width(&self, s: &str) -> usize {
        let mut chars = s.chars();

        if let (Some(ch), None) = (chars.next(), chars.next()) {
            if ch.is_control() {
                return 0;
            }

            if ch <= '\u{007F}' {
                return 1;
            }

            if is_cjk(s) || is_kana(s) || is_hangul(s) {
                return self.cjk;
            }

            if is_fullwidth_variant(s) || is_fullwidth_punct(s) {
                return self.variant;
            }
        }

        // Emoji lookup is deferred to the end for two reasons:
        // - It supports multi-codepoint graphemes (e.g. "👩‍❤️‍💋‍👨"), which cannot be matched earlier.
        // - The emoji dataset is broad and may include symbols like "©" or "™",
        //   which are normally rendered with width 1 unless followed by variation selectors.
        if is_emoji(s) {
            return self.emoji;
        }

        self.fallback
    }
}

//
// ─── Internal Fallback (No Policy) ─────────────────────────────────
//

/// Internal default width resolver used when `policy` is disabled.
#[cfg(not(feature = "policy"))]
struct DefaultPolicy;

#[cfg(not(feature = "policy"))]
impl DefaultPolicy {
    /// Resolves width using terminal-style fallback logic.
    fn resolve_width(&self, s: &str) -> usize {
        let mut chars = s.chars();

        if let (Some(ch), None) = (chars.next(), chars.next()) {
            if ch.is_control() {
                return 0;
            }

            if ch <= '\u{007F}' {
                return 1;
            }

            if is_cjk(s) || is_kana(s) || is_hangul(s) {
                return 2;
            }

            if is_fullwidth_variant(s) || is_fullwidth_punct(s) {
                return 2;
            }
        }

        // Defer emoji lookup to avoid misclassifying short graphemes
        if is_emoji(s) {
            return 2;
        }

        1
    }
}
