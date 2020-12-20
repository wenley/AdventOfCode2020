extern crate regex;

use std::io;
use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use regex::Regex;

fn main() {
    let input = parse_input().unwrap();

    let corner_tiles: Vec<_> = input.corner_tiles();

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
enum TileEdge {
    Top(Edge),
    Bottom(Edge),
    Left(Edge),
    Right(Edge),
}
impl TileEdge {
    fn edge(&self) -> &Edge {
        match self {
            TileEdge::Top(e) => e,
            TileEdge::Bottom(e) => e,
            TileEdge::Left(e) => e,
            TileEdge::Right(e) => e,
        }
    }
}

impl Tile {
    fn tile_edges(&self) -> Vec<TileEdge> {
        vec![
            TileEdge::Top(self.top_edge()),
            TileEdge::Bottom(self.bottom_edge()),
            TileEdge::Left(self.left_edge()),
            TileEdge::Right(self.right_edge()),
        ]
    }

    fn top_edge(&self) -> Edge {
        Edge {
            pixels: self.row(0)
        }
    }
    fn bottom_edge(&self) -> Edge {
        Edge {
            pixels: self.row(self.pixels.len() - 1)
        }
    }
    fn left_edge(&self) -> Edge {
        Edge {
            pixels: self.col(0)
        }
    }
    fn right_edge(&self) -> Edge {
        Edge {
            pixels: self.col(self.pixels.first().map(|v| v.len() - 1).unwrap())
        }
    }

    fn row(&self, i: usize) -> Vec<Pixel> {
        self.pixels.iter().nth(i).map(|vec| vec.iter().map(|p| *p).collect()).unwrap()
    }
    fn col(&self, j: usize) -> Vec<Pixel> {
        self.pixels.iter().map(|row| row.iter().nth(j).map(|p| *p).unwrap()).collect()
    }

    fn rotate_right(&self) -> Tile {
        Tile {
            pixels: (0..self.pixels.len()).map(|new_row| {
                self.col(new_row).iter().rev().map(|p| *p).collect()
            }).collect(),
        }
    }
    fn flip(&self) -> Tile {
        Tile {
            pixels: self.pixels.
                iter().
                map(|row| row.iter().rev().map(|p| *p).collect()).
                collect()
        }
    }

    fn transformed_tiles(&self) -> Vec<Tile> {
        vec![
            self.clone(),
            self.rotate_right(),
            self.rotate_right().rotate_right(),
            self.rotate_right().rotate_right().rotate_right(),
            self.flip(),
            self.flip().rotate_right(),
            self.flip().rotate_right().rotate_right(),
            self.flip().rotate_right().rotate_right().rotate_right(),
        ]
    }
}

struct InputData {
    tiles: HashMap<usize, Tile>,
}

impl InputData {
    fn complete_picture(&self) -> Vec<Vec<Pixel>> {
        vec![]
    }

    fn aligned_tiles(&self) -> Vec<Vec<Tile>> {
        let mut rows = vec![];
        // let mut current_row = vec![];
        rows
    }

    fn edge_to_tile_ids(&self) -> HashMap<u64, Vec<usize>> {
        let mut edge_to_tile_ids: HashMap<u64, Vec<usize>>  = HashMap::new();
        self.tiles.iter().for_each(|(identifier, tile)| {
            tile.tile_edges().iter().for_each(|tile_edge| {
                let edge = tile_edge.edge();
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
                tile_edges().
                iter().
                map(|tile_edge| {
                    edge_to_tile_ids.get(&tile_edge.edge().identifier()).map_or(0, |v| v.len())
                }).
                filter(|count| *count == 1).
                count();
            tile_to_unique_edge_count.insert(*identifier, unique_edges);
        });
        tile_to_unique_edge_count
    }

    fn corner_tiles(&self) -> Vec<usize> {
        let tile_to_unique_edge_count = self.tile_to_unique_edge_count();
        tile_to_unique_edge_count.iter().filter(|(_, unique_edges)| {
            **unique_edges >= 2
        }).map(|(id, _)| id).collect()
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

