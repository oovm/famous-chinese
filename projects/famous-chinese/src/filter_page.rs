use std::{collections::BTreeSet, sync::LazyLock};

use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use tl::queryselector::iterable::QueryIterable;
use zhconv::{zhconv_mw, Variant};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub raw: String,
    pub title: String,
    pub categories: BTreeSet<String>,
}

// [[Category:1955年啟用的鐵路車站]]
pub static CATEGORY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:\[\[)Category:([^]]+)(?:]])").unwrap());

pub static TITLE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:<title>)([^<]+)(?:</title>)").unwrap());

impl Page {
    pub fn build<S: Into<String>>(text: S) -> Option<Self> {
        let page = text.into();
        let raw = TITLE_REGEX.captures(&page)?.get(1)?.as_str();
        let title = zhconv_mw(raw, Variant::ZhHans);
        let mut categories = BTreeSet::default();
        for item in CATEGORY_REGEX.captures_iter(&page) {
            let cats = item.get(1)?.as_str();
            let hans = zhconv_mw(cats, Variant::ZhHans);
            categories.insert(hans);
        }
        Some(Self { raw: raw.to_string(), title, categories })
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
