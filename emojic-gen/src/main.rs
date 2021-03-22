#![feature(assoc_char_funcs)]
#![recursion_limit = "256"]

mod emoji;
mod gemoji;
mod strutil;

use emoji::Emojis;
use emoji::Group;
use emoji::Subgroup;
use inflections::case::to_snake_case;
use lazy_static::lazy_static;
use std::fmt;
use std::fs::File;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
};

// mod constants;
use tera::Context;
use tera::Tera;

const EMOJI_URL: &str = "https://unicode.org/Public/emoji/13.1/emoji-test.txt";

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.tpl") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

pub struct Emojik(&'static str);

impl fmt::Display for Emojik {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{})", self.0)
    }
}

fn main() {
    println!("Fetching...");
    //let emoji_text = strutil::fetch_data(EMOJI_URL);
    let a = gemoji::fetch_gemoji();
    let mut e = fetch_emojis().unwrap();
    //dbg!(&a);

    println!("Sorting...");
    e.sort();

    save_constants(generate_constants(&e));
    save_aliasses(generate_aliases(&mut e, &a));
}

fn read_lines<'a>(content: &Vec<u8>, mut f: impl FnMut(&mut str) -> ()) {
    let reader = BufReader::new(&content[..]);
    for line in reader.lines().into_iter() {
        match line {
            Ok(mut l) => f(&mut l),
            Err(_) => (),
        }
    }
}

fn fetch_emojis() -> Result<Emojis, String> {
    let emoji_text = strutil::fetch_data(EMOJI_URL);

    let mut emojis: Emojis = Emojis::default();
    let mut current_group: String = String::new();
    let mut current_sub_group: String = String::new();

    read_lines(&emoji_text.unwrap(), |line| {
        let line = line.trim();
        println!("Process: {:?}", line);
        if line.is_empty() {
            // Just ignore it
        } else if line.starts_with("# group:") {
            let name = line.replace("# group:", "").trim().to_owned();
            emojis.append(name.to_owned()).unwrap();
            current_group = name.to_owned();
        } else if line.starts_with("# subgroup:") {
            let name = line.replace("# subgroup:", "").trim().to_owned();
            emojis
                .get_group(&current_group)
                .unwrap()
                .append(name.to_owned())
                .unwrap();
            current_sub_group = name.to_owned();
        } else if !line.starts_with('#') {
            //println!("Current group : {} subgroup {}", current_group,current_sub_group);
            emojis
                .get_group(&current_group)
                .unwrap()
                .get_subgroup(&current_sub_group)
                .unwrap()
                .append_line(line);
        }
    });
    Ok(emojis)
}

fn emojis_for_sub_group_list(sub: &Subgroup) -> Vec<String> {
    sub.constants
        .iter()
        .flat_map(|c| sub.get_emoji(c).unwrap().default_grapheme())
        .map(|s| s.to_string())
        .take(3)
        .collect()
}

fn emojis_for_subgroup(sub: &Subgroup) -> String {
    emojis_for_sub_group_list(sub).join("")
}

fn emojis_for_group(grp: &Group) -> String {
    grp.subgroups
        .iter()
        .take(3)
        .flat_map(|sub| emojis_for_sub_group_list(sub).get(0).cloned())
        .collect()
}

pub fn generate_constants(e: &Emojis) -> String {
    // Generate separate lists for flat & grouped
    let mut flat = String::new();
    let mut grouped = String::new();

    e.groups.iter().for_each(|g| {
        grouped.push_str(&format!("\n/// {} {}\n", g.name, emojis_for_group(g)));
        grouped.push_str(&format!("pub mod {} {{\n", g.identifier));

        g.subgroups.iter().for_each(|s| {
            grouped.push_str(&format!("\n/// {} {}\n", s.name, emojis_for_subgroup(s)));
            grouped.push_str(&format!(
                "pub mod {} {{ // {}::{}\n",
                s.identifier, g.identifier, s.identifier
            ));
            grouped.push_str("use crate::emojis::Emoji;\n");
            grouped.push_str("use crate::emojis::Family;\n");
            grouped.push_str("use crate::emojis::Gender;\n");
            grouped.push_str("use crate::emojis::Hair;\n");
            grouped.push_str("use crate::emojis::OneOrTwo;\n");
            grouped.push_str("use crate::emojis::Pair;\n");
            grouped.push_str("use crate::emojis::Tone;\n");
            grouped.push_str("use crate::emojis::TonePair;\n");
            //grouped.push_str("use crate::emojis::TonePairReduced;\n");
            grouped.push_str("use crate::emojis::With;\n");
            grouped.push_str("use crate::emojis::WithNoDef;\n");

            s.emoji_iter().for_each(|value| {
                println!("Writing emoji {:?}", value);
                grouped.push_str(&value.to_source_code());
                grouped.push('\n');

                flat.push_str("#[doc(inline)]\n");
                flat.push_str(&format!(
                    "pub use crate::grouped::{}::{}::{};\n",
                    g.identifier,
                    s.identifier,
                    value.identifier()
                ));
            });
            grouped.push_str(&format!("}} // {}::{}\n", g.identifier, s.identifier));
        });
        grouped.push_str(&format!("}} // {}\n", g.identifier));
    });

    // Combine the list from above
    let mut res = String::new();

    res.push_str(
        r#"
/// Grouped list of all emojis with sub modules.
///
/// This module contains the same set of emojis as the [`crate::flat`] module, but
/// categorized into their respective groups and subgroups via sub modules.
/// This make it easier to browse all the emojis in an intelligible way.
///
/// # Examples
///
/// ```rust
/// // prints: 🖼️
/// println!("{}", emojic::grouped::activities::arts_and_crafts::FRAMED_PICTURE);
/// ```
	"#,
    );
    res.push_str("pub mod grouped {\n");
    res.push_str(&grouped);
    res.push_str("}\n");

    res.push_str(
        r#"
/// Flat list of all emojis without sub modules.
///
/// This module contains the same set of emojis as the [`crate::grouped`] module, but
/// without the sub modules. This make it a bit more messy but allows for shorter
/// references from code.
///
/// # Examples
///
/// ```rust
/// // prints: 🖼️
/// println!("{}", emojic::flat::FRAMED_PICTURE);
/// ```
	"#,
    );
    res.push_str("pub mod flat {\n");
    res.push_str(&flat);
    res.push_str("}\n");

    res
}

pub fn generate_aliases(emoji: &mut Emojis, gemojis: &HashMap<String, String>) -> String {
    let mut aliasses: Vec<String> = Vec::new();
    let mut emoji_map: HashMap<String, String> = HashMap::new();
    let mut emoji_map_by_grapheme: HashMap<String, String> = HashMap::new();

    emoji.groups.iter_mut().for_each(|g| {
        g.subgroups.iter_mut().for_each(|s| {
            s.constants.iter().for_each(|c| {
                let em = s.get_emoji(c).unwrap();
                let alias = gemoji::make_alias(&to_snake_case(&em.identifier()));

                // Add the graphemes of all variants
                for (accessor, grapheme) in em.full_emoji_list() {
                    emoji_map_by_grapheme.insert(grapheme.to_string(), accessor);
                }

                // Add an alias for the default name
                if let Some(def) = em.default_grapheme() {
                    aliasses.push(alias.clone());
                    emoji_map.insert(alias, emoji_map_by_grapheme[def].clone());
                }
            });
        })
    });

    gemojis.iter().for_each(|(key, val)| {
        if !emoji_map.contains_key(key) {
            if let Some(emoji) = emoji_map_by_grapheme.get(val) {
                emoji_map.insert(key.clone(), emoji.clone());
                aliasses.push(key.clone());
            } else {
                println!("Couldn't find emoji for {:?} ({})", key, val);
            }
        }
    });

    aliasses[..].sort();
    aliasses = aliasses
        .iter_mut()
        .map(|al| {
            format!(
                "(\"{}\" , &crate::flat::{} as &crate::Emoji),\n",
                al,
                emoji_map.get(al).unwrap()
            )
        })
        .collect::<Vec<String>>();

    aliasses[..].join("")
}

fn save_constants(constants: String) {
    let mut context = Context::new();

    use chrono::{DateTime, Utc};
    let now: DateTime<Utc> = Utc::now();

    let today = format!("{}", now);
    context.insert("Link", EMOJI_URL);
    context.insert("Date", &today);
    context.insert("Data", &constants);

    let bytes = TEMPLATES
        .render("constants.rs.tpl", &context)
        .expect("Failed to render");
    File::create("./constants.rs")
        .unwrap()
        .write_all(bytes.as_bytes());
}

fn save_aliasses(aliasses: String) {
    let mut context = Context::new();

    use chrono::{DateTime, Utc};
    let now: DateTime<Utc> = Utc::now();

    let today = format!("{}", now);
    context.insert("Link", gemoji::GEMOJI_URL);
    context.insert("Date", &today);
    context.insert("Data", &aliasses);

    let bytes = TEMPLATES
        .render("alias.rs.tpl", &context)
        .expect("Failed to render alias");
    File::create("./alias.rs")
        .unwrap()
        .write_all(bytes.as_bytes());
}
