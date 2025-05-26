/// Unicode Version used by this build (auto-synced).
/// auto-updated: 2025-05-26
pub const UNICODE_VERSION: &str = "16.0.0";

#[allow(dead_code)]
/// ⚠️ Deprecated: `DOUBLE_WIDTH_BLOCKS` is a static Unicode block-based heuristic.
///
/// This was originally used to determine whether a character belongs to a wide-width
/// Unicode range, such as CJK Unified Ideographs or fullwidth variants. However, this
/// approach is imprecise and not recommended for production use.
///
/// ### Drawbacks:
/// - Too coarse-grained (e.g., `0x3040–0x30FF` includes non-kana symbols)
/// - Poor alignment with real terminal rendering behaviors
/// - Not synchronized with Unicode updates (ranges are hardcoded)
///
/// ### Recommended Replacement:
/// ✅ Use modular per-category functions instead, such as:
/// - [`rules::cjk::is_cjk`]
/// - [`rules::kana::is_kana`]
/// - [`rules::hangul::is_hangul`] etc.
///
/// These offer better maintainability, precision, and extensibility.
pub const DOUBLE_WIDTH_BLOCKS: &[(u32, u32)] = &[
    (0x4E00, 0x9FFF),   // CJK Unified Ideographs
    (0x3400, 0x4DBF),   // CJK Extension A (rare characters)
    (0x3040, 0x30FF),   // Japanese Kana (Hiragana + Katakana)
    (0xAC00, 0xD7AF),   // Hangul Syllables (Korean)
    (0x20000, 0x2A6DF), // CJK Extension B (historical/rare)
    (0xFF01, 0xFF60),   // Fullwidth ASCII and punctuation
    (0xFFE0, 0xFFE6),   // Fullwidth variants (e.g., ￥, ￠, ％)
];

#[allow(dead_code)]
/// ⚠️ Deprecated: `EMOJI_RANGES` is a static heuristic and no longer recommended.
///
/// This table was originally used to roughly detect emoji via Unicode ranges, such as:
/// - (0x1F600–0x1F64F) for faces
/// - (0x1F680–0x1F6FF) for transport icons
///
/// ### Limitations:
/// - ❌ Does not support composite emoji (e.g., `👩‍💻`, `🧑🏿‍🦲`)
/// - ❌ Cannot match ZWJ sequences, skin tone modifiers, or regional flags
/// - ❌ Does not auto-update with new Unicode emoji releases
///
/// ### Recommended Replacement:
/// ✅ Use [`rules::emoji::is_emoji`] instead. It supports all fully-qualified emoji,
/// including ZWJ and variation sequences, with proper Unicode compliance.
///
/// ### Compatibility Use:
/// - ✅ Can be used for fast pre-filtering or fallback indexing
/// - ❌ Should **not** be used for terminal display width calculations
pub const EMOJI_RANGES: &[(u32, u32)] = &[
    (0x1F600, 0x1F64F), // Emoticons (faces)
    (0x1F300, 0x1F5FF), // Misc symbols (weather, food, etc.)
    (0x1F680, 0x1F6FF), // Transport and map symbols
    (0x2600, 0x26FF),   // Misc symbols (sun, umbrella, zodiac)
    (0x2700, 0x27BF),   // Dingbats (arrows, hands)
    (0x1F900, 0x1F9FF), // Supplemental symbols and pictographs
];
