use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read input file
    let filename = "./input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let sum = 2020;

    // Convert input lines to numbers (i32)
    let mut input_vec = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let value = line.parse::<i32>().unwrap();
        input_vec.push(value);
    }

    let result1 = sum_of_two(&mut input_vec, &sum);
    let result2 = sum_of_three(&mut input_vec, &sum);
    println!("Answer #1 found! {} x {} = {}", result1.0, result1.1, result1.0 * result1.1);
    println!("Answer #2 found! {} x {} x {} = {}", result2.0, result2.1, result2.2, result2.0 * result2.1 * result2.2);
}

fn sum_of_two(input: &mut Vec<i32>, sum: &i32) -> (i32, i32) {
    let mut first_num = 0;
    let mut second_num = 0;
    let mut found = false;
    for i in 0..input.len() {
        if found { break; }
        for j in 0..input.len() {
            first_num = input[i];
            if first_num + input[j] == *sum {
                second_num = input[j];
                found = true;
                break;
            }
        }
    }
    (first_num, second_num)
}

fn sum_of_three(input: &mut Vec<i32>, sum: &i32) -> (i32, i32, i32) {
    let mut first_num = 0;
    let mut second_num = 0;
    let mut third_num = 0;
    let mut found = false;
    for i in 0..input.len() {
        if found { break; }
        for j in 0..input.len() {
            if found { break; }
            for k in 0..input.len() {
                first_num = input[i];
                second_num = input[j];
                if first_num + second_num + input[k] == *sum {
                    third_num = input[k];
                    found = true;
                    break;
                }
            }
        }
    }
    (first_num, second_num, third_num)
}