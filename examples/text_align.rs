/// Naive version (does not account for Unicode display widths)
///
/// Uses `chars().count()` to center content,
/// which fails for emoji and CJK characters in terminal environments.
pub fn print_naive_centered_box(content: &str) {
    let total_width = 120;
    let content_width = total_width - 2; // borders

    let naive_pad = (content_width - content.chars().count()) / 2;

    println!();
    println!("┏{}┓", "━".repeat(content_width));
    println!("┃{:width$}┃", "", width = content_width);
    println!("┃{:pad$}{}{:pad$}┃", "", content, "", pad = naive_pad);
    println!("┃{:width$}┃", "", width = content_width);
    println!("┗{}┛", "━".repeat(content_width));
}

/// Fixed version using `runefix_core` for proper Unicode width support.
///
/// Uses `.width()` to handle fullwidth CJK and emoji correctly.
#[rustfmt::skip]
pub fn print_fixed_centered_box(content: &str) {
    use runefix_core::RuneDisplayWidth;

    let total_width = 120;
    let content_width = total_width - 2;

    let width = content.width();
    let left_pad = (content_width - width) / 2;
    let right_pad = content_width - width - left_pad;

    println!();
    println!("┏{}┓", "━".repeat(content_width));
    println!("┃{:width$}┃", "", width = content_width);
    println!("┃{:left$}{}{:right$}┃", "", content, "", left = left_pad, right = right_pad);
    println!("┃{:width$}┃", "", width = content_width);
    println!("┗{}┛", "━".repeat(content_width));
}

#[rustfmt::skip]
fn main() {
    // ASCII only – looks the same
    print_naive_centered_box("[NAIVE] Lorem ipsum dolor sit amet consectetur adipisicing elit.");
    print_fixed_centered_box("[FIXED] Lorem ipsum dolor sit amet consectetur adipisicing elit.");

    // Emoji – misaligned vs fixed
    print_naive_centered_box("[NAIVE] National Language: 🇨🇳 🇯🇵 🇰🇷 🇫🇷 🇩🇪 🇪🇸 🇷🇺 🇬🇧 🇸🇬 🇦🇺 🇨🇦 🇺🇸");
    print_fixed_centered_box("[FIXED] National Language: 🇨🇳 🇯🇵 🇰🇷 🇫🇷 🇩🇪 🇪🇸 🇷🇺 🇬🇧 🇸🇬 🇦🇺 🇨🇦 🇺🇸");

    // CJK text – broken vs aligned
    print_naive_centered_box("[NAIVE]《诗》「字」方正于九州，「かな」散らす桜の韻，「한글」물결처럼 퍼지네～");
    print_fixed_centered_box("[FIXED]《诗》「字」方正于九州，「かな」散らす桜の韻，「한글」물결처럼 퍼지네～");
}
