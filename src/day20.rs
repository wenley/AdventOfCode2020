extern crate regex;

use std::io;
use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let input = parse_input().unwrap();
}

#[derive(Hash, Debug, Clone, Copy)]
enum Pixel {
    On,
    Off,
}
#[derive(Hash, Debug, Clone)]
struct Tile {
    pixels: Vec<Vec<Pixel>>,
}

#[derive(Hash, Debug, Clone)]
struct Edge {
    pixels: Vec<Pixel>
}

impl Tile {
    fn edges(&self) -> Vec<Edge> {
        let mut edges = vec![];
        edges.push(Edge {
            pixels: self.pixels.
                first().
                map(|vec| vec.iter().map(|p| *p).collect()).
                unwrap(),
        });
        edges.push(Edge {
            pixels: self.pixels.
                last().
                map(|vec| vec.iter().map(|p| *p).collect()).
                unwrap(),
        });
        edges.push(Edge {
            pixels: self.pixels.
                iter().
                map(|vec| vec.first().map(|p| *p).unwrap()).
                collect(),
        });
        edges.push(Edge {
            pixels: self.pixels.
                iter().
                map(|vec| vec.last().map(|p| *p).unwrap()).
                collect(),
        });

        edges
    }
}

struct InputData {
    tiles: Vec<Tile>,
}
impl Edge {
    fn identifier(&self) -> u64 {
        let mut forward_hasher = DefaultHasher::new();
        self.pixels.hash(&mut forward_hasher);
        let forward = forward_hasher.finish();

        let mut backward_hasher = DefaultHasher::new();
        self.pixels.iter().rev().collect::<Vec<_>>().hash(&mut backward_hasher);
        let backward = backward_hasher.finish();

        if forward <= backward {
            forward
        } else {
            backward
        }
    }
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

