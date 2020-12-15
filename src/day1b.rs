use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let io_result = lines_in_file("day1input.txt");
    let mut numbers = HashSet::new();
    let mut sum_to_pair = HashMap::new();
    match io_result {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(stuff) => {
                        let i: i64 = stuff.parse().unwrap();
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
                    Err(_) => panic!("Error reading line"),
                }
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}
