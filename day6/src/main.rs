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

    // TODO - Process the input file.
    //        Each "group" is separated by an empty line.
    //        Iterate on the characters for each line, adding the characters to a HashSet (to ensure uniqueness).
    //        When a new line is encountered, add the set to a Vector and then create a new set for the next group.
    //        After processing is complete, output the sum of the number of "yes" answers there are to STDOUT.
    let mut set_vec: Vec<HashSet<char>> = Vec::new();
    let mut char_set: HashSet<char> = HashSet::new();
    for line in input_vec {
        if line == "".to_string() {
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
    let mut count = 0;
    for set in set_vec {
        count += set.len();
    }

    println!("Question Count: {}", count);
}
