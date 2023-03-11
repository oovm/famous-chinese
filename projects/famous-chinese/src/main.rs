#![feature(once_cell)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::{Itertools, PeekingNext};
use serde::Serialize;
use serde_json::Serializer;
use tl::queryselector::iterable::QueryIterable;
use utf8_chars::BufReadCharsExt;

pub use crate::{filter_page::Page, pair_tags::CaptureTag};

mod errors;

mod filter_page;
mod pair_tags;

fn main() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("../wikipedia").canonicalize()?;
    let data = here.join("zhwiki-20230301-pages-articles-multistream.xml");
    let mut reader = BufReader::new(File::open(data)?);
    let mut stream = CaptureTag::new(&mut reader, "page")?;
    let mut pages = vec![];
    for page in stream {
        match Page::build(page) {
            None => {}
            Some(s) => {
                if s.rough_contains("中国") {
                    println!("{:?}", s);
                    pages.push(s);
                }
            }
        }
    }
    let mut file = File::create(here.join("pages.json"))?;
    let mut ser = Serializer::pretty(file);
    pages.serialize(&mut ser).unwrap();
    Ok(())
}
