extern crate regex;

use std::fs;

fn main() {
    let input = parse_input();
}

struct InputData {
    rows: Vec<String>,
}

impl InputData {
}

fn parse_input() -> InputData {
    let content = fs::read_to_string("inputs/day20.txt").unwrap();
    InputData {
        rows: content.split("\n").map(|s| s.to_string()).collect(),
    }
}

