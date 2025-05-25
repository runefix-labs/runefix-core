# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.2] - 2025-05-26
### Added
- Introduced optional `--features policy` runtime engine 
- Added `WidthPolicy` struct with built-in strategies:
  - `terminal()` (default logic, emoji/CJK = 2)
  - `markdown()` (emoji = 1, CJK = 2)
  - `compact()` (emoji/CJK = 1)
- Added API: `display_width_with_policy(&str, Option<&WidthPolicy>) -> usize` 
- Added `.resolve_width(&self, &str)` for runtime dispatch 
- Added placeholder `.override_char(char, usize)` for future character-specific overrides

### Changed
- Internal logic in `get_display_width()` now routes through DefaultPolicy or `WidthPolicy::terminal()` based on feature flag 
- Modularized internal fallback behavior for policy-less builds

### Notes
- This feature is **fully backward-compatible**: default behavior remains unchanged without enabling the `policy` feature
- Future versions may expand `.override_char()` for per-character runtime overrides


## [0.1.1] - 2025-05-26
### Changed
- Upgraded Unicode dependency from `15.1.0` to `16.0.0`
- Removed `version.rs`
- Exposed `UNICODE_VERSION` constant
- Fixed bug in `integrate_char_table.py` where `UNICODE_VERSION` was not updated correctly


## [0.1.0] - 2025-05-25
### Added
- Initial release of `runefix-core`
- Grapheme-aware display width engine
- Support for emoji, CJK, Kana, Hangul, fullwidth variants
- Integration script for char-table datasets
- Example: `examples/text_align.rs`
