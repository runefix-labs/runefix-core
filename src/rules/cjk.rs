use std::collections::HashSet;
use once_cell::sync::Lazy;
use serde_json::Value;

/// Static set of fullwidth CJK Unified Ideographs.
///
/// This set includes commonly used Chinese characters (Hanzi),
/// based on the contents of `cjk_unified.json`, embedded at compile time.
///
/// Used internally by the width engine to determine whether
/// a grapheme should be rendered as double-width in terminal environments.
static CJK_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set: HashSet<&'static str> = HashSet::new();

    // Embed JSON data at compile time
    let json = include_str!("../assets/cjk_unified.json");

    // Parse JSON and extract keys
    let map: Value = serde_json::from_str(json).expect("Invalid cjk_unified.json");

    for (k, _) in map.as_object().unwrap() {
        // Leak to create static lifetime for fast lookup
        set.insert(Box::leak(k.clone().into_boxed_str()));
    }

    set
});

/// Returns `true` if the given grapheme is a fullwidth CJK Unified Ideograph.
///
/// This check is based on a static hash set populated from
/// `cjk_unified.json`. The match is exact and reflects characters
/// considered double-width in East Asian terminals.
///
/// # Arguments
///
/// * `g` - A grapheme cluster (typically a single CJK character)
///
/// # Returns
///
/// `true` if the grapheme is in the CJK fullwidth set.
pub(crate) fn is_cjk(g: &str) -> bool {
    CJK_SET.contains(g)
}
