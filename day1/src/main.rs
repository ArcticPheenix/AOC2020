use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // TODO - Read input file
    let filename = "./input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let sum = 2020;

    // TODO - Convert input lines to numbers (i32)
    let mut input_vec = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let value = line.parse::<i32>().unwrap();
        input_vec.push(value);
    }

    // TODO - Iterate and find the two numbers that sum to 2020.
    let mut first_num = 0;
    let mut second_num = 0;
    let mut found = false;
    for i in 0..input_vec.len() {
        if found { break; }
        for j in 0..input_vec.len() {
            first_num = input_vec[i];
            if first_num + input_vec[j] == sum {
                second_num = input_vec[j];
                found = true;
                break;
            }
        }
    }

    // TODO - Multiply the two numbers together.
    println!("Answer found! {} x {} = {}", first_num, second_num, first_num * second_num);
}
