# runefix-core

> 🎯 A Rust core library that fixes terminal and Markdown misalignment caused by CJK and emoji widths.

[![Crates.io](https://img.shields.io/crates/v/runefix-core)](https://crates.io/crates/runefix-core)
[![Docs.rs](https://img.shields.io/docsrs/runefix-core)](https://docs.rs/runefix-core)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](./LICENSE)
[![CI](https://github.com/runefix-labs/runefix-core/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/runefix-labs/runefix-core/actions/workflows/ci.yml)

**English** | [简体中文](./README_zh.md) | [日本語](./README_ja.md)

---

## ✨ Overview

`runefix-core` is a Unicode-based display width engine that provides precise character width calculation, designed for:

- Terminal CLI alignment 
- Markdown table rendering 
- TUI layout engines 
- Rich text editor rendering

Its goal is to solve a decades-long problem: **misaligned characters** in terminal and UI environments due to inconsistent Unicode width handling.
This includes:

- CJK characters (Chinese, Japanese, Korean), Kana, and Hangul syllables 
- Emoji (base, composite, and flag sequences)
- Fullwidth punctuation and multilingual symbols 
- Grapheme cluster boundary cases

## ✅ Key Features

- ✅ Accurate width detection for emoji, CJK, Hangul, Kana, and fullwidth symbols 
- ✅ Full support for grapheme clusters (emoji composition, regional indicators)
- ✅ Zero dependencies, minimal API, compatible with any terminal or UI renderer 
- ✅ Fully reproducible datasets, powered by [char-table](https://github.com/runefix-labs/char-table)
- ✅ Language bindings planned for JS / Python / Go and more

## 🧩 Optional: Runtime Width Policies

> 🧪 `--features policy` required

By default, `runefix-core` uses the **terminal layout policy**, where both emoji and CJK characters occupy 2 columns. \
You can optionally enable runtime policies to adapt to other rendering environments:
```toml
# Cargo.toml
runefix-core = { version = "0.1", features = ["policy"] }
```
Then:
```rust
use runefix_core::{WidthPolicy, display_width_with_policy};

let w = display_width_with_policy("😂", Some(&WidthPolicy::markdown()));
assert_eq!(w, 1);  // markdown prefers emoji width = 1
```

## 🧠 Built-in Policies

| Policy       | Emoji | CJK | Variant | Use case                          |
| ------------ | ----- | --- | ------- | --------------------------------- |
| `terminal()` | 2     | 2   | 2       | Default. Best for terminal apps   |
| `markdown()` | 1     | 2   | 2       | GitHub / Typora / table renderers |
| `compact()`  | 1     | 1   | 1       | Logs, status bars, tight layouts  |

You can also override policies dynamically at runtime for your renderer.

## 🚀 Quick Example

```rust
use runefix_core::RuneDisplayWidth;

fn main() {
    println!("{}", "你".rune_width());         // Output: 2
    println!("{}", "😂".rune_width());         // Output: 2
    println!("{}", "你a1👇".display_width());  // Output: 6
}
```

## 📷 Real-World Examples

For a full demo of grapheme-aware alignment and terminal behavior, see [examples/text_align.rs](./examples/text_align.rs).

It showcases:

- Naive centering using `.chars().count()` ❌ 
- Fixed centering using `runefix_core::RuneDisplayWidth` ✅ 
- Screenshot comparisons across terminals

👉 Read more in [examples/README.md](./examples/README.md)

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
# Stable release (recommended)
runefix-core = "0.1"
```

See [crates.io](https://crates.io/crates/runefix-core) for the latest version.

## 📚 Use Cases

- 🖥️ Terminal alignment (CLI box drawing, tables)
- 🧾 Markdown renderers (accurate width handling in table cells)
- 📊 TUI layout engines (React-style terminal UIs)
- 📄 Editors and terminal emulators (accurate text wrapping)
- 🧩 Width testing tools, playgrounds, and visualization platforms

## 📁 Dataset Sources

This crate relies on structured, versioned datasets from  [char-table](https://github.com/runefix-labs/char-table), including:

- `emoji_base.json`, `emoji_zwj.json`
- `cjk_unified.json`, `japanese_kana.json`, `korean_syllables.json`
- `fullwidth_variants.json`, `fullwidth_punctuations.json`

All datasets are reproducible, regularly updated, and aligned with the latest Unicode releases.

## 🛠️ Project Status

| Module              | Status      |
| ------------------- | ----------- |
| CJK width detection | ✅ Completed |
| Emoji width support | ✅ Completed |
| Grapheme cluster    | ✅ Completed |

## 📌 CHANGELOG

See [CHANGELOG.md](./CHANGELOG.md) for version history.

## 🔖 License

MIT OR Apache-2.0  
© 2025 Pokeya Z. Chen / Runefix Labs

## 📣 Project Affiliation

This project is maintained by the [runefix-labs](https://github.com/runefix-labs) organization.
It serves as the **core engine** for the runefix suite — a multi-language, cross-platform solution for Unicode width handling.

## 🌐 Contact

GitHub: [github.com/runefix-labs](https://github.com/runefix-labs) \
Author: [pokeyaro](https://github.com/pokeyaro) \
Email: [pokeya.mystic@gmail.com](mailto:pokeya.mystic@gmail.com)
