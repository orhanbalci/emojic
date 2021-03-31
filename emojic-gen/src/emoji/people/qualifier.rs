//!
//! Impls to check whether a supported set of an attribute is present in a emoji.
//!
//! Qualified means here that all (or a supported subset) emoji variants are cover or only a single
//! one. This qualification is done per attribute, thus each level of this type covers a single
//! attribute.
//!

use std::collections::HashMap;

use super::groups::PersonKind;
use super::groups::PersonKindSelector;
use super::PersonVariant;
use crate::emoji::emoji_render_single_example;
use crate::emoji::Gender;
use crate::emoji::Hair;
use crate::emoji::OneOrTwo;
use crate::emoji::Pair;
use crate::emoji::Tone;

impl OneOrTwo {
    fn fancy_source(self) -> String {
        match self {
            Self::One(g) => format!("Gender::{:?}", g),
            Self::Two(p) => format!("Pair::{:?}", p),
        }
    }
    fn fancy_full_source(self) -> String {
        match self {
            Self::One(g) => format!("OneOrTwo::One(Gender::{:?})", g),
            Self::Two(p) => format!("OneOrTwo::Two(Pair::{:?})", p),
        }
    }
}

/// Specifies a qualified set of an attribute.
pub struct QualifierSet<T> {
    set: Vec<T>,
    type_name: &'static str,
    const_access_generator: Box<dyn Fn(T) -> String>,
    access_generator: Box<dyn Fn(T) -> String>,
    kind_translator: Option<Box<dyn Fn(PersonKindSelector) -> PersonKindSelector>>,
}

impl<T> QualifierSet<T> {
    fn new(
        set: Vec<T>,
        type_name: &'static str,
        const_access_generator: Box<dyn Fn(T) -> String>,
        access_generator: Box<dyn Fn(T) -> String>,
    ) -> Self {
        QualifierSet {
            set,
            type_name,
            access_generator,
            const_access_generator,
            kind_translator: None,
        }
    }
    fn with_translator(
        set: Vec<T>,
        type_name: &'static str,
        const_access_generator: Box<dyn Fn(T) -> String>,
        access_generator: Box<dyn Fn(T) -> String>,
        kind_translator: Box<dyn Fn(PersonKindSelector) -> PersonKindSelector>,
    ) -> Self {
        QualifierSet {
            set,
            type_name,
            access_generator,
            const_access_generator,
            kind_translator: Some(kind_translator),
        }
    }
}

/// Qualifies a set of emojis base on an attribute (e.g. checking for a complete set of a attribute is present).
pub trait Qualifier: Sized + Clone {
    /// Extracts this qualifier attribute from the given selector
    fn extract_selector(sel: &mut PersonKindSelector) -> &mut Option<Option<Self>>;

    /// Returns a specification of valid sets of this attribute
    fn supported_sets() -> Vec<QualifierSet<Self>>;

    /// Sets this attribute in the given selector
    fn set_selector(
        mut sel: PersonKindSelector,
        value: Option<Option<Self>>,
    ) -> PersonKindSelector {
        *Self::extract_selector(&mut sel) = value;
        sel
    }

    /// Retrieves this attribute from the given selector
    fn get_selector(mut sel: PersonKindSelector) -> Option<Option<Self>> {
        Self::extract_selector(&mut sel).clone()
    }

    /// Validates the given set of selectors whether they constitute a qualified set for this attribute.
    fn validate(
        gen: PersonKindSelector,
        mut entries: HashMap<PersonKindSelector, PersonQualified>,
    ) -> Vec<(PersonKindSelector, PersonQualified)> {
        let allow_sets = Self::supported_sets();

        for QualifierSet {
            set,
            type_name,
            access_generator,
            const_access_generator,
            kind_translator,
        } in allow_sets
        {
            if set
                .into_iter()
                .all(|t| entries.contains_key(&Self::set_selector(gen, Some(Some(t)))))
            {
                // Join entries
                let def_selector = Self::set_selector(gen, Some(None));

                let def = entries.remove(&def_selector).map(Box::new);
                let subs_iter = entries.into_iter();
                let mut subs: Vec<_> = {
                    if let Some(translator) = kind_translator {
                        subs_iter.map(|(k, v)| (translator(k), v)).collect()
                    } else {
                        subs_iter.collect()
                    }
                };
                subs.sort_by_key(|(k, _v)| *k);
                let subs = subs
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            const_access_generator(Self::get_selector(k).unwrap().unwrap()),
                            access_generator(Self::get_selector(k).unwrap().unwrap()),
                            v,
                        )
                    })
                    .collect();

                let node = PersonQualifiedNode {
                    def,
                    subs,
                    type_name,
                };

                return vec![(gen, node.into())];
            }
        }

        // Default case
        entries.into_iter().collect()
    }

    fn qualify(
        subs: impl IntoIterator<Item = (PersonKindSelector, PersonQualified)>,
    ) -> Vec<(PersonKindSelector, PersonQualified)> {
        let mut grouping: HashMap<_, HashMap<_, _>> = HashMap::new();

        for k in subs {
            let gen = Self::set_selector(k.0, None);
            grouping.entry(gen).or_default().insert(k.0, k.1);
        }

        grouping
            .into_iter()
            .flat_map(|(gen, set)| Self::validate(gen, set))
            .collect()
    }
}

impl Qualifier for (Tone, Option<Tone>) {
    fn extract_selector(sel: &mut PersonKindSelector) -> &mut Option<Option<Self>> {
        &mut sel.tone
    }

    fn supported_sets() -> Vec<QualifierSet<Self>> {
        let only_primary = Tone::ALL.iter().map(|t| (*t, None)).collect();
        let with_secondary = Tone::ALL
            .iter()
            .flat_map(|t| {
                Tone::ALL
                    .iter()
                    .map(|s| (*t, (t != s).then(|| *s)))
                    .collect::<Vec<_>>()
            })
            .collect();
        let reduced = Tone::ALL
            .iter()
            .flat_map(|t| {
                Tone::ALL
                    .iter()
                    .filter(|s| t < s)
                    .map(|s| (*t, Some(*s)))
                    .collect::<Vec<_>>()
            })
            .collect();

        vec![
            QualifierSet::with_translator(
                with_secondary,
                "TonePair",
                Box::new(|(a, b_opt): Self| {
                    let b = b_opt.unwrap();
                    format!(
                        "tone_pair(TonePair{{left: Tone::{:?}, right: Tone::{:?} }})",
                        a, b
                    )
                }),
                Box::new(|(a, b_opt): Self| {
                    let b = b_opt.unwrap();
                    if a != b {
                        format!("tone((Tone::{:?}, Tone::{:?}))", a, b)
                    } else {
                        format!("tone(Tone::{:?})", a)
                    }
                }),
                Box::new(|sel| {
                    let me = Self::get_selector(sel);
                    if let Some(Some(m)) = me {
                        if m.1.is_none() {
                            let m2 = (m.0, Some(m.0));
                            Self::set_selector(sel, Some(Some(m2)))
                        } else {
                            sel
                        }
                    } else {
                        sel
                    }
                }),
            ),
            QualifierSet::new(
                // Does not exist as of Unicode 13.1
                reduced,
                "TonePairReduced",
                Box::new(|(a, b_opt): Self| {
                    if let Some(b) = b_opt {
                        format!(
                            "tone_pair(TonePair{{left: Tone::{:?}, right: Tone::{:?} }})",
                            a, b
                        )
                    } else {
                        format!(
                            "tone_pair(TonePair{{left: Tone::{:?}, right: Tone::{:?} }})",
                            a, a
                        )
                    }
                }),
                Box::new(|(a, b_opt): Self| {
                    if let Some(b) = b_opt {
                        if a != b {
                            format!("tone((Tone::{:?}, Tone::{:?}))", a, b)
                        } else {
                            format!("tone(Tone::{:?})", a)
                        }
                    } else {
                        format!("tone(Tone::{:?})", a)
                    }
                }),
            ),
            QualifierSet::new(
                only_primary,
                "Tone",
                Box::new(|(a, b_opt): Self| {
                    debug_assert!(b_opt.is_none());
                    format!("tone(Tone::{:?})", a)
                }),
                Box::new(|(a, b_opt): Self| {
                    debug_assert!(b_opt.is_none());
                    format!("tone(Tone::{:?})", a)
                }),
            ),
        ]
    }
}

impl Qualifier for (OneOrTwo, Option<OneOrTwo>) {
    fn extract_selector(sel: &mut PersonKindSelector) -> &mut Option<Option<Self>> {
        &mut sel.people
    }

    fn supported_sets() -> Vec<QualifierSet<Self>> {
        let pseudo_one = Gender::ALL
            .iter()
            .map(|t| (OneOrTwo::Two((*t, *t).into()), None))
            .collect();
        let only_one = Gender::ALL
            .iter()
            .map(|t| (OneOrTwo::One(*t), None))
            .collect();
        let only_two = Pair::ALL
            .iter()
            .map(|t| (OneOrTwo::Two(*t), None))
            .collect();
        let only_primary = OneOrTwo::ALL.iter().map(|t| (*t, None)).collect();
        let with_secondary = OneOrTwo::ALL
            .iter()
            .flat_map(|t| {
                OneOrTwo::ALL
                    .iter()
                    .map(|s| (*t, Some(*s)))
                    .collect::<Vec<_>>()
            })
            .collect();

        vec![
            QualifierSet::new(
                with_secondary,
                "Family",
                Box::new(|(a, b_opt): Self| {
                    let b = b_opt.unwrap();
                    format!(
                        "family(Family{{parents: {}, children: {} }})",
                        a.fancy_full_source(),
                        b.fancy_full_source()
                    )
                }),
                Box::new(|(a, b_opt): Self| {
                    let b = b_opt.unwrap();
                    format!("gender(({}, {}))", a.fancy_source(), b.fancy_source())
                }),
            ),
            QualifierSet::new(
                // actually, this should not appear within Unicode 13.1
                only_primary,
                "OneOrTwo",
                Box::new(|(a, b_opt): Self| {
                    debug_assert!(b_opt.is_none());
                    format!("one_or_more({:?})", a.fancy_full_source())
                }),
                Box::new(|(a, b_opt): Self| {
                    debug_assert!(b_opt.is_none());
                    format!("gender({:?})", a.fancy_source())
                }),
            ),
            QualifierSet::new(
                only_two,
                "Pair",
                Box::new(|(a, b_opt): Self| {
                    debug_assert!(b_opt.is_none());
                    if let OneOrTwo::Two(p) = a {
                        format!("pair(Pair::{:?})", p)
                    } else {
                        panic!("Found single person emoji in a 'only_two' emoji")
                    }
                }),
                Box::new(|(a, b_opt): Self| {
                    debug_assert!(b_opt.is_none());
                    if let OneOrTwo::Two(p) = a {
                        format!("gender(Pair::{:?})", p)
                    } else {
                        panic!("Found single person emoji in a 'only_two' emoji")
                    }
                }),
            ),
            QualifierSet::new(
                only_one,
                "Gender",
                Box::new(|(a, b_opt): Self| {
                    debug_assert!(b_opt.is_none());
                    if let OneOrTwo::One(g) = a {
                        format!("gender(Gender::{:?})", g)
                    } else {
                        panic!("Found multi person emoji in a 'only_one' emoji")
                    }
                }),
                Box::new(|(a, b_opt): Self| {
                    debug_assert!(b_opt.is_none());
                    if let OneOrTwo::One(g) = a {
                        format!("gender(Gender::{:?})", g)
                    } else {
                        panic!("Found multi person emoji in a 'only_one' emoji")
                    }
                }),
            ),
            QualifierSet::new(
                pseudo_one,
                "Gender",
                Box::new(|(a, b_opt): Self| {
                    debug_assert!(b_opt.is_none());
                    match a {
                        OneOrTwo::Two(Pair::Males) => "gender(Gender::Male)".to_string(),
                        OneOrTwo::Two(Pair::Females) => "gender(Gender::Female)".to_string(),
                        _ => panic!("Found other person emoji in a 'pseudo_one' emoji"),
                    }
                }),
                Box::new(|(a, b_opt): Self| {
                    debug_assert!(b_opt.is_none());
                    match a {
                        OneOrTwo::Two(Pair::Males) => "gender(Gender::Male)".to_string(),
                        OneOrTwo::Two(Pair::Females) => "gender(Gender::Female)".to_string(),
                        _ => panic!("Found other person emoji in a 'pseudo_one' emoji"),
                    }
                }),
            ),
        ]
    }
}

impl Qualifier for Hair {
    fn extract_selector(sel: &mut PersonKindSelector) -> &mut Option<Option<Self>> {
        &mut sel.hair
    }

    fn supported_sets() -> Vec<QualifierSet<Self>> {
        vec![QualifierSet::new(
            Self::ALL.to_vec(),
            "Hair",
            Box::new(|h: Self| format!("hair(Hair::{:?})", h)),
            Box::new(|h: Self| format!("hair(Hair::{:?})", h)),
        )]
    }
}

/// Represents a well qualified set of emoji variants.
///
/// Qualified means here that all (or a supported subset) emoji variants are cover or only a single
/// one. This qualification is done per attribute, thus each level of this type covers a single
/// attribute.
///
/// Emoji variants are represented by [`PersonKind`].
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PersonQualified {
    Node(PersonQualifiedNode),
    Leaf(PersonQualifiedLeaf),
}
impl PersonQualified {
    pub fn to_type_n_value(
        &self,
        accessor: &str,
        variants: &HashMap<PersonKind, PersonVariant>,
    ) -> (String, String, String) {
        match self {
            Self::Node(n) => n.to_type_n_value(accessor, variants),
            Self::Leaf(l) => l.to_type_n_value(accessor, variants),
        }
    }

    pub fn to_accessor_n_kind(&self, identifier: &str) -> Vec<(String, String, PersonKind)> {
        self.to_accessor_n_kind_internal(identifier, identifier)
    }

    fn to_accessor_n_kind_internal(
        &self,
        const_accessor: &str,
        pub_accessor: &str,
    ) -> Vec<(String, String, PersonKind)> {
        match self {
            Self::Node(n) => n.to_accessor_n_kind(const_accessor, pub_accessor),
            Self::Leaf(l) => l.to_accessor_n_kind(const_accessor, pub_accessor),
        }
    }
}
impl From<PersonQualifiedNode> for PersonQualified {
    fn from(node: PersonQualifiedNode) -> Self {
        PersonQualified::Node(node)
    }
}
impl From<PersonKind> for PersonQualified {
    fn from(leaf: PersonKind) -> Self {
        PersonQualified::Leaf(PersonQualifiedLeaf { leaf })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersonQualifiedNode {
    pub def: Option<Box<PersonQualified>>,
    pub subs: Vec<(String, String, PersonQualified)>,
    pub type_name: &'static str,
}
impl PersonQualifiedNode {
    fn to_type_n_value(
        &self,
        accessor: &str,
        variants: &HashMap<PersonKind, PersonVariant>,
    ) -> (String, String, String) {
        // Process some super group

        let mut value = String::new();
        let mut ty = String::new();
        let mut docs = String::new();

        let ty_name = format!("With{}", if self.def.is_none() { "NoDef" } else { "" });

        let mut sub_types = Vec::new();

        if let Some(def) = &self.def {
            let (def_type, def_value, def_doc) = def.to_type_n_value(accessor, variants);
            docs.push_str(&def_doc);
            sub_types.push(def_type);
            value.push_str(&format!("{}::new({}, \n\t&[\n\t\t", ty_name, def_value));
        } else {
            value.push_str(&format!("{}::new(\n\t&[\n\t\t", ty_name));
        }

        for (_const_acc, pub_acc, sub) in &self.subs {
            let sub_accessor = format!("{}.{}", accessor, pub_acc);
            let (inner_ty, inner_value, inner_doc) = sub.to_type_n_value(&sub_accessor, variants);
            docs.push_str(&inner_doc);
            sub_types.push(inner_ty);

            value.push_str(&inner_value);
            value.push_str(",\n\t");
        }
        value.push_str("])");

        let sub_type = sub_types[0].as_str();

        // The type should match otherwise, we get invalid code
        assert!(
            sub_types.iter().all(|st| st == sub_type),
            "sub types differ for emoji {:?}: {:?} != {:?}",
            accessor,
            sub_type,
            sub_types.iter().find(|st| *st != sub_type)
        );

        ty.push_str(&format!("{}<{},", ty_name, self.type_name));
        ty.push_str(&sub_type);
        ty.push('>');

        (ty, value, docs)
    }

    fn to_accessor_n_kind(
        &self,
        const_accessor: &str,
        pub_accessor: &str,
    ) -> Vec<(String, String, PersonKind)> {
        // Process some super group

        let mut list = Vec::new();

        if let Some(def) = &self.def {
            let sub_const_accessor = format!("{}.default", const_accessor);
            list.extend(def.to_accessor_n_kind_internal(&sub_const_accessor, pub_accessor));
        }

        for (const_acc, pub_acc, sub) in &self.subs {
            let sub_const_accessor = format!("{}.{}", const_accessor, const_acc);
            let sub_pub_accessor = format!("{}.{}", pub_accessor, pub_acc);
            list.extend(sub.to_accessor_n_kind_internal(&sub_const_accessor, &sub_pub_accessor));
        }

        list
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersonQualifiedLeaf {
    pub leaf: PersonKind,
}
impl PersonQualifiedLeaf {
    fn to_type_n_value(
        &self,
        accessor: &str,
        variants: &HashMap<PersonKind, PersonVariant>,
    ) -> (String, String, String) {
        // Process the final fully qualified kind

        debug_assert!(
            variants.contains_key(&self.leaf),
            "Variant not found: {:?} for {:?}, all: {:?}",
            self.leaf,
            accessor,
            variants
        );
        let variant = &variants[&self.leaf];

        (
            "Emoji".to_string(),
            format!(
                r#"Emoji::new({:?}, {:?},"{}")"#,
                variant.full_name, variant.since, variant.grapheme
            ),
            emoji_render_single_example(&accessor, &variant.grapheme),
        )
    }
    fn to_accessor_n_kind(
        &self,
        const_accessor: &str,
        pub_accessor: &str,
    ) -> Vec<(String, String, PersonKind)> {
        // Process the final fully qualified kind

        vec![(const_accessor.into(), pub_accessor.into(), self.leaf)]
    }
}
