extern crate regex;

use std::io;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = parse_input().unwrap();
    let matching_messages = input.messages.iter().filter(|message| {
        match input.rule_set.matches_rule(&0, message) {
            None => false,
            Some("") => true,
            Some(_) => false,
        }
    }).count();

    println!("{} messages match rule 0", matching_messages);

    let ordered_rules = &input.rule_set.order();
    let mut collapsed_rules: HashMap<usize, Vec<String>> = HashMap::new();
    for (index, rule) in ordered_rules.iter() {
        match rule {
            Rule::Char(c) => {
                collapsed_rules.insert(*index, vec![c.to_string()]);
            }
            Rule::Sequence(sequences) => {
                let collapsed = sequences.iter().flat_map(|indexes| {
                    indexes.iter().fold(vec![String::new()], |combos, i| {
                        match collapsed_rules.get(i) {
                            None => panic!("Couldn't find index {}", i),
                            Some(strings) => pairings(&combos, strings).iter().map(|(a, b)| {
                                let mut copy = String::new();
                                copy.push_str(a);
                                copy.push_str(b);
                                copy
                            }).collect::<Vec<_>>()
                        }
                    })
                }).collect();
                collapsed_rules.insert(*index, collapsed);
            }
        }
    }
    for (index, rule) in collapsed_rules {
        println!("{}: {:?}", index, &rule[0..10]);
    }

    // let mut test_rules = HashMap::new();
    // test_rules.insert(0, Rule::Char('a'));
    // test_rules.insert(1, Rule::Sequence(vec![vec![0, 1, 2], vec![0, 2]]));
    // test_rules.insert(2, Rule::Char('b'));
    // test_rules.insert(3, Rule::Sequence(vec![vec![0, 3], vec![0]]));
    // let rule_set = RuleSet { rules: test_rules };
    // let test_messages = vec![
    //     "aaaabbbb",
    //     "aaa",
    // ];
    // test_messages.iter().for_each(|message| {
    //     let matches = match rule_set.matches_rule(&1, message) {
    //         None => false,
    //         Some("") => true,
    //         Some(_) => false,
    //     };
    //     println!("{} matches? {}", message, matches);
    // });
}

fn pairings<'a, 'b, T, U>(left: &'a Vec<T>, right: &'b Vec<U>) -> Vec<(&'a T, &'b U)> {
    left.iter().flat_map(|left_item: &T| {
        right.iter().map(|right_item: &U| (left_item, right_item)).collect::<Vec<_>>()
    }).collect()
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
    fn order(&self) -> Vec<(usize, &Rule)> {
        let mut solved = HashSet::new();
        let mut result = vec![];

        while solved.len() < self.rules.len() {
            let solved_before = solved.len();
            self.rules.iter().for_each(|(index, rule)| {
                if solved.contains(index) {
                    return;
                }
                match rule {
                    Rule::Char(_) => {
                        solved.insert(index);
                        result.push((*index, rule));
                    }
                    Rule::Sequence(indexes) => {
                        if indexes.iter().flat_map(|is| is).collect::<HashSet<_>>().is_subset(&solved) {
                            solved.insert(index);
                            result.push((*index, rule));
                        }
                    }
                }
            });
            if solved_before == solved.len() {
                panic!("Couldn't add any more rules!");
            } else {
                println!("Now have {} rules", solved.len());
            }
        }

        result
    }

    fn matches_rule<'a>(&self, index: &usize, message: &'a str) -> Option<&'a str> {
        match self.rules.get(index) {
            None => panic!("Unknown rule index {}", index),
            Some(Rule::Char(c)) => {
                match message.chars().nth(0) {
                    None => None,
                    Some(cc) => {
                        if cc == *c {
                            Some(&message[1..])
                        } else {
                            None
                        }
                    }
                }
            },
            Some(Rule::Sequence(indexes)) => {
                indexes.iter().fold(
                    None,
                    |msg, sequence| {
                        msg.or_else(|| {
                            sequence.iter().fold(
                                Some(message),
                                |msg, i| {
                                    msg.and_then(|rest| self.matches_rule(i, rest))
                                },
                            )
                        })
                    },
                )
            }
        }
    }

    // fn parser_for_rule_0<'a, P>(&self) -> impl FnMut(&'a str) -> nom::IResult<&'a str, ()> {
    //     // let parsers = HashMap::new();
    //     map(char('0'), |_| ())
    // }

    // fn parser_for_rule<'a, 'b, P>(&self, index: usize, parsers: &'b mut HashMap<usize, P>) -> &'b P
    //     where P: FnMut(&'a str) -> nom::IResult<&'a str, ()>
    // {
    //     if let Some(p) = parsers.get(&index) {
    //         return p;
    //     }
    //     let parser = match self.rules.get(&index) {
    //         None => panic!("Couldn't find rule for index {}", index),
    //         Some(Rule::Char(c)) => map(char(*c), |_| ()),
    //         Some(Rule::Single(a, b)) => {
    //             map(
    //                 pair(self.parser_for_rule(*a, parsers), self.parser_for_rule(*b, parsers)),
    //                 |(_, _)| { () },
    //             )
    //         }
    //         Some(Rule::Pair((a, b), (c, d))) => {
    //             map(
    //                 alt((
    //                     pair(self.parser_for_rule(*a, parsers), self.parser_for_rule(*b, parsers)),
    //                     pair(self.parser_for_rule(*c, parsers), self.parser_for_rule(*d, parsers)),
    //                 )),
    //                 |(_, _)| { () },
    //             )
    //         }
    //     };
    //     parsers.insert(index, parser);
    //     parsers.get(&index).unwrap()
    // }
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

