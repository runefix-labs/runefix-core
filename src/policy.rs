/// Defines per-category width behavior for grapheme display.
///
/// This struct allows customizing how wide each category of character
/// (emoji, CJK, fullwidth symbols) is treated at runtime.
///
/// Requires enabling the `policy` feature.
#[cfg(feature = "policy")]
#[derive(Debug, Clone)]
pub struct WidthPolicy {
    /// Width for emoji graphemes (e.g., 😄, 🧑‍🤝‍🧑)
    pub emoji: usize,
    /// Width for CJK ideographs (e.g., 漢字), kana, and hangul
    pub cjk: usize,
    /// Width for fullwidth symbol variants and East Asian punctuations (e.g., Ａ, 、)
    pub variant: usize,
    /// Fallback width for unknown or uncategorized graphemes
    pub fallback: usize,
}

#[cfg(feature = "policy")]
impl WidthPolicy {
    /// Standard terminal policy (emoji = 2, CJK = 2, variant = 2, fallback = 1).
    ///
    /// Recommended for monospaced environments like terminals and TUI apps.
    pub fn terminal() -> Self {
        Self {
            emoji: 2,
            cjk: 2,
            variant: 2,
            fallback: 1,
        }
    }

    /// Markdown-friendly policy (emoji = 1, CJK = 2).
    ///
    /// Optimized for Markdown tables and web text rendering where emoji occupy 1 cell.
    pub fn markdown() -> Self {
        Self {
            emoji: 1,
            cjk: 2,
            variant: 2,
            fallback: 1,
        }
    }

    /// Compact layout policy (everything = 1).
    ///
    /// Useful for logs, command-line tables, or space-constrained TUI components.
    pub fn compact() -> Self {
        Self {
            emoji: 1,
            cjk: 1,
            variant: 1,
            fallback: 1,
        }
    }

    /// (Optional extension) Override width for a specific character.
    ///
    /// Currently, a placeholder for future per-character adjustments.
    pub fn override_char(self, _ch: char, _w: usize) -> Self {
        // optional: implement override logic later
        self
    }
}
