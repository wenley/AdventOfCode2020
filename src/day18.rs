extern crate regex;
extern crate nom;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;

use nom::{
    IResult,
    branch::alt,
    character::complete::{char, digit1, space0},
    combinator::{all_consuming, map,map_res},
    multi::many0,
    sequence::{delimited, preceded, separated_pair},
};

fn main() {
    let input = parse_input();
    println!("Total = {}", input.equations.iter().map(|eq| eq.evaluate()).sum::<usize>());
}

struct InputData {
    equations: Vec<Equation>
}

enum Value {
    Literal(usize),
    Paren(Box<Equation>),
}
impl Value {
    fn evaluate(&self) -> usize {
        match self {
            Value::Literal(v) => *v,
            Value::Paren(b) => b.evaluate(),
        }
    }
}

enum MathPart {
    Add(Value),
    Multiply(Value),
}

struct Equation {
    first: Value,
    rest: Vec<MathPart>,
}
impl Equation {
    fn evaluate(&self) -> usize {
        self.rest.iter().fold(self.first.evaluate(), |acc, part| {
            match part {
                MathPart::Add(v) => acc + v.evaluate(),
                MathPart::Multiply(v) => acc * v.evaluate(),
            }
        })
    }
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("inputs/day18.txt");
    match io_result {
        Ok(lines) => {
            let equations = lines.map(|line| match line {
                Ok(stuff) => {
                    match parse_line(&stuff) {
                        Ok((_, eq)) => eq,
                        Err(_) => panic!("Couldn't parse {}", stuff),
                    }
                }
                Err(_) => panic!("Error reading line"),
            }).collect();
            InputData {
                equations: equations,
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

/*
 * Parsing whole Equations
 */
fn parse_line(s: &str) -> IResult<&str, Equation> {
    all_consuming(parse_equation)(s)
}
fn parse_equation(s: &str) -> IResult<&str, Equation> {
    let (s, first) = parse_value(s)?;
    let (s, rest) = many0(preceded(space0, parse_math_part))(s)?;

    Ok((s, Equation {
        first: first,
        rest: rest,
    }))
}

/*
 * Parsing MathPart
 */
fn parse_math_part(s: &str) -> IResult<&str, MathPart> {
    alt((parse_add, parse_multiply))(s)
}
fn parse_add(s: &str) -> IResult<&str, MathPart> {
    map(
        separated_pair(
            char('+'),
            space0,
            parse_value,
        ),
        |(_, value)| MathPart::Add(value),
    )(s)
}
fn parse_multiply(s: &str) -> IResult<&str, MathPart> {
    map(
        separated_pair(
            char('*'),
            space0,
            parse_value,
        ),
        |(_, value)| MathPart::Multiply(value),
    )(s)
}

/*
 * Parsing Values
 */
fn parse_value(s: &str) -> IResult<&str, Value> {
    alt((parse_literal, parse_paren_value))(s)
}
fn parse_literal(s: &str) -> IResult<&str, Value> {
    map(parse_number, |num| Value::Literal(num))(s)
}
fn parse_paren_value(s: &str) -> IResult<&str, Value> {
    map(
        delimited(parse_open_paren, parse_equation, parse_close_paren),
        |eq| Value::Paren(Box::new(eq)),
    )(s)
}

/*
 * Parsing primitives
 */
fn parse_open_paren(s: &str) -> IResult<&str, char> {
    delimited(space0, char('('), space0)(s)
}
fn parse_close_paren(s: &str) -> IResult<&str, char> {
    delimited(space0, char(')'), space0)(s)
}
fn parse_number(s: &str) -> IResult<&str, usize> {
    map_res(
        digit1,
        |digits: &str| digits.parse(),
    )(s)
}
