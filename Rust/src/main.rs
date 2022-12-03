use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("test-set.txt").expect("File not found!");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let (first, second)
    }
}
