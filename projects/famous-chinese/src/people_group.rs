use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

use serde_derive::{Deserialize, Serialize};

use crate::Page;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeopleGroup {
    map: BTreeMap<char, PeopleGroupItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeopleGroupItem {
    pub surname: char,
    pub count: usize,
    pub people: BTreeSet<Page>,
}

impl PartialOrd<Self> for PeopleGroupItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.count.partial_cmp(&other.count)
    }
}

impl Ord for PeopleGroupItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PeopleGroup {
    pub fn insert(&mut self, page: &Page) {
        let surname = page.surname();
        let item = self.map.entry(surname).or_insert_with(|| PeopleGroupItem::empty(surname));
        item.count += 1;
        item.people.insert(page.clone());
    }
    pub fn as_set(&self) -> BTreeSet<PeopleGroupItem> {
        self.map.values().cloned().collect()
    }
}

impl PeopleGroupItem {
    pub fn empty(surname: char) -> Self {
        Self { surname, count: 0, people: BTreeSet::default() }
    }
}
