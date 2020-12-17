extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;
use std::ops::RangeInclusive;

fn main() {
    let input = parse_input();

    let invalid_sum: usize = input.other_tickets.iter().map(|ticket| {
        ticket.values.iter().filter(|v| !input.is_valid_value(v)).sum::<usize>()
    }).sum();

    println!("Invalid sum = {}", invalid_sum);
    let valid_tickets: Vec<_> = input.other_tickets.iter().filter(|ticket| {
        ticket.values.iter().all(|v| input.is_valid_value(v))
    }).collect();

    let mut index_to_values: HashMap<usize, Vec<_>> = HashMap::new();
    valid_tickets.iter().for_each(|ticket| {
        ticket.values.iter().enumerate().for_each(|(i, val)| {
            if index_to_values.contains_key(&i) {
                index_to_values.get_mut(&i).unwrap().push(val);
            } else {
                index_to_values.insert(i, vec![val]);
            }
        });
    });
    let mut rules = input.rules;
    let mut name_to_index: HashMap<_, _> = HashMap::new();
    while rules.len() > 0 {
        let mut solved_indexes = HashMap::new();
        index_to_values.iter().for_each(|(index, values)| {
            let mut candidates: Vec<_> = rules.iter().filter(|(name, rule)| {
                values.iter().all(|v| rule.is_valid_value(v))
            }).map(|(name, _)| name).collect();

            if candidates.len() == 1 {
                let name = candidates.pop().unwrap();
                solved_indexes.insert(index, name.to_string());
            }
        });

        if solved_indexes.len() == 0 {
            eprintln!("Couldn't solve any more!");
            eprintln!("Remaining rules: {:?}", rules);
            panic!("Stopping");
        } else {
            solved_indexes.iter().for_each(|(index, name)| {
                rules.remove(name);
                name_to_index.insert(name.to_string(), **index);
            });
        }
    }
    let my_ticket = input.my_ticket;
    println!("Index to rule {:?}", name_to_index);
    println!("My ticket: {:?}", my_ticket.values);
    let product = name_to_index.iter().filter(|(name, _)| {
        &name[..2] == "de"
    }).map(|(_, index)| {
        my_ticket.values[*index]
    }).fold(1, |acc, num| acc * num );
    println!("{}", product);
}

struct InputData {
    rules: HashMap<String, Rule>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

#[derive(Debug)]
struct Rule {
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}
impl Rule {
    fn is_valid_value(&self, value: &usize) -> bool {
        self.range1.contains(value) || self.range2.contains(value)
    }
}

impl InputData {
    fn is_valid_value(&self, value: &usize) -> bool {
        self.rules.values().any(|rule| rule.is_valid_value(value))
    }
}

struct Ticket {
    values: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
enum ParseState {
    Rules,
    MyTicket,
    OtherTickets
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("inputs/day16.txt");
    let rule_regex = Regex::new(r"^(?P<name>[^:]+): (?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)$").unwrap();
    match io_result {
        Ok(lines) => {
            let mut rules = HashMap::new();
            let mut parse_state = ParseState::Rules;
            let mut my_ticket = None;
            let mut tickets = vec![];

            lines.for_each(|line| match line {
                Ok(stuff) => {
                    match (parse_state, &stuff[..]) {
                        (ParseState::Rules, "") => {
                            parse_state = ParseState::MyTicket;
                        }
                        (ParseState::MyTicket, "") => {
                            parse_state = ParseState::OtherTickets
                        }
                        (ParseState::Rules, _) => {
                            let captures = rule_regex.captures(&stuff).unwrap();
                            let name = captures.name("name").unwrap().as_str().to_string();
                            let min1 = captures.name("min1").unwrap().as_str().parse().unwrap();
                            let max1 = captures.name("max1").unwrap().as_str().parse().unwrap();
                            let min2 = captures.name("min2").unwrap().as_str().parse().unwrap();
                            let max2 = captures.name("max2").unwrap().as_str().parse().unwrap();

                            rules.insert(name, Rule {
                                range1: min1..=max1,
                                range2: min2..=max2,
                            });
                        }
                        (ParseState::MyTicket, _) => {
                            if stuff.chars().nth(0).unwrap() == 'y' {
                            } else {
                                my_ticket = Some(parse_ticket(&stuff));
                                parse_state = ParseState::OtherTickets;
                            }
                        }
                        (ParseState::OtherTickets, _) => {
                            match stuff.chars().nth(0) {
                                None => {}
                                Some('n') => {}
                                _ => { tickets.push(parse_ticket(&stuff)); }
                            }
                        }
                        _ => {}
                    }
                }
                Err(_) => panic!("Error reading line"),
            });
            match my_ticket {
                None => panic!("Couldn't find my ticket"),
                Some(t) => InputData {
                    rules: rules,
                    my_ticket: t,
                    other_tickets: tickets,
                }
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn parse_ticket(line: &str) -> Ticket {
    Ticket {
        values: line.split(",").map(|s| s.parse().unwrap()).collect()
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

