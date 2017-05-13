extern crate regex;

use std::time::Instant;
use regex::RegexBuilder;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;

fn main() {
    let pattern = "^[a-zA-Z0-9_\\s\\r\\n\\t]*$";
    let regex = RegexBuilder::new(pattern).build().unwrap();
    let lines: Vec<String> = BufReader::new(File::open(env::args().nth(1)
        .expect("Expected file path argument"))
        .expect("File did not exist"))
        .lines()
        .map(|l| l.expect("I/O error"))
        .collect();
    let mut start_instant: Option<Instant> = None;
    let mut matched: usize = 0;
    let line_count = lines.len();
    let total = line_count * 10;
    for i in 0..total {
        if i == line_count {    // Make measurement same as Java
            start_instant = Some(Instant::now())
        }
        matched += if regex.clone().is_match(&lines[i % line_count]) { 1 } else { 0 };
    }
    println!("{} out of {} lines matched, timing {} millis\n", matched, total, start_instant.unwrap().elapsed().subsec_nanos() / 1000000)
}
