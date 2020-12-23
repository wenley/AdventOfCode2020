extern crate regex;

use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut input = part1_input();
    println!("{:?}", input.ring);

    for _ in 0..100 {
        input.do_move();
        // println!("{:?}", input.ring);
    }

    println!("After 100 moves: {:?}", input.ring);
}

struct InputData {
    max: usize,
    ring: VecDeque<usize>,
}

struct Cup {
    value: usize,
    next: Option<Rc<RefCell<Cup>>>,
}

struct EfficientCircle {
    cups: HashMap<usize, Rc<RefCell<Cup>>>,
}

fn link_cups(cups: &mut Vec<Rc<RefCell<Cup>>>, from: usize, to: usize) {
    let cup2 = cups.get(to).unwrap().clone();
    let cup1 = cups.get_mut(from).unwrap();
    Rc::get_mut(cup1).unwrap().borrow_mut().next = Some(cup2.clone());
}

impl EfficientCircle {
    fn from_values(values: Vec<usize>) -> EfficientCircle {
        let mut cups: Vec<Rc<RefCell<Cup>>> = values.iter().map(|v| {
            Rc::new(
                RefCell::new(Cup {
                    value: *v,
                    next: None,
                })
            )
        }).collect();

        let num_cups = cups.len();
        link_cups(&mut cups, num_cups - 1, 0);
        for i in (0..num_cups - 1) {
            link_cups(&mut cups, i, i + 1);
        }

        EfficientCircle {
            cups: cups.drain(0..).map(|c| {
                let val = c.borrow().value;
                (val, c)
            }).collect()
        }
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
    for i in (10..=1_000_000) {
        input.ring.push_back(i);
    }
    input.max = 1_000_000;
    input
}

fn part1_input() -> InputData {
    let mut deque = vec![6, 2, 4, 3, 9, 7, 1, 5, 8].iter().map(|i| *i).collect();

    InputData {
        max: 9,
        ring: deque,
    }
}

