use super::strutil::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::string::ToString;

// Use the same attribute definition as in the public API of the main crate:
#[path = "../../src/emojis/attributes.rs"]
mod attributes;
use attributes::*;

mod people;
use people::*;

lazy_static! {
    /// Parses lines form the unicode Emoji test list as specified in [`crate::EMOJI_URL`]
    ///
    /// Should be compatible with Emoji 12.0, 12.1, 13.0, and 13.1
    ///
    /// Sample input:
    /// ```
    /// 1F3CC 1F3FB 200D 2642 FE0F                 ; fully-qualified     # 🏌🏻‍♂️ E4.0 man golfing: light skin tone
    /// ```
    ///
    /// Output:
    /// code: `1F3CC 1F3FB 200D 2642 FE0F`
    /// version: `4.0`
    /// name: `man golfing: light skin tone`
    static ref REMOJI: Regex = Regex::new(
        r"^(?m)\s*(?P<code>[A-Z\d ]+[A-Z\d])\s+;\s+(:?fully-qualified|component)\s+#\s+\S+\s+(?:E(?P<version>\d+\.\d+)\s+)?(?P<name>.+)$"
    )
    .unwrap();

    /// Matches strings containing at least one person, optionally mix with an activity,
    /// more people, and a colon separated list of skin tones and hair styles.
    ///
    /// ```
    /// man teacher
    /// old man: light skin tone
    /// person: blond hair
    /// men holding hands: medium skin tone, dark skin tone
    /// ```
    static ref PERSON_WITH_ACTIVITY: Regex = Regex::new(r"^\s*(?:(?P<activity_pre>[^:\n]+) )??(?P<adult_left>person|woman|man|people|men|women)(?:(?:, | and )(?P<adult_right>person|woman|man))?(?:,? ?(?P<child_left>child|boy|girl)(?:,? (?P<child_right>child|boy|girl))?)?(?: (?P<activity_post>[^:\n]+))?(?::(?:,? (?P<skin_first>(?:medium-)?(?:light|medium|dark)) skin tone(?:,? (?P<skin_sec>(?:medium-)?(?:light|medium|dark)) skin tone)?)?(?:,? (?P<hair>bald|beard|blond|red|curly|white)(?: hair)?)?)?\s*$").unwrap();

    /// Matches strings containing any activity with a colon separated list of people, skin tones
    /// and/or hair stiles.
    ///
    /// Notice, for best result input string should be first matched against `PERSON_WITH_ACTIVITY`
    ///
    /// ```
    /// kiss: light skin tone
    /// family: man, woman, boy
    /// kiss: person, person, light skin tone, medium-light skin tone
    /// couple with heart: woman, man
    /// ```
    static ref ACTIVITY_WITH_COLON: Regex = Regex::new(r"^\s*(?P<activity>[^:\n]+):(?: (?P<adult_left>person|woman|man))?(?:(?:, | and )(?P<adult_right>person|woman|man))?(?:, (?P<child_left>child|boy|girl)(?:, (?P<child_right>child|boy|girl))?)?(?:,? (?P<skin_first>(?:medium-)?(?:light|medium|dark)) skin tone(?:,? (?P<skin_sec>(?:medium-)?(?:light|medium|dark)) skin tone)?)?(:?,? (?P<hair>bald|beard|blond|red|curly|white)(?: hair)?)?\s*$").unwrap();


}

impl FromStr for Version {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [major, minor] = s.split('.').collect::<Vec<_>>().as_slice() {
            Ok(Version(
                FromStr::from_str(major)?,
                FromStr::from_str(minor)?,
            ))
        } else {
            panic!("Invalid version string format");
        }
    }
}

fn person_identifier(activity_pre: Option<&str>, activity_post: Option<&str>) -> String {
    format!(
        "{}person{}",
        if let Some(s) = activity_pre {
            format!("{} ", s)
        } else {
            "".to_string()
        },
        if let Some(s) = activity_post {
            format!(" {}", s)
        } else {
            "".to_string()
        }
    )
}

#[derive(Default)]
pub struct Emojis {
    pub groups: Vec<Group>,
}

impl Emojis {
    pub fn append(&mut self, name: String) -> Option<&mut Group> {
        let g = Group::new(name);
        self.groups.push(g);
        self.groups.last_mut()
    }
    pub fn get_group(&mut self, gn: &str) -> Option<&mut Group> {
        self.groups.iter_mut().find(|g| (*g).name == gn)
    }
    /// Sort the groups.
    pub fn sort(&mut self) {
        self.groups.sort_by_key(|g| g.name.clone());
        for g in self.groups.iter_mut() {
            g.sort();
        }
    }
}

pub struct Group {
    pub name: String,
    pub identifier: String,
    pub subgroups: Vec<Subgroup>,
}

impl Group {
    pub fn new(name: String) -> Self {
        Group {
            identifier: generate_module(&name),
            name,
            subgroups: Vec::new(),
        }
    }
    pub fn append(&mut self, subgroup: String) -> Option<&mut Subgroup> {
        let sg = Subgroup::new(subgroup);
        self.subgroups.push(sg);
        self.subgroups.last_mut()
    }
    pub fn get_subgroup(&mut self, sgn: &str) -> Option<&mut Subgroup> {
        self.subgroups.iter_mut().find(|g| (*g).name == sgn)
    }
    /// Sort the subgroups.
    pub fn sort(&mut self) {
        self.subgroups.sort_by_key(|s| s.name.clone());
        for s in self.subgroups.iter_mut() {
            s.sort();
        }
    }
}

pub struct Subgroup {
    pub name: String,
    pub identifier: String,
    pub emojis: HashMap<String, Emoji>,
    pub constants: Vec<String>,
    pub person_emojis: HashMap<String, PersonEmoji>,
}

impl Subgroup {
    fn new(name: String) -> Self {
        Subgroup {
            identifier: generate_module(&name),
            name,
            emojis: HashMap::new(),
            constants: Vec::new(),
            person_emojis: HashMap::new(),
        }
    }
    pub fn get_emoji(&self, identifier: &str) -> Option<&dyn ToSourceCode> {
        self.emojis
            .get(identifier)
            .map(|e| e as &dyn ToSourceCode)
            .or_else(|| {
                self.person_emojis
                    .get(identifier)
                    .map(|e| e as &dyn ToSourceCode)
            })
    }
    /// Sort the emojis.
    pub fn sort(&mut self) {
        // Try to combine standalone emoji with persons
        for (k, p) in self.person_emojis.iter_mut() {
            let shorten = k.replace("_PERSON", "").replace("PERSON_", "");
            if let Some(e) = self.emojis.remove(k) {
                println!("Join emoji {} with {}", p.identifier, e.identifier);
                let res = p.variants.insert(Default::default(), e.into());

                debug_assert!(res.is_none(), "Emoji had already a default: {:?}", res);
            } else if let Some(e) = self.emojis.remove(&shorten) {
                println!("Join emoji {} with {}", p.identifier, e.identifier);
                p.identifier = shorten;
                let res = p.variants.insert(Default::default(), e.into());

                debug_assert!(res.is_none(), "Emoji had already a default: {:?}", res);
            }
        }

        // Try to combine persons (required to join e.g. TEACHER with MAN_TEACHER & WOMAN_TEACHER)
        let mut to_merge = Vec::new();
        for k in self.person_emojis.keys() {
            let shorten = k.replace("_PERSON", "").replace("PERSON_", "");
            if shorten != *k && !shorten.is_empty() && self.person_emojis.contains_key(&shorten) {
                println!("Merge person {} with {}", k, shorten);
                to_merge.push((shorten, k.clone()));
            }
        }
        for (short, k) in to_merge {
            let old = self.person_emojis.remove(&k).unwrap().variants;
            let new = &mut self.person_emojis.get_mut(&short).unwrap().variants;
            for (k, v) in old {
                let res = new.insert(k, v);
                debug_assert!(
                    res.is_none(),
                    "Person had already a given variant: {:?} from {:?} to {:?}",
                    res,
                    k,
                    short
                );
            }
        }

        // Do consistency checks and split insufficient emojis
        let mut to_add = Vec::new();
        for (_k, p) in self.person_emojis.drain() {
            to_add.extend(p.scrub());
        }
        for e in to_add {
            self.person_emojis.insert(e.identifier.clone(), e);
        }

        // Clean up constants and sort
        self.constants.clear();
        self.constants.extend(self.emojis.keys().cloned());
        self.constants.extend(self.person_emojis.keys().cloned());
        self.constants.sort();
        self.constants.dedup();
    }
    /// Returns the list of emojis as ordered iterator.
    pub fn emoji_iter(&self) -> impl Iterator<Item = &dyn ToSourceCode> {
        self.constants.iter().map(move |id| {
            self.emojis
                .get(id)
                .map(|e| e as &dyn ToSourceCode)
                .or_else(|| self.person_emojis.get(id).map(|e| e as &dyn ToSourceCode))
                .unwrap()
        })
    }
    fn append_person(&mut self, entry: PersonEntry) {
        let name = entry.name.clone();
        let res = self
            .person_emojis
            .entry(generate_constant(&entry.name))
            .or_insert_with(|| PersonEmoji::new(name))
            .variants
            .insert(entry.kind, entry.variant);

        debug_assert!(
            res.is_none(),
            "Person had already a given variant: {:?} of {:?}",
            entry.kind,
            entry.name
        );
    }
    pub fn append_line(&mut self, line: &str) {
        let line = line.trim();
        if line.is_empty() {
            return;
        }
        if let Some(cap) = REMOJI.captures(&line) {
            println!(
                "Captures: {:?}, {:?}, {:?}",
                cap.name("code").map(|s| s.as_str()), // unconditional
                cap.name("version").map(|s| s.as_str()),
                cap.name("name").map(|s| s.as_str()), // unconditional
            );
            let version = cap
                .name("version")
                .map(|s| s.as_str())
                .map(FromStr::from_str)
                .map(Result::ok)
                .flatten()
                .unwrap_or(Version(0, 0));

            let code = cap["code"].to_owned();
            let name = cap["name"].to_owned();

            if let Some(pcap) = PERSON_WITH_ACTIVITY.captures(&name) {
                println!(
						"Found PERSON_WITH_ACTIVITY: {:?}&{:?} ({:?},{:?}|{:?},{:?}): ({:?},{:?}), {:?}",
						&pcap.name("activity_pre").map(|s| s.as_str()),
						&pcap.name("activity_post").map(|s| s.as_str()),
						&pcap.name("adult_left").map(|s| s.as_str()), // unconditional
						&pcap.name("adult_right").map(|s| s.as_str()),
						&pcap.name("child_left").map(|s| s.as_str()),
						&pcap.name("child_right").map(|s| s.as_str()),
						&pcap.name("skin_first").map(|s| s.as_str()),
						&pcap.name("skin_sec").map(|s| s.as_str()),
						&pcap.name("hair").map(|s| s.as_str()),
					);

                let grapheme = generate_unicode(&code);
                let person = PersonEntry::parse(
                    name.clone(),
                    grapheme,
                    person_identifier(
                        pcap.name("activity_pre").map(|s| s.as_str()),
                        pcap.name("activity_post").map(|s| s.as_str()),
                    ),
                    version,
                    Some((
                        &pcap["adult_left"],
                        pcap.name("adult_right").map(|s| s.as_str()),
                        pcap.name("child_left")
                            .map(|s| (s.as_str(), pcap.name("child_right").map(|s| s.as_str()))),
                    )),
                    pcap.name("skin_first")
                        .map(|s| (s.as_str(), pcap.name("skin_sec").map(|s| s.as_str()))),
                    pcap.name("hair").map(|s| s.as_str()),
                );
                self.append_person(person);
            } else if let Some(pcap) = ACTIVITY_WITH_COLON.captures(&name) {
                println!(
                    "Found ACTIVITY_WITH_COLON: {:?} ({:?},{:?}|{:?},{:?}): ({:?},{:?}), {:?}",
                    &pcap.name("activity").map(|s| s.as_str()), // unconditional
                    &pcap.name("adult_left").map(|s| s.as_str()),
                    &pcap.name("adult_right").map(|s| s.as_str()),
                    &pcap.name("child_left").map(|s| s.as_str()),
                    &pcap.name("child_right").map(|s| s.as_str()),
                    &pcap.name("skin_first").map(|s| s.as_str()),
                    &pcap.name("skin_sec").map(|s| s.as_str()),
                    &pcap.name("hair").map(|s| s.as_str()),
                );

                let grapheme = generate_unicode(&code);
                let person = PersonEntry::parse(
                    name.clone(),
                    grapheme,
                    pcap["activity"].to_string(),
                    version,
                    pcap.name("adult_left").map(|s| s.as_str()).map(|left| {
                        (
                            left,
                            pcap.name("adult_right").map(|s| s.as_str()),
                            pcap.name("child_left").map(|s| {
                                (s.as_str(), pcap.name("child_right").map(|s| s.as_str()))
                            }),
                        )
                    }),
                    pcap.name("skin_first")
                        .map(|s| s.as_str())
                        .zip(Some(pcap.name("skin_sec").map(|s| s.as_str()))),
                    pcap.name("hair").map(|s| s.as_str()),
                );
                self.append_person(person);
            } else {
                println!("Not a person {:?}", name);

                let id = generate_constant(&extract_attr(&name));

                let e = Emoji {
                    identifier: id.clone(),
                    name,
                    since: version,
                    grapheme: generate_unicode(&code),
                };

                if self.emojis.insert(id.clone(), e).is_none() {
                    self.constants.push(id.clone());
                } else {
                    eprintln!("Found emoji twice: {}", id);
                }
            }
        } else {
            eprintln!("Can not match line: {:?}", line);
        }
    }
}

fn extract_attr(name: &str) -> String {
    let spv: Vec<&str> = name.split(':').collect();

    if spv.len() < 2 {
        return name.to_string();
    }
    let mut c = spv[0].to_string();
    let attr = spv[1].split(',');
    #[allow(clippy::if_same_then_else)]
    attr.for_each(|attribute| {
        if attribute.contains("tone") {
            c.push_str(" with ");
            c.push_str(attribute);
        } else if attribute.contains("hair") {
            c.push_str(" with ");
            c.push_str(attribute);
        } else if attribute.contains("flag") {
            c.push_str(" for ");
            c.push_str(attribute);
        } else {
            c.push(' ');
            c.push_str(attribute);
        }
    });

    c
}
fn generate_unicode(code: &str) -> String {
    //println!("{}", self.code);
    let mut unicodes = String::new();

    code.split(' ').for_each(|c| {
        //println!("{}", c);
        let without_prefix = c.trim_start_matches("0x");
        let z = u32::from_str_radix(without_prefix, 16);
        match z {
            Ok(v) => unicodes.push(char::from_u32(v).unwrap()),
            Err(s) => println!("Invalid Unicode token: {:?}", s),
        }
    });

    unicodes
}

/// Represents a standalone emoji.
#[derive(Clone, Debug)]
pub struct Emoji {
    pub name: String,
    pub identifier: String,
    pub since: Version,
    pub grapheme: String,
}

impl ToSourceCode for Emoji {
    fn to_source_code(&self) -> String {
        let basic = &self;

        // Generate docs for the statics
        let docs = {
            // Single emoji
            format!(
                r#"#[doc="{} {}"]#[doc=""]#[doc="Since E{}"]#[doc=""]{}
"#,
                basic.name,
                basic.grapheme,
                basic.since,
                emoji_render_example_section(
                    &emoji_render_single_example(&basic.identifier, &basic.grapheme),
                    &basic.identifier
                )
            )
        };

        format!(
            r#"{}pub static {} :  Emoji = Emoji::new({:?}, {:?}, "{}"); // {}"#,
            docs, basic.identifier, basic.name, basic.since, basic.grapheme, basic.name
        )
    }
    fn identifier(&self) -> &str {
        &self.identifier
    }
    fn graphemes(&self) -> String {
        self.grapheme.to_string()
    }
    fn default_grapheme(&self) -> Option<&str> {
        Some(&self.grapheme)
    }
    fn name(&self) -> &str {
        &self.name
    }
    /// Returns a list of all addressable emojis as a set of access string and grapheme.
    fn full_emoji_list(&self) -> Vec<(String, &str)> {
        vec![(self.identifier.clone(), &self.grapheme)]
    }
}

/// Represents an emoji that can be turned into source code.
pub trait ToSourceCode: std::fmt::Debug {
    /// Returns the source code for a static definition.
    fn to_source_code(&self) -> String;
    /// Returns the identifier string.
    fn identifier(&self) -> &str;
    /// Returns the default emoji graphemes (if there are multiple eligibles all are returned together.
    ///
    /// An example where there is no default is for the dancer emojis which consists of
    /// a male and female without a default.
    fn graphemes(&self) -> String;
    /// Return the default emoji grapheme if there is a unique one.
    ///
    /// An example where there is no default is for the dancer emojis which consists of
    /// a male and female without a default.
    fn default_grapheme(&self) -> Option<&str>;
    /// Returns the descriptive name of this emoji.
    fn name(&self) -> &str;
    /// Returns a list of all addressable emojis as a set of access string and grapheme.
    fn full_emoji_list(&self) -> Vec<(String, &str)>;
}

/// Returns a string containing the plain unicode grapheme as well as a list of the actual
/// unicode code points.
fn emoji_render_text(grapheme: &str) -> String {
    format!(
        "{} (`{}`)",
        grapheme,
        grapheme
            .chars()
            .map(|c| format!("U+{:04X}", c as u32))
            .collect::<Vec<_>>()
            .join(" ")
    )
}

/// Generates a string containing the rust docs for an examples section with code snipped.
///
/// The code snipped is filled with `content` which is supposed to come from
/// [`emoji_render_single_example`] or a concatenation of such strings.
fn emoji_render_example_section(content: &str, identifier: &str) -> String {
    format!(
        r##"#[doc="# Examples"] #[doc="```"]
#[doc="use emojic::flat::{};"]#[doc="use emojic::Tone;"]#[doc="use emojic::Gender;"]#[doc="use emojic::Hair;"]#[doc="use emojic::Pair;"] #[doc=""]
{}
#[doc="```"]"##,
        identifier, content
    )
}

/// Generates a single rust doc containing an example printout code of the given `accessor` which
/// is supposed to print the given `grapheme`.
///
/// This function will also inject an invisible `assert` as test case.
///
/// This rust code is supposed to be part of an code snipped, is generated by
/// [`emoji_render_example_section`].
fn emoji_render_single_example(accessor: &str, grapheme: &str) -> String {
    format!(
        r##"#[doc="println!(\"{{}}\", {}); // {}"] #[doc="# assert_eq!({}.to_string().as_str(), \"{}\");"]"##,
        accessor,
        emoji_render_text(grapheme),
        accessor,
        grapheme
    )
}
