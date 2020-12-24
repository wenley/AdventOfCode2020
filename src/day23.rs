extern crate regex;

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let mut input = part1_input();
    println!("{:?}", input.ring);

    for _ in 0..100 {
        input.do_move();
        // println!("{:?}", input.ring);
    }

    println!("After 100 moves: {:?}", input.ring);

    let input2 = part2_input();
    let mut circle = EfficientCircle::from_values(input2.ring.iter().map(|i| *i).collect());
    let mut current_cup = *input.ring.front().unwrap();
    for i in 0..10_000_000 {
        if i % 100_000 == 0 {
            println!("{} iterations", i);
        }
        current_cup = circle.next_current_cup(current_cup);
    }
    println!("After 10m moves: {:?} and {:?}", circle.cups.get(&1), circle.cups.get(circle.cups.get(&1).unwrap()));
}

struct InputData {
    max: usize,
    ring: VecDeque<usize>,
}

struct EfficientCircle {
    max: usize,
    cups: HashMap<usize, usize>
}

impl EfficientCircle {
    fn from_values(values: Vec<usize>) -> EfficientCircle {
        let mut cups = HashMap::new();

        cups.insert(*values.last().unwrap(), values[0]);
        for i in 0..values.len() - 1 {
            cups.insert(values[i], values[i + 1]);
        }

        EfficientCircle {
            max: values.iter().max().map(|i| *i).unwrap(),
            cups: cups,
        }
    }

    fn next_current_cup(&mut self, current_cup: usize) -> usize {
        let first_cup = *self.cups.get(&current_cup).unwrap();
        let second_cup = *self.cups.get(&first_cup).unwrap();
        let third_cup = *self.cups.get(&second_cup).unwrap();
        let fourth_cup = *self.cups.get(&third_cup).unwrap();

        let next_cup = self.next_cup(
            current_cup,
            &vec![first_cup, second_cup, third_cup],
        );
        let after_cup = *self.cups.get(&next_cup).unwrap();

        self.cups.insert(current_cup, fourth_cup);
        self.cups.insert(next_cup, first_cup);
        self.cups.insert(third_cup, after_cup);

        fourth_cup
    }

    fn next_cup(&self, current_cup: usize, picked_up_cups: &Vec<usize>) -> usize {
        let cups_to_move_set: HashSet<_> = picked_up_cups.iter().map(|i| *i).collect();

        let mut cup_to_place_after = current_cup - 1;
        if cup_to_place_after == 0 {
            cup_to_place_after = self.max;
        }
        while cups_to_move_set.contains(&cup_to_place_after) {
            cup_to_place_after -= 1;
            if cup_to_place_after == 0 {
                cup_to_place_after = self.max;
            }
        }
        cup_to_place_after
    }
}

impl InputData {
    fn do_move(&mut self) {
        let mut work_space = VecDeque::new();
        let current_cup_num = self.ring.pop_front().unwrap();
        work_space.push_back(current_cup_num);

        let cups_to_move = self.pick_up();

        let cup_to_place_after = self.next_cup(current_cup_num, &cups_to_move);

        let mut next_cup = self.ring.pop_front().unwrap();
        work_space.push_back(next_cup);
        while next_cup != cup_to_place_after {
            next_cup = self.ring.pop_front().unwrap();
            work_space.push_back(next_cup);
        }
        cups_to_move.iter().rev().for_each(|i| { self.ring.push_front(*i); });
        work_space.iter().rev().for_each(|i| { self.ring.push_front(*i); });
        self.ring.rotate_left(1);
    }

    fn pick_up(&mut self) -> Vec<usize> {
        vec![
            self.ring.pop_front().unwrap(),
            self.ring.pop_front().unwrap(),
            self.ring.pop_front().unwrap(),
        ]
    }

    fn next_cup(&self, current_cup: usize, picked_up_cups: &Vec<usize>) -> usize {
        let cups_to_move_set: HashSet<_> = picked_up_cups.iter().map(|i| *i).collect();

        let mut cup_to_place_after = current_cup - 1;
        if cup_to_place_after == 0 {
            cup_to_place_after = self.max;
        }
        while cups_to_move_set.contains(&cup_to_place_after) {
            cup_to_place_after -= 1;
            if cup_to_place_after == 0 {
                cup_to_place_after = self.max;
            }
        }
        cup_to_place_after
    }
}

fn part2_input() -> InputData {
    let mut input = part1_input();
    for i in 10..=1_000_000 {
        input.ring.push_back(i);
    }
    input.max = 1_000_000;
    input
}

fn part1_input() -> InputData {
    let deque = vec![6, 2, 4, 3, 9, 7, 1, 5, 8].iter().map(|i| *i).collect();

    InputData {
        max: 9,
        ring: deque,
    }
}

