extern crate regex;

use std::fs;
use regex::Regex;

fn main() {
    let input_rules = parse_input();
    let valid_rows = input_rules.iter().filter(|rule| rule.valid()).count();
    let valid_rows_2 = input_rules.iter().filter(|rule| rule.valid_2()).count();
    println!("{} rows passed Part 1 rules", valid_rows);
    println!("{} rows passed Part 2 rules", valid_rows_2);
}

struct PasswordRule {
    x: usize,
    y: usize,
    letter: char,
    test_password: String,
}

fn parse_input() -> Vec<PasswordRule> {
    let content = fs::read_to_string("inputs/day2.txt").unwrap();
    let parse_regex: Regex = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<pass>[a-z]+)$").unwrap();
    content.split("\n").map(|line| {
        parse_regex.captures(line).map(|cap| {
            PasswordRule {
                x: cap.name("min").and_then(|m| m.as_str().parse().ok()).unwrap(),
                y: cap.name("max").and_then(|m| m.as_str().parse().ok()).unwrap(),
                letter: cap.name("letter").and_then(|m| m.as_str().chars().next()).unwrap(),
                test_password: cap.name("pass").unwrap().as_str().to_string(),
            }
        }).unwrap()
    }).collect()
}

impl PasswordRule {
    fn valid(&self) -> bool {
        let letter_counts = self.test_password.chars().filter(|c| *c == self.letter).count();

        letter_counts >= self.x && letter_counts <= self.y
    }

    fn valid_2(&self) -> bool {
        let position1_matches = match self.test_password.chars().nth(self.x - 1) {
            Some(c) => c == self.letter,
            None => {
                eprintln!("index {} is longer than {} ({} long)", self.x, self.test_password, self.test_password.len());
                false
            }
        };
        let position2_matches = match self.test_password.chars().nth(self.y - 1) {
            Some(c) => c == self.letter,
            None => {
                eprintln!("index {} is longer than {} ({} long)", self.y, self.test_password, self.test_password.len());
                false
            }
        };

        position1_matches ^ position2_matches
    }
}
