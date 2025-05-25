use std::collections::HashSet;
use once_cell::sync::Lazy;
use serde_json::Value;

/// Static set of fullwidth Korean Hangul syllables.
///
/// This set includes precomposed Hangul syllables used in modern Korean,
/// populated from the `korean_syllables.json` file embedded at compile time.
///
/// It is used internally by the display width engine to determine whether
/// a grapheme should be rendered as double-width in monospace terminals.
static HANGUL_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set: HashSet<&'static str> = HashSet::new();

    // Embed JSON data at compile time
    let json = include_str!("../assets/korean_syllables.json");

    // Parse JSON and extract keys
    let map: Value = serde_json::from_str(json).expect("Invalid korean_syllables.json");

    for (k, _) in map.as_object().unwrap() {
        // Leak to extend lifetime for static lookup
        set.insert(Box::leak(k.clone().into_boxed_str()));
    }

    set
});

/// Returns `true` if the provided grapheme is a fullwidth Hangul syllable.
///
/// This function checks whether the input is a precomposed Hangul character
/// from the Unicode Hangul Syllables block (U+AC00 to U+D7AF), and listed
/// in the embedded JSON.
///
/// # Arguments
///
/// * `g` - A grapheme cluster to check
///
/// # Returns
///
/// `true` if the grapheme is a fullwidth Korean Hangul syllable.
pub(crate) fn is_hangul(g: &str) -> bool {
    HANGUL_SET.contains(g)
}
