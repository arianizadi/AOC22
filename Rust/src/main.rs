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

    // Print vector
    // for (pos, e) in elf_vector.iter().enumerate() {
    //     println!("Elf {} Total: {}", pos + 1, e);
    // }

    // Print largest elf
    let max_elf_calories = elf_vector.iter().max().unwrap();
    let max_elf_number = elf_vector.iter().position(|elf| elf == max_elf_calories).unwrap();

    println!("Elf {}: {}", max_elf_number, max_elf_calories);

    // Return success
    Ok(())
}
