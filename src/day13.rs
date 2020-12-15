extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input = parse_input();

    match input.buses.iter().map(|i| (i, i - (input.earliest_departure % i))).min_by_key(|(_bus, wait)| *wait) {
        Some((bus, wait)) => println!("{} * {} = {}", bus, wait, bus * wait),
        None => panic!("Couldn't find bus??"),
    };
}

struct InputData {
    earliest_departure: usize,
    buses: Vec<usize>,
}

impl InputData {
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("inputs/day13.txt");
    match io_result {
        Ok(mut lines) => {
            let earliest_departure = lines.next().unwrap().unwrap().parse().unwrap();
            let numbers = lines.next().unwrap().unwrap().split(",").filter(|s| *s != "x").map(|s| s.parse().unwrap()).collect();

            InputData {
                earliest_departure: earliest_departure,
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

