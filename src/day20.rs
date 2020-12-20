extern crate regex;

use std::io;
use std::fs;
use std::collections::HashMap;

fn main() {
    let input = parse_input().unwrap();
}

enum Pixel {
    On,
    Off,
}
struct Tile {
    pixels: Vec<Vec<Pixel>>,
}

struct InputData {
    tiles: Vec<Tile>,
}

fn parse_input() -> io::Result<InputData> {
    let content = fs::read_to_string("inputs/day20.txt")?;
    let tile_lines = content.split("\n\n");

    let tiles = tile_lines.map(|lines| {
        let pixels = lines.split("\n").skip(1).map(|line| {
            line.chars().map(|c| {
                match c {
                    '.' => Pixel::Off,
                    '#' => Pixel::On,
                    _ => panic!("Unknown char in inpu {}", c),
                }
            }).collect()
        }).collect();
        Tile { pixels: pixels }
    }).collect();

    Ok(InputData {
        tiles: tiles,
    })
}

