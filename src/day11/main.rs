extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input = parse_input();
}

enum Spot {
    Floor,
    Empty,
    Occupied,
}

struct Ferry {
    spots: Vec<Vec<Spot>>,
}

struct InputData {
    ferry: Ferry,
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("day11input.txt");
    match io_result {
        Ok(lines) => {
            let spots = lines.map(|line| match line {
                Ok(stuff) => {
                    stuff.chars().map(|c| match c {
                        '.' => Spot::Floor,
                        'L' => Spot::Empty,
                        '#' => Spot::Occupied,
                        _ => panic!("Unknown char {}", c),
                    }).collect()
                }
                Err(_) => panic!("Error reading line"),
            }).collect();
            InputData {
                ferry: Ferry { spots: spots },
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

