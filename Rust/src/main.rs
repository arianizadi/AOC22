use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("real-set.txt").expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut total = 0;
    let mut map: HashMap<char, u8> = HashMap::new();
    let mut counter = 0;
    let mut total_badge = 0;

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

        let mut line_chars: Vec<char> = line.chars().collect();
        line_chars.sort();
        line_chars.dedup();

        for c in line_chars {
            map.entry(c).and_modify(|a| *a += 1).or_insert(1);
        }

        if counter == 2 {
            for (key, value) in &map {
                if *value == 3 as u8 {
                    if *key as u32 >= 97 && *key as u32 <= 122 {
                        total_badge += *key as u32 - 96;
                    } else if *key as u32 >= 65 && *key as u32 <= 90 {
                        total_badge += *key as u32 - 38;
                    } else {
                        println!("Error: {}", key);
                    }
                    // println!("{} / {}", key, value);
                    // println!("Badge: {}", total_badge);
                }
            }
            map.clear();
            counter = -1;
        }
        counter += 1;
    }

    println!("Part 1: {}", total);

    // for (key, value) in &map {
    //     if *value == 3 as u8 {
    //         // println!("{} / {}", key, value);
    //         if *key as u32 >= 97 && *key as u32 <= 122 {
    //             total_badge += *key as u32 - 96;
    //         } else if *key as u32 >= 65 && *key as u32 <= 90 {
    //             total_badge += *key as u32 - 38;
    //         } else {
    //             println!("Error: {}", key);
    //         }
    //     }
    // }

    println!("Part 2: {}", total_badge);
}
