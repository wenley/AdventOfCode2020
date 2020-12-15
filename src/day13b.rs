extern crate regex;

use std::io;
use std::fs;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input = parse_input();
    let buses = input.buses;
    let rules = buses.
        iter().
        filter(|b| **b == Constraint::Blank).
        enumerate().
        map(|(i, b)| {
            match b {
                Constraint::Bus(num) => {
                    let p = *num as i64;
                    Rule {
                        remainder: -(i as i64) % p,
                        modulo: p,
                    }
                }
                Constraint::Blank => panic!("Filter didn't work"),
            }
        });

    println!("{:?}", euclid(27, 20));
}

/*
 * function extended_gcd(a, b)
    (old_r, r) := (a, b)
    (old_s, s) := (1, 0)
    (old_t, t) := (0, 1)

    while r ≠ 0 do
        quotient := old_r div r
        (old_r, r) := (r, old_r − quotient × r)
        (old_s, s) := (s, old_s − quotient × s)
        (old_t, t) := (t, old_t − quotient × t)

    output "Bézout coefficients:", (old_s, old_t)
    output "greatest common divisor:", old_r
    output "quotients by the gcd:", (t, s)
 */
fn euclid(n1: i64, n2: i64) -> (i64, i64) {
    if n1 < n2 { panic!("n1 must be at least n2"); }

    let (mut old_r, mut r) = (n1, n2);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        let (new_r, new_s, new_t) = (
            old_r - quotient * r,
            old_s - quotient * s,
            old_t - quotient * t,
        );
        old_r = r;
        r = new_r;
        old_s = s;
        s = new_s;
        old_t = t;
        t = new_t;
    }

    (old_s, old_t)
}

struct Rule {
    remainder: i64,
    modulo: i64,
}

impl Rule {
    fn merge(&self, other: &Rule) -> Rule {
        if self.modulo >= other.modulo {
            let (m1, m2) = euclid(self.modulo, other.modulo);
        } else {
            let (m2, m1) = euclid(self.modulo, other.modulo);
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Constraint {
    Bus(usize),
    Blank,
}

struct InputData {
    buses: Vec<Constraint>,
}

fn parse_input() -> InputData {
    let io_result = lines_in_file("inputs/day13.txt");
    match io_result {
        Ok(mut lines) => {
            let _ = lines.next();
            let numbers = lines.next().unwrap().unwrap().split(",").map(|s| {
                match s {
                    "x" => Constraint::Blank,
                    _ => Constraint::Bus(s.parse().unwrap()),
                }
            }).collect();

            InputData {
                buses: numbers,
            }
        },
        Err(_) => panic!("Error reading file"),
    }
}

fn lines_in_file<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

