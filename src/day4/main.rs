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
    index: usize,
    birth_year: String,
    issue_year: String,
    expire_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: String,
}

struct InputData {
    passports: Vec<Passport>,
}

impl Passport {
    fn from_hash(data: HashMap<String, String>, index: usize) -> Passport {
        Passport {
            index: index,
            birth_year: data.get("byr").map(|s| s.to_string()).unwrap_or("".to_string()),
            issue_year: data.get("iyr").map(|s| s.to_string()).unwrap_or("".to_string()),
            expire_year: data.get("eyr").map(|s| s.to_string()).unwrap_or("".to_string()),
            height: data.get("hgt").map(|s| s.to_string()).unwrap_or("".to_string()),
            hair_color: data.get("hcl").map(|s| s.to_string()).unwrap_or("".to_string()),
            eye_color: data.get("ecl").map(|s| s.to_string()).unwrap_or("".to_string()),
            passport_id: data.get("pid").map(|s| s.to_string()).unwrap_or("".to_string()),
            country_id: data.get("cid").map(|s| s.to_string()).unwrap_or("".to_string()),
        }
    }

    fn is_valid(&self) -> bool {
        let mut missing_keys = vec![];
        if self.birth_year.len() == 0 { missing_keys.push("byr"); }
        if self.issue_year.len() == 0 { missing_keys.push("iyr"); }
        if self.expire_year.len() == 0 { missing_keys.push("eyr"); }
        if self.height.len() == 0 { missing_keys.push("hgt"); }
        if self.hair_color.len() == 0 { missing_keys.push("hcl"); }
        if self.eye_color.len() == 0 { missing_keys.push("ecl"); }
        if self.passport_id.len() == 0 { missing_keys.push("pid"); }
        // if self.country_id.len() == 0 { missing_keys.push("cid"); }

        let valid = missing_keys.is_empty();
        eprintln!("({}) is valid? {:?} (missing {:?})", self.index, valid, missing_keys);
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
                            let passport = Passport::from_hash(data, index);
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
            let passport = Passport::from_hash(data, index);
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

