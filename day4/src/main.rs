use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
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

    fn new() -> Self {
        Passport {
            byr: "".to_string(),
            iyr: "".to_string(),
            eyr: "".to_string(),
            hgt: "".to_string(),
            hcl: "".to_string(),
            ecl: "".to_string(),
            pid: "".to_string(),
            cid: "".to_string()
        }
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

    // Begin processing input data.
    let mut count = 0;
    let mut passport_count = 0;
    // TODO - Iterate over input_vec.
    //        Each line should be considered part of a passport record.
    //        Empty lines should be considered the terminator between passport records.
    let mut passport = Passport::new();
    for line in input_vec {
        if line == "".to_string() {
            passport_count += 1;
            if passport.is_valid() {
                println!("VALID - {:?}", passport);
                passport = Passport::new();
                count += 1;
                continue;
            }
            println!("INVALID - {:?}", passport);
            passport = Passport::new();
            continue;
        }
        let field_vec: Vec<&str> = line.split_whitespace().collect();
        for records in field_vec {
            let record_vec: Vec<&str> = records.split(":").collect();
            match record_vec[0] {
                "byr" => passport.byr = record_vec[1].to_string(),
                "iyr" => passport.iyr = record_vec[1].to_string(),
                "eyr" => passport.eyr = record_vec[1].to_string(),
                "hgt" => passport.hgt = record_vec[1].to_string(),
                "hcl" => passport.hcl = record_vec[1].to_string(),
                "ecl" => passport.ecl = record_vec[1].to_string(),
                "pid" => passport.pid = record_vec[1].to_string(),
                "cid" => passport.cid = record_vec[1].to_string(),
                _ => continue
            }
        }
    }
    passport_count += 1;
    if passport.is_valid() {
        println!("VALID - {:?}", passport);
        count += 1;
    } else {
        println!("INVALID - {:?}", passport);
    }

    println!("Total passports: {}", passport_count);
    println!("Count of valid passports: {}", count);
}
