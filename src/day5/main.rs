extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::fmt;
use std::collections::HashSet;

fn main() {
    let input = parse_input();
    let val = "FBFBBFFRLR";
    println!("{} => {}", val, line_to_seat_number(val));

    let min = input.taken_seats.iter().min().unwrap();
    let max = input.taken_seats.iter().max().unwrap();
    println!("Max seat is {}", max);
    for i in (*min..*max) {
        if !input.taken_seats.contains(&i) {
            println!("My seat is {}", i);
            break;
        }
    }
}

struct InputData {
    taken_seats: HashSet<usize>,
}

impl InputData {
}

fn line_to_seat_number(line: &str) -> usize {
    let mut seat_number = 0;

    for c in line.chars() {
        match c {
            'F' => { seat_number *= 2; }
            'B' => { seat_number *= 2; seat_number += 1 }
            'R' => { seat_number *= 2; seat_number += 1 }
            'L' => { seat_number *= 2; }
            _ => {}
        }
    }

    seat_number
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("day5input.txt");
    match io_result {
        Ok(lines) => {
            let taken_seats = lines.map(|line| match line {
                Ok(stuff) => {
                    line_to_seat_number(&stuff)
                }
                Err(_) => panic!("Error reading line"),
            }).collect();
            InputData {
                taken_seats: taken_seats,
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

