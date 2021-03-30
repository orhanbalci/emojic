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

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Emoji {
    pub name: &'static str,
    pub since: Version,
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

    pub const fn name(&self) -> &'static str {
        &self.name
    }
}

impl Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grapheme)
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

impl<T> With<Tone, T> {
    pub const fn tone(&self, tone: Tone) -> &T {
        &self.entries[tone as usize]
    }
}
impl<T> WithNoDef<Tone, T> {
    pub const fn tone(&self, tone: Tone) -> &T {
        &self.entries[tone as usize]
    }
}

impl<T> With<TonePair, T> {
    pub fn tone(&self, tone: impl Into<TonePair>) -> &T {
        self.tone_pair(tone.into())
    }
    pub const fn tone_pair(&self, tone_pair: TonePair) -> &T {
        &self.entries[tone_pair.to_id()]
    }
}
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

impl<T> With<Gender, T> {
    pub const fn gender(&self, gender: Gender) -> &T {
        &self.entries[gender as usize]
    }
}
impl<T> WithNoDef<Gender, T> {
    pub const fn gender(&self, gender: Gender) -> &T {
        &self.entries[gender as usize]
    }
}

impl<T> With<Hair, T> {
    pub const fn hair(&self, hair: Hair) -> &T {
        &self.entries[hair as usize]
    }
}
impl<T> WithNoDef<Hair, T> {
    pub const fn hair(&self, hair: Hair) -> &T {
        &self.entries[hair as usize]
    }
}

impl<T> With<Pair, T> {
    pub fn gender(&self, pair: impl Into<Pair>) -> &T {
        &self.entries[pair.into() as usize]
    }
    pub const fn pair(&self, pair: Pair) -> &T {
        &self.entries[pair as usize]
    }
}
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

impl<T> With<Family, T> {
    pub fn gender(&self, family: impl Into<Family>) -> &T {
        &self.entries[family.into().to_id()]
    }
    pub const fn family(&self, family: Family) -> &T {
        &self.entries[family.to_id()]
    }
}
impl<T> WithNoDef<Family, T> {
    pub fn gender(&self, family: impl Into<Family>) -> &T {
        &self.entries[family.into().to_id()]
    }
    pub const fn family(&self, family: Family) -> &T {
        &self.entries[family.to_id()]
    }
}
