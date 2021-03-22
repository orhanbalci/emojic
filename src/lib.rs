use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

#[rustfmt::skip]
mod alias; // Generated module
#[rustfmt::skip]
mod constants; // Generated module

use alias::GEMOJI_MAP;

#[doc(inline)]
pub use constants::flat;
#[doc(inline)]
pub use constants::grouped;

const TONE_PLACE_HOLDER: &str = "@";

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tone {
    Default,
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
}

impl Tone {
    fn unicode(self) -> &'static str {
        use Tone::*;

        match self {
            Default => "",
            Light => "\u{1F3FB}",
            MediumLight => "\u{1F3FC}",
            Medium => "\u{1F3FD}",
            MediumDark => "\u{1F3FE}",
            Dark => "\u{1F3FF}",
        }
    }
}
impl Default for Tone {
    fn default() -> Self {
        Tone::Default
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Emoji(pub &'static str);

impl fmt::Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EmojiWithTone {
    pub one_toned_code: &'static str,
    pub two_toned_code: &'static str,
    pub default_tone: &'static str,
}

impl EmojiWithTone {
    pub const fn one_toned(code: &'static str) -> Self {
        EmojiWithTone {
            one_toned_code: code,
            two_toned_code: code,
            default_tone: "",
        }
    }

    pub const fn two_toned(mut self, code: &'static str) -> Self {
        self.two_toned_code = code;
        self
    }

    pub const fn default_tone(mut self, tone: &'static str) -> Self {
        self.default_tone = tone;
        self
    }

    pub fn tone(&self, tones: &[Tone]) -> String {
        if tones.len() == 0 {
            self.one_toned_code
                .to_string()
                .replace(TONE_PLACE_HOLDER, self.default_tone)
        } else {
            if tones.len() == 1 || self.two_toned_code.is_empty() {
                if tones[0] == Tone::Default {
                    self.one_toned_code
                        .to_string()
                        .replace(TONE_PLACE_HOLDER, self.default_tone)
                } else {
                    self.one_toned_code
                        .to_string()
                        .replace(TONE_PLACE_HOLDER, tones[0].unicode())
                }
            } else if tones.len() > 1 && !self.two_toned_code.is_empty() {
                self.two_toned_code
                    .replacen(
                        TONE_PLACE_HOLDER,
                        if tones[0] == Tone::Default {
                            self.default_tone
                        } else {
                            tones[0].unicode()
                        },
                        1,
                    )
                    .replacen(
                        TONE_PLACE_HOLDER,
                        if tones[1] == Tone::Default {
                            self.default_tone
                        } else {
                            tones[1].unicode()
                        },
                        1,
                    )
            } else {
                self.to_string()
            }
        }
    }
}

impl fmt::Display for EmojiWithTone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tone(&[]))
    }
}

lazy_static! {
    static ref ALIAS_REGEX: Regex = Regex::new(r":(\S*):").unwrap();
}

/// Parses the given Emoji name into a unicode Emoji.
///
/// ```ignore
/// # // THIS HAS CURRENTLY ISSUES!
/// # // THUMBS_UP and parse_alias have different types!
/// # // TODO: FIX THIS
/// use emojic::parse_alias;
///
/// assert_eq!(
///     Some(emojic::flat::ALIEN_MONSTER), //👾
///     parse_alias(":alien_monster:")
/// );
///
/// assert_eq!(
///     Some(emojic::flat::THUMBS_UP), //👍
///     parse_alias(":+1:")
/// );
/// ```
///
pub fn parse_alias(inp: &str) -> Option<Emoji> {
    ALIAS_REGEX.captures(inp).and_then(|cap| {
        cap.iter()
            .next()?
            .and_then(|v| GEMOJI_MAP.get(v.as_str()).cloned().map(Emoji))
    })
}

#[cfg(test)]
mod tests {
    use super::parse_alias;

    #[test]
    fn parse_test() {
        assert_eq!(
            Some(crate::flat::FLAG_ECUADOR),
            parse_alias(":flag_ecuador:")
        );
    }

    #[test]
    fn parse_fail() {
        assert_eq!(None, parse_alias(":hebele:"));
    }
}
