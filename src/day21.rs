extern crate regex;

use std::fs;
use regex::Regex;

fn main() {
    let input = parse_input();
}

struct Row {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

struct InputData {
    rows: Vec<Row>,
}

impl InputData {
}

fn parse_input() -> InputData {
    let content = fs::read_to_string("inputs/day20.txt").unwrap();
    let stuff_regex = Regex::new(r"^(?P<ingredients>[a-z ]+) \(contains (?P<allergens>[a-z, ]+)\)$").unwrap();
    let rows = content.split("\n").map(|line| {
        let caps = stuff_regex.captures(line).unwrap();
        let ingredients = caps.name("ingredients").unwrap().as_str().split(" ").map(|s| s.to_string()).collect();
        let allergens = caps.name("allergens").unwrap().as_str().split(", ").map(|s| s.to_string()).collect();

        Row {
            ingredients: ingredients,
            allergens: allergens,
        }
    }).collect();
    InputData {
        rows: rows,
    }
}

