use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    // File handler
    let file = File::open("real-set.txt").expect("File not found.");
    let buf_reader = BufReader::new(file);

    // Create vector for elves
    let mut elf_vector: Vec<u64> = Vec::new();

    // Sum variable for each elf
    let mut elf_sum: u64 = 0;

    // Read file lines
    for line in buf_reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            elf_sum += line.parse::<u64>().unwrap();
        } else {
            elf_vector.push(elf_sum);
            elf_sum = 0;
        }
    }

    // Add last elf
    elf_vector.push(elf_sum);

    // Sort by decreasing order
    elf_vector.sort_by(|elf_one, elf_two| elf_two.cmp(elf_one));

    let mut top_three_sum = 0;

    for index in 0..3 {
        top_three_sum += elf_vector[index];
        println!("Elf {}: {}", index + 1, elf_vector[index]);
    }

    println!("Total: {}", top_three_sum);

    // Return success
    Ok(())
}
