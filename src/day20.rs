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
    let corner_tiles: Vec<_> = input.corner_tiles(&tile_to_unique_edge_count);

    println!("Corner tiles = {:?}", corner_tiles.iter().fold(1, |acc, i| acc * *i));
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
    identifier: usize,
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
            identifier: self.identifier,
            pixels: (0..self.pixels.len()).map(|new_row| {
                self.col(new_row).iter().rev().map(|p| *p).collect()
            }).collect(),
        }
    }
    fn flip(&self) -> Tile {
        Tile {
            identifier: self.identifier,
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

    fn trim_edges(&self) -> Tile {
        let row_count = self.pixels.len();
        let trimmed_pixels = (1..row_count - 1).map(|i| {
            let row = self.row(i);
            let row_length = row.len();
            row[1..row_length - 1].iter().map(|p| *p).collect()
        }).collect();
        Tile { identifier: self.identifier, pixels: trimmed_pixels }
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
        let edge_to_tile_ids = self.edge_to_tile_ids();
        let tile_to_unique_edge_count = self.tile_to_unique_edge_count();
        let first_corner_tile = self.corner_tiles(&tile_to_unique_edge_count).
            first().
            and_then(|id| self.tiles.get(id)).
            and_then(|tile| {
                tile.transformed_tiles().iter().find(|t| {
                    let top_unique = edge_to_tile_ids.get(&t.top_edge().identifier()).map_or(false, |ids| ids.len() == 1);
                    let left_unique = edge_to_tile_ids.get(&t.left_edge().identifier()).map_or(false, |ids| ids.len() == 1);

                    top_unique && left_unique
                }).map(|t| t.clone())
            }).unwrap();

        let mut rows = vec![];
        let mut current_row = vec![first_corner_tile.clone()];
        let mut current_tile = first_corner_tile;
        loop {
            match self.tile_right_of(&current_tile, &edge_to_tile_ids) {
                Some(tile) => {
                    current_row.push(tile.clone());
                    current_tile = tile;
                }
                None => {
                    let current_first_tile = current_row.first().unwrap();
                    match self.tile_below(&current_first_tile, &edge_to_tile_ids) {
                        Some(new_row_tile) => {
                            rows.push(current_row);
                            current_row = vec![new_row_tile.clone()];
                            current_tile = new_row_tile;
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }

        rows
    }

    fn tile_right_of(&self, current_tile: &Tile, edge_to_tile_ids: &HashMap<u64, Vec<usize>>) -> Option<Tile> {
        edge_to_tile_ids.
            get(&current_tile.right_edge().identifier()).
            and_then(|ids| ids.iter().find(|id| **id != current_tile.identifier)).
            and_then(|id| self.tiles.get(id)).
            and_then(|tile| {
                tile.transformed_tiles().iter().find(|t| {
                    match current_tile.right_edge().alignment(&t.left_edge()) {
                        EdgeAlignment::Good => true,
                        _ => false,
                    }
                }).map(|t| t.clone())
            }).
            map(|t| t.clone())
    }

    fn tile_below(&self, current_tile: &Tile, edge_to_tile_ids: &HashMap<u64, Vec<usize>>) -> Option<Tile> {
        edge_to_tile_ids.
            get(&current_tile.bottom_edge().identifier()).
            and_then(|ids| ids.iter().find(|id| **id != current_tile.identifier)).
            and_then(|id| self.tiles.get(id)).
            and_then(|tile| {
                tile.transformed_tiles().iter().find(|t| {
                    match current_tile.bottom_edge().alignment(&t.top_edge()) {
                        EdgeAlignment::Good => true,
                        _ => false,
                    }
                }).map(|t| t.clone())
            }).
            map(|t| t.clone())
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

    fn tile_type(&self, identifier: usize, tile_to_unique_edge_count: &HashMap<usize, usize>) -> TileType {
        match tile_to_unique_edge_count.get(&identifier) {
            None => panic!("Couldn't find tile {}", identifier),
            Some(2) => TileType::Corner,
            Some(1) => TileType::Edge,
            Some(0) => TileType::Center,
            Some(count) => panic!("Unexpected unique edge count {}", count),
        }
    }

    fn corner_tiles(&self, tile_to_unique_edge_count: &HashMap<usize, usize>) -> Vec<usize> {
        tile_to_unique_edge_count.iter().filter(|(_, unique_edges)| {
            **unique_edges >= 2
        }).map(|(id, _)| *id).collect()
    }
}

enum TileType {
    Corner,
    Edge,
    Center,
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

        (identifier, Tile { identifier: identifier, pixels: pixels })
    }).collect();

    Ok(InputData {
        tiles: tiles,
    })
}

