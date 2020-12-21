use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input_numbers = parse_input();
    let mut numbers = HashSet::new();
    let mut sum_to_pair = HashMap::new();

    for i in input_numbers {
        let i_bar = 2020 - i;

        let maybe_pair = sum_to_pair.get(&i_bar);
        if let Some((j, k)) = maybe_pair {
            println!("Found: {} * {} * {} = {}", i, j, k, i * j * k);
            break;
        }
        // No duplicates
        if numbers.contains(&i) {
            eprintln!("Duplicate number {}", i);
            continue;
        }

        for j in &numbers {
            let sum = i + j;
            if sum_to_pair.contains_key(&sum) {
                // eprintln!("Duplicate sum");
                // Duplicates would result in non-deterministic output -> cannot be
                // valid
                sum_to_pair.remove(&sum);
            } else {
                sum_to_pair.insert(sum, (i, *j));
            }
        }
        numbers.insert(i);
    }
}

fn parse_input() -> HashSet<usize> {
    let content = fs::read_to_string("inputs/day1.txt").unwrap();
    content.split("\n").map(|line| line.parse().unwrap()).collect()
}
