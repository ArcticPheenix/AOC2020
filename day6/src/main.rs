use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

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

    let count_1 = process(&input_vec, 1);
    let count_2 = process(&input_vec, 2);

    println!("Question Count (1): {}", count_1);
    println!("Question Count (2): {}", count_2);
}

fn process(input: &Vec<String>, challenge_num: i32) -> i32 {
    //        Process the input file.
    //        Each "group" is separated by an empty line.
    //        Iterate on the characters for each line, adding the characters to a HashSet (to ensure uniqueness).
    //        When a new line is encountered, add the set to a Vector and then create a new set for the next group.
    //        After processing is complete, output the sum of the number of "yes" answers there are to STDOUT.
    let mut count = 0;
    if challenge_num == 1 {
        let mut set_vec: Vec<HashSet<char>> = Vec::new();
        let mut char_set: HashSet<char> = HashSet::new();
        for line in input {
            if *line == "".to_string() {
                set_vec.push(char_set);
                char_set = HashSet::new();
                continue;
            }
            for c in line.chars() {
                char_set.insert(c);
            }
        }
        // Final push to set_vec
        set_vec.push(char_set);
        for set in set_vec {
            count += set.len() as i32;
        }
    }
    if challenge_num == 2 {
        let mut set_vec: Vec<HashSet<char>> = Vec::new();
        let mut char_set: HashSet<char> = HashSet::new();
        for line in input {
            if *line == "".to_string() {
                set_vec.push(char_set);
                char_set = HashSet::new();
                continue;
            }
            for c in line.chars() {
                if char_set.insert(c) {

                }
            }
        }
    }
    
    count
}
