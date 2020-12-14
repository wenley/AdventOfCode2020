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

    println!("{:?}", input.rules[0]);
}

fn rule_set(rules: Vec<Rule>) -> HashMap<Bag, Vec<Vec<Content>>> {
    let mut map: HashMap<Bag, Vec<Vec<Content>>> = HashMap::new();
    for rule in rules {
        if map.contains_key(&rule.bag) {
            map.get_mut(&rule.bag).unwrap().push(rule.contents);
        } else {
            map.insert(rule.bag, vec![rule.contents]);
        }
    }
    map
}

fn traverse_shiny_gold(rules: Vec<Rule>) {
    let mut found_bags: HashSet<Bag> = HashSet::new();
    let mut bags_to_explore: HashSet<Bag> = HashSet::new();
    let mut next_bags_to_explore: HashSet<Bag> = HashSet::new();
    bags_to_explore.insert(Bag { adjective: "shiny".to_string(), color: "gold".to_string() });

    while !bags_to_explore.is_empty() {
        bags_to_explore.iter().for_each(|bag| {
        });

        bags_to_explore = next_bags_to_explore;
        next_bags_to_explore = HashSet::new();
    }
}

#[derive(Debug, Eq, Hash)]
struct Bag {
    adjective: String,
    color: String,
}
impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.adjective == other.adjective && self.color == other.color
    }
}

#[derive(Debug)]
struct Content {
    count: usize,
    bag: Bag,
}

#[derive(Debug)]
struct Rule {
    bag: Bag,
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
                                    bag: Bag {
                                        adjective: content_capture.name("adj").unwrap().as_str().to_string(),
                                        color: content_capture.name("color").unwrap().as_str().to_string(),
                                    },
                                }
                            }).collect()
                        };

                        Rule {
                            bag: Bag {
                                adjective: captures.name("adj").unwrap().as_str().to_string(),
                                color: captures.name("color").unwrap().as_str().to_string(),
                            },
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

