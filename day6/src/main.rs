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
}
