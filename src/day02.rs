extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

fn main() {
    let io_result = lines_in_file("input/day2.txt");
    match io_result {
        Ok(lines) => {
            let mut valid_rows = 0;
            for line in lines {
                match line {
                    Ok(stuff) => {
                        if parse_line(&stuff).valid() {
                            valid_rows += 1;
                        }
                    }
                    Err(_) => panic!("Error reading line"),
                }
            }
            println!("{} rows passed rules", valid_rows);
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

struct PasswordRule<'a> {
    min_count: i64,
    max_count: i64,
    letter: char,
    test_password: &'a str,
}

fn parse_line(line: &str) -> PasswordRule {
    let parse_regex: Regex = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<pass>[a-z]+)$").unwrap();
    let captures: regex::Captures = match parse_regex.captures(line) {
        Some(thing) => thing,
        None => panic!("Couldn't parse {}", line),
    };

    PasswordRule {
        min_count: captures.name("min").unwrap().as_str().parse().unwrap(),
        max_count: captures.name("max").unwrap().as_str().parse().unwrap(),
        letter: captures.name("letter").unwrap().as_str().chars().next().unwrap(),
        test_password: captures.name("pass").unwrap().as_str(),
    }
}

impl PasswordRule<'_> {
    fn valid(&self) -> bool {
        let letter_counts = self.test_password.chars().filter(|c| *c == self.letter).count() as i64;

        letter_counts >= self.min_count && letter_counts <= self.max_count
    }
}
