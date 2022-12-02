use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("real-set.txt").expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut score_part1: u32 = 0;
    let mut score_part2: u32 = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let (mut elf, mut player) = line.split_at(1);
        elf = elf.trim();
        player = player.trim();

        match (elf, player) {
            ("A", "X") => score_part1 += 3 + 1,
            ("A", "Y") => score_part1 += 6 + 2,
            ("A", "Z") => score_part1 += 0 + 3,
            ("B", "X") => score_part1 += 0 + 1,
            ("B", "Y") => score_part1 += 3 + 2,
            ("B", "Z") => score_part1 += 6 + 3,
            ("C", "X") => score_part1 += 6 + 1,
            ("C", "Y") => score_part1 += 0 + 2,
            ("C", "Z") => score_part1 += 3 + 3,
            _ => println!("Invalid input"),
        }

        match (elf, player) {
            ("A", "X") => score_part2 += 0 + 3,
            ("A", "Y") => score_part2 += 3 + 1,
            ("A", "Z") => score_part2 += 6 + 2,
            ("B", "X") => score_part2 += 0 + 1,
            ("B", "Y") => score_part2 += 3 + 2,
            ("B", "Z") => score_part2 += 6 + 3,
            ("C", "X") => score_part2 += 0 + 2,
            ("C", "Y") => score_part2 += 3 + 3,
            ("C", "Z") => score_part2 += 6 + 1,
            _ => println!("Invalid input"),
        }
    }

    println!("Total Score (Part 1): {}", score_part1);
    println!("Total Score (Part 2): {}", score_part2);
}
