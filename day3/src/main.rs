use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read input file
    let filename = "./input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Grab record lines from input file and store them in a Vector.
    let mut input_vec = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        input_vec.push(line);
    }
    let mut index = 0;
    let mut tree_count = 0;
    for i in 0..input_vec.len() {
        if i == 0 {
            continue;
        }
        index += 3;
        if index >= 31 {
            index = 31 % 3
        }
        let record = input_vec[i].to_string();
        if is_tree(record, index) {
            tree_count += 1;
        }
    }
    println!("Total trees hit: {}", tree_count);
}

fn is_tree(record: String, index: i32) -> bool {
    let mut hit = false;
    println!("Index passed in: {}", index);
    println!("Record value: {}", record);
    let c = record.chars().nth(index as usize).unwrap();
    if c == '#' {
        hit = true;
        println!("Hit!");
    }
    hit
}