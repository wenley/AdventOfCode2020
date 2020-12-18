extern crate regex;

use std::collections::HashMap;

fn main() {
    let numbers = vec![8,11,0,19,1,2];
    let mut last_turn_spoken = HashMap::new();
    let mut turn = 1;
    let mut last_number = -1;

    for n in numbers.iter() {
        last_turn_spoken.insert(last_number, turn);
        println!("{} spoken on turn {}", n, turn);
        last_number = *n;
        turn += 1;
    }

    while turn <= 30_000_000 {
        let n = last_turn_spoken.get(&last_number).map(|i| turn - i).unwrap_or(0);
        last_turn_spoken.insert(last_number, turn);
        if turn % 1_000_000 == 0 {
            println!("{} spoken on turn {}", n, turn);
        }
        last_number = n;
        turn += 1;
    }
    println!("{} spoken on turn {}", last_number, turn);
}

