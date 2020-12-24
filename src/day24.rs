extern crate regex;
extern crate nom;

use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::Add;
use std::convert::{Into, From};
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::many0,
};

fn main() {
    let input = parse_input();
    let mut black_tiles = HashSet::new();
    for line in input.directions.iter() {
        let final_point = line.iter().fold(Point::origin(), |point, direction| {
            point + direction.into()
        });
        if black_tiles.contains(&final_point) {
            black_tiles.remove(&final_point);
        } else {
            black_tiles.insert(final_point);
        }
    }
    println!("{} black tiles remain", black_tiles.len());

    let mut room = Room { black_tiles: black_tiles };
    for _ in 0..100 {
        room = room.advance_one_day();
    }
    println!("{} black tiles after 100 days", room.black_tiles.len());
}

struct InputData {
    directions: Vec<Vec<Direction>>,
}

impl InputData {
}

struct Room {
    black_tiles: HashSet<Point>,
}
impl Room {
    fn advance_one_day(&self) -> Room {
        let mut black_neighbor_counts = HashMap::new();
        for black_point in self.black_tiles.iter() {
            for direction in &ALL_DIRECTIONS {
                let neighbor_point = *black_point + direction.into();
                let existing_count = black_neighbor_counts.get(&neighbor_point).map(|i| *i).unwrap_or(0);
                black_neighbor_counts.insert(neighbor_point, existing_count + 1);
            }
        }
        let min_x = self.black_tiles.iter().map(|p| p.x).min().unwrap() - 1;
        let max_x = self.black_tiles.iter().map(|p| p.x).max().unwrap() + 1;
        let min_y = self.black_tiles.iter().map(|p| p.y).min().unwrap() - 1;
        let max_y = self.black_tiles.iter().map(|p| p.y).max().unwrap() + 1;

        let mut next_black_tiles = HashSet::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let point = Point::new(x, y);
                match (self.black_tiles.contains(&point), black_neighbor_counts.get(&point).map(|i| *i).unwrap_or(0)) {
                    (false, 2) => { next_black_tiles.insert(point); }
                    (false, _) => { /* tile stays white */ }
                    (true, 0) => { /* tile flips white */ }
                    (true, 1) => { next_black_tiles.insert(point); } // Tile stays black
                    (true, 2) => { next_black_tiles.insert(point); } // Tile stays black
                    (true, _) => { /* tile flips white */ }
                };
            }
        }
        Room { black_tiles: next_black_tiles }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}
impl Add<Point> for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl From<&Direction> for Point {
    fn from(direction: &Direction) -> Point {
        match direction {
            Direction::East => Point::new(1, 0),
            Direction::West => Point::new(-1, 0),
            Direction::SouthEast => Point::new(1, -1),
            Direction::SouthWest => Point::new(0, -1),
            Direction::NorthEast => Point::new(0, 1),
            Direction::NorthWest => Point::new(-1, 1),
        }
    }
}

fn parse_input() -> InputData {
    let content = fs::read_to_string("inputs/day24.txt").unwrap();
    let directions = content.split("\n").map(|line| parse_line(line).unwrap().1).collect();
    InputData {
        directions: directions,
    }
}

enum Direction {
    East,
    West,
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
}
const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::East,
    Direction::West,
    Direction::SouthEast,
    Direction::SouthWest,
    Direction::NorthEast,
    Direction::NorthWest,
];

fn parse_line(s: &str) -> IResult<&str, Vec<Direction>> {
    all_consuming(many0(parse_direction))(s)
}

fn parse_direction(s: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("se"), |_| Direction::SouthEast),
        map(tag("sw"), |_| Direction::SouthWest),
        map(tag("ne"), |_| Direction::NorthEast),
        map(tag("nw"), |_| Direction::NorthWest),
        map(tag("e"), |_| Direction::East),
        map(tag("w"), |_| Direction::West),
    ))(s)
}
