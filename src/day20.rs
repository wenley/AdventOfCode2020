extern crate regex;

use std::io;
use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use regex::Regex;

fn main() {
    let input = parse_input().unwrap();

    let tile_to_unique_edge_count = input.tile_to_unique_edge_count();

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
#[derive(Hash, Debug, Clone, PartialEq, Eq)]
struct Edge {
    pixels: Vec<Pixel>
}
impl Edge {
    fn identifier(&self) -> u64 {
        let (f, b) = (self.forward_identifier(), self.backward_identifier());
        if f <= b {
            f
        } else {
            b
        }
    }

    fn forward_identifier(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.pixels.hash(&mut hasher);
        hasher.finish()
    }

    fn backward_identifier(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.pixels.iter().rev().collect::<Vec<_>>().hash(&mut hasher);
        hasher.finish()
    }

    fn alignment(&self, other: &Edge) -> EdgeAlignment {
        if self.forward_identifier() == other.forward_identifier() {
            EdgeAlignment::Good
        } else if self.forward_identifier() == other.backward_identifier() {
            EdgeAlignment::Flip
        } else {
            EdgeAlignment::None
        }
    }
}

enum EdgeAlignment {
    Good,
    Flip,
    None,
}

#[derive(Hash, Debug, Clone)]
struct Tile {
    pixels: Vec<Vec<Pixel>>,
}

impl Tile {
    fn edges(&self) -> Vec<Edge> {
        vec![
            self.top_edge(),
            self.bottom_edge(),
            self.left_edge(),
            self.right_edge(),
        ]
    }

    fn top_edge(&self) -> Edge {
        Edge {
            pixels: self.pixels.
                first().
                map(|vec| vec.iter().map(|p| *p).collect()).
                unwrap(),
        }
    }
    fn bottom_edge(&self) -> Edge {
        Edge {
            pixels: self.pixels.
                last().
                map(|vec| vec.iter().map(|p| *p).collect()).
                unwrap(),
        }
    }
    fn left_edge(&self) -> Edge {
        Edge {
            pixels: self.pixels.
                iter().
                map(|vec| { vec.first().map(|p| *p).unwrap() }).
                collect(),
        }
    }
    fn right_edge(&self) -> Edge {
        Edge {
            pixels: self.pixels.
                iter().
                map(|vec| vec.last().map(|p| *p).unwrap()).
                collect(),
        }
    }
}

struct InputData {
    tiles: HashMap<usize, Tile>,
}

impl InputData {
    fn edge_to_tile_ids(&self) -> HashMap<u64, Vec<usize>> {
        let mut edge_to_tile_ids: HashMap<u64, Vec<usize>>  = HashMap::new();
        self.tiles.iter().for_each(|(identifier, tile)| {
            tile.edges().iter().for_each(|edge| {
                let id = edge.identifier();
                match edge_to_tile_ids.remove(&id) {
                    None => {
                        edge_to_tile_ids.insert(id, vec![*identifier]);
                    }
                    Some(mut tiles) => {
                        tiles.push(*identifier);
                        edge_to_tile_ids.insert(id, tiles);
                    }
                }
            });
        });
        edge_to_tile_ids
    }

    fn tile_to_unique_edge_count(&self) -> HashMap<usize, usize> {
        let edge_to_tile_ids = self.edge_to_tile_ids();
        let mut tile_to_unique_edge_count = HashMap::new();
        self.tiles.iter().for_each(|(identifier, tile)| {
            let unique_edges = tile.
                edges().
                iter().
                map(|edge| {
                    edge_to_tile_ids.get(&edge.identifier()).map_or(0, |v| v.len())
                }).
                filter(|count| *count == 1).
                count();
            tile_to_unique_edge_count.insert(*identifier, unique_edges);
        });
        tile_to_unique_edge_count
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

        (identifier, Tile { pixels: pixels })
    }).collect();

    Ok(InputData {
        tiles: tiles,
    })
}

