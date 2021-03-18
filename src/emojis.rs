use std::fmt;
use std::fmt::Display;
use std::ops::Deref;

mod attributes;
pub use attributes::Family;
pub use attributes::Gender;
pub use attributes::Hair;
pub use attributes::OneOrTwo;
pub use attributes::Pair;
pub use attributes::Tone;
pub use attributes::TonePair;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Emoji {
    pub name: &'static str,
    pub grapheme: &'static str,
}
impl Emoji {
    pub(crate) const fn new(name: &'static str, grapheme: &'static str) -> Self {
        Emoji { name, grapheme }
    }

    pub fn name(&self) -> &'static str {
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
    _m: std::marker::PhantomData<M>,
}

impl<M, T> WithNoDef<M, T> {
    pub(crate) const fn new(entries: &'static [T]) -> Self {
        //assert_eq!(entries.len(), M::SIZE); invalid in const fn
        WithNoDef {
            entries,
            _m: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct With<M, T: 'static> {
    default: T,
    entries: &'static [T],
    _m: std::marker::PhantomData<M>,
}

impl<M, T> With<M, T> {
    pub(crate) const fn new(default: T, entries: &'static [T]) -> Self {
        //assert_eq!(entries.len(), M::SIZE); invalid in const fn
        With {
            default,
            entries,
            _m: std::marker::PhantomData,
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
    pub fn tone(&self, tone: Tone) -> &T {
        &self.entries[tone as usize]
    }
}
impl<T> WithNoDef<Tone, T> {
    pub fn tone(&self, tone: Tone) -> &T {
        &self.entries[tone as usize]
    }
}

impl<T> With<TonePair, T> {
    pub fn tone(&self, tone: Tone) -> &T {
        self.tone_pair((tone, tone))
    }
    pub fn tone_pair(&self, tone_pair: impl Into<TonePair>) -> &T {
        &self.entries[tone_pair.into().to_id()]
    }
}
impl<T> WithNoDef<TonePair, T> {
    pub fn tone(&self, tone: Tone) -> &T {
        self.tone_pair((tone, tone))
    }
    pub fn tone_pair(&self, tone_pair: impl Into<TonePair>) -> &T {
        &self.entries[tone_pair.into().to_id()]
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
    pub fn gender(&self, gender: Gender) -> &T {
        &self.entries[gender as usize]
    }
}
impl<T> WithNoDef<Gender, T> {
    pub fn gender(&self, gender: Gender) -> &T {
        &self.entries[gender as usize]
    }
}

impl<T> With<Hair, T> {
    pub fn hair(&self, hair: Hair) -> &T {
        &self.entries[hair as usize]
    }
}
impl<T> WithNoDef<Hair, T> {
    pub fn hair(&self, hair: Hair) -> &T {
        &self.entries[hair as usize]
    }
}

impl<T> With<Pair, T> {
    pub fn pair(&self, pair: impl Into<Pair>) -> &T {
        &self.entries[pair.into() as usize]
    }
}
impl<T> WithNoDef<Pair, T> {
    pub fn pair(&self, pair: impl Into<Pair>) -> &T {
        &self.entries[pair.into() as usize]
    }
}

impl<T> With<OneOrTwo, T> {
    pub fn pair(&self, oot: impl Into<OneOrTwo>) -> &T {
        &self.entries[oot.into().to_id()]
    }
}
impl<T> WithNoDef<OneOrTwo, T> {
    pub fn pair(&self, oot: impl Into<OneOrTwo>) -> &T {
        &self.entries[oot.into().to_id()]
    }
}

impl<T> With<Family, T> {
    pub fn family(&self, family: impl Into<Family>) -> &T {
        &self.entries[family.into().to_id()]
    }
}
impl<T> WithNoDef<Family, T> {
    pub fn family(&self, family: impl Into<Family>) -> &T {
        &self.entries[family.into().to_id()]
    }
}
