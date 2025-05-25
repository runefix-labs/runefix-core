use crate::rules::cjk::is_cjk;
use crate::rules::kana::is_kana;
use crate::rules::emoji::is_emoji;
use crate::rules::hangul::is_hangul;
use crate::rules::punct::is_fullwidth_punct;
use crate::rules::variants::is_fullwidth_variant;

/// Returns the display width of a grapheme in terminal environments.
///
/// This function determines how many columns a grapheme cluster (i.e., a user-perceived character)
/// occupies when rendered in a monospace terminal or console. It follows Unicode-aware rules with
/// fullwidth handling for East Asian scripts and emoji combinations.
///
/// The evaluation order is carefully structured to minimize misclassification:
///
/// 1. **Control characters** (e.g. `\x01`) → width = `0`
/// 2. **ASCII characters** (<= U+007F) → width = `1`
/// 3. **Fullwidth single characters** (exact match in lookup tables):
///     - CJK Unified Ideographs
///     - Japanese Kana (Hiragana/Katakana)
///     - Hangul syllables
///     - Fullwidth symbol variants (e.g. `Ａ`, `！`)
///     - Fullwidth punctuations (e.g. `。`, `、`)
///     → width = `2`
/// 4. **Emoji** (including multi-codepoint ZWJ sequences) → width = `2`
///
/// Characters not matching any of the above are treated as width `1`.
///
/// # Arguments
///
/// * `s` - A string slice, typically one grapheme cluster (`&str`)
///
/// # Returns
///
/// A `usize` indicating the display width: `0`, `1`, or `2`.
///
/// # Example (internal usage only)
///
/// ```rust,ignore
/// // This function is crate-private and used internally:
/// assert_eq!(get_display_width("R"), 1);
/// assert_eq!(get_display_width("語"), 2);
/// assert_eq!(get_display_width("🦀"), 2);
/// assert_eq!(get_display_width("\x00"), 0);
/// ```
pub(crate) fn get_display_width(s: &str) -> usize {
    let mut chars = s.chars();

    // Handle single-character graphemes
    if let (Some(ch), None) = (chars.next(), chars.next()) {
        if ch.is_control() {
            return 0;
        }

        if ch <= '\u{007F}' {
            return 1;
        }

        if is_cjk(s)
            || is_kana(s)
            || is_hangul(s)
            || is_fullwidth_variant(s)
            || is_fullwidth_punct(s)
        {
            return 2;
        }
    }

    // Emoji lookup is deferred to the end for two reasons:
    // - It supports multi-codepoint graphemes (e.g. "👩‍❤️‍💋‍👨"), which cannot be matched earlier.
    // - The emoji dataset is broad and may include symbols like "©" or "™",
    //   which are normally rendered with width 1 unless followed by variation selectors.
    if is_emoji(s) {
        return 2;
    }

    1
}
