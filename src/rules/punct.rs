use std::collections::HashSet;
use once_cell::sync::Lazy;
use serde_json::Value;

/// Static set of fullwidth punctuation marks (e.g., `，`, `。`, `！`, `？`, `【】`, `《》`).
///
/// These characters are commonly used in Chinese, Japanese, and Korean typography,
/// and are typically rendered as occupying two terminal columns (fullwidth).
///
/// The list is loaded from `fullwidth_punctuations.json` and embedded at compile time.
/// Used internally to determine whether a grapheme should be treated as double-width.
static FULLWIDTH_PUNCT_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set: HashSet<&'static str> = HashSet::new();

    // Embed JSON data at compile time
    let json = include_str!("../assets/fullwidth_punctuations.json");

    // Parse JSON and extract keys
    let map: Value = serde_json::from_str(json).expect("Invalid fullwidth_punctuations.json");

    for (k, _) in map.as_object().unwrap() {
        // Leak boxed string to obtain a &'static str
        set.insert(Box::leak(k.clone().into_boxed_str()));
    }

    set
});

/// Returns `true` if the given grapheme is a fullwidth punctuation mark.
///
/// This includes typographically wide symbols used in East Asian languages,
/// such as `。`, `、`, `！`, `《`, `》`, etc.
///
/// # Arguments
///
/// * `g` - A grapheme cluster to check
///
/// # Returns
///
/// `true` if the grapheme is a fullwidth punctuation character.
pub(crate) fn is_fullwidth_punct(g: &str) -> bool {
    FULLWIDTH_PUNCT_SET.contains(g)
}
