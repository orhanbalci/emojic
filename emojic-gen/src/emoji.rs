use super::strutil::*;
use inflections::case::to_snake_case;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::string::ToString;

lazy_static! {
    static ref REMOJI: Regex = Regex::new(
        r"^(?m)(?P<code>[A-Z\d ]+[A-Z\d])\s+;\s+(fully-qualified|component)\s+#\s+.+\s+E\d+\.\d+ (?P<name>.+)$"
    )
    .unwrap();

    static ref RTONE: Regex = Regex::new(r":\s.*tone,?").unwrap();
}
#[non_exhaustive]
struct Tone;

impl Tone {
    pub const DEFAULT: &'static str = "";
    pub const LIGHT: &'static str = "\u{1F3FB}";
    pub const MEDIUM_LIGHT: &'static str = "\u{1F3FC}";
    pub const MEDIUM: &'static str = "\u{1F3FD}";
    pub const MEDIUM_DARK: &'static str = "\u{1F3FE}";
    pub const DARK: &'static str = "\u{1F3FF}";
    pub const TONE_PLACE_HOLDER: &'static str = "@";
}

pub struct Emojis {
    pub groups: Vec<Group>,
}

impl Emojis {
    pub fn append(&mut self, name: String) -> Option<&mut Group> {
        let g = Group {
            name: name,
            subgroups: Vec::new(),
        };
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
    pub subgroups: Vec<Subgroup>,
}

impl Group {
    pub fn append(&mut self, subgroup: String) -> Option<&mut Subgroup> {
        let sg = Subgroup {
            name: subgroup,
            emojis: HashMap::new(),
            constants: Vec::new(),
        };
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
    pub emojis: HashMap<String, Vec<Emoji>>,
    pub constants: Vec<String>,
}

impl Subgroup {
    pub fn append(&mut self, e: &Emoji) {
        if self.emojis.contains_key(&e.constant) {
            self.emojis.get_mut(&e.constant).unwrap().push(e.clone());
        } else {
            let mut nv = Vec::new();
            nv.push(e.clone());
            self.emojis.insert(e.constant.to_owned(), nv);
            self.constants.push(e.constant.to_owned());
        }
    }
    pub fn get_emoji(&self, code: &str) -> Option<&Vec<Emoji>> {
        self.emojis.get(code)
    }
    /// Sort the emojis.
    pub fn sort(&mut self) {
        self.constants.sort();
    }
    /// Returns the list of emojis as ordered iterator.
    pub fn emoji_iter(&self) -> impl Iterator<Item = &Vec<Emoji>> {
        self.constants.iter().map(move |c| &self.emojis[c])
    }
}
#[derive(Clone, Debug)]
pub struct Emoji {
    pub name: String,
    pub constant: String,
    pub code: String,
    pub tones: Vec<String>,
}

impl ToString for Emoji {
    fn to_string(&self) -> String {
        format!(
            "name:{}, constant:{}, code:{}, tones: {:?}\n",
            self.name, self.constant, self.code, self.tones
        )
    }
}

impl Emoji {
    pub fn new(mut line: String) -> Option<Emoji> {
        line.push_str("\n");
        if !REMOJI.is_match(&line) {
            println!("Can not match line {}", line);
        }
        REMOJI.captures(&line).and_then(|cap| {
            println!("{} Captures", cap.len());
            if cap.len() < 4 {
                return None;
            } else {
                let code = cap[1].to_owned();
                let name = cap[3].to_owned();
                let mut e = Emoji {
                    name: name.clone(),
                    constant: name,
                    code: code,
                    tones: Vec::new(),
                };
                e.extract_attr();
                e.generate_constant();
                e.generate_unicode();
                Some(e)
            }
        })
    }
    pub fn extract_attr(&mut self) {
        let constant = self.constant.clone();
        let spv: Vec<&str> = constant.split(":").collect();

        if spv.len() < 2 {
            return;
        }
        let mut c: String = spv[0].to_string();
        let attr = spv[1].split(",");
        attr.for_each(|attribute| {
            if attribute.contains("tone") {
                self.tones.push(attribute.to_string());
            } else if attribute.contains("hair") {
                c.push_str(" with ");
                c.push_str(attribute);
            } else if attribute.contains("flag") {
                c.push_str(" for ");
                c.push_str(attribute);
            } else {
                c.push_str(" ");
                c.push_str(attribute);
            }
        });
        self.constant = c;
    }
    pub fn generate_constant(&mut self) {
        let mut c = clean(self.constant.to_owned());
        c = to_snake_case(&c.to_lowercase()).to_uppercase();
        c = remove_spaces(c);
        self.constant = c;
    }
    pub fn generate_unicode(&mut self) {
        println!("{}", self.code);
        let mut unicodes = String::new();
        self.code.split(" ").for_each(|c| {
            println!("{}", c);
            let without_prefix = c.trim_start_matches("0x");
            let z = u32::from_str_radix(without_prefix, 16);
            match z {
                Ok(v) => unicodes.push(char::from_u32(v).unwrap()),
                Err(s) => println!("{}", s),
            }
        });
        self.code = unicodes;
    }
}

pub fn replace_tones(mut inp: String) -> String {
    let v = vec![
        Tone::LIGHT,
        Tone::MEDIUM_LIGHT,
        Tone::MEDIUM,
        Tone::MEDIUM_DARK,
        Tone::DARK,
    ];
    println!("replacing {}", inp);
    v.iter().for_each(|&t| {
        inp = inp.replace(t, Tone::TONE_PLACE_HOLDER);
    });
    inp
}

pub fn default_tone(basic: String, toned: String) -> String {
    let i = toned.find(Tone::TONE_PLACE_HOLDER.chars().next().unwrap());

    let res = basic
        .char_indices()
        .filter(|(ind, c)| *ind == i.unwrap_or_default() && *c == '\u{fe0f}')
        .map(|(_a, _b)| "\u{fe0f}")
        .next();

    match res {
        Some(r) => r.to_owned(),
        None => String::new(),
    }
}

/// Returns a string containing the plain unicode grapheme as well as a list of the actual
/// unicode code points.
fn emoji_render_text(emoji: &Emoji) -> String {
    format!(
        "{} ({})",
        emoji.code,
        emoji
            .code
            .chars()
            .map(|c| format!("U+{:04X}", c as u32))
            .collect::<Vec<_>>()
            .join(" ")
    )
}

pub fn emoji_constant_line(emos: &Vec<Emoji>) -> String {
    let basic = &emos[0];

    // Generate docs for the statics
    let docs = if emos.len() == 1 {
        // Single emoji
        format!(
            r#"#[doc="{} {}"]#[doc=""]#[doc="Rendered as: {}"]
"#,
            basic.name,
            basic.code,
            emoji_render_text(&basic),
        )
    } else {
        // Emoji list
        format!(
            r#"#[doc="{} {}"]#[doc=""]#[doc="Rendered as:"]{}
"#,
            basic.name,
            basic.code,
            emos.iter()
                .map(|e| { format!(r#"#[doc="- {}"]"#, emoji_render_text(e)) })
                .collect::<String>()
        )
    };

    //   println!("Emoji length {}", emos.len());
    match emos.len() {
        0 => {
            // Should not appear
            panic!("Found a emoji constant without emojis")
        }
        1 => format!(
            r#"{}pub static {} :  Emoji = Emoji("{}"); // {}"#,
            docs, basic.constant, basic.code, basic.name
        ),
        6 => {
            let one_toned_code = replace_tones(emos[1].code.clone());
            let default_tone = default_tone(basic.code.clone(), one_toned_code.clone());
            if !default_tone.is_empty() {
                format!(
                    r#"{}pub static {} : EmojiWithTone = EmojiWithTone::one_toned("{}").default_tone("{}"); // {}"#,
                    docs, basic.constant, one_toned_code, default_tone, basic.name
                )
            } else {
                format!(
                    "{}pub static {} : EmojiWithTone = EmojiWithTone::one_toned(\"{}\"); // {}",
                    docs, basic.constant, one_toned_code, basic.name
                )
            }
        }
        26 => {
            let one_toned_code = replace_tones(emos[1].code.clone());
            let two_toned_code = replace_tones(emos[2].code.clone());
            format!(
                "{}pub static {} : EmojiWithTone = EmojiWithTone::one_toned(\"{}\").two_toned(\"{}\"); // {}",
                docs, basic.constant, one_toned_code, two_toned_code, basic.name
            )
        }
        _ => {
            // Should not appear
            panic!("Found a emoji constant with invalid count of emojis")
        }
    }
}
