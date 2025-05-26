//! Basic grapheme-aware width processing functions.
//!
//! This module provides the core, always-available APIs for:
//!
//! - Unicode grapheme segmentation
//! - Terminal-style display width measurement
//! - Safe truncation and line wrapping
//!
//! These functions use a default [`terminal`](crate::policy::WidthPolicy::terminal) layout strategy,
//! without requiring any additional features.
//!
//! See [`policy_ext`](crate::grapheme::policy_ext) for configurable width behavior.

use unicode_segmentation::UnicodeSegmentation;
use crate::width::get_display_width;

/// Returns all grapheme clusters ("atoms") in the input string as a vector of string slices.
///
/// A **grapheme atom** is the smallest unit of text perceived by users — which may consist
/// of one or more Unicode codepoints. This function respects complex characters like:
///
/// - Emoji ZWJ sequences (e.g., "👩‍❤️‍💋‍👨")
/// - Composed Hangul syllables
/// - Accented characters (e.g., "é")
///
/// Internally uses Unicode segmentation to ensure accurate human-perceived boundaries.
///
/// # Arguments
///
/// * `s` - The input string to split
///
/// # Returns
///
/// A `Vec<&str>` where each item is a grapheme cluster (user-perceived character).
///
/// # Example
///
/// ```rust
/// use runefix_core::grapheme_atoms;
///
/// let clusters = grapheme_atoms("Love👩‍❤️‍💋‍👨爱");
/// assert_eq!(clusters, vec!["L", "o", "v", "e", "👩‍❤️‍💋‍👨", "爱"]);
/// ```
pub fn grapheme_atoms(s: &str) -> Vec<&str> {
    UnicodeSegmentation::graphemes(s, true).collect()
}

/// Returns the total display width (in columns) of a string, based on grapheme clusters.
///
/// This function segments the input string into Unicode grapheme clusters and sums
/// the display width of each one using [`display_width`]. The result reflects
/// how much horizontal space the entire string occupies in a monospace terminal,
/// accounting for wide characters such as CJK ideographs and emoji.
///
/// # Arguments
///
/// * `s` - The input string to measure
///
/// # Returns
///
/// The total display width of the string in terminal columns.
///
/// # Example
///
/// ```rust
/// use runefix_core::display_width;
///
/// let width = display_width("Hi，世界");
/// assert_eq!(width, 8); // 1 + 1 + 2 + 2 + 2
/// ```
pub fn display_width(s: &str) -> usize {
    UnicodeSegmentation::graphemes(s, true).map(get_display_width).sum()
}

/// Returns the display width (in columns) of each grapheme cluster in the input string.
///
/// This function segments the input string into Unicode grapheme clusters and computes
/// the display width of each one individually. It is useful for scenarios like monospace
/// text layout, visual alignment, or rendering terminals where East Asian characters
/// and emoji take more than one column.
///
/// # Arguments
///
/// * `s` - The input string to analyze
///
/// # Returns
///
/// A vector of display widths (`usize`) for each grapheme cluster in order.
///
/// # Example
///
/// ```rust
/// use runefix_core::display_widths;
///
/// let widths = display_widths("Hi，世界");
/// assert_eq!(widths, vec![1, 1, 2, 2, 2]);
/// ```
pub fn display_widths(s: &str) -> Vec<usize> {
    UnicodeSegmentation::graphemes(s, true).map(get_display_width).collect()
}

/// Returns the display width of each grapheme cluster in the input string.
///
/// This function splits the string into Unicode grapheme clusters and pairs
/// each one with its terminal display width (in columns). This is useful for
/// visually aligned rendering, layout calculation, and Unicode debugging,
/// especially with complex emoji or East Asian characters.
///
/// # Arguments
///
/// * `s` - The input string to analyze
///
/// # Returns
///
/// A vector of tuples, where each item is a grapheme cluster and its
/// corresponding display width: `(&str, usize)`
///
/// # Example
///
/// ```rust
/// use runefix_core::grapheme_widths;
///
/// let result = grapheme_widths("Hi，世界");
/// assert_eq!(
///     result,
///     vec![("H", 1), ("i", 1), ("，", 2), ("世", 2), ("界", 2)]
/// );
/// ```
pub fn grapheme_widths(s: &str) -> Vec<(&str, usize)> {
    UnicodeSegmentation::graphemes(s, true)
        .map(|g| (g, get_display_width(g)))
        .collect()
}

/// Truncates a string by display width while preserving grapheme cluster boundaries.
///
/// This function ensures that wide characters such as emoji or CJK ideographs are
/// never split in the middle. It safely cuts off the string so that its total
/// display width does not exceed the given `max_width`, making it ideal for
/// terminal or TUI rendering.
///
/// # Arguments
///
/// * `s` - The input string to truncate
/// * `max_width` - Maximum allowed display width in terminal columns
///
/// # Returns
///
/// A string slice that fits within the specified display width without cutting graphemes.
///
/// # Example
///
/// ```rust
/// use runefix_core::truncate_by_width;
///
/// let s = "Hi 👋，世界";
/// let short = truncate_by_width(s, 6);
/// assert_eq!(short, "Hi 👋");
/// ```
pub fn truncate_by_width(s: &str, max_width: usize) -> &str {
    let mut total_width = 0;
    let mut end_byte = 0;

    for g in UnicodeSegmentation::graphemes(s, true) {
        let w: usize = get_display_width(g);

        if total_width + w > max_width {
            break;
        }

        total_width += w;
        end_byte += g.len(); // Byte offset to cut safely
    }

    &s[..end_byte]
}

/// Splits a string into lines based on display width, preserving grapheme boundaries.
///
/// This function ensures that wide characters such as emoji, CJK ideographs, or
/// fullwidth punctuation are not split mid-grapheme. It breaks the input string
/// into a sequence of lines, each with a total display width that does not exceed
/// the given `max_width`. Ideal for terminal word wrapping and monospace layout.
///
/// # Arguments
///
/// * `s` - The input string to wrap
/// * `max_width` - Maximum display width (in columns) for each line
///
/// # Returns
///
/// A vector of strings, each representing a wrapped line within the given width.
///
/// # Example
///
/// ```rust
/// use runefix_core::split_by_width;
///
/// let lines = split_by_width("Hello 👋 世界！", 5);
/// assert_eq!(lines, vec!["Hello", " 👋 ", "世界", "！"]);
/// ```
pub fn split_by_width(s: &str, max_width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for g in UnicodeSegmentation::graphemes(s, true) {
        let w: usize = get_display_width(g);

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
