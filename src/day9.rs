extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::collections::VecDeque;

fn main() {
    let input = parse_input();
    let nums = input.rows;
    let invalid_num = invalid_number(&nums).unwrap();
    println!("Found invalid number {}", invalid_num);

    let bad_window = window_summing_to(&nums, invalid_num);
    println!("Found window {:?} adding to {}", bad_window, invalid_num);

    let min = bad_window.iter().min().unwrap();
    let max = bad_window.iter().max().unwrap();
    println!("Key = {}", min + max);
}

struct InputData {
    rows: Vec<usize>,
}

impl InputData {
}

struct Window {
    capacity: usize,
    nums: VecDeque<usize>
}

fn window_summing_to(nums: &Vec<usize>, invalid_num: usize) -> VecDeque<usize> {
    let mut deque = VecDeque::new();
    let mut total = 0;
    for nn in nums.iter() {
        let n = *nn;
        total += n;
        deque.push_back(n);

        while total > invalid_num {
            let removed = deque.pop_front().unwrap();
            total -= removed;
        }
        if total == invalid_num {
            return deque;
        }
    }
    panic!("blah")
}

fn invalid_number(nums: &Vec<usize>) -> Option<usize> {
    let mut window = Window { capacity: 25, nums: VecDeque::new() };

    for (i, num) in nums.iter().enumerate() {
        if i < 25 {
            window.push(*num);
        } else {
            if window.has_sum_using(*num) {
                window.push(*num);
            } else {
                return Some(*num);
            }
        }
    };

    None
}

impl Window {
    fn push(&mut self, new_value: usize) {
        if self.nums.len() == self.capacity {
            self.nums.pop_front();
        }
        self.nums.push_back(new_value);
    }

    fn has_sum_using(&self, x: usize) -> bool {
        for (idx, i) in self.nums.iter().enumerate() {
            for (idy, j) in self.nums.iter().enumerate() {
                if idx == idy { continue; }

                if i + j == x { return true; }
            }
        }
        false
    }
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("day9input.txt");
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

