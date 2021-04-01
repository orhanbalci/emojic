//!
//! Contains various emoji attributes.
//!

use core::fmt;

/// Unicode Emoji version.
///
/// This struct is used by [`Emoji`](super::Emoji) to denote when an emoji was introduced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(pub u64, pub u64);
impl fmt::Display for Version {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}.{}", self.0, self.1)
    }
}

/// Skin tone attribute 🖐️🖐🏻🖐🏼🖐🏽🖐🏾🖐🏿
///
/// Allows to specify the skin tone of supported emojis. Generally speaking, those involving people
/// or (some) body parts.
///
/// The default skin tone is 🖐️ (typically some yellow-ish)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Tone {
    /// Represents the least pigmented skin tone 🖐🏻
    Light,
    /// Represents the second least pigmented skin tone 🖐🏼
    MediumLight,
    /// Represents intermediately pigmented skin tone 🖐🏽
    Medium,
    /// Represents the second most pigmented skin tone 🖐🏾
    MediumDark,
    /// Represents the most pigmented skin tone 🖐🏿
    Dark,
}
impl Tone {
    /// Exhaustive list of all variants
    pub const ALL: [Tone; 5] = [
        Self::Light,
        Self::MediumLight,
        Self::Medium,
        Self::MediumDark,
        Self::Dark,
    ];

    /// Descriptive name of this attribute variant
    pub const fn name(self) -> &'static str {
        match self {
            Self::Light => "light skin tone",
            Self::MediumLight => "medium-light skin tone",
            Self::Medium => "medium skin tone",
            Self::MediumDark => "medium-dark skin tone",
            Self::Dark => "dark skin tone",
        }
    }
}

/// Represents a skin [`Tone`] pair.
///
/// Simply contains two independent skin tones. This allows emojis with two people to be
/// differently toned. E.g.: 🧑🏻‍🤝‍🧑🏼🧑🏼‍🤝‍🧑🏾🧑🏿‍🤝‍🧑🏻
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TonePair {
    /// The left hand side person's skin tone
    pub left: Tone,
    /// The right hand side person's skin tone
    pub right: Tone,
}
impl TonePair {
    /// Returns the entry index for this pair
    pub(crate) const fn to_id(self) -> usize {
        self.left as usize * Tone::ALL.len() + self.right as usize
    }
}
impl From<Tone> for TonePair {
    fn from(both: Tone) -> Self {
        TonePair {
            left: both,
            right: both,
        }
    }
}
impl From<(Tone, Tone)> for TonePair {
    fn from((left, right): (Tone, Tone)) -> Self {
        TonePair { left, right }
    }
}

/* Actually this does not exist currently in Unicode 13.1, so it doesn't make a lot of sens to put
   it into the public API.

/// Represents a skin [`Tone`] pair with limited extend.
///
// This is only a reduced pair of tones, always ensure: left <= right
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TonePairReduced {
    pub left: Tone,
    pub right: Tone,
}
impl TonePairReduced {
    /// Returns the entry index for this pair
    pub(crate) to_id(self) -> usize {
        assert!(self.left <= self.right);
        const D: usize = Tone::ALL.len();

        let l = self.left as usize;
        let r = self.right as usize;

        D * l - l * (l + 1) / 2 + r
    }
}
impl From<Tone> for TonePairReduced {
    fn from(both: Tone) -> Self {
        TonePairReduced {
            left: both,
            right: both,
        }
    }
}
impl From<(Tone, Tone)> for TonePairReduced {
    fn from((mut left, mut right): (Tone, Tone)) -> Self {
        if left <= right {
            TonePairReduced { left, right }
        } else {
            // Ensure order
            std::mem::swap(&mut left, &mut right);
            TonePairReduced { left, right }
        }
    }
}

*/

/// Gender attribute 🧑👨👩
///
/// Allows to specify the gender of supported emojis. Generally speaking, those involving people.
///
/// The default gender is 🧑 (a generic person somewhat genderless)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Gender {
    /// Represents the male gender 👨
    Male,
    /// Represents the female gender 👩
    Female,
}
impl Gender {
    /// Exhaustive list of all variants
    pub const ALL: [Gender; 2] = [Self::Male, Self::Female];

    /// [`Family`] composer using `self` as parent
    pub fn with_children(self, children: impl Into<OneOrTwo>) -> Family {
        (self, children).into()
    }

    /// Descriptive name of this attribute variant as adults
    pub const fn name_adults(self) -> &'static str {
        match self {
            Self::Male => "man",
            Self::Female => "woman",
        }
    }

    /// Descriptive name of this attribute variant as children
    pub const fn name_children(self) -> &'static str {
        match self {
            Self::Male => "boy",
            Self::Female => "girl",
        }
    }
}

/// Represents the gender of a pair of people 🧑‍🤝‍🧑👬👫👭
///
/// This allows emojis with two people to specify their gender.
///
/// The default is 🧑‍🤝‍🧑 (two genderless people)
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Pair {
    /// Two males 👬
    Males,
    /// A female and a male 👫
    Mixed,
    /// Two females 👭
    Females,
}
impl Pair {
    /// Exhaustive list of all variants
    pub const ALL: [Pair; 3] = [Self::Males, Self::Mixed, Self::Females];

    /// [`Family`] composer using `self` as parents
    pub fn with_children(self, children: impl Into<OneOrTwo>) -> Family {
        (self, children).into()
    }

    /// Descriptive name of this attribute variant as adults
    pub const fn name_adults(self) -> &'static str {
        match self {
            Self::Males => "men",
            Self::Mixed => "man & woman",
            Self::Females => "women",
        }
    }
    /// Descriptive name of this attribute variant as children
    pub const fn name_children(self) -> &'static str {
        match self {
            Self::Males => "boys",
            Self::Mixed => "boy & girl",
            Self::Females => "girls",
        }
    }
}
impl From<(Gender, Gender)> for Pair {
    fn from(pair: (Gender, Gender)) -> Self {
        match pair {
            (Gender::Male, Gender::Male) => Pair::Males,
            (Gender::Male, Gender::Female) => Pair::Mixed,
            (Gender::Female, Gender::Male) => Pair::Mixed,
            (Gender::Female, Gender::Female) => Pair::Females,
        }
    }
}

/// Represents one's or two person's gender while defining whether it's one or two.
///
/// Actually, this attribute is not used as such by any emoji, instead, it is used to compose
/// a [`Family`], which consists of two `OneOrTwo` structs.
///
/// E.g. one: 👨‍👦 (parent), two: 👨‍👨‍👦 (parents)
///
/// To get a `OneOrTwo` value, it is recommended to use any of the `From` impls e.g.:
///
/// ```rust
/// # use emojic::emojis::{OneOrTwo,Pair,Gender};
/// // From<Gender>
/// assert_eq!(OneOrTwo::One(Gender::Male), Gender::Male.into());
/// // From<Pair>
/// assert_eq!(OneOrTwo::Two(Pair::Males), Pair::Males.into());
/// ```
///
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OneOrTwo {
    /// Only one person
    One(Gender),
    /// Two people
    Two(Pair),
}
impl OneOrTwo {
    /// Exhaustive list of all variants
    pub const ALL: [OneOrTwo; 5] = [
        Self::One(Gender::Male),
        Self::One(Gender::Female),
        Self::Two(Pair::Males),
        Self::Two(Pair::Mixed),
        Self::Two(Pair::Females),
    ];

    /// Returns the entry index for this pair
    pub(crate) const fn to_id(self) -> usize {
        match self {
            Self::One(Gender::Male) => 0,
            Self::One(Gender::Female) => 1,
            Self::Two(Pair::Males) => 2,
            Self::Two(Pair::Mixed) => 3,
            Self::Two(Pair::Females) => 4,
        }
    }

    /// [`Family`] composer using `self` as parents
    pub fn with_children(self, children: impl Into<OneOrTwo>) -> Family {
        (self, children).into()
    }

    /// Descriptive name of this attribute variant
    pub const fn name_adults(self) -> &'static str {
        match self {
            Self::One(one) => one.name_adults(),
            Self::Two(two) => two.name_adults(),
        }
    }
    /// Descriptive name of this attribute variant
    pub const fn name_children(self) -> &'static str {
        match self {
            Self::One(one) => one.name_children(),
            Self::Two(two) => two.name_children(),
        }
    }
}
impl From<Gender> for OneOrTwo {
    fn from(g: Gender) -> Self {
        Self::One(g)
    }
}
impl From<Pair> for OneOrTwo {
    fn from(couple: Pair) -> Self {
        Self::Two(couple)
    }
}
impl From<(Gender, Gender)> for OneOrTwo {
    fn from(pair: (Gender, Gender)) -> Self {
        Self::Two(Pair::from(pair))
    }
}
impl From<(Gender, Option<Gender>)> for OneOrTwo {
    fn from(pair: (Gender, Option<Gender>)) -> Self {
        if let Some(g2) = pair.1 {
            Self::from((pair.0, g2))
        } else {
            Self::from(pair.0)
        }
    }
}

/// Represents the genders of an entire family with parents and children.
///
/// A Family consists of parents and children, each either a single or two persons as
/// expressed by the [`OneOrTwo`] struct.
///
/// E.g.:
///  - one parent & one child: 👨‍👦
///  - two parents & one child: 👨‍👨‍👦
///  - one parent & two children: 👩‍👧‍👦
///  - and so on...
///
/// To get a `Family` value, it is recommended to use any of the `From` impls e.g.:
///
/// ```rust
/// # use emojic::emojis::{Family,OneOrTwo,Pair,Gender};
/// // From<(Gender,Gender)>
/// assert_eq!(
///     Family {
///         parents: OneOrTwo::One(Gender::Male),
///         children: OneOrTwo::One(Gender::Male),
///     },
///     (Gender::Male,Gender::Male).into()
/// );
/// // From<(Pair,Pair)>
/// assert_eq!(
///     Family {
///         parents: OneOrTwo::Two(Pair::Males),
///         children: OneOrTwo::Two(Pair::Males),
///     },
///     (Pair::Males,Pair::Males).into()
/// );
/// ```
///
/// Or use the `with_children` composer of [`Gender`] and [`Pair`]:
///
/// ```rust
/// # use emojic::emojis::{Family,OneOrTwo,Pair,Gender};
/// // Gender::with_children
/// assert_eq!(
///     Family {
///         parents: OneOrTwo::One(Gender::Male),
///         children: OneOrTwo::One(Gender::Male),
///     },
///     Gender::Male.with_children(Gender::Male)
/// );
/// // Pair::with_children
/// assert_eq!(
///     Family {
///         parents: OneOrTwo::Two(Pair::Males),
///         children: OneOrTwo::Two(Pair::Males),
///     },
///     Pair::Males.with_children(Pair::Males)
/// );
/// ```
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Family {
    pub parents: OneOrTwo,
    pub children: OneOrTwo,
}
impl Family {
    /// Returns the entry index for this constellation
    pub(crate) const fn to_id(self) -> usize {
        self.parents.to_id() * OneOrTwo::ALL.len() + self.children.to_id()
    }
}
impl<A: Into<OneOrTwo>, B: Into<OneOrTwo>> From<(A, B)> for Family {
    fn from((parents, children): (A, B)) -> Self {
        Family {
            parents: parents.into(),
            children: children.into(),
        }
    }
}

/// Hair style attribute 🧑🧔👱👨‍🦰👨‍🦱👨‍🦳👨‍🦲
///
/// Allows to specify the hair style of supported emojis. Generally speaking, those involving
/// people (well currently only of a single person).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Hair {
    /// Represents a bearded person 🧔
    Beard,
    /// Represents a person with blond hair 👱
    Blond,
    /// Represents a person with red hair 👨‍🦰
    Red,
    /// Represents a person with curly hair 👨‍🦱
    Curly,
    /// Represents a person with white hair 👨‍🦳
    ///
    /// Not to be confused with an older person 🧓
    White,
    /// Represents a person without hair 👨‍🦲
    Bald,
}
impl Hair {
    /// Exhaustive list of all variants
    pub const ALL: [Hair; 6] = [
        Self::Beard,
        Self::Blond,
        Self::Red,
        Self::Curly,
        Self::White,
        Self::Bald,
    ];

    /// Descriptive name of this attribute variant
    pub const fn name(self) -> &'static str {
        match self {
            Self::Beard => "beard",
            Self::Bald => "no hair",
            Self::Blond => "blond hair",
            Self::Red => "red hair",
            Self::Curly => "curly hair",
            Self::White => "white hair",
        }
    }
}
