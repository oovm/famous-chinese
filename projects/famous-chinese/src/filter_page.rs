use std::{collections::BTreeSet, sync::LazyLock};

use regex::Regex;
use tl::queryselector::iterable::QueryIterable;

#[derive(Debug)]
pub struct Page {
    pub title: String,
    pub categories: BTreeSet<String>,
}

// [[Category:1955年啟用的鐵路車站]]
pub static CATEGORY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:\[\[)Category:([^]]+)(?:]])").unwrap());

pub static TITLE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:<title>)([^<]+)(?:</title>)").unwrap());

impl Page {
    pub fn build<S: Into<String>>(text: S) -> Option<Self> {
        let page = text.into();
        let title = TITLE_REGEX.captures(&page)?.get(1)?.as_str().to_string();
        let mut categories = BTreeSet::default();
        for item in CATEGORY_REGEX.captures_iter(&page) {
            categories.insert(item.get(1)?.as_str().to_string());
        }
        Some(Self { title, categories })
    }
    pub fn exact_contains(&self, s: &str) -> bool {
        self.categories.contains(s)
    }
    pub fn rough_contains(&self, s: &str) -> bool {
        for i in self.categories.iter() {
            if i.contains(s) {
                return true;
            }
        }
        false
    }
}
