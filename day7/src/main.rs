mod equation;
mod error;

use std::fs::File;
use std::io::Read;
use crate::equation::{Equation, Create};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let rows: Vec<&str> = contents.split("\n").collect();
    let mut data: Vec<Equation> = rows.iter()
                        .map(|row|  {
                            let row_parts: Vec<&str> = row.split(": ").collect();
                            let left_part: i64 = row_parts[0].parse::<i64>().unwrap();
                            let right_part = row_parts[1];
                            let right_vals: Vec<&str> = right_part.split(" ").collect();
                            let parts: Vec<i32> = right_vals.iter().map(|part| part.parse::<i32>().unwrap()).collect();
                            Equation::create(parts, left_part)
                        })
                        .collect();
    
    for eq in data.iter_mut() {
        println!("{:?}", eq);
        eq.is_solvable();
    }

    let final_sum: i64 = data.iter().filter(|eq| eq.is_solved).map(|eq| eq.result).sum();

    println!("Final sum: {}", final_sum);
}

// 1. read file
// 2. parse file into equations
// 3. evaluate equations
