extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input = parse_input();
    let buses = input.buses;
    let mut rules: Vec<Rule> = buses.
        iter().
        enumerate().
        map(|(i, b)| {
            match b {
                Constraint::Bus(num) => {
                    let p = *num as u64;
                    let i = i as u64;
                    let remainder = (p - (i % p)) % p;
                    println!("x + {} === 0 mod {}", i, p);
                    println!("x === {} mod {}", remainder, p);
                    Rule {
                        remainder: remainder,
                        modulo: p,
                    }
                }
                Constraint::Blank => Rule { remainder: 0, modulo: 1 },
            }
        }).
        collect();
    // Make them ascending to sieve from the back;
    rules.sort_by_key(|r| r.modulo);

    let mut rule = Rule { remainder: 0, modulo: 1 };
    while let Some(next_rule) = rules.pop() {
        println!("Coalescing {:?} and {:?}", rule, next_rule);
        rule = rule.merge(&next_rule);
        println!("Found partial solution: {:?}", rule);
    }

    println!("{:?}", rule);
}

#[derive(Debug)]
struct Rule {
    remainder: u64,
    modulo: u64,
}
impl Rule {
    fn matches(&self, x: &u64) -> bool {
        x % self.modulo == self.remainder
    }

    fn merge(&self, other: &Rule) -> Rule {
        if self.modulo < other.modulo { return other.merge(self) }

        let new_modulo = self.modulo * other.modulo;
        let new_remainder = (self.remainder..).step_by(self.modulo as usize).filter(|i| other.matches(i)).nth(0).unwrap() % new_modulo;

        Rule {
            remainder: new_remainder,
            modulo: new_modulo,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Constraint {
    Bus(usize),
    Blank,
}

struct InputData {
    buses: Vec<Constraint>,
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("inputs/day13.txt");
    match io_result {
        Ok(mut lines) => {
            let _ = lines.next();
            let numbers = lines.next().unwrap().unwrap().split(",").map(|s| {
                match s {
                    "x" => Constraint::Blank,
                    _ => Constraint::Bus(s.parse().unwrap()),
                }
            }).collect();

            InputData {
                buses: numbers,
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

