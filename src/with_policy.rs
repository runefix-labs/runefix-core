#![cfg(feature = "policy")]

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
//! let wrapped = binding.apply(text);
//! assert_eq!(wrapped.display_width(), 12);
//! ```
//!
//! Requires the `policy` feature.

/// Fluent entry point for applying a [`WidthPolicy`] to strings.
pub struct WithPolicy<'a> {
    policy: &'a crate::policy::WidthPolicy,
}

impl<'a> WithPolicy<'a> {
    /// Creates a new wrapper around the given [`WidthPolicy`].
    pub fn new(policy: &'a crate::policy::WidthPolicy) -> Self {
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

/// A width-aware view of a string with an attached [`WidthPolicy`].
pub struct AppliedPolicy<'a, 's> {
    s: &'s str,
    policy: &'a crate::policy::WidthPolicy,
}

impl<'a, 's> AppliedPolicy<'a, 's> {
    /// Returns the Unicode grapheme clusters of the string.
    pub fn grapheme_atoms(&self) -> Vec<&str> {
        crate::grapheme::grapheme_atoms(self.s)
    }

    /// Returns the total display width under the current policy.
    pub fn display_width(&self) -> usize {
        crate::grapheme::policy_ext::display_width_with_policy(self.s, Some(self.policy))
    }

    /// Returns the display width of each grapheme cluster.
    pub fn display_widths(&self) -> Vec<usize> {
        crate::grapheme::policy_ext::display_widths_with_policy(self.s, Some(self.policy))
    }

    /// Returns a list of `(grapheme, width)` tuples.
    pub fn widths_grapheme(&self) -> Vec<(&str, usize)> {
        crate::grapheme::policy_ext::grapheme_widths_with_policy(self.s, Some(self.policy))
    }

    /// Truncates the string by width without cutting grapheme boundaries.
    pub fn truncate_by_width(&self, max_width: usize) -> &str {
        crate::grapheme::policy_ext::truncate_by_width_with_policy(self.s, max_width, Some(self.policy))
    }

    /// Wraps the string into lines by display width.
    pub fn split_by_width(&self, max_width: usize) -> Vec<String> {
        crate::grapheme::policy_ext::split_by_width_with_policy(self.s, max_width, Some(self.policy))
    }
}
