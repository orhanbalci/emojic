//!
//! Types for representing and customizing emoji
//!

use core::fmt;
use core::fmt::Display;
use core::marker::PhantomData;
use core::ops::Deref;

mod attributes;
pub use attributes::Family;
pub use attributes::Gender;
pub use attributes::Hair;
pub use attributes::OneOrTwo;
pub use attributes::Pair;
pub use attributes::Tone;
pub use attributes::TonePair;
pub use attributes::Version;

/// A specific emoji.
///
/// This is the basic type for all emojis, whether obtained via any of the statics (as defined in
/// the [`grouped`](crate::grouped) and [`flat`](crate::flat) module) or functions such as
/// [`parse_alias`](crate::parse_alias).
///
/// Tho, some statics are declared as [`With`] or [`WithNoDef`]. These represent customizable
/// emojis (i.e. a set of similar emojis), and provide functions for customizations (such as
/// [`With::tone`], [`With::gender`], and [`With::hair`]), which take an attribute to be customized
/// (such as [`Tone`], [`Gender`], or [`Hair`] respectively) and will eventually yield an `Emoji`.
///
/// `Emoji` implements `Display` to be directly printable (e.g. with `println!`). This will simply
/// print the [`grapheme`](Self::grapheme) (the Unicode sequence) of this emoji.
/// Additionally, this struct contains some meta data such as the explanatory
/// [`name`](Self::grapheme) of the emoji.
///
/// ```
/// # use emojic::emojis::Emoji;
/// # use emojic::emojis::Version;
/// let art = Emoji {
///     name: "artist palette",
///     since: Version(0,6), // E0.6
///     grapheme: "🎨",
/// };
/// assert_eq!(emojic::flat::ARTIST_PALETTE, art);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Emoji {
    /// The full name of this emoji, much like a short description.
    pub name: &'static str,
    /// The Unicode Emoji version when this grapheme (i.e. emoji) was first introduced.
    ///
    /// Notice, that since this stated version the recommended visuals may have changed or
    /// additional variants might have been added related to this emoji. In that case, the
    /// individual variants (which have their own `Emoji` instance) may have a different version
    /// than the 'default' variant, depending on when they were first added, respetively.
    pub since: Version,
    /// The Unicode codepoint sequence of this emoji. The actual/rendered emoji.
    pub grapheme: &'static str,
}
impl Emoji {
    pub(crate) const fn new(name: &'static str, since: Version, grapheme: &'static str) -> Self {
        Emoji {
            name,
            since,
            grapheme,
        }
    }
}

impl Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grapheme)
    }
}

/// Customizable emoji composer without default.
///
/// This struct contains a set of [`Emoji`] which can be differentiated by an attribute `M` such as
/// [`Tone`], [`Gender`], and [`Hair`]. Depending on the attribute type this struct provides
/// customization functions such as [`WithNoDef::tone`], [`WithNoDef::gender`],
/// and [`WithNoDef::hair`], respectively.
///
/// Notice unlike the [`With`], this struct has no default variant and thus can not directly be
/// used, instead customization is mandatory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WithNoDef<M, T: 'static> {
    entries: &'static [T],
    _m: PhantomData<M>,
}

impl<M, T> WithNoDef<M, T> {
    pub(crate) const fn new(entries: &'static [T]) -> Self {
        //assert_eq!(entries.len(), M::SIZE); invalid in const fn
        WithNoDef {
            entries,
            _m: PhantomData,
        }
    }
}

/// Customizable emoji composer.
///
/// This struct contains a set of [`Emoji`] which can be differentiated by an attribute `M` such as
/// [`Tone`], [`Gender`], and [`Hair`]. Depending on the attribute type this struct provides
/// customization functions such as [`With::tone`], [`With::gender`], and [`With::hair`],
/// respectively.
///
/// Notice unlike the [`WithNoDef`], this struct has an default variant and thus `Deref`s to `T`,
/// and implements `Display` if `T` does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct With<M, T: 'static> {
    pub default: T,
    entries: &'static [T],
    _m: PhantomData<M>,
}

impl<M, T> With<M, T> {
    pub(crate) const fn new(default: T, entries: &'static [T]) -> Self {
        //assert_eq!(entries.len(), M::SIZE); invalid in const fn
        With {
            default,
            entries,
            _m: PhantomData,
        }
    }
}

impl<M, T> Deref for With<M, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.default
    }
}

impl<M, T: Display> Display for With<M, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.default)
    }
}

/// Customizing by [`Tone`].
///
/// # Examples
/// ```
/// # use emojic::flat::RAISING_HANDS;
/// # use emojic::Tone;
/// assert_eq!(RAISING_HANDS.to_string(), "🙌"); // default, derefs directly to `Emoji`
/// assert_eq!(RAISING_HANDS.tone(Tone::Medium).to_string(), "🙌🏽"); // Toned variant
/// ```
impl<T> With<Tone, T> {
    pub const fn tone(&self, tone: Tone) -> &T {
        &self.entries[tone as usize]
    }
}
/// Customizing by [`Tone`] without defaults.
///
/// _There is currently no such emoji_
///
impl<T> WithNoDef<Tone, T> {
    pub const fn tone(&self, tone: Tone) -> &T {
        &self.entries[tone as usize]
    }
}

/// Customizing by [`TonePair`].
///
/// Notice that [`Tone`] and `(Tone, Tone)` both implement `Into<TonePair>`.
///
/// # Examples
/// ```
/// # use emojic::flat::COUPLE_WITH_HEART;
/// # use emojic::Tone;
/// assert_eq!(COUPLE_WITH_HEART.to_string(), "💑"); // default, derefs directly to `Emoji`
/// assert_eq!(COUPLE_WITH_HEART.tone(Tone::Medium).to_string(), "💑🏽"); // Same skin tone
/// assert_eq!(COUPLE_WITH_HEART.tone((Tone::Light, Tone::Dark)).to_string(), "🧑🏻‍❤️‍🧑🏿"); // Two different skin tones
/// ```
impl<T> With<TonePair, T> {
    pub fn tone(&self, tone: impl Into<TonePair>) -> &T {
        self.tone_pair(tone.into())
    }
    pub const fn tone_pair(&self, tone_pair: TonePair) -> &T {
        &self.entries[tone_pair.to_id()]
    }
}
/// Customizing by [`TonePair`] without defaults.
///
/// _There is currently no such emoji_
///
impl<T> WithNoDef<TonePair, T> {
    pub fn tone(&self, tone: impl Into<TonePair>) -> &T {
        self.tone_pair(tone.into())
    }
    pub const fn tone_pair(&self, tone_pair: TonePair) -> &T {
        &self.entries[tone_pair.to_id()]
    }
}

/* Actually this does not exist currently in Unicode 13.1, so it doesn't make a lot of sens to put
   it into the public API.
impl<T> With<TonePairReduced, T> {
    pub fn tone(&self, tone: Tone) -> &T {
        self.tone_pair((tone, tone))
    }
    pub fn tone_pair(&self, tone_pair: impl Into<TonePairReduced>) -> &T {
        &self.entries[tone_pair.into().to_id()]
    }
}
impl<T> WithNoDef<TonePairReduced, T> {
    pub fn tone(&self, tone: Tone) -> &T {
        self.tone_pair((tone, tone))
    }
    pub fn tone_pair(&self, tone_pair: impl Into<TonePairReduced>) -> &T {
        &self.entries[tone_pair.into().to_id()]
    }
}
*/

/// Customizing by [`Gender`].
///
/// # Examples
/// ```
/// # use emojic::flat::ELF;
/// # use emojic::Gender;
/// assert_eq!(ELF.to_string(), "🧝"); // default, derefs directly to `Emoji`
/// assert_eq!(ELF.gender(Gender::Female).to_string(), "🧝‍♀️"); // Variant with gender
/// ```
impl<T> With<Gender, T> {
    pub const fn gender(&self, gender: Gender) -> &T {
        &self.entries[gender as usize]
    }
}
/// Customizing by [`Gender`] without defaults.
///
/// # Examples
/// ```
/// # use emojic::flat::PERSON_DANCING;
/// # use emojic::Gender;
/// //assert_eq!(PERSON_DANCING.to_string(), "?"); // no default, would not compile
/// assert_eq!(PERSON_DANCING.gender(Gender::Male).to_string(), "🕺"); // Variant with male gender
/// assert_eq!(PERSON_DANCING.gender(Gender::Female).to_string(), "💃"); // Variant with female gender
/// ```
impl<T> WithNoDef<Gender, T> {
    pub const fn gender(&self, gender: Gender) -> &T {
        &self.entries[gender as usize]
    }
}

/// Customizing by [`Hair`].
///
/// # Examples
/// ```
/// # use emojic::flat::PERSON;
/// # use emojic::Hair;
/// assert_eq!(PERSON.to_string(), "🧑"); // default, derefs directly to `Emoji`
/// assert_eq!(PERSON.hair(Hair::Red).to_string(), "🧑‍🦰"); // Variant with hair style
/// ```
impl<T> With<Hair, T> {
    pub const fn hair(&self, hair: Hair) -> &T {
        &self.entries[hair as usize]
    }
}
/// Customizing by [`Hair`] without defaults.
///
/// _There is currently no such emoji_
///
impl<T> WithNoDef<Hair, T> {
    pub const fn hair(&self, hair: Hair) -> &T {
        &self.entries[hair as usize]
    }
}

/// Customizing by (gender) [`Pair`].
///
/// # Examples
/// ```
/// # use emojic::flat::PERSON_HOLDING_HANDS;
/// # use emojic::Pair;
/// assert_eq!(PERSON_HOLDING_HANDS.to_string(), "🧑‍🤝‍🧑"); // default, derefs directly to `Emoji`
/// assert_eq!(PERSON_HOLDING_HANDS.gender(Pair::Mixed).to_string(), "👫"); // With defined gender
/// ```
impl<T> With<Pair, T> {
    pub fn gender(&self, pair: impl Into<Pair>) -> &T {
        &self.entries[pair.into() as usize]
    }
    pub const fn pair(&self, pair: Pair) -> &T {
        &self.entries[pair as usize]
    }
}
/// Customizing by [`Pair`] without defaults.
///
/// _There is currently no such emoji_
///
impl<T> WithNoDef<Pair, T> {
    pub fn gender(&self, pair: impl Into<Pair>) -> &T {
        &self.entries[pair.into() as usize]
    }
    pub const fn pair(&self, pair: Pair) -> &T {
        &self.entries[pair as usize]
    }
}

impl<T> With<OneOrTwo, T> {
    pub fn gender(&self, oot: impl Into<OneOrTwo>) -> &T {
        &self.entries[oot.into().to_id()]
    }
    pub const fn pair(&self, oot: OneOrTwo) -> &T {
        &self.entries[oot.to_id()]
    }
}
impl<T> WithNoDef<OneOrTwo, T> {
    pub fn gender(&self, oot: impl Into<OneOrTwo>) -> &T {
        &self.entries[oot.into().to_id()]
    }
    pub const fn pair(&self, oot: OneOrTwo) -> &T {
        &self.entries[oot.to_id()]
    }
}

/// Customizing by [`Family`].
///
/// Notice that various type that implement `Into<Family>` such as `(Gender,Gender)`,
/// and `(Pair,Pair)`.
///
/// # Examples
/// ```
/// # use emojic::flat::FAMILY;
/// # use emojic::Gender;
/// # use emojic::Pair;
/// assert_eq!(FAMILY.to_string(), "👪"); // default, derefs directly to `Emoji`
/// assert_eq!(FAMILY.gender((Gender::Male, Gender::Female)).to_string(), "👨‍👧"); // Variant two single genders
/// assert_eq!(FAMILY.gender((Pair::Males, Pair::Females)).to_string(), "👨‍👨‍👧‍👧"); // Variant with two gender pairs
/// assert_eq!(FAMILY.gender(Gender::Female.with_children(Pair::Mixed)).to_string(), "👩‍👧‍👦"); // Variant based on composer chain
/// ```
impl<T> With<Family, T> {
    pub fn gender(&self, family: impl Into<Family>) -> &T {
        &self.entries[family.into().to_id()]
    }
    pub const fn family(&self, family: Family) -> &T {
        &self.entries[family.to_id()]
    }
}
/// Customizing by [`Family`] without defaults.
///
/// _There is currently no such emoji_
///
impl<T> WithNoDef<Family, T> {
    pub fn gender(&self, family: impl Into<Family>) -> &T {
        &self.entries[family.into().to_id()]
    }
    pub const fn family(&self, family: Family) -> &T {
        &self.entries[family.to_id()]
    }
}
