#![feature(assoc_char_funcs)]
#![recursion_limit = "256"]

mod emoji;
mod strutil;
use std::fmt;
use emoji::{Emojis, Group, Subgroup};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Read;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    time::SystemTime,
};

// mod constants;
use tera::Context;
use tera::Tera;

use reqwest::IntoUrl;

const EMOJI_URL: &str = "https://unicode.org/Public/emoji/13.0/emoji-test.txt";
const CONSTANTS_FILE: &str = "constants.rs";

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
    let emoji_text = fetch_data(EMOJI_URL);
    save_constants(generate_constants(&fetch_emojis().unwrap()));    
}

fn fetch_data<T: IntoUrl>(url: T) -> Result<Vec<u8>, reqwest::Error> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = Vec::new();
    res.read_to_end(&mut body).unwrap();
    if res.status().is_success() {
        Ok(body)
    } else {
        res.error_for_status().map(|_| Vec::new())
    }
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
    let emoji_text = fetch_data(EMOJI_URL);
    //     let emoji_text:Result<Vec<u8>,String> = Ok(r#"
    // # group: hede
    // # subgroup: family
    // 1F3CC FE0F 200D 2642 FE0F                  ; fully-qualified     # 🏌️‍♂️ E4.0 man golfing
    // 1F3CC 1F3FB 200D 2642 FE0F                 ; fully-qualified     # 🏌🏻‍♂️ E4.0 man golfing: light skin tone
    // 1F3CC 1F3FC 200D 2642 FE0F                 ; fully-qualified     # 🏌🏼‍♂️ E4.0 man golfing: medium-light skin tone
    // 1F3CC 1F3FD 200D 2642 FE0F                 ; fully-qualified     # 🏌🏽‍♂️ E4.0 man golfing: medium skin tone
    // 1F3CC 1F3FE 200D 2642 FE0F                 ; fully-qualified     # 🏌🏾‍♂️ E4.0 man golfing: medium-dark skin tone
    // 1F3CC 1F3FF 200D 2642 FE0F                 ; fully-qualified     # 🏌🏿‍♂️ E4.0 man golfing: dark skin tone
    // "#.as_bytes().to_vec());

    let mut emojis: Emojis = Emojis { groups: Vec::new() };
    let mut current_group: String = String::new();
    let mut current_sub_group: String = String::new();

    read_lines(&emoji_text.unwrap(), |line| {
        if line.starts_with("# group:") {
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
        } else if !line.starts_with("#") {
            let e = emoji::Emoji::new(line.to_owned());
            //println!("Current group : {} subgroup {}", current_group,current_sub_group);
            match e {
                Some(k) => emojis
                    .get_group(&current_group)
                    .unwrap()
                    .get_subgroup(&current_sub_group)
                    .unwrap()
                    .append(&k),
                None => (),
            }
        }
    });
    Ok(emojis)
}

pub fn generate_constants(e: &Emojis) -> String {
    let mut res = String::new();
    e.groups.iter().for_each(|g| {
        res.push_str(&format!("\n// GROUP: {}\n", g.name));
        g.subgroups.iter().for_each(|s| {
            res.push_str(&format!("// SUBGROUP: {}\n", s.name));
            s.emojis.iter().for_each(|(key, value)| {
                println!("Writing emoji {:?}", value);
                res.push_str(&emoji::emoji_constant_line(value));
                res.push_str("\n");
            })
        })
    });

    res
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
