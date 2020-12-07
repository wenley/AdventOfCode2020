extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::fmt;

fn main() {
    let input = parse_input();
    println!("Tree collisions = {}", input.tree_collisions());
}

#[derive(PartialEq, Eq, Debug)]
enum Slope {
    Empty,
    Tree,
}
impl fmt::Display for Slope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Slope::Empty => write!(f, "."),
            Slope::Tree => write!(f, "#"),
        }
    }
}

struct InputData {
    slope: Vec<Vec<Slope>>,
}

impl InputData {
    fn tree_collisions(&self) -> usize {
        let mut collisions = 0;
        let mut column = 0;

        for row in self.slope.iter() {
            if row[column] == Slope::Tree {
                // println!(
                //     "{:?} at {} is a tree",
                //     row.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
                //     column,
                // );
                collisions += 1;
            }
            column = (column + 3) % row.len();
        }

        collisions
    }
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("day3input.txt");
    match io_result {
        Ok(lines) => {
            InputData {
                slope: lines.map(|line| match line {
                    Ok(stuff) => {
                        stuff.chars().map(|c| match c {
                            '.' => Slope::Empty,
                            '#' => Slope::Tree,
                            _ => panic!("Unknown char {} in line {}", c, stuff),
                        }).collect()
                    }
                    Err(_) => panic!("Error reading line"),
                }).collect(),
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

