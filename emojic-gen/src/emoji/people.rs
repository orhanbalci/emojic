//!
//! Manages emoji sets which are semantically equal but differ in attributes.
//!
//! This kind of emojis with varying attributes exist only for the 'person' emojis, i.e. emojis
//! containing people and some body parts.
//! Attributes are skin tone, gender, and hair style.
//!
//! The difficulty is that some some emojis may only have a variation of gender (e.g. the vampire),
//! others might only differ in skin tone (e.g. the waving hand), yet other may differ in both
//! attributes (e.g. the artist), it gets even more complicated if multiple person are
//! involved.
//!
//! Thus this entire module (including sub modules) is dedicated to cope with this complexity.
//!

use super::*;

mod qualifier;
use qualifier::*;

mod parsing;
use parsing::*;

mod groups;
use groups::*;

/// Represents a set of emojis which are semantically equal but exist is multiple variations of
/// attributes.
#[derive(Clone, Debug)]
pub struct PersonEmoji {
    pub identifier: String,
    pub fancy_name: String,
    pub grouping: Option<PersonQualified>,
    pub variants: HashMap<PersonKind, PersonVariant>,
}
impl PersonEmoji {
    pub fn new(fancy_name: String) -> Self {
        PersonEmoji {
            identifier: generate_constant(&fancy_name),
            fancy_name,
            grouping: None,
            variants: HashMap::new(),
        }
    }

    pub fn default_variants(&self) -> impl Iterator<Item = &PersonVariant> {
        let mut most_general =
            self.variants
                .keys()
                .fold(Vec::<PersonKind>::new(), |mut vec, var| {
                    if vec.is_empty() || vec[0].default_level() == var.default_level() {
                        vec.push(*var);
                    } else if vec[0].default_level() < var.default_level() {
                        vec.clear();
                        vec.push(*var);
                    }
                    vec
                });

        most_general.sort();
        most_general.dedup();

        debug_assert!(
            !most_general.is_empty(),
            "Missing default variant for {:?}\n  full set: {:?}",
            self.identifier,
            self.variants
        );

        most_general.into_iter().map(move |k| &self.variants[&k])
    }

    /// Check the total consistency of this emoji, and split it if it violates rules.
    pub fn scrub(self) -> Vec<Self> {
        println!("Scrub: {:?}", self.identifier);

        self.qualify(PersonKindGroup::All)
            .into_iter()
            .map(|(class, group)| {
                let new_name = class.adapt_identifier(&self.identifier);
                println!("Scrubbed: {:?}", generate_constant(&new_name));
                PersonEmoji {
                    identifier: generate_constant(&new_name),
                    fancy_name: self.fancy_name.clone(),
                    grouping: Some(group),
                    variants: self
                        .variants
                        .iter()
                        .filter_map(|(k, v)| class.selects(*k).then(|| (*k, v.clone())))
                        .collect(),
                }
            })
            .collect()
    }

    /// Classify all variants into possibly multiple groups.
    ///
    /// If this emoji is consistent (i.e. no missing variants) only a single group will be returned.
    fn qualify(&self, grp: PersonKindGroup) -> Vec<(PersonKindSelector, PersonQualified)> {
        match grp.next_iter() {
            Ok(sub_grps) => {
                let subs = sub_grps.into_iter().map(|g| self.qualify(g)).flatten();
                grp.qualify(subs)
            }
            Err(k) => {
                if self.variants.contains_key(&k) {
                    vec![(k.into(), k.into())]
                } else {
                    vec![]
                }
            }
        }
    }
}
impl ToSourceCode for PersonEmoji {
    fn to_source_code(&self) -> String {
        let mut source = String::new();

        let (ty, value, docs) = {
            if let Some(group) = &self.grouping {
                group.to_type_n_value(&self.identifier, &self.variants)
            } else {
                panic!("PersonEmoji must be scrubbed before it can be rendered!")
            }
        };

        source.push_str(&format!(
            r#"#[doc="{} {}"]#[doc=""] {}"#,
            self.fancy_name,
            self.graphemes(),
            emoji_render_example_section(&docs, &self.identifier)
        ));
        source.push_str(&format!(
            "\npub static {}: {} = {};\n",
            self.identifier, ty, value
        ));

        source
    }
    fn identifier(&self) -> &str {
        &self.identifier
    }
    fn name(&self) -> &str {
        &self.fancy_name
    }
    fn graphemes(&self) -> String {
        self.default_variants()
            .map(|v| &v.grapheme as &str)
            .collect::<Vec<_>>()
            .join("")
    }
    fn default_grapheme(&self) -> Option<&str> {
        let mut variants = self.default_variants();
        let first = variants.next().unwrap();
        variants.next().is_none().then(|| &first.grapheme as &str)
    }
    fn full_emoji_list(&self) -> Vec<(String, &str)> {
        if let Some(group) = &self.grouping {
            group.to_accessor_n_grapheme(&self.identifier, &self.variants)
        } else {
            panic!("PersonEmoji must be scrubbed before it can be rendered!")
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonVariant {
    full_name: String,
    grapheme: String,
}
impl From<Emoji> for PersonVariant {
    fn from(e: Emoji) -> Self {
        PersonVariant {
            full_name: e.name,
            grapheme: e.grapheme,
        }
    }
}

/// Represents a freshly parsed personated emoji.
///
/// This struct helps in parsing the attributes and then supposed to be joined into some
/// [`PersonEmoji`] via [`Subgroup::append_person`].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonEntry {
    pub name: String,
    pub kind: PersonKind,
    pub variant: PersonVariant,
}
impl PersonEntry {
    /// Parses the given string snipets into the respective attribute values.
    pub fn parse(
        full_name: String,
        grapheme: String,
        name: String,
        people: Option<(&str, Option<&str>, Option<(&str, Option<&str>)>)>,
        tone: Option<(&str, Option<&str>)>,
        hair: Option<&str>,
    ) -> Self {
        let people = people
            .map(|a| {
                ParsedOneOrTwo::from_str(&format!("{},{}", a.0, a.1.unwrap_or_default()))
                    .unwrap()
                    .0
                    .zip(Some(
                        a.2.map(|c| {
                            ParsedOneOrTwoChildren::from_str(&format!(
                                "{},{}",
                                c.0,
                                c.1.unwrap_or_default()
                            ))
                            .unwrap()
                            .into()
                        })
                        .flatten(),
                    ))
            })
            .flatten();

        let tone = tone.map(|a| {
            (
                Tone::from_str(a.0).unwrap(),
                a.1.map(|c| Tone::from_str(c).unwrap()),
            )
        });

        PersonEntry {
            name,
            kind: PersonKind {
                hair: hair.map(Hair::from_str).transpose().unwrap(),
                people,
                tone,
            },
            variant: PersonVariant {
                full_name,
                grapheme,
            },
        }
    }
}
