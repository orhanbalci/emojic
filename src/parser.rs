use core::fmt;

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

                    // TODO check bounds!
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
}
