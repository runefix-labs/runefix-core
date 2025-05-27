# runefix-core

> 🎯 一个用于修复终端 / Markdown 中文、emoji、CJK 字符错位的 Rust 核心库。

[![Crates.io](https://img.shields.io/crates/v/runefix-core)](https://crates.io/crates/runefix-core)
[![Docs.rs](https://img.shields.io/docsrs/runefix-core)](https://docs.rs/runefix-core)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](./LICENSE)
[![CI](https://github.com/runefix-labs/runefix-core/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/runefix-labs/runefix-core/actions/workflows/ci.yml)

[English](./README.md) | **简体中文** | [日本語](./README_ja.md)

---

## ✨ 项目简介

`runefix-core` 是一个基于 Unicode 数据构建的显示宽度引擎，提供更精准的字符显示宽度计算，专为：

- 终端 CLI 对齐
- Markdown 表格渲染
- TUI 框架布局
- 富文本编辑器视图控制

设计目标是解决过去数十年来广泛存在、却始终缺乏统一修复方案的 “字符宽度错位” 问题，特别关注：

- 中日韩（CJK）字符、假名、韩文音节
- emoji 基础符号、合成体、国旗等变体
- 全角符号和多语言标点
- grapheme cluster 等宽度边界情况


## ✅ 核心特性

- ✅ 精准识别 emoji、CJK 汉字、假名、韩文、全角符号的显示宽度
- ✅ 提供符合 [UAX #29] 标准的 Unicode 字素分割：[`graphemes()`]
- ✅ 提供专为终端/TUI 优化的自定义布局单元：[`atoms()`]
- ✅ 支持按宽度截断、换行、分段、测量等排版功能 
- ✅ 支持运行时布局策略（terminal / markdown / compact）：[`WidthPolicy`]
- ✅ 提供 `char` 与 `str` 的 trait 扩展，如 `.rune_width()`、`.display_width()` 等 
- ✅ 无依赖、构建轻量、适用于任意 CLI / TUI / 渲染引擎 
- ✅ 所有宽度表均来自 [char-table](https://github.com/runefix-labs/char-table)，完全可复现、自动生成


## 🧬 Atom 分割

> ✨ 一种 runefix 特有的替代方案，用于 Unicode 字素切分

与标准的 Unicode 字素边界（如 `graphemes()`）不同，`atoms()` 会将字符串分割为 **影响布局的单元**，这些单元直接映射到终端或 TUI 中的可视宽度。

这种分割是基于 **显示宽度驱动** 的，专为渲染而优化：

- 将零宽字符（如 ZWJ、变体选择器）与其基础字符归为一组 
- 保留 emoji ZWJ 序列、CJK 字符等为原子级的布局单位 
- 忽略语言规则，仅关注实际对布局产生影响的部分

```rust
use runefix_core::atoms;

let parts = atoms("👩‍❤️‍💋‍👨");
assert_eq!(parts, vec!["👩", "\u{200d}", "❤", "\u{fe0f}", "\u{200d}", "💋", "\u{200d}", "👨"]);
```

该函数适用于： 

- 可视宽度分析与调试 
- CLI / TUI 布局单元的手动控制 
- 较底层的文本渲染场景

注意 🧠：若你需要 Unicode 标准的字素分割，请使用 [`graphemes()`]。


## 🧩 运行时布局策略

### 🔧 启用（可选）

> 🧪 需要启用 `--features policy`

默认情况下，`runefix-core` 使用 **终端策略**，即 emoji 与 CJK 字符都占据 2 列。 \
你可以启用运行时策略，以适配不同的渲染环境：

```toml
# Cargo.toml
runefix-core = { version = "0.1", features = ["policy"] }
```

然后这样使用：

```rust
use runefix_core::{WidthPolicy, WithPolicy};

let policy = WidthPolicy::markdown();
let width = WithPolicy::new(&policy).apply("😂").display_width();

assert_eq!(width, 1); // 在 Markdown 中 emoji 宽度为 1
```

ℹ️ 注意：某些高级用法（例如 `.truncate_by_width(...)`）由于 `&str` 的借用规则，可能需要使用中间变量。参见 [`with_policy.rs`](./src/with_policy.rs) 获取惯用示例。

### 🧠 内置策略一览

| 策略名        | Emoji | CJK | Variant | 推荐用途                         |
| ------------ | ----- | --- | ------- |---------------------------------|
| `terminal()` | 2     | 2   | 2       | 默认策略，适合终端类应用            |
| `markdown()` | 1     | 2   | 2       | 适配 GitHub / Typora 等 Markdown |
| `compact()`  | 1     | 1   | 1       | 日志、状态栏、紧凑输出等            |

你也可以在运行时动态构建或修改策略，以适配自定义渲染器。


## 🚀 使用示例

```rust
use runefix_core::RuneDisplayWidth;

fn main() {
    println!("{}", "你".rune_width());         // 输出: 2
    println!("{}", "😂".rune_width());         // 输出: 2
    println!("{}", "你a1👇".display_width());  // 输出: 6
}
```


## 📷 实战示例

想要了解如何在终端中实现基于 grapheme 的居中对齐，以及不同终端渲染行为的对比，请查看 [examples/text_align.rs](./examples/text_align.rs).

示例包括：

- 使用 `.chars().count()` 进行居中对齐（❌ 存在偏差）
- 使用 `runefix_core::RuneDisplayWidth` 精准对齐（✅ 推荐）
- 各类终端的截图对比效果

👉 阅读更多内容：[examples/README.md](./examples/README.md)


## 📦 安装方式

将以下内容添加至你的 Cargo.toml：

```toml
# 稳定版本（推荐）
runefix-core = "0.1"
```

请查看 [crates.io](https://crates.io/crates/runefix-core) 获取最新版本号。


## 📚 应用场景

- 🖥️ CLI 应用显示对齐（避免边框错位） 
- 🧾 Markdown 表格渲染器宽度修正 
- 📊 TUI 框架内文本布局模块 
- 📄 文本编辑器、终端模拟器等底层排版引擎 
- 🧩 自定义 Terminal 宽度测试与图形化展示


## 📁 数据来源

本项目依赖于 [char-table](https://github.com/runefix-labs/char-table) 提供的数据构建而成，涵盖：

- `emoji_base.json` / `emoji_zwj.json`
- `cjk_unified.json` / `japanese_kana.json` / `korean_syllables.json` 
- `fullwidth_variants.json` / `fullwidth_punctuations.json`

所有数据均可 reproducible，定期更新，版本同步自 Unicode 官网。


## 🛠️ 项目状态

| 功能模块           | 状态        |
|-------------------|------------|
| CJK 宽度识别       | ✅ 完成     |
| emoji 宽度识别     | ✅ 完成     |
| grapheme cluster  | ✅ 完成     |


## 📌 更新日志

版本历史详见 [CHANGELOG.md](./CHANGELOG.md) 文件。


## 🔖 许可证

MIT OR Apache-2.0  
© 2025 Pokeya Z. Chen / Runefix Labs


## 📣 项目归属

该项目由 [runefix-labs](https://github.com/runefix-labs) 维护，是 runefix 全栈多语言字符宽度解决方案 的核心组件之一。


## 🌐 联系我们

GitHub: [github.com/runefix-labs](https://github.com/runefix-labs) \
作者主页: [pokeyaro](https://github.com/pokeyaro) \
邮箱联系: [pokeya.mystic@gmail.com](mailto:pokeya.mystic@gmail.com)
