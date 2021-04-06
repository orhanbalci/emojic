//! Text processing utilities
//!
//! This module contains some utilities to enrich text with emojis.
//! The basis of this module is the [`parse_alias`] function which takes
//! emoji names (e.g. `:crab:`) and tries to look up the appropriate emoji
//! (e.g. `🦀`).
//!
//! Based on the [`parse_alias`] function, the [`parse_text`] function takes
//! an entire text and looks for such colon-fenced emoji names, which will then
//! be translated into the Unicode equivalent, and the entire text, with the
//! emojis replaced is returned.

use core::fmt;

use crate::emojis::Emoji;

#[cfg(feature = "alloc")]
use alloc::string::String;

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
pub fn parse_alias(inp: &str) -> Option<&'static Emoji> {
    // make some basic checks
    if inp.starts_with(':') && inp.ends_with(':') && inp.is_ascii() && inp.len() > 2 {
        // go on with the middle part
        parse_pure_alias(&inp[1..(inp.len() - 1)])
    } else {
        None
    }
}

/// Parses a pice of string into an emoji (no colons)
fn parse_pure_alias(inp: &str) -> Option<&'static Emoji> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "alloc")] {
            // If we have alloc, we use the faster hash map
            crate::alias::GEMOJI_MAP.get(inp).cloned()
        } else {
            // As a fallback, we can also use a huge match statement
            crate::matching::matching(inp)
        }
    }
}

/// Replaces all gemojis (`:[a-z0-9_+-]+:`) found in `text` with their Unicode equivalent.
///
/// This function is a convenience function for [`EmojiTextParser`]:
/// ```rust
/// # use emojic::text::{EmojiTextParser,parse_text};
/// # let text = ":some: :cat:";
/// # assert_eq!(
/// EmojiTextParser::new(text).to_string()
/// # , parse_text(text)
/// # );
/// ```
///
/// Notice, this convenience function requires `alloc` unlike the
/// [`EmojiTextParser`] iterator.
///
/// # Example
///
/// ```rust
/// use emojic::text::parse_text;
/// assert_eq!(
///     &parse_text("Hello :waving_hand:, I am a :technologist:."),
///     "Hello 👋, I am a 🧑‍💻.",
/// );
/// ```
///
/// ```rust
/// use emojic::text::parse_text;
/// assert_eq!(
///     &parse_text("Neither std::iter::Iterator nor :rustaceans: are emojis"),
///     "Neither std::iter::Iterator nor :rustaceans: are emojis",
/// );
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "alloc")))]
pub fn parse_text(text: &str) -> String {
    EmojiTextParser::new(text).collect()
}

/// Finds and replaces gemojis (`:[a-z0-9_+-]+:`) in text.
///
/// This is the iterator behind [`parse_text`].
///
/// `EmojiTextParser` simply splits its input text into multiple fragments,
/// those of plain text, and those which are replacement emoji sequences.
///
/// Notice, that since this is simple iterator, it dose not depend on `alloc`,
/// unlike the convenience function [`parse_text`].
///
/// # Example
///
/// ```rust
/// use emojic::text::EmojiTextParser;
///
/// let input = "Hello :waving_hand:, I am a :technologist:.";
/// let mut parser = EmojiTextParser::new(input);
///
/// assert_eq!(Some("Hello "), parser.next());
/// assert_eq!(Some("👋"), parser.next());
/// assert_eq!(Some(", I am a "), parser.next());
/// assert_eq!(Some("🧑‍💻"), parser.next());
/// assert_eq!(Some("."), parser.next());
/// assert_eq!(None, parser.next());
/// ```
///
#[derive(Debug, Clone)]
pub struct EmojiTextParser<'a> {
    /// The original string that is parsed, used to fetch the output strings of
    /// the plain text sequences.
    original: &'a str,
    /// The next index into `original` to be processed, might be `original.len()`
    next_pos: usize,
    /// Indicates whether the next call to `next` has to process an emoji.
    emoji_fragment_start: bool,
}
impl<'a> EmojiTextParser<'a> {
    /// Creates a new parser for the given `original` text.
    pub fn new(original: &'a str) -> Self {
        Self {
            original,
            next_pos: 0,
            /// The very beginning is never a emoji
            emoji_fragment_start: false,
        }
    }

    fn is_valid_emoji_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_' || c == '+' || c == '-'
    }

    fn text_until_next_colon(&mut self, start_idx: usize, skip: usize) -> &'a str {
        if let Some(colon_idx) = self.original[(start_idx + skip)..].find(':') {
            let true_colon_idx = start_idx + skip + colon_idx;
            // Found a colon, so let's continue next time behind it
            self.emoji_fragment_start = true;
            self.next_pos = true_colon_idx + 1;

            &self.original[start_idx..true_colon_idx]
        } else {
            // There are no further fragment
            self.emoji_fragment_start = false;
            self.next_pos = self.original.len(); // the end

            &self.original[start_idx..]
        }
    }
}
impl<'a> Iterator for EmojiTextParser<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.emoji_fragment_start {
            // We must be preceded by a colon, so `next_pos` must not be zero
            debug_assert!(self.next_pos > 0);

            let start_idx = self.next_pos - 1;

            let chars = self.original[self.next_pos..].char_indices();

            // Validate all chars for an emoji alias
            for (i, c) in chars {
                if c == ':' {
                    let current_pos = self.next_pos + i;
                    // This is the second colon

                    let emoji_name = &self.original[start_idx..=current_pos];

                    if let Some(e) = crate::parse_alias(emoji_name) {
                        self.emoji_fragment_start = false;
                        self.next_pos = current_pos + 1;
                        return Some(e.grapheme);
                    } else {
                        // Here a user might have misspelled a emoji name.
                        // However, the conservative thing to do is to ignore it
                        // => meaning we output it as normal text
                        self.emoji_fragment_start = true;
                        self.next_pos = current_pos + 1;
                        return Some(&self.original[start_idx..current_pos]);
                    }
                } else if Self::is_valid_emoji_char(c) {
                    // A valid emoji char, lets continue
                } else {
                    // An invalid char, this makes this part just normal text,
                    // so lets output everything until the next colon
                    return Some(self.text_until_next_colon(start_idx, 1));
                }
            }

            // Here we hit the end of the text, but we have not found our ending
            // colon, so this is just text

            // There are no further fragment
            self.emoji_fragment_start = false;
            self.next_pos = self.original.len(); // the end

            Some(&self.original[start_idx..])
        } else if self.next_pos < self.original.len() {
            // we basically look for the next colon
            Some(self.text_until_next_colon(self.next_pos, 0))
        } else {
            // No more text left
            None
        }
    }
}
impl<'a> fmt::Display for EmojiTextParser<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let copy = self.clone();
        for frag in copy {
            write!(fmt, "{}", frag)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Tests are going to be on development systems => there will be std.
    extern crate std;
    use std::prelude::v1::*;

    use super::*;

    #[test]
    fn parser_test() {
        let input = "Hello :waving_hand:, I am a :technologist:.";
        let mut parser = EmojiTextParser::new(input);

        assert_eq!(Some("Hello "), parser.next());
        assert_eq!(Some("👋"), parser.next());
        assert_eq!(Some(", I am a "), parser.next());
        assert_eq!(Some("🧑‍💻"), parser.next());
        assert_eq!(Some("."), parser.next());
        assert_eq!(None, parser.next());
    }

    #[test]
    fn parser_misspelled() {
        let input = "Hello :wavinghand:, I am a :tchnologist:."; // sic
        let mut parser = EmojiTextParser::new(input);

        assert_eq!(Some("Hello "), parser.next());
        assert_eq!(Some(":wavinghand"), parser.next());
        assert_eq!(Some(":, I am a "), parser.next());
        assert_eq!(Some(":tchnologist"), parser.next());
        assert_eq!(Some(":."), parser.next());
        assert_eq!(None, parser.next());
    }

    #[test]
    fn parser_thumbs() {
        let input = ":thumbs_up::+1::-1::thumbs_down:";
        let output = "👍👍👎👎";

        let parser = EmojiTextParser::new(input);

        assert_eq!(output, &parser.collect::<String>());
    }

    #[test]
    fn parser_nothing() {
        let input = "";
        let mut parser = EmojiTextParser::new(input);

        assert_eq!(None, parser.next());
    }

    #[test]
    fn parser_corner_cases() {
        let input = "100: :100:100:100: :100";
        let output = "100: 💯100💯 :100";

        let parser = EmojiTextParser::new(input);

        assert_eq!(output, &parser.collect::<String>());
    }

    #[test]
    fn parser_no_emoji() {
        let input = "Hello :: I am: a technologist, :=: :).";
        let parser = EmojiTextParser::new(input);

        assert_eq!(input, &parser.collect::<String>());
    }

    #[test]
    fn parser_no_colons() {
        let input = "Hello, I am a technologist.";
        let parser = EmojiTextParser::new(input);

        assert_eq!(input, &parser.collect::<String>());
    }

    #[test]
    fn parser_single_colon() {
        let input = ":";
        let parser = EmojiTextParser::new(input);

        assert_eq!(input, &parser.collect::<String>());
    }

    #[test]
    fn parser_only_colons() {
        let input = ":::";
        let parser = EmojiTextParser::new(input);

        assert_eq!(input, &parser.collect::<String>());
    }

    #[test]
    fn parser_double_colons() {
        let input = "abc::technologist::def";
        let output = "abc:🧑‍💻:def";

        let parser = EmojiTextParser::new(input);

        assert_eq!(output, &parser.collect::<String>());
    }

    #[test]
    fn parser_many_colons() {
        let input = "abc:::technologist:::def";
        let output = "abc::🧑‍💻::def";

        let parser = EmojiTextParser::new(input);

        assert_eq!(output, &parser.collect::<String>());
    }

    #[test]
    fn parse_alias_test() {
        assert_eq!(
            Some(&crate::flat::FLAG_ECUADOR),
            parse_alias(":flag_ecuador:")
        );
    }

    #[test]
    fn parse_alias_none() {
        assert_eq!(None, parse_alias(":hebele:"));
    }
}
