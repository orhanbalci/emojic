# emojic 😀 🙂 😇

[![Crates.io](https://img.shields.io/crates/v/emojic.svg)](https://crates.io/crates/emojic)
[![Documentation](https://docs.rs/emojic/badge.svg)](https://docs.rs/emojic)
[![License](https://img.shields.io/github/license/orhanbalci/emojic.svg)](https://github.com/orhanbalci/emojic/blob/master/LICENSE)

<!-- cargo-sync-readme start -->


Emoji constants for your rusty strings. This crate is Rust port of Go library [emoji](https://github.com/enescakir/emoji) written by [@enescakir](https://github.com/enescakir)


# 📦 Cargo.toml

```toml
[dependencies]
emojic = "0.3"
```

# 🔧 Example

```rust
use emojic::Gender;
use emojic::Pair;
use emojic::Tone;
use emojic::flat::*;

println!("Hello {}", WAVING_HAND);
println!(
    "I'm {} from {}",
    TECHNOLOGIST.gender(Gender::Male),
    FLAG_TURKEY
);
println!(
    "Different skin tones default {} light {} dark {}",
    THUMBS_UP,
    OK_HAND.tone(Tone::Light),
    CALL_ME_HAND.tone(Tone::Dark)
);
println!(
    "Emojis with multiple skin tones.\nDefault: {}, both medium: {} light and dark: {}",
    PERSON_HOLDING_HANDS,
    PERSON_HOLDING_HANDS.tone(Tone::Medium),
    PERSON_HOLDING_HANDS.tone((Tone::Light, Tone::Dark))
);
println!(
    "Emojis with different sexes.\nMen: {}, women: {}, both: {}",
    PERSON_HOLDING_HANDS.gender(Pair::Males),
    PERSON_HOLDING_HANDS.gender(Pair::Females),
    PERSON_HOLDING_HANDS.gender(Pair::Mixed),
);
println!(
    "Emojis with sexes and skin tone.\nLight Men: {} and dark women: {}",
    PERSON_HOLDING_HANDS.gender(Pair::Males).tone(Tone::Light),
    PERSON_HOLDING_HANDS.gender(Pair::Females).tone(Tone::Dark),
);
```

# 🖨️ Output

```text
Hello 👋
I'm 👨‍💻 from 🇹🇷
Different skin tones default 👍 light 👌🏻 dark 🤙🏿
Emojis with multiple skin tones.
Both medium: 🧑🏽‍🤝‍🧑🏽 light and dark: 🧑🏻‍🤝‍🧑🏿
```

This package contains emojis constants based on [Full Emoji List v13.1](https://unicode.org/Public/emoji/13.1/emoji-test.txt).

```rust
CALL_ME_HAND // 🤙
CALL_ME_HAND.tone(Tone::Dark) // 🤙🏿
```

Also, it has additional emoji aliases from [github/gemoji](https://github.com/github/gemoji).

```rust
parse_alias(":+1:") // 👍
parse_alias(":100:") // 💯
```




<!-- cargo-sync-readme end -->

# 📝 License

Licensed under MIT License ([LICENSE](LICENSE)).

## 👀🚧🔨👥🤝👆👌 Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
