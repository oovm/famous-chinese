#![feature(once_cell)]

use itertools::Itertools;
use std::{fs::File, io::BufReader, path::Path};

use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};

pub use crate::{filter_page::Page, pair_tags::CaptureTag};

mod errors;

mod filter_page;
mod pair_tags;
mod people_group;

fn main() -> std::io::Result<()> {
    // classify_pages()?;
    find_people_like()?;
    Ok(())
}

pub fn classify_pages() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("../wikipedia").canonicalize()?;
    let data = here.join("zhwiki-20230301-pages-articles-multistream.xml");
    let mut reader = BufReader::new(File::open(data)?);
    let stream = CaptureTag::new(&mut reader, "page")?;
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
    save_json(pages, &here.join("pages.json"))?;
    Ok(())
}

fn save_json<T: Serialize>(data: T, path: &Path) -> std::io::Result<()> {
    let file = File::create(path)?;
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(file, formatter);
    data.serialize(&mut ser).unwrap();
    Ok(())
}

pub fn find_people_like() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("../wikipedia").canonicalize()?;
    let mut data: Vec<Page> = serde_json::from_reader(File::open(here.join("pages.json"))?).unwrap();
    println!("已载入 {} 个页面", data.len());
    let mut people = vec![];
    for item in data.iter_mut() {
        item.decomposition();
        if item.end_with(&["人", "姓", "员", "家", "长", "士", "帝", "校友", "书记"]) {
            println!("{:?}", item);
            people.push(item.clone());
        }
    }
    let people = people.into_iter().sorted_by(|l, r| l.title.cmp(&r.title)).collect_vec();
    save_json(people, &here.join("people.json"))?;
    Ok(())
}
