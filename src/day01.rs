use std::fs;
use std::collections::HashSet;

fn main() {
    let input_numbers = parse_input();
    let mut numbers: HashSet<usize> = HashSet::new();
    for i in input_numbers {
        if numbers.contains(&(2020 - i)) {
            println!("Found: {}", i * (2020 - i));
        }
        numbers.insert(i);
    }
}

fn parse_input() -> HashSet<usize> {
    let content = fs::read_to_string("inputs/day1.txt").unwrap();
    content.split("\n").map(|line| line.parse().unwrap()).collect()
}
