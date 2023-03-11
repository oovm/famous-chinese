use std::{fs::File, io::BufReader, iter::Peekable};

use itertools::Itertools;
use utf8_chars::{BufReadCharsExt, Chars};

pub struct CaptureTag<'i> {
    pattern: String,
    reader: Peekable<Chars<'i, BufReader<File>>>,
}

impl<'i> CaptureTag<'i> {
    pub fn new<S: Into<String>>(br: &'i mut BufReader<File>, tag: S) -> std::io::Result<Self> {
        let reader = br.chars().peekable();
        Ok(Self { pattern: tag.into(), reader })
    }

    fn is_start_pattern(&self, s: &str) -> bool {
        format!("<{}", self.pattern).eq(s)
    }
    fn is_end_pattern(&self, s: &str) -> bool {
        format!("</{}", self.pattern).eq(s)
    }
    // peek until >
    fn peek_tag(&mut self) -> String {
        let mut peek_buffer = "<".to_string();
        for item in self.reader.peeking_take_while(continue_peek) {
            match item {
                Ok(c) => peek_buffer.push(c),
                Err(_) => {
                    break;
                }
            }
        }
        peek_buffer
    }
}

fn continue_peek(c: &std::io::Result<char>) -> bool {
    match c {
        Ok(c) => '>'.ne(c),
        Err(_) => false,
    }
}

impl<'i> Iterator for CaptureTag<'i> {
    type Item = String;
    /// `<tag> ... </tag>`
    fn next(&mut self) -> Option<Self::Item> {
        let mut text_buffer = String::new();
        let mut in_page = false;
        while let Some(c) = self.reader.next() {
            match c {
                // <tag>
                Ok('<') if !in_page => match self.peek_tag().as_str() {
                    s if self.is_start_pattern(s) => {
                        in_page = true;
                        text_buffer.push_str(s);
                    }
                    _ => continue,
                },
                // </tag>
                Ok('<') if in_page => match self.peek_tag().as_str() {
                    s if self.is_end_pattern(s) => {
                        // in_page = false;
                        text_buffer.push_str(s);
                        text_buffer.push('>');
                        return Some(text_buffer);
                    }
                    s => text_buffer.push_str(s),
                },
                Ok(c) if in_page => {
                    text_buffer.push(c);
                }
                _ => {
                    continue;
                }
            }
        }
        if text_buffer.is_empty() { None } else { Some(text_buffer) }
    }
}
