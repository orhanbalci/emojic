use crate::strutil::fetch_data;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Gemoji {
    emoji: String,
    description: String,
    category: String,
    aliases: Vec<String>,
    tags: Vec<String>,
    unicode_version: String,
    ios_version: String,
}

pub const GEMOJI_URL: &'static str =
    "https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json";

pub fn make_alias(name: &str) -> String {
    format!("{}", name)
}

pub fn fetch_gemoji() -> HashMap<String, String> {
    let content = fetch_data(GEMOJI_URL).unwrap();
    //dbg!(std::str::from_utf8(&content[..]).unwrap());
    let gemojis: Vec<Gemoji> =
        serde_json::from_str(std::str::from_utf8(&content[..]).unwrap()).unwrap();
    //dbg!(gemojis);
    gemojis.iter().fold(HashMap::new(), |hm, g| {
        g.aliases.iter().fold(hm, |mut s, alias| {
            if !alias.trim().is_empty() {
                s.insert(make_alias(alias.trim()), g.emoji.clone());
            }
            s
        })
    })
}
