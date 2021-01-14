use lazy_static::lazy_static;
use maplit::hashmap;
use regex::Regex;
use reqwest::IntoUrl;
use std::collections::HashMap;
use std::io::Read;

lazy_static! {
    static ref NON_ALPHA_NUM_REGEX: Regex = Regex::new(r"[^\w\d]+").unwrap();
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
    static ref CHANGES: HashMap<&'static str, &'static str> = hashmap! {
        "*" =>    "asterisk",
        "#"=>    "hash",
        "1st"=>  "first",
        "2nd"=>  "second",
        "3rd"=>  "third",
        "&"=>    "and",
        "U.S."=> "US",
        "Š"=>    "S",
        "š"=>    "s",
        "Đ"=>    "Dj",
        "đ"=>    "dj",
        "Ž"=>    "Z",
        "ž"=>    "z",
        "Č"=>    "C",
        "č"=>    "c",
        "Ć"=>    "C",
        "ć"=>    "c",
        "À"=>    "A",
        "Á"=>    "A",
        "Â"=>    "A",
        "Ã"=>    "A",
        "Ä"=>    "A",
        "Å"=>    "A",
        "Æ"=>    "A",
        "Ç"=>    "C",
        "È"=>    "E",
        "É"=>    "E",
        "Ê"=>    "E",
        "Ë"=>    "E",
        "Ì"=>    "I",
        "Í"=>    "I",
        "Î"=>    "I",
        "Ï"=>    "I",
        "Ñ"=>    "N",
        "Ò"=>    "O",
        "Ó"=>    "O",
        "Ô"=>    "O",
        "Õ"=>    "O",
        "Ö"=>    "O",
        "Ø"=>    "O",
        "Ù"=>    "U",
        "Ú"=>    "U",
        "Û"=>    "U",
        "Ü"=>    "U",
        "Ý"=>    "Y",
        "Þ"=>    "B",
        "ß"=>    "Ss",
        "à"=>    "a",
        "á"=>    "a",
        "â"=>    "a",
        "ã"=>    "a",
        "ä"=>    "a",
        "å"=>    "a",
        "æ"=>    "a",
        "ç"=>    "c",
        "è"=>    "e",
        "é"=>    "e",
        "ê"=>    "e",
        "ë"=>    "e",
        "ì"=>    "i",
        "í"=>    "i",
        "î"=>    "i",
        "ï"=>    "i",
        "ð"=>    "o",
        "ñ"=>    "n",
        "ò"=>    "o",
        "ó"=>    "o",
        "ô"=>    "o",
        "õ"=>    "o",
        "ö"=>    "o",
        "ø"=>    "o",
        "ù"=>    "u",
        "ú"=>    "u",
        "û"=>    "u",
        "ý"=>    "y",
        "þ"=>    "b",
        "ÿ"=>    "y",
        "Ŕ"=>    "R",
        "ŕ"=>    "r",
    };
}

pub fn clean(mut inp: String) -> String {
    CHANGES.iter().for_each(|(&k, &v)| {
        inp = inp.replace(k, v);
    });
    inp = NON_ALPHA_NUM_REGEX.replace_all(&inp, " ").into();
    inp
}

pub fn remove_spaces(mut inp: String) -> String {
    inp = WHITESPACE_REGEX.replace_all(&inp, "").into();
    inp
}

pub fn make_alias(inp: String) -> String {
    format!(":{}:", inp)
}

pub fn fetch_data<T: IntoUrl>(url: T) -> Result<Vec<u8>, reqwest::Error> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = Vec::new();
    res.read_to_end(&mut body).unwrap();
    if res.status().is_success() {
        Ok(body)
    } else {
        res.error_for_status().map(|_| Vec::new())
    }
}
