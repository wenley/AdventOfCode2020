extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input = parse_input();
    let buses = input.buses;
    let other_buses = &buses[1..];
    let product = buses.iter().map(|b| match b {
        Constraint::Bus(x) => *x,
        Constraint::Blank => 1,
    }).fold(1, |acc, num| acc * num);
    println!("product = {}", product);
    match buses[0] {
        Constraint::Bus(x) => {
            for i in (0..product).step_by(x) {
                let times = i..(i + other_buses.len());
                if times.zip(other_buses.iter()).all(|(t, bus)| bus.matches(t)) {
                    println!("Time t = {} works!", i);
                    break;
                }
            }
        },
        Constraint::Blank => panic!("First bus is not a bus"),
    }
}

enum Constraint {
    Bus(usize),
    Blank,
}

impl Constraint {
    fn matches(&self, timestamp: usize) -> bool {
        match self {
            Constraint::Blank => true,
            Constraint::Bus(i) => timestamp % i == 0,
        }
    }
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

