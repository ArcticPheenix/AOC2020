use std::fs::File;
use std::io::{BufRead, BufReader};

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String
}

impl Passport {
    fn is_valid(&self) -> bool {
        if self.byr != "".to_string() 
        && self.iyr != "".to_string()
        && self.eyr != "".to_string()
        && self.hgt != "".to_string()
        && self.hcl != "".to_string()
        && self.ecl != "".to_string()
        && self.pid != "".to_string() {
            return true;
        }
        false
    }

    fn new(& mut self) {
        self.byr = "".to_string();
        self.iyr = "".to_string();
        self.eyr = "".to_string();
        self.hgt = "".to_string();
        self.hcl = "".to_string();
        self.ecl = "".to_string();
        self.pid = "".to_string();
        self.cid = "".to_string();
    }
}

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
}
