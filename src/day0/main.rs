extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input = parse_input();
}

struct InputData {
    rows: Vec<String>,
}

impl InputData {
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("day3input.txt");
    match io_result {
        Ok(lines) => {
            let rows = lines.map(|line| match line {
                Ok(stuff) => {
                    stuff
                }
                Err(_) => panic!("Error reading line"),
            }).collect();
            InputData {
                rows: rows,
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

