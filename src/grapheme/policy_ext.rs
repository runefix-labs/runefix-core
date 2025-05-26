#![cfg(feature = "policy")]

//! Policy-aware variants of functions in [`crate::grapheme::basic`].
//!
//! These functions provide the same grapheme-based string segmentation
//! and measurement capabilities, but allow the caller to explicitly
//! specify a [`WidthPolicy`] to control layout behavior.
//!
//! Use this module when targeting non-default display contexts such as:
//!
//! - Terminal rendering with wide emoji (`WidthPolicy::terminal()`)
//! - Markdown table alignment (`WidthPolicy::markdown()`)
//! - Log output or tight layout (`WidthPolicy::compact()`)
//!
//! All functions in this module are gated behind the `policy` feature flag.

use unicode_segmentation::UnicodeSegmentation;
use crate::{policy::WidthPolicy, width::get_display_width_with_policy};

/// Same as [`display_width`](crate::display_width), but applies the given [`WidthPolicy`] strategy.
pub fn display_width_with_policy(s: &str, policy: Option<&WidthPolicy>) -> usize {
    UnicodeSegmentation::graphemes(s, true)
        .map(|g| get_display_width_with_policy(g, policy))
        .sum()
}

/// Same as [`display_widths`](crate::display_widths), but applies the given [`WidthPolicy`] strategy.
pub fn display_widths_with_policy(s: &str, policy: Option<&WidthPolicy>) -> Vec<usize> {
    UnicodeSegmentation::graphemes(s, true)
        .map(|g| get_display_width_with_policy(g, policy))
        .collect()
}

/// Same as [`grapheme_widths`](crate::grapheme_widths), but applies the given [`WidthPolicy`] strategy.
pub fn grapheme_widths_with_policy<'a>(
    s: &'a str,
    policy: Option<&WidthPolicy>
) -> Vec<(&'a str, usize)> {
    UnicodeSegmentation::graphemes(s, true)
        .map(|g| (g, get_display_width_with_policy(g, policy)))
        .collect()
}

/// Same as [`truncate_by_width`](crate::truncate_by_width), but applies the given [`WidthPolicy`] strategy.
pub fn truncate_by_width_with_policy<'a>(
    s: &'a str,
    max_width: usize,
    policy: Option<&WidthPolicy>
) -> &'a str {
    let mut total_width = 0;
    let mut end_byte = 0;

    for g in UnicodeSegmentation::graphemes(s, true) {
        let w: usize = get_display_width_with_policy(g, policy);
        
        if total_width + w > max_width {
            break;
        }
        
        total_width += w;
        end_byte += g.len();
    }

    &s[..end_byte]
}

/// Same as [`split_by_width`](crate::split_by_width), but applies the given [`WidthPolicy`] strategy.
pub fn split_by_width_with_policy(
    s: &str,
    max_width: usize,
    policy: Option<&WidthPolicy>
) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for g in UnicodeSegmentation::graphemes(s, true) {
        let w: usize = get_display_width_with_policy(g, policy);

        if current_width + w > max_width && !current_line.is_empty() {
            result.push(current_line.clone());
            current_line.clear();
            current_width = 0;
        }

        current_line.push_str(g);
        current_width += w;
    }

    if !current_line.is_empty() {
        result.push(current_line);
    }

    result
}
