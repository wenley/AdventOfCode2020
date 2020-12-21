use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let io_result = lines_in_file("input/day1.txt");
    let mut numbers = HashSet::new();
    match io_result {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(stuff) => {
                        let i: i64 = stuff.parse().unwrap();
                        if numbers.contains(&(2020 - i)) {
                            println!("Found: {}", i * (2020 - i));
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
