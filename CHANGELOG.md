# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

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
