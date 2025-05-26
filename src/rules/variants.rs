use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashSet;

/// Static set of fullwidth symbol variants (e.g., `！`, `Ａ`, `￥`).
///
/// This includes East Asian fullwidth counterparts of ASCII punctuation and letters,
/// such as `！` (U+FF01) for `!`, `Ａ` (U+FF21) for `A`, and `￥` (U+FFE5) for `¥`.
///
/// The data is loaded from `fullwidth_variants.json` and embedded at compile time.
/// Used to detect characters that should be rendered as double-width in terminals.
static VARIANT_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set: HashSet<&'static str> = HashSet::new();

    // Embed JSON data at compile time
    let json = include_str!("../assets/fullwidth_variants.json");

    // Parse JSON and extract keys
    let map: Value = serde_json::from_str(json).expect("Invalid fullwidth_variants.json");

    for (k, _) in map.as_object().unwrap() {
        // Leak boxed string to obtain a &'static str
        set.insert(Box::leak(k.clone().into_boxed_str()));
    }

    set
});

/// Returns `true` if the given grapheme is a fullwidth variant symbol.
///
/// These characters typically belong to the Unicode range U+FF01–U+FF60
/// and represent wide-presentation forms of ASCII characters.
///
/// # Arguments
///
/// * `g` - A grapheme cluster to check
///
/// # Returns
///
/// `true` if the grapheme is a fullwidth variant symbol.
pub(crate) fn is_fullwidth_variant(g: &str) -> bool {
    VARIANT_SET.contains(g)
}
