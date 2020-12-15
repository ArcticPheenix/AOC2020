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
    let mut count = 0;
    for record in input_vec.iter() {
        let parsed_record = parse_record(record);
        let valid = compare_old(parsed_record.0, parsed_record.1, parsed_record.2, parsed_record.3);
        if valid {
            count += 1;
        }
    }
    println!("Total count of valid passwords: {}", count);
}

// Implement custom parsing logic for input file lines.
fn parse_record(record: &String) -> (i32, i32, String, String) {
    // 1 - Split on ':' to separate between the policy and the password.
    let record_split: Vec<&str> = record.split(":").collect();
    // Strip whitespace from password.
    let password_vec: Vec<&str> = record_split[1].split_whitespace().collect();

    // 2 - Break up the policy portion into three discrete bits (min, max, policy char).
    // Split between min/max and policy char.
    let policy_vec: Vec<&str> = record_split[0].split_whitespace().collect();
    // Split min/max into their own bits.
    let policy_min_max: Vec<&str> = policy_vec[0].split("-").collect();

    let password = password_vec[0].to_string();

    let min: i32 = policy_min_max[0].parse().unwrap();
    let max: i32 = policy_min_max[1].parse().unwrap();

    (min, max, policy_vec[1].to_string(), password)
}

// Compare password with password policy and return true if password complies with policy.
fn compare_old(min: i32, max: i32, character: String, password: String) -> bool {
    let mut c: char = '-';
    for ch in character.chars() {
        c = ch;
    }
    let mut count = 0;
    for char in password.chars() {
        if char == c {
            count += 1;
        }
    }
    if count >= min && count <= max {
        return true;
    }
    
    false
}