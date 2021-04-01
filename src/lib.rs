#![no_std]

//!
//! Emoji constants for your rusty strings. This crate is inspired by the Go library
//! [emoji](https://github.com/enescakir/emoji) written by
//! [@enescakir](https://github.com/enescakir).
//!
//! _Notice that this file uses the actual Unicode emojis to given visual example of the result.
//! However, depending on the font and support on your device, not all emojis might be represented
//! correctly, especially the newer ones._
//!
//!
//! ## 📦 Cargo.toml
//!
//! ```toml
//! [dependencies]
//! emojic = "0.3"
//! ```
//!
//! ## 🔧 Example
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
//!     "Multiple skin tones: default: {}, same: {} different: {}",
//!     PERSON_HOLDING_HANDS,
//!     PERSON_HOLDING_HANDS.tone(Tone::Medium),
//!     PERSON_HOLDING_HANDS.tone((Tone::Light, Tone::Dark))
//! );
//! println!(
//!     "Different sexes: default: {} male: {}, female: {}",
//!     GENIE,
//!     GENIE.gender(Gender::Male),
//!     GENIE.gender(Gender::Female),
//! );
//! println!(
//!     "Mixing attributes: men & light: {} and women & drak: {}",
//!     PERSON_TIPPING_HAND.gender(Gender::Male).tone(Tone::Light),
//!     PERSON_TIPPING_HAND.gender(Gender::Female).tone(Tone::Dark),
//! );
//! ```
//!
//! ## 🖨️ Output
//!
//! ```text
//! Hello 👋
//! I'm 👨‍💻 from 🇹🇷
//! Different skin tones default 👍 light 👌🏻 dark 🤙🏿
//! Multiple skin tones: default: 🧑‍🤝‍🧑, same: 🧑🏽‍🤝‍🧑🏽 different: 🧑🏻‍🤝‍🧑🏿
//! Different sexes: default: 🧞 male: 🧞‍♂️, female: 🧞‍♀️
//! Mixing attributes: men & light: 💁🏻‍♂️ and women & drak: 💁🏿‍♀️
//! ```
//!
//! This crate contains emojis constants based on the
//! [Full Emoji List v13.1](https://unicode.org/Public/emoji/13.1/emoji-test.txt).
//! Including its categorization:
//!
//! ```rust
//! assert_eq!(
//!     emojic::grouped::people_and_body::hands::OPEN_HANDS, //🤲
//!     emojic::flat::OPEN_HANDS, //🤲
//! );
//! ```
//!
//! As well as iterators to list all the emojis in each group and subgroup:
//!
//! ```rust
//! # let text =
//! // Iterates all hand emoji: 👏, 🙏, 🤝, 👐, 🤲, 🙌
//! emojic::grouped::people_and_body::hands::base_emojis()
//! #    .map(|e| e.to_string())
//! #    .collect::<String>();
//! # assert_eq!("👏🙏🤝👐🤲🙌", text);
//! ```
//!
//! Additionally, it has additional emoji aliases from
//! [github/gemoji](https://github.com/github/gemoji).
//!
//! ```rust
//! # use emojic::parse_alias;
//! # assert_eq!(Some("👍"),
//! parse_alias(":+1:") // 👍
//! # .map(|e| e.grapheme));
//! # assert_eq!(Some("💯"),
//! parse_alias(":100:") // 💯
//! # .map(|e| e.grapheme));
//! # assert_eq!(Some("👩‍🚀"),
//! parse_alias(":woman_astronaut:") // 👩‍🚀
//! # .map(|e| e.grapheme));
//! ```
//!
//! Finally, it has functions to generate (arbitrary) country and regional flags.
//!
//! ```rust
//! # use emojic::regional_flag;
//! # use emojic::country_flag;
//! // 🏴󠁧󠁢󠁥󠁮󠁧󠁿 ∩ 🏴󠁧󠁢󠁳󠁣󠁴󠁿 ⊂ 🇬🇧 ⊄ 🇪🇺
//! println!("{} ∩ {} ⊂ {} ⊄ {}",
//!     regional_flag("GB-ENG"),
//!     regional_flag("GB-SCT"),
//!     country_flag("GB"),
//!     country_flag("EU"),
//! )
//! ```
//!
//! ## 🔭 Examples
//!
//! For more examples have a look at the
//! [examples](https://github.com/orhanbalci/emojic/tree/master/examples) folder.
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
pub mod flat; // Generated module

#[rustfmt::skip]
pub mod grouped; // Generated module

pub mod emojis;
pub use emojis::Gender;
pub use emojis::Hair;
pub use emojis::Pair;
pub use emojis::Tone;

use emojis::Emoji;

/// Parses the given Emoji name into a unicode Emoji.
///
/// This function accepts strings of the form `:name:` and looks up an emojis for it.
/// The list of valid names is taken from: [github/gemoji](https://github.com/github/gemoji)
/// And additonally all the constant names (as listed in [`crate::flat`]) are also valid aliases
/// when spelled in lowercase.
///
/// # Examples
///
/// ```
/// use emojic::parse_alias;
///
/// // gemoji style
/// assert_eq!(
///     Some(&*emojic::flat::THUMBS_UP),
///     parse_alias(":+1:") //👍
/// );
///
/// // constant name style
/// assert_eq!(
///     Some(&emojic::flat::ALIEN_MONSTER),
///     parse_alias(":alien_monster:") //👾
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
/// use emojic::country_flag;
///
/// assert_eq!(
///     country_flag("EU"), // 🇪🇺
///     emojic::flat::FLAG_EUROPEAN_UNION.to_string()
/// );
/// println!("{}",
///     country_flag("ZZ"), // 🇿🇿 (an invalid flag)
/// );
/// ```
#[cfg(feature = "alloc")]
pub fn country_flag(country_code: &str) -> String {
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


// TODO: Remove `contry_flag` (without U) before releasing v0.4.0!

// That's embarrassing: Originally `country_flag` had been misspelled as `contry_flag`
// and with that name it has been released as v0.3.0!
// Therefore, this misspelled function is kept here to keep it compatible, however it will just
// redirect to the now correctly named function.

/// Generate an ad-hoc country flag (use [`country_flag`] instead).
#[cfg(feature = "alloc")]
#[doc(hidden)] // we don't really need this in the docs.
#[deprecated = "Just use country_flag instead (with U)"]
pub fn contry_flag(country_code: &str) -> String {
    country_flag(country_code)
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
/// assert_eq!(
///     regional_flag("GB-ENG"), // 🏴󠁧󠁢󠁥󠁮󠁧󠁿 (England region of United Kingdom (GB))
///     emojic::flat::FLAG_ENGLAND.to_string()
/// );
/// println!("{}",
///     regional_flag("ZZ-ABC") // 🏴󠁺󠁺󠁡󠁢󠁣󠁿 (an invalid flag)
/// );
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
