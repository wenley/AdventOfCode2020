extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let input = parse_input();
    let program = input.program;

    println!("{}", program.find_loop());

    println!("Looking for termination...");
    for i in 0..program.instructions.len() {
        let new_program = program.copy_with_mutation_at_index(i);
        match new_program.find_terminate() {
            Some(acc) => {
                println!("Found result {} by changing index {}", acc, i);
                break;
            }
            None => {}
        }
    }
}

struct InputData {
    program: Program,
}

#[derive(Clone, Copy)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}

struct Instruction {
    op_code: OpCode,
    offset: i32,
}

struct Program {
    instructions: Vec<Instruction>
}

impl Program {
    fn find_loop(&self) -> i32 {
        let mut acc_value = 0;
        let mut instruction_pointer: usize = 0;
        let mut visited_indexes = HashSet::new();

        while !visited_indexes.contains(&instruction_pointer) {
            visited_indexes.insert(instruction_pointer);
            let instruction = &self.instructions[instruction_pointer];
            match instruction.op_code {
                OpCode::Acc => {
                    acc_value += instruction.offset;
                    instruction_pointer += 1;
                }
                OpCode::Jmp => {
                    if instruction.offset > 0 {
                        instruction_pointer = instruction_pointer + (instruction.offset as usize);
                    } else {
                        let abs: i32 = -instruction.offset;
                        instruction_pointer = instruction_pointer - (abs as usize);
                    }
                }
                OpCode::Nop => {
                    instruction_pointer += 1;
                }
            }
        }

        acc_value
    }

    fn find_terminate(&self) -> Option<i32> {
        let mut acc_value = 0;
        let mut instruction_pointer: usize = 0;
        let mut visited_indexes = HashSet::new();

        loop {
            // Looped
            if visited_indexes.contains(&instruction_pointer) {
                return None;
            }

            visited_indexes.insert(instruction_pointer);
            if instruction_pointer >= self.instructions.len() {
                return Some(acc_value);
            }

            let instruction = &self.instructions[instruction_pointer];
            match instruction.op_code {
                OpCode::Acc => {
                    acc_value += instruction.offset;
                    instruction_pointer += 1;
                }
                OpCode::Jmp => {
                    if instruction.offset > 0 {
                        instruction_pointer = instruction_pointer + (instruction.offset as usize);
                    } else {
                        let abs: i32 = -instruction.offset;
                        instruction_pointer = instruction_pointer - (abs as usize);
                    }
                }
                OpCode::Nop => {
                    instruction_pointer += 1;
                }
            }
        }
    }

    fn copy_with_mutation_at_index(&self, mutation_index: usize) -> Program {
        Program {
            instructions: self.instructions.iter().enumerate().map(|(index, instruction)| {
                if index == mutation_index {
                    Instruction {
                        op_code: match instruction.op_code {
                            OpCode::Acc => OpCode::Acc,
                            OpCode::Jmp => OpCode::Nop,
                            OpCode::Nop => OpCode::Jmp,
                        },
                        offset: instruction.offset,
                    }
                } else {
                    Instruction {
                        op_code: instruction.op_code,
                        offset: instruction.offset,
                    }
                }
            }).collect(),
        }
    }
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("day8input.txt");
    let instruction_regex = Regex::new(r"^(?P<op>acc|jmp|nop) (?P<num>(\+|-)\d+)$").unwrap();
    match io_result {
        Ok(lines) => {
            let instructions = lines.map(|line| {
                match line {
                    Ok(stuff) => {
                        let captures = instruction_regex.captures(&stuff).unwrap();
                        let op_code_str = captures.name("op").unwrap().as_str();
                        let op_code = match op_code_str {
                            "acc" => OpCode::Acc,
                            "jmp" => OpCode::Jmp,
                            "nop" => OpCode::Nop,
                            _ => panic!("Unknown op code {}", op_code_str),
                        };
                        let offset = captures.name("num").unwrap().as_str().parse().unwrap();
                        Instruction {
                            op_code: op_code,
                            offset: offset,
                        }
                    }
                    Err(_) => panic!("Error reading line"),
                }
            }).collect();
            InputData {
                program: Program { instructions: instructions },
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

