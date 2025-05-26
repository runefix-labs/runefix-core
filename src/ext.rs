//! Extension trait for ergonomic width measurement.
//!
//! This module defines [`RuneDisplayWidth`], a trait implemented for both `char` and `str`,
//! providing convenient methods like `.width()`, `.rune_width()`, and `.display_widths()`.
//!
//! These methods offer a unified interface for querying terminal display width,
//! automatically applying grapheme segmentation where appropriate.
//!
//! See also:
//! - [`display_width`](crate::grapheme::display_width)
//! - [`WidthPolicy`](crate::policy::WidthPolicy) for configurable strategies

/// Extension trait for measuring the display width of runes, graphemes, and strings.
///
/// This trait provides unified access to terminal display width calculations
/// for both `char` and `&str`. It supports:
///
/// - Single runes (`char`)
/// - Full Unicode strings with grapheme segmentation (`&str`)
///
/// All widths are measured in terminal columns, respecting East Asian widths,
/// emoji sequences, and control characters.
///
/// # Examples
///
/// ```rust
/// use runefix_core::RuneDisplayWidth;
///
/// assert_eq!('語'.rune_width(), 2);
/// assert_eq!("你👋a".display_widths(), vec![2, 2, 1]);
/// assert_eq!("Hi👋".display_width(), 4);
/// assert_eq!("👋".width(), 2);
/// ```
pub trait RuneDisplayWidth {
    /// Returns the display width of a single rune or grapheme.
    ///
    /// For `char`, this is the width of the character.
    /// For `&str`, this assumes the string is a single grapheme cluster.
    fn rune_width(&self) -> usize;

    /// Returns the total display width in terminal columns.
    ///
    /// Equivalent to summing the result of `display_widths()`.
    fn display_width(&self) -> usize;

    /// Returns the display width of each grapheme cluster in the value.
    ///
    /// For `&str`, this segments the string using Unicode grapheme rules.
    /// For `char`, returns a single-item vector.
    fn display_widths(&self) -> Vec<usize>;

    /// Returns the total display width in terminal columns
    /// (alias of `display_width()`).
    fn width(&self) -> usize {
        self.display_width()
    }
}

impl RuneDisplayWidth for str {
    fn rune_width(&self) -> usize {
        crate::width::get_display_width(self)
    }

    fn display_width(&self) -> usize {
        crate::grapheme::display_width(self)
    }

    fn display_widths(&self) -> Vec<usize> {
        crate::grapheme::display_widths(self)
    }
}

impl RuneDisplayWidth for char {
    fn rune_width(&self) -> usize {
        crate::width::get_display_width(&self.to_string())
    }

    fn display_width(&self) -> usize {
        self.rune_width()
    }

    fn display_widths(&self) -> Vec<usize> {
        vec![self.rune_width()]
    }
}
