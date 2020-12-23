extern crate regex;

use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let input = parse_input();
    let mut game1 = input.clone();
    // game1.play();

    let winner = if game1.player1.len() > 0 {
        game1.player1
    } else {
        game1.player2
    };
    let num_cards = winner.len();
    let score: usize = winner.iter().enumerate().map(|(i, n)| n * (num_cards - i)).sum();
    println!("Part 1 Winner score = {}", score);

    let mut game2 = input.clone();
    let winner_deck = match game2.play2() {
        Player::One => game2.player1,
        Player::Two => game2.player2,
    };
    let num_cards = winner_deck.len();
    let score: usize = winner_deck.iter().enumerate().map(|(i, n)| n * (num_cards - i)).sum();
    println!("Part 2 Winner score = {}", score);
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct InputData {
    game_layer: usize,
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

impl InputData {
    fn play(&mut self) {
        while self.player1.len() > 0 && self.player2.len() > 0 {
            let card1 = self.player1.pop_front().unwrap();
            let card2 = self.player2.pop_front().unwrap();

            if card1 > card2 {
                self.player1.push_back(card1);
                self.player1.push_back(card2);
            } else {
                self.player2.push_back(card2);
                self.player2.push_back(card1);
            }
        }
    }

    fn play2(&mut self) -> Player {
        let mut seen_states = HashSet::new();
        let mut winner = Player::One;
        let mut round = 1;

        while self.player1.len() > 0 && self.player2.len() > 0 {
            if seen_states.contains(self) {
                println!("Already saw {:?}", self);
                return Player::One
            }
            seen_states.insert(self.clone());

            winner = self.play2_round();
            println!("Game {} Round {} winner = {:?}", self.game_layer, round, winner);
            round += 1;
        }

        println!("Game {} winner = {:?}", self.game_layer, winner);
        winner
    }

    fn play2_round(&mut self) -> Player {
        println!("(Game {}) {:?} vs {:?}", self.game_layer, self.player1, self.player2);

        let card1 = self.player1.pop_front().unwrap();
        let card2 = self.player2.pop_front().unwrap();

        let winner = if self.player1.len() >= card1 && self.player2.len() >= card2 {
            let mut subgame = self.clone();
            subgame.game_layer += 1;
            let result = subgame.play2();
            result
        } else {
            if card1 > card2 {
                Player::One
            } else {
                Player::Two
            }
        };
        match winner {
            Player::One => {
                self.player1.push_back(card1);
                self.player1.push_back(card2);
            }
            Player::Two => {
                self.player2.push_back(card2);
                self.player2.push_back(card1);
            }
        };
        winner
    }
}

#[derive(Debug, Clone, Copy)]
enum Player {
    One,
    Two
}

fn parse_input() -> InputData {
    let mut content = fs::read_to_string("inputs/day22-test.txt").
        unwrap().
        split("\n\n").
        map(|s| s.to_string()).
        collect::<Vec<_>>();
    let player2 = parse_player(&content.pop().unwrap());
    let player1 = parse_player(&content.pop().unwrap());

    InputData {
        game_layer: 1,
        player1: player1,
        player2: player2,
    }
}

fn parse_player(deck: &str) -> VecDeque<usize> {
    deck.trim().split("\n").skip(1).map(|s| {
        match s.parse::<usize>() {
            Ok(n) => n,
            Err(_) => panic!("Couldn't parse line {} from deck {}", s, deck),
        }
    }).collect()
}
