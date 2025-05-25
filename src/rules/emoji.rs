use std::collections::HashSet;
use once_cell::sync::Lazy;
use serde_json::Value;

/// A static set of fully-qualified emoji strings used for display width detection.
///
/// This set is built at startup by parsing two embedded JSON files:
/// - `emoji_base.json`: simple (single-codepoint) emoji
/// - `emoji_zwj.json`: complex ZWJ sequences (multi-codepoint emoji)
///
/// All keys are leaked into `'static` lifetime via `Box::leak` to allow zero-copy
/// lookup during runtime. The resulting `HashSet<&'static str>` enables fast
/// detection of known emoji graphemes.
///
/// This mechanism supports both simple and composed emoji, which is critical for
/// accurate terminal width detection (e.g., `"👩‍❤️‍💋‍👨"` should count as width 2).
static EMOJI_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set: HashSet<&'static str> = HashSet::new();

    // Embed JSON files at compile time
    let base_json = include_str!("../assets/emoji_base.json");
    let zwj_json = include_str!("../assets/emoji_zwj.json");

    // Deserialize JSON content into `Value` maps
    let base_map: Value = serde_json::from_str(base_json).expect("Invalid emoji_base.json");
    let zwj_map: Value = serde_json::from_str(zwj_json).expect("Invalid emoji_zwj.json");

    // Leak keys to static references for efficient storage in the HashSet
    for (k, _) in base_map.as_object().unwrap() {
        set.insert(Box::leak(k.clone().into_boxed_str()));
    }
    for (k, _) in zwj_map.as_object().unwrap() {
        set.insert(Box::leak(k.clone().into_boxed_str()));
    }

    set
});

/// Returns `true` if the provided grapheme is a known emoji (base or ZWJ sequence).
///
/// This is used internally by the width engine to treat emoji as double-width
/// in monospace terminal rendering. The match is exact and covers fully-qualified
/// sequences only.
///
/// # Note
/// - This is an internal utility and not part of the public API.
/// - The emoji set is intentionally private to avoid exposing implementation details.
///
/// # Example (internal usage only)
/// ```rust,ignore
/// assert!(is_emoji("😄"));
/// assert!(is_emoji("👨‍👩‍👧‍👦"));
/// assert!(!is_emoji("A"));
/// ```
pub(crate) fn is_emoji(grapheme: &str) -> bool {
    EMOJI_SET.contains(grapheme)
}
