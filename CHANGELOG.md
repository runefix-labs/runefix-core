# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.7] - 2025-05-27

### Changed
- Changed `UNICODE_VERSION` from `&str` to `(u8, u8, u8)` format for more structured semver logic.
- Improved Python script `integrate_char_table.py` to reflect tuple output and auto-update safely.
- Renamed test files for consistency:
  - `grapheme_test.rs` → `grapheme.rs`
  - `width_test.rs` → `width.rs`
  - `with_policy_test.rs` → `with_policy.rs`


## [0.1.6] - 2025-05-27

### Added
- Implemented `Display`, `Debug`, `AsRef<str>`, `Deref<Target=str>`, `PartialEq<str>`, and `PartialEq<&str>` for `AppliedPolicy`.
- Added `tests/with_policy_test.rs` with full coverage for `WithPolicy::apply` and related methods.
- Introduced top-level `Makefile` for local development, supporting:
  - `make test`, `make lint`, `make fmt`, `make check` (full CI parity).
- CI now includes:
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo fmt --check` for formatting enforcement.

### Changed
- Restructured `with_policy.rs` internal API for better clarity and trait coherence.
- Improved doc comments and doctest examples to reflect real-world usage patterns.
- Updated README:
  - Added `.apply(...)` chaining note on `&str` borrow behavior.
  - Clarified `WithPolicy` idioms with intermediate bindings when necessary.

### Fixed
- Removed duplicated `#![cfg(feature = "policy")]` declarations from submodules.
- Resolved Clippy lifetime warnings (`needless_lifetimes`) via `'_` elision.
- Fixed formatting violations in `examples/text_align.rs`.
- Ensured `make lint` and CI pass with zero warnings or diff.


## [0.1.5] - 2025-05-26

### Added
- Added `with_policy_test.rs` for testing WithPolicy chaining API. 
- Added CI coverage for `--features policy` and doctests.

### Changed
- Updated CI workflow: now includes both regular tests and `--features policy` + doctest checks. 
- Refactored `AppliedPolicy` visibility and doctest to ensure safe lifetime usage.

### Fixed
- Fixed doctest failure caused by temporary value borrow in `with_policy` example. 
- Silenced warnings related to unused imports and dead code in feature-gated modules.


## [0.1.4] - 2025-05-26

### Fixed
- Fixed missing lifetime annotations in `grapheme_widths_with_policy` and `truncate_by_width_with_policy`.
- Previous version `0.1.3` failed to compile under `--features policy`.


## [0.1.3] - 2025-05-26

### Added
- Introduced `with_policy.rs`: enables ergonomic chained usage like `.apply("text").display_width()` for applying width policies
- Added `policy_ext.rs`: a set of policy-aware API variants (e.g. `display_width_with_policy`, `split_by_width_with_policy`, etc.)

### Changed
- Refactored `grapheme.rs` into a directory module (`mod.rs`, `basic.rs`, `policy_ext.rs`) for better modularity and future extensibility
- No changes to core logic; only internal structure and annotations reorganized
- Improved documentation coverage with complete Rustdoc examples and semantic comments on all public APIs


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
