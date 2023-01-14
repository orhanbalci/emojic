# emojic 😀 🙂 😇

[![Crates.io](https://img.shields.io/crates/v/emojic.svg)](https://crates.io/crates/emojic)
[![Documentation](https://docs.rs/emojic/badge.svg)](https://docs.rs/emojic)
[![License](https://img.shields.io/github/license/orhanbalci/emojic.svg)](https://github.com/orhanbalci/emojic/blob/master/LICENSE)

<!-- cargo-sync-readme start -->


Emoji constants for your rusty strings. This crate is inspired by the Go library
[emoji](https://github.com/enescakir/emoji) written by
[@enescakir](https://github.com/enescakir).

_Notice that this file uses the actual Unicode emojis to given visual example of the result.
However, depending on the font and support on your device, not all emojis might be represented
correctly, especially the newer ones._


## 📦 Cargo.toml

```toml
[dependencies]
emojic = "0.4"
```

## 🔧 Example

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
    "Multiple skin tones: default: {}, same: {} different: {}",
    PERSON_HOLDING_HANDS,
    PERSON_HOLDING_HANDS.tone(Tone::Medium),
    PERSON_HOLDING_HANDS.tone((Tone::Light, Tone::Dark))
);
println!(
    "Different sexes: default: {} male: {}, female: {}",
    GENIE,
    GENIE.gender(Gender::Male),
    GENIE.gender(Gender::Female),
);
println!(
    "Mixing attributes: men & light: {} and women & dark: {}",
    PERSON_TIPPING_HAND.gender(Gender::Male).tone(Tone::Light),
    PERSON_TIPPING_HAND.gender(Gender::Female).tone(Tone::Dark),
);
```


## 🖨️ Output

```text
Hello 👋
I'm 👨‍💻 from 🇹🇷
Different skin tones default 👍 light 👌🏻 dark 🤙🏿
Multiple skin tones: default: 🧑‍🤝‍🧑, same: 🧑🏽‍🤝‍🧑🏽 different: 🧑🏻‍🤝‍🧑🏿
Different sexes: default: 🧞 male: 🧞‍♂️, female: 🧞‍♀️
Mixing attributes: men & light: 💁🏻‍♂️ and women & dark: 💁🏿‍♀️
```

This crate contains emojis constants based on the
[Full Emoji List v13.1](https://unicode.org/Public/emoji/13.1/emoji-test.txt).
Including its categorization:

```rust
assert_eq!(
    emojic::grouped::people_and_body::hands::OPEN_HANDS, //🤲
    emojic::flat::OPEN_HANDS, //🤲
);
```

As well as iterators to list all the emojis in each group and subgroup:

```rust
// Iterates all hand emoji: 👏, 🙏, 🤝, 👐, 🤲, 🙌
emojic::grouped::people_and_body::hands::base_emojis()
```

Additional, it has functions to generate (arbitrary) country and regional flags.

```rust
// 🏴󠁧󠁢󠁥󠁮󠁧󠁿 ∩ 🏴󠁧󠁢󠁳󠁣󠁴󠁿 ⊂ 🇬🇧 ⊄ 🇪🇺
println!("{} ∩ {} ⊂ {} ⊄ {}",
    regional_flag("GB-ENG"),
    regional_flag("GB-SCT"),
    country_flag("GB"),
    country_flag("EU"),
)
```

Finally, this crate has allows parsing emoji aliases from the [github/gemoji](https://github.com/github/gemoji)
list via [`parse_alias`](https://docs.rs/emojic/latest/emojic/text/fn.parse_alias.html).

```rust
parse_alias(":+1:") // 👍
parse_alias(":100:") // 💯
parse_alias(":woman_astronaut:") // 👩‍🚀
```

And it has also an utility to parse and replace these emoji aliases in text via
[`parse_text`](https://docs.rs/emojic/latest/emojic/text/fn.parse_text.html).

```rust
// a 🥪 consists of 🍞, 🥓, and some 🧀
parse_text("a :sandwich: consists of :bread:, :bacon:, and some :cheese:")
```

## 🔭 Examples

For more examples have a look at the
[examples](https://github.com/orhanbalci/emojic/tree/master/examples) folder.


## 🧩 Crate features

This crate is `no_std` by default, means it should be usable in WASM and other restricted
platforms. However, some additional functions need the `alloc` crate (normally part of `std`),
thus it is enabled by default.

- `default`: (implies `alloc`) \
  Automatically enabled if not opt-out:
  ```toml
  [dependencies.emojic]
  version = "0.4"
  default-features = false
  ```
- `alloc`: (implies `hashbrown` and `lazy_static`) \
  Requires a global allocator,
  enables some additional functions: the [`parse_text`](https://docs.rs/emojic/latest/emojic/text/fn.parse_text.html) function and the
  ad-hoc flag-functions ([`country_flag`](https://docs.rs/emojic/latest/emojic/fn.country_flag.html) & [`regional_flag`](https://docs.rs/emojic/latest/emojic/fn.regional_flag.html))
  (the flag constants are unaffected).

  Notice, that `lazy_static`, by default, pulls-in `std` to use mutices for waiting.
  This is good if you do have `std` available, and bad if not. However, the alternative is
  to instruct `lazy_static` to use spinlocks instead. Yet, since crate-features are unified by
  Cargo, it would be bad for all user that have `std`, to requiring it by default.
  Instead, if you want to use this `alloc` feature, but you don't have `std`
  (e.g. in your binary crate), you can simply add `lazy_static` yourself, and make it to use
  spinlocks, which will apply globally. E.g. add to your `Cargo.toml`:
  ```toml
  [dependencies.lazy_static]
  version = "1.4"
  features = ["spin_no_std"]
  ```
  Also see: <https://github.com/rust-lang-nursery/lazy-static.rs/issues/150>




<!-- cargo-sync-readme end -->

## 📝 License

Licensed under MIT License ([LICENSE](LICENSE)).

### 🚧 Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
