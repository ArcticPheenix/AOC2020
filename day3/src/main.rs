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
    let mut record_index = 0;
    let mut tree_count = 0;
    for i in 0..input_vec.len() {
        if i == 0 {
            continue;
        }
        let record = input_vec[i].to_string();
        record_index += 3;
        if record_index >= record.len() {
            record_index = record_index % record.len();
        }
        if is_tree(record, record_index) {
            tree_count += 1;
        }
    }
    println!("Total trees hit: {}", tree_count);
}

fn is_tree(record: String, index: usize) -> bool {
    let mut hit = false;
    let mut string = String::new();
    let mut i = 0;
    let chars: Vec<char> = record.chars().collect();
    for c in chars {
        if i == index && c == '#' {
            string.push('X');
            i += 1;
            hit = true;
            continue;
        }
        if i == index && c == '.' {
            string.push('O');
            i += 1;
            continue;
        }
        string.push(c);
        i += 1;
    }
    println!("{}", string);

    hit
}