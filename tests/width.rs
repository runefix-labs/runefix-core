//! Unit tests for display width calculation using `RuneDisplayWidth` trait.
//!
//! These tests verify the accuracy of terminal width measurement for
//! various types of Unicode characters, including:
//!
//! - Basic Latin (ASCII)
//! - Greek letters
//! - CJK ideographs (Chinese, Japanese, Korean)
//! - Fullwidth symbol variants
//! - Emoji (including base, ZWJ sequences, and families)
//! - Edge cases such as unassigned or unknown codepoints

use runefix_core::RuneDisplayWidth;

#[test]
fn test_ascii_width() {
    assert_eq!('a'.rune_width(), 1); // ASCII character
}

#[test]
fn test_greek_omega_width() {
    assert_eq!('ω'.rune_width(), 1); // Greek lowercase omega
}

#[test]
fn test_chinese_width() {
    assert_eq!('你'.rune_width(), 2); // Common CJK ideograph
}

#[test]
fn test_japanese_katakana() {
    assert_eq!('ツ'.rune_width(), 2); // Katakana character
}

#[test]
fn test_korean_hangul() {
    assert_eq!('한'.rune_width(), 2); // Hangul syllable
}

#[test]
fn test_emoji_base() {
    assert_eq!('😂'.rune_width(), 2); // Emoji (single codepoint)
}

#[test]
fn test_emoji_family() {
    assert_eq!("👨‍👩‍👧‍👦".rune_width(), 2); // Emoji ZWJ family sequence
}

#[test]
fn test_emoji_zwj() {
    let emoji = "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}\u{1F466}";
    assert_eq!(emoji.rune_width(), 2); // Same as 👨‍👩‍👧‍👦 (decomposed)
}

#[test]
fn test_fullwidth_latin() {
    assert_eq!("\u{FF21}".rune_width(), 2); // Fullwidth 'A'
}

#[test]
fn test_cjk_ext_b() {
    assert_eq!("\u{20000}".rune_width(), 2); // CJK Unified Ideographs Extension B (𠀀)
}

#[test]
fn test_unknown_char() {
    assert_eq!("\u{10FFFF}".rune_width(), 1); // Max valid Unicode codepoint (unassigned)
}
