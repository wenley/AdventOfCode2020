extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

fn main() {
    let input = parse_input();

    println!("{:?}", input.rules[0]);
}

#[derive(Debug)]
struct Content {
    count: usize,
    adjective: String,
    color: String,
}

#[derive(Debug)]
struct Rule {
    adjective: String,
    color: String,
    contents: Vec<Content>,
}

struct InputData {
    rules: Vec<Rule>,
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("day7input.txt");
    let sentence_split = Regex::new(r"^(?P<adj>[a-z]+) (?P<color>[a-z]+) bags contain (?P<contents>[^.]+).$").unwrap();
    let contents_split = Regex::new(r"^(?P<count>\d+) (?P<adj>[a-z]+) (?P<color>[a-z]+) bags?$").unwrap();

    match io_result {
        Ok(lines) => {
            let rules = lines.map(|line| {
                match line {
                    Ok(stuff) => {
                        let captures = sentence_split.captures(&stuff).unwrap();

                        let content_string = captures.name("contents").unwrap().as_str();
                        let contents = if content_string == "no other bags" {
                            vec![]
                        } else {
                            content_string.split(", ").map(|s| {
                                let var = contents_split.captures(s);
                                let content_capture = match var {
                                    Some(v) => v,
                                    None => panic!("Couldn't parse {} into pieces", s),
                                };

                                Content {
                                    count: content_capture.name("count").unwrap().as_str().parse().unwrap(),
                                    adjective: content_capture.name("adj").unwrap().as_str().to_string(),
                                    color: content_capture.name("color").unwrap().as_str().to_string(),
                                }
                            }).collect()
                        };

                        Rule {
                            adjective: captures.name("adj").unwrap().as_str().to_string(),
                            color: captures.name("color").unwrap().as_str().to_string(),
                            contents: contents,
                        }
                    },
                    Err(_) => panic!("Error reading line"),
                }
            }).collect();

            InputData {
                rules: rules,
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

