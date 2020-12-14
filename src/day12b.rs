extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

fn main() {
    let input = parse_input();
    let instructions = input.rows;
    let mut position = Position {
        x: 0,
        y: 0,
        way_x: 10,
        way_y: 1,
    };

    for instruction in instructions {
        position.follow(&instruction);
    }

    println!("Final position: ({}, {})", position.x, position.y);
}

struct InputData {
    rows: Vec<Instruction>,
}

#[derive(PartialEq, Clone, Copy)]
enum Angle {
    Ninety,
    NegativeNinety,
    OneEighty,
}
use Angle::*;

impl Angle {
    fn to_left(&self) -> Angle {
        *self
    }

    fn to_right(&self) -> Angle {
        match self {
            OneEighty => *self,
            Ninety => NegativeNinety,
            NegativeNinety => Ninety,
        }
    }
}

enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(Angle),
    Right(Angle),
    Forward(usize),
}
use Instruction::*;

struct Position {
    x: i64, // Positive = East
    y: i64, // Positive = North
    way_x: i64, // Relative to ship
    way_y: i64, // Relative to ship
}

impl Position {
    fn follow(&mut self, instruction: &Instruction) {
        match instruction {
            North(dy) => { self.way_y += *dy as i64 }
            South(dy) => { self.way_y -= *dy as i64 }
            East(dx) => { self.way_x += *dx as i64 }
            West(dx) => { self.way_x -= *dx as i64 }
            Forward(d) => {
                for _ in 0..*d {
                    self.x += self.way_x;
                    self.y += self.way_y;
                }
            }
            Left(angle) => { self.turn(angle.to_left()) }
            Right(angle) => { self.turn(angle.to_right()) }
        }
    }

    fn turn(&mut self, angle: Angle) {
        match angle {
            Ninety => {
                let (old_x, old_y) = (self.way_x, self.way_y);
                self.way_x = -old_y;
                self.way_y = old_x;
            }
            OneEighty => {
                self.way_x = -self.way_x;
                self.way_y = -self.way_y;
            }
            NegativeNinety => {
                let (old_x, old_y) = (self.way_x, self.way_y);
                self.way_x = old_y;
                self.way_y = -old_x;
            }
        }
    }
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("day12input.txt");
    let instruction_regex = Regex::new(r"^(?P<code>[NSEWLRF])(?P<amount>\d+)$").unwrap();

    match io_result {
        Ok(lines) => {
            let rows = lines.map(|line| match line {
                Ok(stuff) => {
                    let captures = instruction_regex.captures(&stuff).unwrap();
                    let amount: usize = captures.name("amount").unwrap().as_str().parse().unwrap();
                    let code: &str = captures.name("code").unwrap().as_str();
                    match code {
                        "N" => North(amount),
                        "S" => South(amount),
                        "E" => East(amount),
                        "W" => West(amount),
                        "F" => Forward(amount),
                        "L" | "R" => {
                            let angle = match amount {
                                90 => Ninety,
                                180 => OneEighty,
                                270 => NegativeNinety,
                                _ => panic!("Unknown Angle {}", amount),
                            };
                            match code {
                                "L" => Left(angle),
                                "R" => Right(angle),
                                _ => panic!("Unknown code {}", code),
                            }
                        },
                        _ => panic!("Unknown code {}", code),
                    }
                }
                Err(_) => panic!("Error reading line"),
            }).collect();
            InputData {
                rows: rows,
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

