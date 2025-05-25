# runefix-core

> 🎯 一个用于修复终端 / Markdown 中文、emoji、CJK 字符错位的 Rust 核心库。

[![Crates.io](https://img.shields.io/crates/v/runefix-core)](https://crates.io/crates/runefix-core)
[![Docs.rs](https://img.shields.io/docsrs/runefix-core)](https://docs.rs/runefix-core)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](./LICENSE)
[![CI](https://github.com/runefix-labs/runefix-core/actions/workflows/ci.yml/badge.svg)](https://github.com/runefix-labs/runefix-core/actions/workflows/ci.yml)

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

- ✅ 精准识别 Emoji（含合成）、CJK、韩文、假名、全角标点等宽度
- ✅ Grapheme cluster 分析支持（支持所有合成字符和国旗）
- ✅ 零依赖，极简接口，适配任何终端 / UI 场景
- ✅ 数据源可溯源，构建结果可复现（由 [char-table](https://github.com/runefix-labs/char-table) 自动生成）
- ✅ 多语言适配：未来支持 JS / Python / Go 等语言绑定

## 🚀 使用示例

```rust
use runefix_core::RuneDisplayWidth;

fn main() {
    println!("{}", "你".rune_width());           // 输出: 2
    println!("{}", "😂".rune_width());           // 输出: 2
    println!("{}", "你a1👇".rune_width_total()); // 输出: 6
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
