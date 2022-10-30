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
use serde::Serialize;
use std::fmt;
use std::fs::File;
use std::{
    collections::BTreeMap,
    collections::HashMap,
    io::{BufRead, BufReader, Write},
};

use regex;

// mod constants;
use tera::Context;
use tera::Tera;

const EMOJI_URL: &str = "https://unicode.org/Public/emoji/13.1/emoji-test.txt";

lazy_static! {
    static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.tera") {
            Ok(t) => {
                for template_name in t.get_template_names() {
                    println!("Found template: {}", template_name);
                }
                t
            }
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

struct Emojik(&'static str);

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

    let mut all_emojis: Vec<String> = generate_all_graphemes(&e);
    all_emojis.sort_by_key(|g| -(g.len() as i128));
    let all_emojis = all_emojis; // make immutable now
    let regex_str = generate_regex(&all_emojis);
    save_regex(&regex_str, &all_emojis, all_emojis.len());

    let constants = generate_constants(&e);
    save_flat_constants(&constants);
    save_grouped_constants(&constants);

    let (map_alias, match_aliases) = generate_aliases(&mut e, &a);
    save_aliasses(map_alias);
    save_big_matcher(match_aliases);
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

fn generate_all_graphemes(e: &Emojis) -> Vec<String> {
    e.groups
        .iter()
        .map(|g| g.subgroups.iter())
        .flatten()
        .map(|sg| sg.emojis.values())
        .flatten()
        .map(|e| {
            let escaped_grapheme = ::regex::escape(&e.grapheme);
            escaped_grapheme
        })
        .collect()
}

fn generate_regex(all_emojis: &Vec<String>) -> String {
    all_emojis.join("|")
}

fn save_regex(regex: &str, all_emojis: &Vec<String>, num_emojis: usize) {
    let mut context = Context::new();

    let all_emojis: Vec<String> = all_emojis
        .iter()
        .map(|e| e.escape_unicode().to_string())
        .collect();

    use chrono::{DateTime, Utc};
    let now: DateTime<Utc> = Utc::now();

    let today = format!("{}", now);
    context.insert("Link", EMOJI_URL);
    context.insert("Date", &today);
    context.insert("regex_text", &regex);
    context.insert("num_emojis", &num_emojis);
    context.insert("all_emojis", &all_emojis);

    let bytes = TEMPLATES
        .render("regex.tera", &context)
        .expect("Failed to render flat");
    File::create("./regex.rs")
        .unwrap()
        .write_all(bytes.as_bytes());
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

#[derive(Debug, Clone, Serialize)]
struct GroupedConstant<'a> {
    pub identifier: &'a str,
    pub preview_emojis: String,
    pub subgroups: Vec<SubgroupConstant<'a>>,
}

#[derive(Debug, Clone, Serialize)]
struct SubgroupConstant<'a> {
    pub identifier: &'a str,
    pub preview_emojis: String,
    pub emojis: Vec<EmojiConstant<'a>>,
}

#[derive(Debug, Clone, Serialize)]
struct EmojiConstant<'a> {
    pub identifier: &'a str,
    pub preview_emojis: String,
    pub source_code: String,
    pub full_list_accessors: Vec<String>,
    pub default_list_accessors: Vec<String>,
}

fn generate_constants(e: &Emojis) -> Vec<GroupedConstant> {
    // Collect all groups
    e.groups
        .iter()
        .map(|g| {
            // Collect all subgroups
            let subgroups = g
                .subgroups
                .iter()
                .map(|s| {
                    // Collect all emojis
                    let emojis = s
                        .emoji_iter()
                        .map(|emoji| {
                            println!("Writing emoji {:?}", emoji.identifier());

                            let full_list_accessors = emoji
                                .full_emoji_list()
                                .into_iter()
                                .map(|(acc, _, _)| acc)
                                .collect();

                            let default_list_accessors = emoji
                                .default_emoji_list()
                                .into_iter()
                                .map(|(acc, _, _)| acc)
                                .collect();

                            EmojiConstant {
                                identifier: emoji.identifier(),
                                preview_emojis: emoji.graphemes(),
                                source_code: emoji.to_source_code(),
                                full_list_accessors,
                                default_list_accessors,
                            }
                        })
                        .collect();

                    SubgroupConstant {
                        identifier: &s.identifier,
                        preview_emojis: emojis_for_subgroup(s),
                        emojis,
                    }
                })
                .collect();

            GroupedConstant {
                identifier: &g.identifier,
                preview_emojis: emojis_for_group(g),
                subgroups,
            }
        })
        .collect()
}

fn generate_aliases(
    emoji: &mut Emojis,
    gemojis: &HashMap<String, String>,
) -> (String, (String, String)) {
    let mut aliasses: Vec<String> = Vec::new();
    let mut emoji_map: HashMap<String, String> = HashMap::new();
    let mut emoji_map_by_grapheme: HashMap<String, String> = HashMap::new();

    emoji.groups.iter_mut().for_each(|g| {
        g.subgroups.iter_mut().for_each(|s| {
            s.constants.iter().for_each(|c| {
                let em = s.get_emoji(c).unwrap();
                let alias = gemoji::make_alias(&to_snake_case(&em.identifier()));

                // Add the graphemes of all variants
                for (const_accessor, pub_accessor, grapheme) in em.full_emoji_list() {
                    emoji_map_by_grapheme.insert(grapheme.to_string(), const_accessor);
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

    let map_aliasses = aliasses
        .iter()
        .map(|al| {
            format!(
                "(\"{}\" , &crate::flat::{} as &crate::Emoji),\n",
                al,
                emoji_map.get(al).unwrap()
            )
        })
        .collect::<String>();

    let match_aliasses = {
        let mut single_bytes: BTreeMap<char, &str> = BTreeMap::new();
        let mut two_byte_groups: BTreeMap<char, BTreeMap<char, Vec<&str>>> = BTreeMap::new();

        for al in &aliasses {
            let len = al.len();
            if len == 0 {
                panic!("Found an empty alias");
            } else if len == 1 {
                let first = al.chars().next().unwrap();
                single_bytes.insert(first, &al);
            } else {
                let mut chars = al.chars();
                let first = chars.next().unwrap();
                let sec = chars.next().unwrap();

                two_byte_groups
                    .entry(first)
                    .or_default()
                    .entry(sec)
                    .or_default()
                    .push(&al);
            }
        }

        let mut out_single = String::new();
        for (first, al) in single_bytes {
            out_single.push_str(&format!(
                "\t\t\tb'{}' => Some(&crate::flat::{} as &crate::Emoji),\n",
                first,
                emoji_map.get(al).unwrap()
            ));
        }

        let mut out_two = String::new();
        for (first, grp) in two_byte_groups {
            out_two.push_str(&format!("\t\t\tb'{}' => match sec {{\n", first));
            for (sec, sub) in grp {
                out_two.push_str(&format!("\t\t\t\tb'{}' => match rest {{\n", sec));
                for al in sub {
                    out_two.push_str(&format!(
                        "\t\t\t\t\t{:?} => Some(&crate::flat::{} as &crate::Emoji),\n",
                        &al[2..],
                        emoji_map.get(al).unwrap()
                    ));
                }
                out_two.push_str("\t\t\t\t\t_ => None,\n");
                out_two.push_str("\t\t\t\t},\n");
            }
            out_two.push_str("\t\t\t\t_ => None,\n");
            out_two.push_str("\t\t\t},\n");
        }

        (out_single, out_two)
    };

    (map_aliasses, match_aliasses)
}

fn save_flat_constants(constants: &[GroupedConstant]) {
    let mut context = Context::new();

    use chrono::{DateTime, Utc};
    let now: DateTime<Utc> = Utc::now();

    let today = format!("{}", now);
    context.insert("Link", EMOJI_URL);
    context.insert("Date", &today);
    context.insert("Constants", &constants);

    let bytes = TEMPLATES
        .render("flat.tera", &context)
        .expect("Failed to render flat");
    File::create("./flat.rs")
        .unwrap()
        .write_all(bytes.as_bytes());
}

fn save_grouped_constants(constants: &[GroupedConstant]) {
    let mut context = Context::new();

    use chrono::{DateTime, Utc};
    let now: DateTime<Utc> = Utc::now();

    let today = format!("{}", now);
    context.insert("Link", EMOJI_URL);
    context.insert("Date", &today);
    context.insert("Constants", &constants);

    let bytes = TEMPLATES
        .render("grouped.tera", &context)
        .expect("Failed to render grouped");
    File::create("./grouped.rs")
        .unwrap()
        .write_all(bytes.as_bytes());
}

fn save_big_matcher(aliasses: (String, String)) {
    let mut context = Context::new();

    use chrono::{DateTime, Utc};
    let now: DateTime<Utc> = Utc::now();

    let today = format!("{}", now);
    context.insert("Link", EMOJI_URL);
    context.insert("Date", &today);
    context.insert("SingleBytes", &aliasses.0);
    context.insert("TwoBytes", &aliasses.1);

    let bytes = TEMPLATES
        .render("matching.tera", &context)
        .expect("Failed to render matching");
    File::create("./matching.rs")
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
        .render("alias.tera", &context)
        .expect("Failed to render alias");
    File::create("./alias.rs")
        .unwrap()
        .write_all(bytes.as_bytes());
}
