use core::fmt;
use core::str::CharIndices;

#[derive(Debug, Clone)]
pub struct EmojiTextParser<'a> {
    original: &'a str,
    progress: CharIndices<'a>,
    /// Tracks the latest colon, if searching for a second one.
    last_colon_idx: Option<usize>,
    next_output_idx: usize,
}
impl<'a> EmojiTextParser<'a> {
    pub fn new(original: &'a str) -> Self {
        let progress = original.char_indices();
        Self {
            original,
            progress,
            /// Tracks the latest colon, if searching for a second one.
            last_colon_idx: None,
            next_output_idx: 0,
        }
    }
}
impl<'a> Iterator for EmojiTextParser<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((i, c)) = self.progress.next() {
            if c == ':' {
                if let Some(lc) = self.last_colon_idx {
                    // Found a secondary colon

                    let len = i - lc;
                    if len == 1 {
                        // Found just a `::`, that is not a colon, so

                        // Found first colon
                        self.last_colon_idx = Some(i);

                        // Update output index
                        let start_idx = self.next_output_idx;
                        self.next_output_idx = i; // possibly continue with this colon

                        // Output the preceeding str as is
                        return Some(&self.original[start_idx..i]);
                    } else {
                        self.last_colon_idx = None;

                        // Update output index
                        let start_idx = self.next_output_idx; // The start point to a colon, as set below
                        self.next_output_idx = i + 1; // actually continue after this colon;

                        if let Some(e) =
                            crate::parse_alias(&self.original[start_idx..self.next_output_idx])
                        {
                            return Some(e.grapheme);
                        } else {
                            // Well, what are going to do?
                            // Lets just output something that indicates that here was (maybe)
                            // something invalid.
                            return Some(crate::flat::COLLISION.grapheme);
                        }
                    }
                } else {
                    // Found first colon
                    self.last_colon_idx = Some(i);

                    // Update output index
                    let start_idx = self.next_output_idx;
                    self.next_output_idx = i; // possibly continue with this colon

                    // Output the preceeding str as is
                    return Some(&self.original[start_idx..i]);
                }
            } else if c.is_ascii_alphanumeric() || c == '_' || c == '+' {
                // Good char, go on ...
            } else if self.last_colon_idx.is_some() {
                // Bad char, which brakes the current run
                self.last_colon_idx = None
            } else {
                // just ignore
            }
        }

        if self.next_output_idx < self.original.len() {
            let start_idx = self.next_output_idx;
            self.next_output_idx = self.original.len();
            Some(&self.original[start_idx..])
        } else {
            None
        }
    }
}
impl<'a> fmt::Display for EmojiTextParser<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut copy = self.clone();
        for frag in copy {
            write!(fmt, "{}", frag)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
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
    #[cfg(feature = "alloc")]
    fn parser_nothing() {
        use alloc::string::String;

        let input = "Hello :: I am: a technologist, :=: :).";
        let mut parser = EmojiTextParser::new(input);

        assert_eq!(
            "Hello :: I am: a technologist, :=: :).",
            &parser.collect::<String>()
        );
    }
}
