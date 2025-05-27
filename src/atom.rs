//! Atom segmentation (width-aware layout units).
//!
//! This module defines [`atoms()`], a runefix-specific alternative to graphemes.
//! It segments a string into visual display units for terminal and TUI rendering.
//!
//! This helps solve alignment bugs in monospaced environments caused by emoji and CJK widths.

use crate::RuneDisplayWidth;

/// Splits the input string into **layout atoms** — visual units used for width-aware layout.
///
/// This is a **runefix-specific segmentation**, based on actual display width, not linguistic boundaries.
/// It differs from [`graphemes()`] (which follows Unicode UAX #29) by focusing purely on units that affect layout:
///
/// - Characters with width = 0 (e.g., combining marks, control codes) are grouped with their leading base
/// - Emoji sequences (e.g. ZWJ, variation selectors) are preserved as atomic units
/// - Output is suitable for TUI rendering, Markdown table layout, and CLI alignment
///
/// # Example
/// ```
/// use runefix_core::atoms;
/// assert_eq!(atoms("👩‍❤️‍💋‍👨"), vec!["👩", "\u{200d}", "❤", "\u{fe0f}", "\u{200d}", "💋", "\u{200d}", "👨"]);
/// ```
///
/// # Note
/// This function is **not** Unicode-compliant segmentation. For that, see [`graphemes()`].
pub fn atoms(s: &str) -> Vec<&str> {
    let mut atoms = Vec::new(); // Store resulting display atoms
    let mut start = 0; // Current segment start position

    for (i, c) in s.char_indices() {
        // Determine if this char has visual width
        let w = c.width();

        if w > 0 {
            if start < i {
                // Push preceding zero-width chars (e.g. ZWJ, marks)
                atoms.push(&s[start..i]);
            }

            // Push current width-bearing char as an atom
            atoms.push(&s[i..i + c.len_utf8()]);
            start = i + c.len_utf8();
        }
    }

    if start < s.len() {
        // Push trailing zero-width sequence if any
        atoms.push(&s[start..]);
    }

    atoms
}
