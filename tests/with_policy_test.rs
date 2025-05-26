#![cfg(feature = "policy")]

//! Unit tests for `WithPolicy` and `AppliedPolicy`.
//!
//! Verifies fluent policy-based display width operations including:
//! - policy switching (terminal / markdown / compact)
//! - width computation
//! - truncation and line splitting
//! - integration with Display, Debug, Deref, etc.

use runefix_core::{WidthPolicy, WithPolicy};

#[test]
fn test_terminal_policy() {
    let policy = WidthPolicy::terminal(); // emoji = 2
    let binding = WithPolicy::new(&policy);
    let view = binding.apply("👋");
    assert_eq!(view.display_width(), 2);
    assert_eq!(view.display_widths(), vec![2]);
}

#[test]
fn test_markdown_policy() {
    let policy = WidthPolicy::markdown(); // emoji = 1
    let binding = WithPolicy::new(&policy);
    let view = binding.apply("👋");
    assert_eq!(view.display_width(), 1);
    assert_eq!(view.display_widths(), vec![1]);
}

#[test]
fn test_compact_policy() {
    let policy = WidthPolicy::compact(); // emoji = 1, cjk = 1
    let binding = WithPolicy::new(&policy);
    let view = binding.apply("👋你好");
    assert_eq!(view.display_width(), 3);
    assert_eq!(view.display_widths(), vec![1, 1, 1]);
}

#[test]
fn test_truncate_behavior() {
    let policy = WidthPolicy::terminal();
    let binding = WithPolicy::new(&policy);
    let view = binding.apply("Hi 👋 世界");
    let truncated = view.truncate_by_width(5);
    assert_eq!(truncated, "Hi 👋");
}

#[test]
fn test_split_behavior() {
    let policy = WidthPolicy::markdown();
    let binding = WithPolicy::new(&policy);
    let view = binding.apply("Hello 👋 世界!");
    let lines = view.split_by_width(5);

    assert_eq!(lines, vec!["Hello", " 👋 世", "界!"]);
}

#[test]
fn test_display_trait() {
    let policy = WidthPolicy::markdown();
    let binding = WithPolicy::new(&policy);
    let view = binding.apply("Hello 👋");
    assert_eq!(format!("{}", view), "Hello 👋");
}

#[test]
fn test_debug_trait() {
    let policy = WidthPolicy::compact();
    let binding = WithPolicy::new(&policy);
    let view = binding.apply("Hi 👋 世界");
    let debug_output = format!("{:?}", view);
    assert!(debug_output.contains("AppliedPolicy"));
    assert!(debug_output.contains("Hi 👋 世界"));
}

#[test]
fn test_partial_eq_str() {
    let policy = WidthPolicy::markdown();
    let binding = WithPolicy::new(&policy);
    let view = binding.apply("abc");
    assert_eq!(view, "abc");
}

#[test]
fn test_deref_behavior() {
    let policy = WidthPolicy::markdown();
    let binding = WithPolicy::new(&policy);
    let view = binding.apply("abc");
    assert!(view.starts_with("a")); // thanks to Deref<Target=str>
}
