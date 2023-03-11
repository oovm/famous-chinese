#![feature(once_cell)]

use std::cell::LazyCell;
use std::fs::{File, read_to_string};
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

pub use errors::{Error, Result};

mod errors;

use quick_xml::events::Event;
use quick_xml::reader::Reader;
// [[Category:1955年啟用的鐵路車站]]
pub static CATEGORY_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[\[Category:([^]]+)]]").unwrap());


fn main() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("../wikipedia").canonicalize().unwrap();
    let data = here.join("zhwiki-20230301-pages-articles-multistream.xml");
    let mut reader = Reader::from_file(&data).unwrap();
    reader.trim_text(true);


    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

// The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"tag1" => println!("attributes values: {:?}",
                                        e.attributes().map(|a| a.unwrap().value)
                                            .collect::<Vec<_>>()),
                    b"tag2" => count += 1,
                    _ => (),
                }
            }
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
}