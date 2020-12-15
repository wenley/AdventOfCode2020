extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = parse_input();
    let instructions = input.instructions;

    let mut mask = None;
    let mut memory: HashMap<usize, u64> = HashMap::new();

    for (i, instruction) in instructions.iter().enumerate() {
        match instruction {
            UpdateMask(m) => { mask = Some(m); }
            Execute(Assign {
                index,
                value,
            }) => {
                if let Some(m) = mask {
                    let masked_value = m.mask_value(value);
                    memory.insert(*index, masked_value);
                } else {
                    panic!("No mask! Row {}", i);
                }
            }
        }
    }

    // 16003257187056
    println!("Final sum of values = {}", memory.values().fold(0, |acc, num| acc + num));
}

struct InputData {
    instructions: Vec<Instruction>,
}

enum Bit {
    Zero,
    One,
    Float,
}
struct Mask {
    bits: Vec<Bit>,
}
impl Mask {
    fn mask_value(&self, value: &u64) -> u64 {
        let mut zeros_to_and = 1;
        let mut ones_to_or = 0;
        for bit in self.bits.iter() {
            zeros_to_and = zeros_to_and << 1;
            ones_to_or = ones_to_or << 1;
            match bit {
                Bit::Float => { zeros_to_and += 1; }
                Bit::One => { ones_to_or += 1; zeros_to_and += 1 }
                Bit::Zero => { }
            }
        }

        (value & zeros_to_and) | ones_to_or
    }
}
struct Assign {
    index: usize,
    value: u64,
}

enum Instruction {
    UpdateMask(Mask),
    Execute(Assign)
}
use Instruction::*;

fn parse_mask(s: &str) -> Mask {
    Mask {
        bits: s.chars().map(|c| {
            match c {
                'X' => Bit::Float,
                '1' => Bit::One,
                '0' => Bit::Zero,
                _ => panic!("Unknown char {}", c),
            }
        }).collect(),
    }
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("inputs/day14.txt");
    let mask_update_regex = Regex::new(r"^mask = (?P<mask>[X01]+)$").unwrap();
    let assign_regex = Regex::new(r"^mem\[(?P<index>\d+)\] = (?P<value>\d+)$").unwrap();
    match io_result {
        Ok(lines) => {
            let instructions = lines.map(|line| match line {
                Ok(stuff) => {
                    let mask_result = mask_update_regex.captures(&stuff);
                    if let Some(captures) = mask_result {
                        let mask_string = captures.name("mask").unwrap().as_str();
                        UpdateMask(parse_mask(mask_string))
                    } else {
                        let captures = assign_regex.captures(&stuff).unwrap();
                        Execute(Assign {
                            index: captures.name("index").unwrap().as_str().parse().unwrap(),
                            value: captures.name("value").unwrap().as_str().parse().unwrap(),
                        })
                    }
                }
                Err(_) => panic!("Error reading line"),
            }).collect();
            InputData {
                instructions: instructions,
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

