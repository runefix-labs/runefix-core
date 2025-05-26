use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashSet;

/// Static set of fullwidth Japanese kana characters (Hiragana + Katakana).
///
/// This set includes both Hiragana and Katakana syllables commonly used in Japanese text,
/// loaded from `japanese_kana.json` and embedded at compile time.
///
/// Used internally to identify kana graphemes that should be rendered as double-width
/// in terminal environments.
static KANA_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set: HashSet<&'static str> = HashSet::new();

    // Embed JSON data at compile time
    let json = include_str!("../assets/japanese_kana.json");

    // Parse JSON and extract keys
    let map: Value = serde_json::from_str(json).expect("Invalid japanese_kana.json");

    for (k, _) in map.as_object().unwrap() {
        // Leak boxed string to obtain a &'static str
        set.insert(Box::leak(k.clone().into_boxed_str()));
    }

    set
});

/// Returns `true` if the provided grapheme is a fullwidth Japanese kana character.
///
/// This includes both Hiragana (U+3040–U+309F) and Katakana (U+30A0–U+30FF),
/// matched exactly from the statically embedded kana dataset.
///
/// # Arguments
///
/// * `g` - A grapheme cluster to check
///
/// # Returns
///
/// `true` if the grapheme is a fullwidth kana character.
pub(crate) fn is_kana(g: &str) -> bool {
    KANA_SET.contains(g)
}
