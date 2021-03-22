//!
//! Contains structs for group by attributes.
//!

use super::*;

/// Represents a single emoji variant by its full set of all attributes.
///
/// A `None` represents the default or absence, e.g. the default skin tone or the genderless person.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PersonKind {
    pub hair: Option<Hair>,
    pub people: Option<(OneOrTwo, Option<OneOrTwo>)>,
    pub tone: Option<(Tone, Option<Tone>)>,
}
impl PersonKind {
    /// Classifies how generic this kind is (i.e. how many `None`s it contains)
    pub fn default_level(self) -> usize {
        self.hair.is_none() as usize
            + 2 * self.people.and_then(|p| p.1).is_none() as usize
            + 4 * self.people.is_none() as usize
            + 8 * self.tone.and_then(|t| t.1).is_none() as usize
            + 16 * self.tone.is_none() as usize
    }
}
impl From<(Hair, Gender, Tone)> for PersonKind {
    fn from(f: (Hair, Gender, Tone)) -> Self {
        PersonKind {
            hair: f.0.into(),
            people: (f.1.into(), None).into(),
            tone: (f.2, None).into(),
        }
    }
}

/// Defines a emoji variant selector by on its attributes.
///
/// Emoji variants are represented by [`PersonKind`].
///
/// A `None` means all values of an attribute are selected (i.e. unspecific), where as a
/// `Some(None)` select only the default value of an attribute, and `Some(Some(T))` selects
/// the `T` value.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PersonKindSelector {
    pub hair: Option<Option<Hair>>,
    pub people: Option<Option<(OneOrTwo, Option<OneOrTwo>)>>,
    pub tone: Option<Option<(Tone, Option<Tone>)>>,
}
impl PersonKindSelector {
    pub fn selects(self, kind: PersonKind) -> bool {
        self.hair.map(|h| h == kind.hair) != Some(false)
            && self.people.map(|h| h == kind.people) != Some(false)
            && self.tone.map(|h| h == kind.tone) != Some(false)
    }
    pub fn adapt_identifier(self, id: &str) -> String {
        // TODO Enhance

        let mut id = id.to_string();
        let mut with = false;
        let mut connector = || {
            if with {
                " and "
            } else {
                with = true;
                " with "
            }
        };

        if let Some(Some(h)) = self.hair {
            id = id + connector() + h.name();
        }
        if let Some(Some((g, children))) = self.people {
            let mut name = g.name_adults().to_string();
            if let Some(c) = children {
                name = name + " with " + c.name_children();
            }

            id = id.replace("PERSON", &name);
        }
        if let Some(Some((t, secondary))) = self.tone {
            id = id + connector() + t.name();

            if let Some(t) = secondary {
                id = id + " & " + t.name();
            }
        }

        id
    }
}
impl From<PersonKind> for PersonKindSelector {
    fn from(f: PersonKind) -> Self {
        PersonKindSelector {
            hair: f.hair.into(),
            people: f.people.into(),
            tone: f.tone.into(),
        }
    }
}

/// Represent a group of [`PersonKind`] base on a qualified prefix.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PersonKindGroup {
    All,
    Hair(Option<Hair>),
    People(Option<Hair>, Option<(OneOrTwo, Option<OneOrTwo>)>),
    Tone(PersonKind),
}
impl PersonKindGroup {
    pub fn qualify(
        self,
        subs: impl IntoIterator<Item = (PersonKindSelector, PersonQualified)>,
    ) -> Vec<(PersonKindSelector, PersonQualified)> {
        match self {
            Self::All => Hair::qualify(subs),
            Self::Hair(_) => <(OneOrTwo, Option<OneOrTwo>)>::qualify(subs),
            Self::People(_, _) => <(Tone, Option<Tone>)>::qualify(subs),
            Self::Tone(_) => panic!("The skin group can not be qualified"),
        }
    }

    pub fn next_iter(self) -> Result<Vec<PersonKindGroup>, PersonKind> {
        use std::iter::once;
        match self {
            Self::All => Ok(once(None)
                .chain(Hair::ALL.iter().map(|h| Some(*h)))
                .map(PersonKindGroup::Hair)
                .collect()),
            Self::Hair(h) => Ok(once(None)
                .chain(OneOrTwo::ALL.iter().flat_map(|p| {
                    once(None)
                        .chain(OneOrTwo::ALL.iter().map(|s| Some(*s)))
                        .map(|sec| Some((*p, sec)))
                        .collect::<Vec<_>>()
                }))
                .map(|p| PersonKindGroup::People(h, p))
                .collect()),
            Self::People(h, p) => Ok(once(None)
                .chain(Tone::ALL.iter().flat_map(|t| {
                    once(None)
                        .chain(Tone::ALL.iter().map(|s| Some(*s)))
                        .map(|sec| Some((*t, sec)))
                        .collect::<Vec<_>>()
                }))
                .map(|t| {
                    PersonKindGroup::Tone(PersonKind {
                        hair: h,
                        people: p,
                        tone: t,
                    })
                })
                .collect()),
            Self::Tone(k) => Err(k),
        }
    }
}
