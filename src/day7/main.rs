extern crate regex;
extern crate petgraph;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;
use petgraph::{
    Graph,
    visit::Bfs,
};

fn main() {
    let input = parse_input();

    println!("{:?}", input.rules[0]);
    traverse_shiny_gold(input.rules);
}

fn rule_set(rules: &Vec<Rule>) -> HashMap<Bag, Vec<Bag>> {
    let mut map: HashMap<Bag, Vec<Bag>> = HashMap::new();
    for rule in rules {
        for content in rule.contents.iter() {
            if map.contains_key(&content.bag) {
                map.get_mut(&content.bag).unwrap().push(rule.bag.clone());
            } else {
                map.insert(content.bag.clone(), vec![rule.bag.clone()]);
            }
        }
    }
    map
}

fn traverse_shiny_gold(rules: Vec<Rule>) {
    // Edge A to B = A contained by B
    let mut contained_by_graph = Graph::<Bag, ()>::new();
    let mut bag_to_index = HashMap::new();
    let mut index_to_bag = HashMap::new();

    for rule in rules {
        let bag = rule.bag.clone();
        if !bag_to_index.contains_key(&bag) {
            let index = contained_by_graph.add_node(rule.bag.clone());
            bag_to_index.insert(rule.bag.clone(), index);
            index_to_bag.insert(index, rule.bag.clone());
        }

        for content in rule.contents {
            if !bag_to_index.contains_key(&content.bag) {
                let index = contained_by_graph.add_node(content.bag.clone());
                bag_to_index.insert(content.bag.clone(), index);
                index_to_bag.insert(index, content.bag.clone());
            }
            let inner_bag_index = bag_to_index.get(&content.bag).unwrap();
            let outer_bag_index = bag_to_index.get(&rule.bag).unwrap();
            contained_by_graph.add_edge(*inner_bag_index, *outer_bag_index, ());
        }
    }

    let shiny_gold_bag = Bag { adjective: "shiny".to_string(), color: "gold".to_string() };
    let shiny_gold_index = bag_to_index.get(&shiny_gold_bag).unwrap();

    let mut bfs_search = Bfs::new(&contained_by_graph, *shiny_gold_index);
    let mut count = 0;
    while let Some(nx) = bfs_search.next(&contained_by_graph) {
        println!("Found {:?}", index_to_bag.get(&nx).unwrap());
        count += 1;
    }

    println!("Traversed {} nodes", count);
}

#[derive(Debug, Eq, Hash, Clone)]
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

