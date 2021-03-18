//!
//! Implements parsing of attributes.
//!

use super::*;

impl FromStr for Tone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(Self::Light),
            "medium-light" => Ok(Self::MediumLight),
            "medium" => Ok(Self::Medium),
            "medium-dark" => Ok(Self::MediumDark),
            "dark" => Ok(Self::Dark),
            _ => Err(()),
        }
    }
}

pub struct ParsedOneOrTwo(pub Option<OneOrTwo>);
impl From<Option<OneOrTwo>> for ParsedOneOrTwo {
    fn from(o: Option<OneOrTwo>) -> Self {
        ParsedOneOrTwo(o)
    }
}
impl From<ParsedOneOrTwo> for Option<OneOrTwo> {
    fn from(p: ParsedOneOrTwo) -> Self {
        p.0
    }
}
impl FromStr for ParsedOneOrTwo {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = input
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        let strs: Vec<_> = parts.iter().map(|s| s.as_str()).collect();

        match strs.as_slice() {
            ["person"] => Ok(None.into()),
            ["people"] => Ok(None.into()),
            ["man"] => Ok(Some(Gender::Male.into()).into()),
            ["men"] => Ok(Some(Pair::Males.into()).into()),
            ["woman"] => Ok(Some(Gender::Female.into()).into()),
            ["women"] => Ok(Some(Pair::Females.into()).into()),
            ["person", "person"] => Ok(None.into()),
            ["man", "man"] => Ok(Some(Pair::Males.into()).into()),
            ["man", "woman"] => Ok(Some(Pair::Mixed.into()).into()),
            ["woman", "man"] => Ok(Some(Pair::Mixed.into()).into()),
            ["woman", "woman"] => Ok(Some(Pair::Females.into()).into()),
            _ => Err(()),
        }
    }
}

pub struct ParsedOneOrTwoChildren(pub Option<OneOrTwo>);
impl From<Option<OneOrTwo>> for ParsedOneOrTwoChildren {
    fn from(o: Option<OneOrTwo>) -> Self {
        ParsedOneOrTwoChildren(o)
    }
}
impl From<ParsedOneOrTwoChildren> for Option<OneOrTwo> {
    fn from(p: ParsedOneOrTwoChildren) -> Self {
        p.0
    }
}
impl FromStr for ParsedOneOrTwoChildren {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = input
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        let strs: Vec<_> = parts.iter().map(|s| s.as_str()).collect();

        match strs.as_slice() {
            ["child"] => Ok(None.into()),
            ["children"] => Ok(None.into()),
            ["boy"] => Ok(Some(Gender::Male.into()).into()),
            ["boys"] => Ok(Some(Pair::Males.into()).into()),
            ["girl"] => Ok(Some(Gender::Female.into()).into()),
            ["girls"] => Ok(Some(Pair::Females.into()).into()),
            ["child", "child"] => Ok(None.into()),
            ["boy", "boy"] => Ok(Some(Pair::Males.into()).into()),
            ["boy", "girl"] => Ok(Some(Pair::Mixed.into()).into()),
            ["girl", "boy"] => Ok(Some(Pair::Mixed.into()).into()),
            ["girl", "girl"] => Ok(Some(Pair::Females.into()).into()),
            _ => Err(()),
        }
    }
}

impl FromStr for Hair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "beard" => Ok(Self::Beard),
            "blond" => Ok(Self::Blond),
            "red" => Ok(Self::Red),
            "curly" => Ok(Self::Curly),
            "white" => Ok(Self::White),
            "bald" => Ok(Self::Bald),
            _ => Err(()),
        }
    }
}
