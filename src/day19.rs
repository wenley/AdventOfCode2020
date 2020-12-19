extern crate regex;

use std::io;
use std::fs;
use std::collections::HashMap;

fn main() {
    let input = parse_input().unwrap();
    let matching_messages = input.messages.iter().filter(|message| {
        input.rule_set.matches_rule(&0, message)
    }).count();

    println!("{} messages match rule 0", matching_messages);

    let mut test_rules = HashMap::new();
    test_rules.insert(0, Rule::Char('a'));
    test_rules.insert(1, Rule::Sequence(vec![vec![0, 1, 2], vec![0, 2]]));
    test_rules.insert(2, Rule::Char('b'));
    test_rules.insert(3, Rule::Sequence(vec![vec![0, 3], vec![0]]));
    let rule_set = RuleSet { rules: test_rules };
    let test_messages = vec![
        "aaaabbbb",
        "aaa",
    ];
    test_messages.iter().for_each(|message| {
        let matches = rule_set.matches_rule(&1, message);
        println!("{} matches? {}", message, matches);
    });
}

struct InputData {
    rule_set: RuleSet,
    messages: Vec<String>,
}

#[derive(Debug)]
enum Rule {
    Char(char),
    Sequence(Vec<Vec<usize>>),
}

struct RuleSet {
    rules: HashMap<usize, Rule>,
}

impl RuleSet {
    fn matches_rule<'a>(&self, index: &usize, message: &'a str) -> bool {
        self.parsings_for_rule(index, message).iter().any(|s| s == &"")
    }

    fn parsings_for_rule<'a>(&self, index: &usize, message: &'a str) -> Vec<&'a str> {
        match self.rules.get(index) {
            None => panic!("Unknown rule index {}", index),
            Some(Rule::Char(c)) => {
                match message.chars().nth(0) {
                    None => vec![],
                    Some(cc) => {
                        if cc == *c {
                            vec![&message[1..]]
                        } else {
                            vec![]
                        }
                    }
                }
            },
            Some(Rule::Sequence(indexes)) => {
                indexes.iter().flat_map(|sequence| {
                    sequence.iter().fold(
                        vec![message],
                        |remaining_possibilities, i| {
                            remaining_possibilities.
                                iter().
                                flat_map(|rest| self.parsings_for_rule(i, rest)).
                                collect()
                        },
                    )
                }).collect()
            }
        }
    }
}

fn parse_input() -> io::Result<InputData> {
    let content = fs::read_to_string("inputs/day19.txt")?;
    let mut parts: Vec<_> = content.split("\n\n").collect();
    let messages = parts.pop().unwrap().split("\n").map(|s| s.to_string()).collect();
    let rule_lines = parts.pop().unwrap();

    let rules = rule_lines.split("\n").map(|line| {
        let mut parts: Vec<_> = line.split(": ").collect();
        let numbers_or_char = parts.pop().unwrap();
        let index: usize = parts.pop().and_then(|digits| digits.parse().ok()).unwrap();
        let rule = match numbers_or_char {
            r#""a""# => Rule::Char('a'),
            r#""b""# => Rule::Char('b'),
            _ => {
                let sequences = numbers_or_char.split(" | ").map(|chunk| {
                    chunk.split(" ").map(|digits| digits.parse().unwrap()).collect()
                }).collect();
                Rule::Sequence(sequences)
            }
        };
        (index, rule)
    }).collect();

    Ok(InputData {
        rule_set: RuleSet { rules: rules },
        messages: messages,
    })
}

