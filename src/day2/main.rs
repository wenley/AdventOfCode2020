extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let io_result = lines_in_file("day1input.txt");
    let mut numbers = HashSet::new();
    match io_result {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(stuff) => {

                    }
                    Err(e) => panic!("Error reading line"),
                }
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

struct PasswordRule {
    min_count: i64,
    max_count: i64,
    letter: char,
    test_password: &str,
}

fn parse_line(line: &str) -> PasswordRule {
    const parse_regex = None;
}
