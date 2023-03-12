use std::{cmp::Ordering, collections::BTreeSet, mem::take, sync::LazyLock};

use regex::Regex;
use serde_derive::{Deserialize, Serialize};

use zhconv::{zhconv_mw, Variant};

pub static CATEGORY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:\[\[)Category:([^]]+)(?:]])").unwrap());

pub static TITLE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?:<title>)([^<]+)(?:</title>)").unwrap());

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    pub title: String,
    pub raw: String,
    pub categories: BTreeSet<String>,
}

impl PartialOrd<Self> for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.title.partial_cmp(&other.title)
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.title.cmp(&other.title)
    }
}

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
    pub fn surname(&self) -> char {
        self.title.chars().next().unwrap()
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
    pub fn decomposition(&mut self) {
        let cats = take(&mut self.categories);
        if self.drop_title() {
            return;
        }
        for item in cats {
            for item in item.split('|') {
                self.categories.insert(item.trim().to_string());
            }
        }
    }
    fn drop_title(&self) -> bool {
        if self.title.starts_with(|c: char| c.is_ascii_alphanumeric()) {
            return true;
        }
        if self.title.starts_with("中国") || self.title.starts_with("中华") {
            return true;
        }
        if self.title.ends_with("列表") {
            return true;
        }
        if self.title.is_empty() {
            return true;
        }
        false
    }
    pub fn end_with(&self, patterns: &[&'static str]) -> bool {
        for i in self.categories.iter() {
            for end in patterns {
                if i.ends_with(end) {
                    return true;
                }
            }
        }
        false
    }
}
