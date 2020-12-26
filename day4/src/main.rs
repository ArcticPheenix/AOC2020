use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

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
        // RegEx for matching years.
        let year_re = Regex::new(r"^\d{4}$").unwrap();
        // RegEx for matching height.
        let hgt_cm_re = Regex::new(r"^\d{3}cm$").unwrap();
        let hgt_in_re = Regex::new(r"^\d{2}in$").unwrap();
        // RegEx for matching hair color.
        let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        // RegEx for matching Passport ID.
        let pid_re = Regex::new(r"^\d{9}$").unwrap();

        let byr_min_max = (1920, 2002);
        let iyr_min_max = (2010, 2020);
        let eyr_min_max = (2020, 2030);
        let hgt_cm_min_max = (150, 193);
        let hgt_in_min_max = (59, 76);

        let mut hgt_cm: i32 = 0;
        let mut hgt_in: i32 = 0;
        if hgt_cm_re.is_match(&self.hgt) {
            let hgt: Vec<&str> = self.hgt.split("cm").collect();
            hgt_cm = hgt[0].parse().unwrap();
        }
        if hgt_in_re.is_match(&self.hgt) {
            let hgt: Vec<&str> = self.hgt.split("in").collect();
            hgt_in = hgt[0].parse().unwrap();
        }

        // Determine if byr is valid.
        let mut byr_valid = {
            self.byr != "".to_string()
            && year_re.is_match(&self.byr)
        };
        if byr_valid {
            let byr: i32 = self.byr.parse().unwrap();
            if byr > byr_min_max.0 && byr < byr_min_max.1 {
                byr_valid = true;
            } else {
                byr_valid = false;
            }
        }
        // Determine if iyr is valid.
        let mut iyr_valid = {
            self.iyr != "".to_string()
            && year_re.is_match(&self.iyr)
        };
        if iyr_valid {
            let iyr: i32 = self.iyr.parse().unwrap();
            if iyr > iyr_min_max.0 && iyr < iyr_min_max.1 {
                iyr_valid = true;
            } else {
                iyr_valid = false;
            }
        }
        // Determine if eyr is valid.
        let mut eyr_valid = {
            self.eyr != "".to_string()
            && year_re.is_match(&self.eyr)
        };
        if eyr_valid {
            let eyr:i32  = self.eyr.parse().unwrap();
            if eyr > eyr_min_max.0 && eyr < eyr_min_max.1 {
                eyr_valid = true;
            } else {
                eyr_valid = false;
            }
        }
        // Determine if hgt is valid.
        let mut hgt_valid = false;
        if hgt_cm > 0 {
            if hgt_cm > hgt_cm_min_max.0 && hgt_cm < hgt_cm_min_max.1 {
                hgt_valid = true;
            }
        }
        if hgt_in > 0 {
            if hgt_in > hgt_in_min_max.0 && hgt_in < hgt_in_min_max.1 {
                hgt_valid = true;
            }
        }
        // Determine if ecl is valid.
        // Hair color (hcl) can only be one of the following values:
        // amb blu brn gry grn hzl oth
        let mut ecl_valid = false;
        let ecl: &str = &self.ecl;
        match ecl {
            "amb" => ecl_valid = true,
            "blu" => ecl_valid = true,
            "brn" => ecl_valid = true,
            "gry" => ecl_valid = true,
            "grn" => ecl_valid = true,
            "hzl" => ecl_valid = true,
            "oth" => ecl_valid = true,
            _ => ecl_valid = false
        }

        if byr_valid
        && iyr_valid
        && eyr_valid
        && hgt_valid
        && ecl_valid
        && hcl_re.is_match(&self.hcl)
        && pid_re.is_match(&self.pid) {
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
