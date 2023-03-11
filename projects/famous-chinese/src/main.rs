#![feature(once_cell)]

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::LazyLock;

use itertools::{Itertools, PeekingNext};
use regex::Regex;
use tl::queryselector::iterable::QueryIterable;
use utf8_chars::BufReadCharsExt;

pub use crate::pair_tags::CaptureTag;

mod errors;

mod pair_tags;

// [[Category:1955年啟用的鐵路車站]]
pub static CATEGORY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:\[\[)Category:([^]]+)(?:]])").unwrap());

pub static TITLE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:<title>)([^<]+)(?:</title>)").unwrap());

fn main() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("../wikipedia").canonicalize().unwrap();
    let data = here.join("zhwiki-20230301-pages-articles-multistream.xml");
    let mut reader = BufReader::new(File::open(data).unwrap());
    let mut stream = CaptureTag::new(&mut reader, "page").unwrap();
    for page in stream {
        match Page::build(page) {
            None => {}
            Some(s) => {
                if s.categories.contains("华人") {
                    println!("{}", s.title);
                    break
                }
            }
        }
    }
}

impl Page {
    pub fn build<S: Into<String>>(text: S) -> Option<Self> {
        let page = text.into();
        let title = TITLE_REGEX.captures(&page)?.get(1)?.as_str().to_string();
        let mut categories = BTreeSet::default();
        for item in CATEGORY_REGEX.captures_iter(&page) {
            categories.insert(item.get(1)?.as_str().to_string());
        }
        Some(Self {
            title,
            categories,
        })
    }
}


pub struct Page {
    pub title: String,
    pub categories: BTreeSet<String>,
}
