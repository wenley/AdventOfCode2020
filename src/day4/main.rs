extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let input = parse_input();
    eprintln!("Found {} entries", input.passports.len());
    println!("Found {} valid passports", input.valid_passports());
    // let passport = Passport {
    //     index: 0,
    //     birth_year: "2026".to_string(),
    //     issue_year: "2028".to_string(),
    //     expire_year: "1943".to_string(),
    //     height: "64cm".to_string(),
    //     hair_color: "z".to_string(),
    //     eye_color: "zzz".to_string(),
    //     passport_id: "160cm".to_string(),
    //     country_id: "74".to_string(),
    // };
    // println!("valid? {}", passport.is_valid());
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
        if !self.valid_birth_year() { missing_keys.push("byr"); }
        if !self.valid_issue_year() { missing_keys.push("iyr"); }
        if !self.valid_expire_year() { missing_keys.push("eyr"); }
        if !self.valid_height() { missing_keys.push("hgt"); }
        if !self.valid_hair_color() { missing_keys.push("hcl"); }
        if !self.valid_eye_color() { eprintln!("bad eye color"); missing_keys.push("ecl"); }
        if !self.valid_passport_id() { missing_keys.push("pid"); }
        // if self.country_id.len() == 0 { missing_keys.push("cid"); }

        let valid = missing_keys.is_empty();
        eprintln!("({}) is valid? {:?} (missing {:?})", self.index, valid, missing_keys);
        valid
    }

    fn valid_birth_year(&self) -> bool {
        match self.birth_year.parse::<usize>() {
            Ok(i) => i >= 1920 && i <= 2002,
            Err(_) => false,
        }
    }

    fn valid_issue_year(&self) -> bool {
        match self.issue_year.parse::<usize>() {
            Ok(i) => i >= 2010 && i <= 2020,
            Err(_) => false,
        }
    }

    fn valid_expire_year(&self) -> bool {
        match self.expire_year.parse::<usize>() {
            Ok(i) => i >= 2020 && i <= 2030,
            Err(_) => false,
        }
    }

    fn valid_height(&self) -> bool {
        let matcher = Regex::new(r"^(?P<num>\d+)(?P<unit>cm|in)$").unwrap();
        match matcher.captures(&self.height) {
            None => false,
            Some(c) => {
                let num = c.name("num").unwrap().as_str().parse::<usize>().unwrap();
                match c.name("unit").unwrap().as_str() {
                    "cm" => num >=150 && num <= 193,
                    "in" => num >= 59 && num <= 76,
                    _ => false,
                }
            }
        }
    }

    fn valid_hair_color(&self) -> bool {
        let matcher = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        matcher.is_match(&self.hair_color)
    }

    fn valid_eye_color(&self) -> bool {
        let mut valid_colors = HashSet::new();
        valid_colors.insert("amb".to_string());
        valid_colors.insert("blu".to_string());
        valid_colors.insert("brn".to_string());
        valid_colors.insert("gry".to_string());
        valid_colors.insert("grn".to_string());
        valid_colors.insert("hzl".to_string());
        valid_colors.insert("oth".to_string());

        let value = valid_colors.contains(&self.eye_color);
        // eprintln!("{:?} contains {}? {}", valid_colors, self.eye_color, value);
        value
    }

    fn valid_passport_id(&self) -> bool {
        let matcher = Regex::new(r"^\d{9}$").unwrap();
        matcher.is_match(&self.passport_id)
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

