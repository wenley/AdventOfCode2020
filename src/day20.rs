extern crate regex;

use std::io;
use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use regex::Regex;

fn main() {
    let input = parse_input().unwrap();

    let mut edge_to_tile_count: HashMap<u64, usize>  = HashMap::new();
    input.tiles.iter().for_each(|tile| {
        tile.edges().iter().for_each(|edge| {
            let id = edge.identifier();
            match edge_to_tile_count.remove(&id) {
                None => {
                    edge_to_tile_count.insert(id, 1);
                }
                Some(count) => {
                    edge_to_tile_count.insert(id, count + 1);
                }
            }
        });
    });

    let mut tile_to_unique_edge_count = HashMap::new();
    input.tiles.iter().for_each(|tile| {
        let unique_edges = tile.
            edges().
            iter().
            map(|edge| {
                edge_to_tile_count.get(&edge.identifier()).unwrap()
            }).
            filter(|count| **count == 1).
            count();
        tile_to_unique_edge_count.insert(tile.identifier, unique_edges);
    });

    let corner_tiles: Vec<_> = tile_to_unique_edge_count.iter().filter(|(_, unique_edges)| {
        **unique_edges >= 2
    }).map(|(id, _)| id).collect();
    println!("Corner tiles = {:?}", corner_tiles.iter().fold(1, |acc, i| acc * **i));
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    On,
    Off,
}
#[derive(Hash, Debug, Clone)]
struct Tile {
    identifier: usize,
    pixels: Vec<Vec<Pixel>>,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
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
                map(|vec| { vec.first().map(|p| *p).unwrap() }).
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
    let identifier_regex = Regex::new(r"^Tile (?P<digits>\d+):$").unwrap();

    let tiles = tile_lines.map(|lines| {
        let mut iter = lines.split("\n");
        let identifier_line = iter.next().unwrap();
        let identifier: usize = identifier_regex.
            captures(identifier_line).
            and_then(|cap| cap.name("digits").and_then(|m| m.as_str().parse().ok())).
            unwrap();
        let pixels = iter.map(|line| {
            line.chars().map(|c| {
                match c {
                    '.' => Pixel::Off,
                    '#' => Pixel::On,
                    _ => panic!("Unknown char in inpu {}", c),
                }
            }).collect::<Vec<_>>()
        }).filter(|v| v.len() > 0).collect();
        Tile {
            identifier: identifier,
            pixels: pixels,
        }
    }).collect();

    Ok(InputData {
        tiles: tiles,
    })
}

