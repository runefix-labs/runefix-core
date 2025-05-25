//! Unit tests for grapheme segmentation and display width utilities.
//!
//! These tests verify width computation, truncation, and segmentation logic
//! for strings containing a mix of CJK characters, ASCII, and emoji.

use runefix_core::*;

#[test]
fn test_display_widths_per_char() {
    // "补塔🔪，不行，怪我咯" ———— 来自 LOL 辅助的无声控诉
    assert_eq!(display_widths("你a1👇"), [2, 1, 1, 2]);
}

#[test]
fn test_display_width_total_sum() {
    // "你a1👇"：终端宽度 6，心态宽度 0
    assert_eq!(display_width("你a1👇"), 6);
}

#[test]
fn test_split_graphemes() {
    // Splits the string into grapheme clusters
    assert_eq!(
        split_graphemes("你a1👇"),
        vec!["你", "a", "1", "👇"]
    );
}

#[test]
fn test_grapheme_widths() {
    // Returns each grapheme with its corresponding width
    assert_eq!(
        grapheme_widths("你a1👇"),
        vec![("你", 2), ("a", 1), ("1", 1), ("👇", 2)]
    );
}

#[test]
fn test_truncate_by_width() {
    // Truncates the string to a target display width (in columns)
    assert_eq!(truncate_by_width("你a1👇", 2), "你");     // width = 2
    assert_eq!(truncate_by_width("你a1👇", 3), "你a");    // width = 3
    assert_eq!(truncate_by_width("你a1👇", 4), "你a1");   // width = 4
    assert_eq!(truncate_by_width("你a1👇", 6), "你a1👇"); // no truncation needed
}

#[test]
fn test_split_by_width() {
    let s = "你a1👇"; // Total width = 6
    let parts = split_by_width(s, 4);
    assert_eq!(parts, vec!["你a1", "👇"]); // Splits before overflowing
}
