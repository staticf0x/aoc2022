use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut inventory: Vec<Vec<u32>> = Vec::new();

    let mut tmp: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line_str = line.unwrap();

        if line_str.is_empty() {
            inventory.push(tmp.clone());
            tmp.clear();
            continue;
        }

        let calories: u32 = line_str.parse().unwrap();
        tmp.push(calories);
    }

    let mut sums: Vec<u32> = inventory
        .iter()
        .map(|one_elf| one_elf.iter().sum())
        .collect();

    sums.sort();

    println!("{}", sums.last().unwrap());
    println!("{}", sums.iter().rev().take(3).sum::<u32>());
}
