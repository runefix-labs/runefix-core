# runefix-core

> 🎯 CJK文字や絵文字の幅に起因する、ターミナルやMarkdownの表示ずれを修正するRust製コアライブラリ。

[![Crates.io](https://img.shields.io/crates/v/runefix-core)](https://crates.io/crates/runefix-core)
[![Docs.rs](https://img.shields.io/docsrs/runefix-core)](https://docs.rs/runefix-core)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](./LICENSE)
[![CI](https://github.com/runefix-labs/runefix-core/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/runefix-labs/runefix-core/actions/workflows/ci.yml)

[English](./README.md) | [简体中文](./README_zh.md) | **日本語**

---

## ✨ 概要（Overview）

`runefix-core` は、Unicode ベースの表示幅エンジンであり、正確な文字幅を判定する機能を提供します。設計目的は次のような場面でのズレやレイアウト崩れを防ぐことです：

- ターミナル上のCLI整列 
- Markdown テーブルの描画 
- TUI レイアウトエンジン 
- リッチテキストエディタの表示制御

本プロジェクトは、長年にわたり解決されてこなかった「文字幅の不整合による表示ズレ」問題を正面から扱います。対象は：

- CJK文字（中国語、日本語、韓国語）、仮名、ハングル音節 
- 絵文字（基本絵文字、合成絵文字、国旗など） 
- 全角記号や多言語句読点 
- グラフェムクラスタ（結合文字や記号の組み合わせ）


## ✅ 主な特徴（Key Features）

- ✅ emoji、CJK漢字、カナ、ハングル、全角記号の幅を正確に測定 
- ✅ Unicode 標準 [UAX #29] に準拠した字素分割 [`graphemes()`] を提供 
- ✅ TUI/端末レイアウト向けの独自セグメント [`atoms()`] をサポート 
- ✅ 幅に応じたトリミング・改行・分割・幅計測などの機能を提供 
- ✅ 実行時のレイアウト戦略（terminal / markdown / compact）を切替可能：[`WidthPolicy`]
- ✅ `char` や `str` に対する trait 拡張：`.rune_width()`、`.display_width()` など 
- ✅ 依存ゼロ・軽量構成・任意の CLI / TUI / レンダラーに組み込み可能 
- ✅ 幅テーブルはすべて [char-table](https://github.com/runefix-labs/char-table) から自動生成・再現性あり


## 🧬 アトム分割（Atom Segmentation）

> ✨ Unicodeのグラフェム分割に代わる、runefix 独自の実装

標準の Unicode グラフェム（`graphemes()`）とは異なり、`atoms()` は文字列を **レイアウトに影響を与える最小単位** に分割します。これはターミナルやTUI環境での **表示幅に基づいた分割** です。

このアプローチは表示最適化に特化しています：

- ZWJ（ゼロ幅接合子）やバリエーションセレクタなどのゼロ幅文字を基底文字とまとめて処理 
- 絵文字の ZWJ シーケンスや CJK 文字をアトム単位で維持 
- 言語学的なルールは無視し、あくまで **視覚上の幅** にフォーカス

```rust
use runefix_core::atoms;

let parts = atoms("👩‍❤️‍💋‍👨");
assert_eq!(parts, vec!["👩", "\u{200d}", "❤", "\u{fe0f}", "\u{200d}", "💋", "\u{200d}", "👨"]);
```

この関数は次のような用途に便利です：

- 表示幅やレイアウト挙動のデバッグ 
- TUI / CLI レンダリングでの細かな制御 
- グラフェムでは表現できない「実幅」ベースの分割

🧠 補足：Unicode標準のグラフェム分割が必要な場合は、[`graphemes()`] をご利用ください。


## 🧩 実行時レイアウトポリシー

### 🔧 有効化（オプション）

> 🧪 `--features policy` が必要です

`runefix-core` はデフォルトで **ターミナルポリシー**（emoji と CJK を幅 2 として扱う）を使用します。\
Markdown やログビューアなど、他の表示環境に合わせたい場合は実行時ポリシーを有効にできます：

```toml
# Cargo.toml
runefix-core = { version = "0.1", features = ["policy"] }
```

Rustコードでは以下のように使えます：

```rust
use runefix_core::{WidthPolicy, WithPolicy};

let policy = WidthPolicy::markdown();
let width = WithPolicy::new(&policy).apply("😂").display_width();

assert_eq!(width, 1); // Markdown環境では絵文字の幅は1になります
```

ℹ️ 補足：`.truncate_by_width(...)` など一部の高機能メソッドでは、Rust の `&str` 借用ルールにより、一時変数の導入が必要になる場合があります。使用例については [`with_policy.rs`](./src/with_policy.rs) をご覧ください。

### 🧠 組み込みポリシー一覧

| ポリシー名     | Emoji | CJK | Variant | 想定用途                     |
| ------------ | ----- | --- | ------- |-----------------------------|
| `terminal()` | 2     | 2   | 2       | デフォルト。ターミナルUI向け    |
| `markdown()` | 1     | 2   | 2       | GitHubやTypora等のMarkdown用 |
| `compact()`  | 1     | 1   | 1       | ログ、ステータスバー、狭いUIなど |

ポリシーは実行時に動的に作成・カスタマイズも可能です。


## 🚀 クイック例（Quick Example）

```rust
use runefix_core::RuneDisplayWidth;

fn main() {
    println!("{}", "語".rune_width());           // 出力: 2 （漢字は全角文字として幅2）
    println!("{}", "😂".rune_width());           // 出力: 2 （絵文字は幅2）
    println!("{}", "ありがとう".display_width()); // 出力: 10（全角5文字 × 2）
}
```


## 📷 実例紹介（Real-World Examples）

Grapheme（書記素）単位の文字幅認識による整列処理や、各ターミナルでの描画差異についてのデモは、 [examples/text_align.rs](./examples/text_align.rs) をご覧ください。

以下の内容を含みます：

- `.chars().count()` を使った素朴なセンタリング（❌ 誤差あり）
- `runefix_core::RuneDisplayWidth` による正確なセンタリング（✅ 推奨）
- 複数のターミナルでの表示比較スクリーンショット

👉 詳細はこちら：[examples/README.md](./examples/README.md)


## 📦 インストール方法（Installation）

`Cargo.toml` に以下を追加してください：

```toml
# 安定版（推奨）
runefix-core = "0.1"
```

最新版については [crates.io](https://crates.io/crates/runefix-core) をご確認ください。


## 📚 想定ユースケース（Use Cases）

- 🖥️ CLI アプリケーションにおける罫線や文字の整列処理 
- 🧾 Markdown の表組み描画処理（正確な幅のセル制御） 
- 📊 TUI（Terminal UI）フレームワークでのレイアウト計算 
- 📄 エディタ／端末エミュレータにおける折り返し処理や幅制御 
- 🧩 表示幅検証ツールやプレイグラウンド型の可視化アプリ


## 📁 データセットソース（Dataset Sources）

本クレートは、[char-table](https://github.com/runefix-labs/char-table) が提供する構造化・バージョン管理済みのデータに依存しています：

- `emoji_base.json` / `emoji_zwj.json`
- `cjk_unified.json` / `japanese_kana.json` / `korean_syllables.json`
- `fullwidth_variants.json` / `fullwidth_punctuations.json`

すべてのデータは再現可能で、Unicode の最新仕様と同期されています。


## 🛠️ 開発状況（Project Status）

| モジュール             | 状況    |
|----------------------|---------|
| CJK 幅判定            | ✅ 完了 |
| Emoji 幅サポート       | ✅ 完了 |
| Grapheme cluster 対応 | ✅ 完了 |


## 📌 変更履歴（CHANGELOG）

バージョン履歴は [CHANGELOG.md](./CHANGELOG.md) をご覧ください。


## 🔖 ライセンス（License）

MIT OR Apache-2.0  
© 2025 Pokeya Z. Chen / Runefix Labs


## 📣 プロジェクトについて（Affiliation）

本プロジェクトは [runefix-labs](https://github.com/runefix-labs) によってメンテナンスされています。\
Unicode 幅問題を多言語・マルチプラットフォームで解決する runefix スイートの コアエンジン に位置づけられます。


## 🌐 お問い合わせ（Contact）

GitHub: [github.com/runefix-labs](https://github.com/runefix-labs) \
作者: [pokeyaro](https://github.com/pokeyaro) \
メール: [pokeya.mystic@gmail.com](mailto:pokeya.mystic@gmail.com)
