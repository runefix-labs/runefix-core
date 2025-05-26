#![cfg(feature = "policy")]

use runefix_core::{WidthPolicy, WithPolicy};

#[test]
fn test_terminal_policy() {
    let policy = WidthPolicy::terminal(); // emoji = 2
    let binding = WithPolicy::new(&policy);
    let wrapped = binding.apply("👋");
    assert_eq!(wrapped.display_width(), 2);
    assert_eq!(wrapped.display_widths(), vec![2]);
}

#[test]
fn test_markdown_policy() {
    let policy = WidthPolicy::markdown(); // emoji = 1
    let binding = WithPolicy::new(&policy);
    let wrapped = binding.apply("👋");
    assert_eq!(wrapped.display_width(), 1);
    assert_eq!(wrapped.display_widths(), vec![1]);
}

#[test]
fn test_compact_policy() {
    let policy = WidthPolicy::compact(); // emoji = 1, cjk = 1
    let binding = WithPolicy::new(&policy);
    let wrapped = binding.apply("👋你好");
    assert_eq!(wrapped.display_width(), 3);
    assert_eq!(wrapped.display_widths(), vec![1, 1, 1]);
}

#[test]
fn test_truncate_behavior() {
    let policy = WidthPolicy::terminal();
    let binding = WithPolicy::new(&policy);
    let wrapped = binding.apply("Hi 👋 世界");
    let truncated = wrapped.truncate_by_width(5);
    assert_eq!(truncated, "Hi 👋");
}

#[test]
fn test_split_behavior() {
    let policy = WidthPolicy::markdown();
    let binding = WithPolicy::new(&policy);
    let wrapped = binding.apply("Hello 👋 世界!");
    let lines = wrapped.split_by_width(5);

    assert_eq!(lines, vec!["Hello", " 👋 世", "界!"]);
}
