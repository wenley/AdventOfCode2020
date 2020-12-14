extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let input = parse_input();
    let total: usize = input.groups.iter().map(|g| g.answers.len()).sum();
    println!("{}", total);
    let nth = input.groups.iter().nth(6).unwrap();
    println!("First: {}", nth.answers.len());
    println!("{:?}", nth.answers);
}

struct Group {
    answers: HashSet<char>,
}

struct InputData {
    groups: Vec<Group>,
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("day6input.txt");

    match io_result {
        Ok(lines) => {
            let mut answers = HashSet::new();
            let mut groups = vec![];
            for line in lines {
                match line {
                    Ok(stuff) => {
                        if stuff.len() <= 0 { // Newlines??
                            groups.push(Group { answers: answers });
                            answers = HashSet::new();
                        } else {
                            for c in stuff.chars() {
                                answers.insert(c);
                            }
                        }
                    },
                    Err(_) => panic!("Error reading line"),
                }
            }
            let last_group = Group { answers: answers };
            groups.push(last_group);

            InputData {
                groups: groups,
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

