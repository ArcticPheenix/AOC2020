use std::fs::File;
use std::io::{BufRead, BufReader};

struct Slope {
    x: i32,
    y: i32
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
    
    let mut tree_vec: Vec<i32> = Vec::new();
    let slope_vec = vec![Slope{x: 1, y: 1}, Slope{x: 3, y: 1}, Slope{x: 5, y: 1}, Slope{x: 7, y: 1}, Slope{x: 1, y: 2}];
    for slope in slope_vec {
        println!("Slope: {}/{}", slope.x, slope.y);
        let mut record_index = 0;
        let mut tree_count = 0;
        let mut skipped_count = 0;
        for i in 0..input_vec.len() {
            let record = input_vec[i].to_string();
            if i == 0 {
                println!("{}", record);
                continue;
            }
            if skipped_count < slope.y - 1 {
                skipped_count += 1;
                println!("{}", record);
                continue;
            }
            record_index += slope.x as usize;
            if record_index >= record.len() {
                record_index = record_index % record.len();
            }
            if is_tree(record, record_index) {
                tree_count += 1;
            }
            skipped_count = 0;
        }
        println!("Tree count for Slope {}/{}: {}", slope.x, slope.y, tree_count);
        tree_vec.push(tree_count);
    }

    let mut tree_count:i64 = 0;
    for tc in tree_vec {
        if tree_count == 0 {
            tree_count = tc as i64;
        } else {
            tree_count = tree_count * tc as i64;
        }
    }
    println!("Tree count: {}", tree_count);
}

fn is_tree(record: String, index: usize) -> bool {
    let mut hit = false;
    let mut string = String::new();
    let mut i = 0;
    let chars: Vec<char> = record.chars().collect();
    for c in chars {
        if i == index && c == '#' {
            string.push('X');
            i += 1;
            hit = true;
            continue;
        }
        if i == index && c == '.' {
            string.push('O');
            i += 1;
            continue;
        }
        string.push(c);
        i += 1;
    }
    println!("{}", string);

    hit
}