//! Unit tests for [`atoms`] segmentation.
//!
//! These tests verify display-aware atom splitting for emoji, ZWJ sequences,
//! and variation selectors — used in layout-sensitive environments like TUI.

use runefix_core::atoms;

#[test]
fn test_atoms_family() {
    let input = "👨‍👩‍👧‍👦";
    assert_eq!(
        atoms(input),
        vec!["👨", "\u{200d}", "👩", "\u{200d}", "👧", "\u{200d}", "👦"],
    );
}

#[test]
fn test_atoms_couple() {
    let input = "👩‍❤️‍💋‍👨";
    assert_eq!(
        atoms(input),
        vec![
            "👩", "\u{200d}", "❤", "\u{fe0f}", "\u{200d}", "💋", "\u{200d}", "👨"
        ],
    );
}

#[test]
fn test_atoms_heart() {
    let input = "❤️";
    assert_eq!(atoms(input), vec!["❤", "\u{fe0f}"]);
}

#[test]
fn test_atoms_female_programmer() {
    let input = "👩‍💻";
    assert_eq!(atoms(input), vec!["👩", "\u{200d}", "💻"]);
}

#[test]
fn test_atoms_mixed_complex() {
    let input = "123，木头人🪵";
    assert_eq!(
        atoms(input),
        vec!["1", "2", "3", "，", "木", "头", "人", "🪵"]
    );
}
