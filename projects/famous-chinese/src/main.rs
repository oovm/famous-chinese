#![feature(once_cell)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::path::Path;
use std::sync::LazyLock;

use itertools::{Itertools, PeekingNext};
use regex::Regex;
use utf8_chars::{BufReadCharsExt, Chars};

mod errors;

// [[Category:1955年啟用的鐵路車站]]
pub static CATEGORY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[\[Category:([^]]+)]]").unwrap());


fn main() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("../wikipedia").canonicalize().unwrap();
    let data = here.join("zhwiki-20230301-pages-articles-multistream.xml");
    let mut reader = BufReader::new(File::open(data).unwrap());
    let mut stream = CaptureTag::new(&mut reader, "page").unwrap();
    for page in stream.take(1) {
        println!("{}", page.text);
    }
}


pub struct Page {
    pub text: String,
}

pub struct CaptureTag<'i>
{
    pattern: String,
    buffer: String,
    reader: Peekable<Chars<'i, BufReader<File>>>,
}

impl<'i> CaptureTag<'i> {
    pub fn new<S: Into<String>>(br: &'i mut BufReader<File>, tag: S) -> std::io::Result<Self> {
        let reader = br.chars().peekable();
        Ok(Self {
            pattern: tag.into(),
            buffer: String::new(),
            reader,
        })
    }
    fn peek_tag(&mut self) -> String {
        let mut peek_buffer = String::new();
        // peek until >
        for item in self.reader.peeking_take_while(end_peek) {
            match item {
                Ok(c) => {
                    peek_buffer.push(c)
                }
                Err(_) => {}
            }
        }
        if !peek_buffer.is_empty() {
            println!("peek: {}", peek_buffer)
        }
        peek_buffer
    }
}

fn end_peek(c: &std::io::Result<char>) -> bool {
    match c {
        Ok(c) => '>'.ne(c),
        Err(_) => false,
    }
}

impl<'i> Iterator for CaptureTag<'i> {
    type Item = Page;
    // <page> ... </page>
    fn next(&mut self) -> Option<Self::Item> {
        let mut text_buffer = String::new();
        let mut in_page = false;
        while let Some(c) = self.reader.next() {
            match c {
                // <tag>
                Ok('<') if !in_page => {
                    if self.peek_tag().eq(&self.pattern) {
                        in_page = true;
                    }
                }
                // </tag>
                Ok('<') if in_page => {
                    if self.peek_tag().eq(&format!("/{}", self.pattern)) {
                        in_page = false;
                        return Some(Page { text: text_buffer });
                    }
                }
                Ok(c) if in_page => {
                    text_buffer.push(c);
                }
                _ => {
                    continue;
                }
            }
        }
        None
    }
}
