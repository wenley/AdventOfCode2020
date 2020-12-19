extern crate regex;

use std::io;
use std::fs;

fn main() {
    let input = parse_input();
}

struct InputData {
    rule_set: RuleSet,
    messages: Vec<String>,
}

enum Rule {
    Char(char),
    Single(usize, usize),
    Pair((usize, usize), (usize, usize)),
}

struct RuleSet {
    rules: Vec<Rule>,
}

fn parse_input() -> io::Result<InputData> {
    let content = fs::read_to_string("inputs/day19.txt")?;
    let mut parts: Vec<_> = content.split("\n\n").collect();
    let messages = parts.pop().unwrap().split("\n").map(|s| s.to_string()).collect();
    let rules = parts.pop().unwrap();

    Ok(InputData {
        rule_set: RuleSet { rules: vec![] },
        messages: messages,
    })
}

