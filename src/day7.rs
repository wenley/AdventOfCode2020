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

    traverse_shiny_gold(&input);
    shiny_gold_definite_contents(&input);
}

fn traverse_shiny_gold(rules: &InputData) {
    // Edge A to B = A contained by B
    let mut contained_by_graph = Graph::<Bag, ()>::new();
    let mut bag_to_index = HashMap::new();
    let mut index_to_bag = HashMap::new();

    for (bag, contents) in rules.rules.iter() {
        let bag = bag.clone();
        if !bag_to_index.contains_key(&bag) {
            let index = contained_by_graph.add_node(bag.clone());
            bag_to_index.insert(bag.clone(), index);
            index_to_bag.insert(index, bag.clone());
        }

        for content in contents {
            if !bag_to_index.contains_key(&content.bag) {
                let index = contained_by_graph.add_node(content.bag.clone());
                bag_to_index.insert(content.bag.clone(), index);
                index_to_bag.insert(index, content.bag.clone());
            }
            let inner_bag_index = bag_to_index.get(&content.bag).unwrap();
            let outer_bag_index = bag_to_index.get(&bag).unwrap();
            contained_by_graph.add_edge(*inner_bag_index, *outer_bag_index, ());
        }
    }

    let shiny_gold_bag = Bag { adjective: "shiny".to_string(), color: "gold".to_string() };
    let shiny_gold_index = bag_to_index.get(&shiny_gold_bag).unwrap();

    let mut bfs_search = Bfs::new(&contained_by_graph, *shiny_gold_index);
    let mut count = 0;
    while let Some(_) = bfs_search.next(&contained_by_graph) {
        count += 1;
    }

    println!("Traversed {} nodes", count);
}

fn shiny_gold_definite_contents(rules: &InputData) {
    let mut previously_computed: HashMap<Bag, usize> = HashMap::new();
    let shiny_gold_bag = Bag { adjective: "shiny".to_string(), color: "gold".to_string() };
    let count = definite_contents(&shiny_gold_bag, rules, &mut previously_computed);
    println!("1 Shiny Gold bag must contain {} other bags", count);
}

// Does NOT include the bag itself
fn definite_contents(bag: &Bag, rules: &InputData, previously_computed: &mut HashMap<Bag, usize>) -> usize {
    match previously_computed.get(bag) {
        Some(count) => *count,
        None => {
            match rules.rules.get(bag) {
                None => panic!("Unknown bag type {:?}", bag),
                Some(contents) => {
                    let mut count = 0;
                    for content in contents.iter() {
                        count += content.count * (1 + definite_contents(&content.bag, rules, previously_computed));
                    }
                    previously_computed.insert(bag.clone(), count);
                    count
                }
            }
        },
    }
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

struct InputData {
    rules: HashMap<Bag, Vec<Content>>,
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("input/day7.txt");
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

                        (
                            Bag {
                                adjective: captures.name("adj").unwrap().as_str().to_string(),
                                color: captures.name("color").unwrap().as_str().to_string(),
                            },
                            contents,
                        )
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

