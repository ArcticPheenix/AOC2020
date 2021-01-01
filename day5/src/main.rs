use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    const AIRPLANE_ROW_SIZE: i32 = 128;
    const AIRPLANE_COLUMN_SIZE: i32 = 8;

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

    let mut highest_seat_id = 0;
    for pass in input_vec {
        let result = process_boarding_pass(AIRPLANE_ROW_SIZE, AIRPLANE_COLUMN_SIZE, pass);
        if result.2 > highest_seat_id {
            highest_seat_id = result.2
        }
    }
    println!("Highest Seat ID: {}", highest_seat_id);
}

fn process_boarding_pass(plane_row_size: i32, plane_column_size: i32, pass: String) -> (i32, i32, i32) {
    // Process boarding pass using binary space partitioning.
    let mut row_min = 0;
    let mut row_max = plane_row_size - 1;
    let mut column_min = 0;
    let mut column_max = plane_column_size - 1;
    for c in pass.chars() {
        match c {
            'F' => {
                // Take (row_max - row_min) and divide it by two. Add the answer to the min, and that becomes the new max.
                row_max = ((row_max - row_min) / 2) + row_min;
            }
            'B' => {
                // Take (row_max - row_min) and divide it by two. Add the answer to the min, and then add one to create the new min.
                row_min = ((row_max - row_min) / 2) + row_min + 1;
            }
            'L' => {
                // Take (column_max - column_min) and divide it by two. Add the answer to the min, and that becomes the new max.
                column_max = ((column_max - column_min) / 2) + column_min;
            }
            'R' => {
                // Take (column_max - column_min) and divide it by two. Add the answer to the min, and then add one to create the new min.
                column_min = ((column_max - column_min) / 2) + column_min + 1;
            }
            _ => continue
        }
    }

    assert_eq!(row_max, row_min);
    assert_eq!(column_max, column_min);
    
    (row_max, column_max, (row_max * 8) + column_max)
}
