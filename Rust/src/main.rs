use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("real-set.txt").expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut total = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let (first, second) = line.split_at(line.chars().count() / 2);

        let mut ignore: Vec<char> = Vec::new();
        let mut bad_list: Vec<char> = Vec::new();

        for c1 in first.chars() {
            for c2 in second.chars() {
                if c1 == c2 && !ignore.contains(&c1) {
                    ignore.push(c1);
                    bad_list.push(c1);
                    // println!("Wrong: {}", c1);
                }
            }
        }

        // lowercase (97-122) : ord - 96 = priority
        // uppercase (65-90)  : ord - 38 = priority

        for c in bad_list {
            if c as u32 >= 97 && c as u32 <= 122 {
                total += c as u32 - 96;
            } else if c as u32 >= 65 && c as u32 <= 90 {
                total += c as u32 - 38;
            } else {
                println!("Error: {}", c);
            }
        }
    }

    println!("Part 1: {}", total);
}
