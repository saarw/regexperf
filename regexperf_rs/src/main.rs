extern crate regex;
extern crate time;

use time::precise_time_ns;
use regex::RegexBuilder;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;

fn main() {
    let pattern = "^[a-zA-Z0-9_\\s\\r\\n\\t]*$";
    let regex = RegexBuilder::new(pattern).dfa_size_limit(0).build().unwrap();
    let lines: Vec<String> = BufReader::new(File::open(env::args().nth(1)
        .expect("Expected file path argument"))
        .expect("File did not exist"))
        .lines()
        .map(|l| l.expect("I/O error"))
        .collect();
    let mut start_time = 0;
    let mut matched: usize = 0;
    let line_count = lines.len();
    let total = line_count * 20;
    let batch_size = env::args().nth(2).map_or(1, |b| b.parse::<usize>().expect("second argument should be a number"));
    let mut cloned = regex.clone(); // Need initial value
    for i in 0..total {
        if i == line_count {    // Make measurement same as Java
            start_time = precise_time_ns();
        }
        if i % batch_size == 0 {
            cloned = regex.clone();
        }
        matched += if cloned.is_match(&lines[i % line_count]) { 1 } else { 0 };
    }
    let elapsed = precise_time_ns() - start_time;
    println!("{} out of {} lines matched, with batch size {}, timing {} ms ({} ns per match)", matched, total, batch_size, elapsed / 1000000, elapsed / ((total-line_count) as u64))
}
