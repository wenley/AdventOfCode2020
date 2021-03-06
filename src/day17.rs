extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let input = parse_input();
    let mut space = input.space;

    for i in 1..=6 {
        space = space.next_space();
        println!("After {} cycles, {} cells alive", i, space.cells.len());
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Alive,
}

#[derive(PartialEq)]
struct Space {
    cells: HashMap<Point, Cell>,
}
type Point = (i64, i64, i64, i64);

fn neighbor_deltas() -> Vec<Point> {
    (-1..=1).flat_map(|x| {
        (-1..=1).flat_map(|y| {
            (-1..=1).flat_map(|z| {
                (-1..=1).map(|w| {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        None
                    } else {
                        Some((x, y, z, w))
                    }
                }).filter_map(|maybe| maybe).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

impl Space {
    fn next_space(&self) -> Space {
        let min_x = self.cells.keys().map(|(x, _, _, _)| x).min().unwrap() - 1;
        let min_y = self.cells.keys().map(|(_, y, _, _)| y).min().unwrap() - 1;
        let min_z = self.cells.keys().map(|(_, _, z, _)| z).min().unwrap() - 1;
        let min_w = self.cells.keys().map(|(_, _, _, w)| w).min().unwrap() - 1;
        let max_x = self.cells.keys().map(|(x, _, _, _)| x).max().unwrap() + 1;
        let max_y = self.cells.keys().map(|(_, y, _, _)| y).max().unwrap() + 1;
        let max_z = self.cells.keys().map(|(_, _, z, _)| z).max().unwrap() + 1;
        let max_w = self.cells.keys().map(|(_, _, _, w)| w).max().unwrap() + 1;

        let points: Vec<Point> = (min_x..=max_x).flat_map(|x| {
            (min_y..=max_y).flat_map(|y| {
                (min_w..=max_w).flat_map(|w| {
                    (min_z..=max_z).map(|z| (x, y, z, w)).collect::<Vec<Point>>()
                }).collect::<Vec<Point>>()
            }).collect::<Vec<Point>>()
        }).collect();

        Space {
            cells: points.iter().filter(|p| {
                match (self.cells.get(p), self.occupied_neighbors(p)) {
                    (Some(Cell::Alive), 2..=3) => true,
                    (None, 3) => true,
                    _ => false
                }
            }).map(|p| (*p, Cell::Alive)).collect(),
        }
    }

    fn occupied_neighbors(&self, point: &Point) -> usize {
        let (x, y, z, w) = point;
        neighbor_deltas().iter().map(|(dx, dy, dz, dw)| {
            let new_point = (x + dx, y + dy, z + dz, w + dw);
            match self.cells.get(&new_point) {
                Some(Cell::Alive) => 1,
                _ => 0,
            }
        }).sum()
    }
}

struct InputData {
    space: Space,
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("inputs/day17.txt");
    match io_result {
        Ok(lines) => {
            let cells = lines.enumerate().flat_map(|(i, line)| match line {
                Ok(stuff) => {
                    stuff.
                        chars().
                        enumerate().
                        filter(|(_, c)| *c == '#').
                        map(|(j, _)| ((i as i64, j as i64, 0, 0), Cell::Alive)).
                        collect::<Vec<_>>()
                }
                Err(_) => panic!("Error reading line"),
            }).collect();
            InputData {
                space: Space { cells: cells },
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

