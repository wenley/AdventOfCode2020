extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = parse_input();
    eprintln!("Found {} entries", input.passports.len());
    println!("Found {} valid passports", input.valid_passports());
}

struct Passport {
    data: HashMap<String, String>,
    index: usize,
}

struct InputData {
    passports: Vec<Passport>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        const expected_keys: [&str; 8] = [
            "byr",
            "iyr",
            "eyr",
            "hgt",
            "hcl",
            "ecl",
            "pid",
            "cid",
        ];

        let missing_keys = expected_keys.iter().filter(|key| !self.data.contains_key(&key.to_string())).collect::<Vec<_>>();
        let valid = missing_keys.is_empty();
        eprintln!("({}) {:?} is valid? {:?} (missing {:?})", self.index, self.data, valid, missing_keys);
        valid
    }
}

impl InputData {
    fn valid_passports(&self) -> usize {
        self.passports.iter().filter(|p| p.is_valid()).count()
    }
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("day4input.txt");
    let part_split = Regex::new(r"^(?P<label>[a-z]+):(?P<value>\S+)$").unwrap();

    match io_result {
        Ok(lines) => {
            let mut data = HashMap::new();
            let mut passports = vec![];
            let mut index = 0;
            for line in lines {
                match line {
                    Ok(stuff) => {
                        if stuff.len() <= 1 {
                            let passport = Passport {
                                data: data,
                                index: index,
                            };
                            passports.push(passport);
                            data = HashMap::new();
                            index += 1;
                        } else {
                            for part in stuff.split(" ") {
                                let captures = part_split.captures(part).unwrap();
                                data.insert(
                                    captures.name("label").unwrap().as_str().to_string(),
                                    captures.name("value").unwrap().as_str().to_string(),
                                );
                            }
                        }
                    },
                    Err(_) => panic!("Error reading line"),
                }
            }
            let passport = Passport {
                data: data,
                index: index,
            };
            passports.push(passport);

            InputData {
                passports: passports,
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

