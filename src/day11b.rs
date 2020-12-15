extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input = parse_input();
    let mut ferry = input.ferry;
    let mut next_ferry = ferry.next_ferry();

    while ferry != next_ferry {
        ferry = next_ferry;
        next_ferry = ferry.next_ferry();
    }
    println!("Finished!");
    let occupied_count: usize = ferry.spots.iter().map(|row| {
        row.iter().filter(|spot| **spot == Spot::Occupied).count()
    }).sum();
    println!(
        "{}",
        ferry.spots.iter().map(|row| {
            row.iter().map(|spot| match spot {
                Spot::Floor => ".",
                Spot::Empty => "L",
                Spot::Occupied => "#",
            }).collect::<Vec<&str>>().join("")
        }).collect::<Vec<String>>().join("\n"),
    );
    println!("{} occupied seats", occupied_count);
}

#[derive(PartialEq, Clone, Copy)]
enum Spot {
    Floor,
    Empty,
    Occupied,
}

#[derive(PartialEq)]
struct Ferry {
    spots: Vec<Vec<Spot>>,
}

const NEIGHBOR_DELTAS: [(i64, i64); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

impl Ferry {
    fn next_ferry(&self) -> Ferry {
        Ferry {
            spots: self.spots.iter().enumerate().map(|(row, row_spots)| {
                row_spots.iter().enumerate().map(|(col, current)| {
                    match (current, self.occupied_neighbors(row as i64, col as i64)) {
                        (Spot::Floor, _) => Spot::Floor,
                        (_, 0) => Spot::Occupied,
                        (_, 5..=8) => Spot::Empty,
                        _ => *current,
                    }
                }).collect()
            }).collect()
        }
    }

    fn occupied_neighbors(&self, row: i64, col: i64) -> usize {
        NEIGHBOR_DELTAS.iter().map(|direction| {
            match self.visible_neighbor(row, col, *direction) {
                Spot::Occupied => 1,
                _ => 0,
            }
        }).sum()
    }

    fn visible_neighbor(&self, row: i64, col: i64, direction: (i64, i64)) -> Spot {
        let mut n = 1;
        let (dx, dy) = direction;
        loop {
            let maybe_seat = self.get_inbounds(row + (n * dx), col + (n * dy));
            match maybe_seat {
                None => { return Spot::Empty; },
                Some(Spot::Floor) => { n += 1 },
                Some(s) => { return s },
            }
        }
    }

    fn get_inbounds(&self, row: i64, col: i64) -> Option<Spot> {
        if row < 0 || row >= self.spots.len() as i64 {
            return None;
        }
        let spot_row = &self.spots[row as usize];
        if col < 0 || col >= spot_row.len() as i64 {
            return None;
        }
        Some(spot_row[col as usize])
    }
}

struct InputData {
    ferry: Ferry,
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("input/day11.txt");
    match io_result {
        Ok(lines) => {
            let spots = lines.map(|line| match line {
                Ok(stuff) => {
                    stuff.chars().map(|c| match c {
                        '.' => Spot::Floor,
                        'L' => Spot::Empty,
                        '#' => Spot::Occupied,
                        _ => panic!("Unknown char {}", c),
                    }).collect()
                }
                Err(_) => panic!("Error reading line"),
            }).collect();
            InputData {
                ferry: Ferry { spots: spots },
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

