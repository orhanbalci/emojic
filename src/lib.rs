#![no_std]

//!
//! Emoji constants for your rusty strings. This crate is Rust port of Go library [emoji](https://github.com/enescakir/emoji) written by [@enescakir](https://github.com/enescakir)
//!
//!
//! # 📦 Cargo.toml
//!
//! ```toml
//! [dependencies]
//! emojic = "0.3"
//! ```
//!
//! # 🔧 Example
//!
//! ```rust
//! use emojic::Gender;
//! use emojic::Pair;
//! use emojic::Tone;
//! use emojic::flat::*;
//!
//! println!("Hello {}", WAVING_HAND);
//! println!(
//!     "I'm {} from {}",
//!     TECHNOLOGIST.gender(Gender::Male),
//!     FLAG_TURKEY
//! );
//! println!(
//!     "Different skin tones default {} light {} dark {}",
//!     THUMBS_UP,
//!     OK_HAND.tone(Tone::Light),
//!     CALL_ME_HAND.tone(Tone::Dark)
//! );
//! println!(
//!     "Emojis with multiple skin tones.\nDefault: {}, both medium: {} light and dark: {}",
//!     PERSON_HOLDING_HANDS,
//!     PERSON_HOLDING_HANDS.tone(Tone::Medium),
//!     PERSON_HOLDING_HANDS.tone((Tone::Light, Tone::Dark))
//! );
//! println!(
//!     "Emojis with different sexes.\nMen: {}, women: {}, both: {}",
//!     PERSON_HOLDING_HANDS.gender(Pair::Males),
//!     PERSON_HOLDING_HANDS.gender(Pair::Females),
//!     PERSON_HOLDING_HANDS.gender(Pair::Mixed),
//! );
//! println!(
//!     "Emojis with sexes and skin tone.\nLight Men: {} and dark women: {}",
//!     PERSON_HOLDING_HANDS.gender(Pair::Males).tone(Tone::Light),
//!     PERSON_HOLDING_HANDS.gender(Pair::Females).tone(Tone::Dark),
//! );
//! ```
//!
//! # 🖨️ Output
//!
//! ```text
//! Hello 👋
//! I'm 👨‍💻 from 🇹🇷
//! Different skin tones default 👍 light 👌🏻 dark 🤙🏿
//! Emojis with multiple skin tones.
//! Both medium: 🧑🏽‍🤝‍🧑🏽 light and dark: 🧑🏻‍🤝‍🧑🏿
//! ```
//!
//! This package contains emojis constants based on [Full Emoji List v13.1](https://unicode.org/Public/emoji/13.1/emoji-test.txt).
//!
//! ```rust
//! # use emojic::flat::CALL_ME_HAND;
//! # use emojic::Tone;
//! # assert_eq!("🤙",
//! CALL_ME_HAND // 🤙
//! # .grapheme);
//! # assert_eq!("🤙🏿",
//! CALL_ME_HAND.tone(Tone::Dark) // 🤙🏿
//! # .grapheme);
//! ```
//!
//! Also, it has additional emoji aliases from [github/gemoji](https://github.com/github/gemoji).
//!
//! ```rust
//! # use emojic::parse_alias;
//! # assert_eq!(Some("👍"),
//! parse_alias(":+1:") // 👍
//! # .map(|e| e.grapheme));
//! # assert_eq!(Some("💯"),
//! parse_alias(":100:") // 💯
//! # .map(|e| e.grapheme));
//! ```
//!
//!
//!

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[rustfmt::skip]
#[cfg(feature = "alloc")]
mod alias; // Generated module

#[rustfmt::skip]
mod constants; // Generated module

pub mod emojis;
pub use emojis::Gender;
pub use emojis::Hair;
pub use emojis::Pair;
pub use emojis::Tone;

use emojis::Emoji;

#[doc(inline)]
pub use constants::flat;
#[doc(inline)]
pub use constants::grouped;

/// Parses the given Emoji name into a unicode Emoji.
///
/// ```
/// use emojic::parse_alias;
///
/// assert_eq!(
///     Some(&emojic::flat::ALIEN_MONSTER),
///     parse_alias(":alien_monster:") //👾
/// );
///
/// assert_eq!(
///     Some(&*emojic::flat::THUMBS_UP),
///     parse_alias(":+1:") //👍
/// );
/// ```
///
#[cfg(feature = "alloc")]
pub fn parse_alias(inp: &str) -> Option<&'static Emoji> {
    alias::GEMOJI_MAP.get(inp).cloned()
}

/// Generate an ad-hoc country flag.
///
/// This function allows to create arbitrary country flags.
///
/// The Unicode standard defines country flags based on the two-letter country codes
/// (see ISO 3166-1 alpha-2). Notice most (if not all) fonts support only the defined
/// codes, however, this function does not test whether the given code is in deed a well defined
/// country code.
///
/// # Panics
/// If the provided string contains characters other than exactly two ASCII letters (A-Z).
///
/// # Examples
/// ```
/// use emojic::contry_flag;
///
/// assert_eq!(contry_flag("EU"), emojic::flat::FLAG_EUROPEAN_UNION.to_string()); // 🇪🇺
/// println!("{}", contry_flag("ZZ")); // 🇿🇿 (an invalid flag)
/// ```
#[cfg(feature = "alloc")]
pub fn contry_flag(country_code: &str) -> String {
    assert!(
        country_code.chars().all(|c| c.is_ascii_alphabetic()),
        "Only chars A-Z are allowed as country_code"
    );
    assert!(
        country_code.len() == 2,
        "Only exactly two chars are allowed as country_code"
    );

    country_code
        .to_ascii_uppercase()
        .chars()
        .map(|c| core::char::from_u32(c as u32 - 'A' as u32 + '\u{1F1E6}' as u32).unwrap())
        .collect()
}

/// Generate an ad-hoc regional flag.
///
/// This function allows to create arbitrary regional flags.
///
/// The Unicode standard defines regional flags based the ISO regions (see ISO 3166-2) which
/// consist of the two-letter country code (ISO 3166-1 alpha-2) combined with up to three
/// further characters to specify the region.
///
/// # Panics
/// If the provided string contains characters other than ASCII.
///
/// # Examples
/// ```
/// use emojic::regional_flag;
///
/// assert_eq!(regional_flag("GB-ENG"), emojic::flat::FLAG_ENGLAND.to_string()); // 🏴󠁧󠁢󠁥󠁮󠁧󠁿 (England region of United Kingdom (GB))
/// println!("{}", regional_flag("ZZ-ABC")); // 🏴󠁺󠁺󠁡󠁢󠁣󠁿 (an invalid flag)
/// ```
#[cfg(feature = "alloc")]
pub fn regional_flag(regional_code: &str) -> String {
    assert!(
        regional_code
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-'),
        "Only ascii chars are allowed as regional_code"
    );

    let regional_code = regional_code.to_ascii_lowercase();

    let code = regional_code
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| core::char::from_u32(c as u32 + '\u{E0000}' as u32).unwrap());

    core::iter::once('🏴') // start symbol
        .chain(code) // code as tag sequence
        .chain(core::iter::once('\u{E007F}')) // end sequence tag
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn parse_test() {
        assert_eq!(
            Some(&crate::flat::FLAG_ECUADOR),
            parse_alias(":flag_ecuador:")
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn parse_fail() {
        assert_eq!(None, parse_alias(":hebele:"));
    }
}
