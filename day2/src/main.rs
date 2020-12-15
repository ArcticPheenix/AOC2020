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
    let mut count_old = 0;
    let mut count_new = 0;
    for record in input_vec.iter() {
        let parsed_record_old = parse_record(record);
        let parsed_record_new = parsed_record_old.clone();
        let valid_old = compare_old(parsed_record_old.0, parsed_record_old.1, parsed_record_old.2, parsed_record_old.3);
        if valid_old {
            count_old += 1;
        }
        let valid_new = compare_new(parsed_record_new.0, parsed_record_new.1, parsed_record_new.2, parsed_record_new.3);
        if valid_new {
            count_new += 1;
        }
    }
    println!("Total count of valid old passwords: {}", count_old);
    println!("Total count of valid new passwords: {}", count_new);
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

fn compare_new(min: i32, max: i32, character: String, password: String) -> bool {
    let mut c: char = '-';
    for ch in character.chars() {
        c = ch;
    }
    let min = min as usize;
    let max = max as usize;
    let password_min = password.chars().nth(min - 1).unwrap();
    let password_max = password.chars().nth(max - 1).unwrap();
    if password_min == c && password_max != c {
        return true;
    }
    if password_min != c && password_max == c {
        return true;
    }
    
    false
}