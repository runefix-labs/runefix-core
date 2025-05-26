//! Fluent wrapper for [`WidthPolicy`] that allows applying it to strings.
//!
//! This module provides ergonomic syntax for width-aware operations like:
//!
//! ```rust
//! use runefix_core::{WidthPolicy, WithPolicy};
//!
//! let policy = WidthPolicy::markdown();
//! let text = "Hello 👋 世界";
//!
//! let binding = WithPolicy::new(&policy);
//! let view = binding.apply(text);
//! println!("{}", view); // Hello 👋 世界
//! println!("{:?}", view); // AppliedPolicy { text: "Hello 👋 世界", display_width: 12 }
//! assert_eq!(view.display_width(), 12);
//! ```
//!
//! Requires the `policy` feature.

// std
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;

// crate
use crate::grapheme::grapheme_atoms;
use crate::grapheme::policy_ext::{
    display_width_with_policy, display_widths_with_policy, grapheme_widths_with_policy,
    split_by_width_with_policy, truncate_by_width_with_policy,
};
use crate::policy::WidthPolicy;

/// Wraps a [`WidthPolicy`] to enable fluent reuse across multiple strings.
/// Avoids needing to repeatedly pass the policy to each function.
pub struct WithPolicy<'a> {
    policy: &'a WidthPolicy,
}

impl<'a> WithPolicy<'a> {
    /// Creates a new wrapper around the given [`WidthPolicy`].
    pub fn new(policy: &'a WidthPolicy) -> Self {
        Self { policy }
    }

    /// Applies the policy to a string and returns a width-aware view.
    pub fn apply<'s>(&'a self, input: &'s str) -> AppliedPolicy<'a, 's> {
        AppliedPolicy {
            s: input,
            policy: self.policy,
        }
    }
}

/// A borrowed view of a string with an attached [`WidthPolicy`].
///
/// This struct offers width-aware operations without taking ownership.
pub struct AppliedPolicy<'a, 's> {
    s: &'s str,
    policy: &'a WidthPolicy,
}

impl AppliedPolicy<'_, '_> {
    /// Returns the Unicode grapheme clusters of the string.
    pub fn grapheme_atoms(&self) -> Vec<&str> {
        grapheme_atoms(self.s)
    }

    /// Returns the total display width under the current policy.
    pub fn display_width(&self) -> usize {
        display_width_with_policy(self.s, Some(self.policy))
    }

    /// Returns the display width of each grapheme cluster.
    pub fn display_widths(&self) -> Vec<usize> {
        display_widths_with_policy(self.s, Some(self.policy))
    }

    /// Returns a list of `(grapheme, width)` tuples.
    pub fn widths_grapheme(&self) -> Vec<(&str, usize)> {
        grapheme_widths_with_policy(self.s, Some(self.policy))
    }

    /// Truncates the string by width without cutting grapheme boundaries.
    pub fn truncate_by_width(&self, max_width: usize) -> &str {
        truncate_by_width_with_policy(self.s, max_width, Some(self.policy))
    }

    /// Wraps the string into lines by display width.
    pub fn split_by_width(&self, max_width: usize) -> Vec<String> {
        split_by_width_with_policy(self.s, max_width, Some(self.policy))
    }
}

/// Enables printing an `AppliedPolicy` directly as a string.
/// Useful for: `println!("{}", wrapped)`.
impl Display for AppliedPolicy<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

/// Custom debug format that includes the string and its computed display width.
impl Debug for AppliedPolicy<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppliedPolicy")
            .field("text", &self.s)
            .field("display_width", &self.display_width())
            .finish()
    }
}

/// Allows implicit conversion from `AppliedPolicy` to `&str`.
impl AsRef<str> for AppliedPolicy<'_, '_> {
    fn as_ref(&self) -> &str {
        self.s
    }
}

/// Derefs `AppliedPolicy` to `str`, enabling string-like behavior.
/// Example: `wrapped.starts_with("abc")`.
impl Deref for AppliedPolicy<'_, '_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.s
    }
}

/// Enables equality checks with string values.
impl PartialEq<str> for AppliedPolicy<'_, '_> {
    fn eq(&self, other: &str) -> bool {
        self.s == other
    }
}

/// Enables equality checks with `&str` references.
impl PartialEq<&str> for AppliedPolicy<'_, '_> {
    fn eq(&self, other: &&str) -> bool {
        self.s == *other
    }
}
