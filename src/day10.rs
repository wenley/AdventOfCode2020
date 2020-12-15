extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let input = parse_input();
    let mut nums = input.rows;
    nums.sort();
    println!("{:?}", nums.iter().take(10).map(|i| *i).collect::<Vec<i64>>());

    let mut one_gaps = 0;
    let mut three_gaps = 1; // Device
    let mut old_value = 0;
    nums.iter().for_each(|new_value| {
        let diff = new_value - old_value;
        match diff {
            3 => { three_gaps += 1; }
            2 => {},
            1 => { one_gaps += 1; }
            _ => panic!("Unknown gap! {} = {} - {}", diff, new_value, old_value)
        };
        old_value = *new_value;
    });
    println!("{} * {} = {}", one_gaps, three_gaps, one_gaps * three_gaps);

    let max_value = nums.iter().max().unwrap();
    let mapping = ways_to_reach(&nums);
    println!("{} ways to get to adapter", mapping.get(&max_value).unwrap());
}

fn ways_to_reach(sorted_nums: &Vec<i64>) -> HashMap<i64, usize> {
    let mut cache = HashMap::new();
    cache.insert(0, 1);
    for n in sorted_nums {
        let mut ways = 0;
        for m in n-3..*n {
            ways += cache.get(&m).unwrap_or(&0);
        }
        cache.insert(*n, ways);
    }
    cache
}

struct InputData {
    rows: Vec<i64>,
}

impl InputData {
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("day10input.txt");
    match io_result {
        Ok(lines) => {
            let rows = lines.map(|line| match line {
                Ok(stuff) => {
                    stuff.parse().unwrap()
                }
                Err(_) => panic!("Error reading line"),
            }).collect();
            InputData {
                rows: rows,
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

