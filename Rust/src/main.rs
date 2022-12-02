use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("real-set.txt").expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut total_score: u32 = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        let (mut elf, mut player) = line.split_at(1);
        elf = elf.trim();
        player = player.trim();

        match (elf, player) {
            ("A", "X") => total_score += 3 + 1,
            ("A", "Y") => total_score += 6 + 2,
            ("A", "Z") => total_score += 0 + 3,
            ("B", "X") => total_score += 0 + 1,
            ("B", "Y") => total_score += 3 + 2,
            ("B", "Z") => total_score += 6 + 3,
            ("C", "X") => total_score += 6 + 1,
            ("C", "Y") => total_score += 0 + 2,
            ("C", "Z") => total_score += 3 + 3,
            _ => println!("Invalid input"),
        }
    }
    println!("Total Score: {}", total_score);
}
