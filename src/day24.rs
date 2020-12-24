extern crate regex;
extern crate nom;

use std::fs;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::many0,
};

fn main() {
    let input = parse_input();
}

struct InputData {
    directions: Vec<Vec<Direction>>,
}

impl InputData {
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
