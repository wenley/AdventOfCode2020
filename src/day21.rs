extern crate regex;

use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input = parse_input();
    let all_ingredients = input.all_ingredients();
    let mut map = input.allergen_to_possible_ingredients();
    let ingredients_without_allergens = map.values().fold(all_ingredients, |ingredients, potential_allergens| {
        ingredients.difference(potential_allergens).map(|s| *s).collect()
    });
    let no_allergen_ingredient_count = ingredients_without_allergens.iter().map(|ingredient| {
        input.rows.iter().filter(|row| row.ingredients.contains(*ingredient)).count()
    }).sum::<usize>();
    println!("No-allergen ingredient appearance count: {}", no_allergen_ingredient_count);

    let solution = deduce(&mut map);
    println!("Full Solution: {:?}", solution);
    let mut pairs = solution.iter().collect::<Vec<_>>();
    pairs.sort_by_key(|(allergen, _)| allergen.to_string());
    let ordered_solution = pairs.iter().map(|(_, ingredient)| ingredient.to_string()).collect::<Vec<Ingredient>>().join(",");
    println!("Ordered: {}", ordered_solution);
}

struct Row {
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}
type Ingredient = String;
type Allergen = String;

struct InputData {
    rows: Vec<Row>,
}

fn deduce<'a>(allergen_to_possible_ingredients: &mut HashMap<&'a Allergen, HashSet<&'a Ingredient>>) -> HashMap<&'a Allergen, &'a Ingredient> {
    let mut solution: HashMap<&'a Allergen, &'a Ingredient> = HashMap::new();
    while solution.len() < allergen_to_possible_ingredients.len() {
        // eprintln!("Fresh Iteration...");
        let made_progress = allergen_to_possible_ingredients.iter().any(|(allergen, possible_ingredients): (&&Allergen, _)| {
            let without_solved: HashSet<_> = possible_ingredients.
                difference(&solution.values().map(|s| *s).collect()).
                map(|s| *s).
                collect();
            // eprintln!("Testing {} - got {:?} candidates", allergen, without_solved);
            if without_solved.len() == 1 {
                solution.insert(allergen, without_solved.iter().nth(0).map(|s| *s).unwrap());
                true
            } else {
                false
            }
        });
        if !made_progress {
            panic!("Partial Solution: {:?}", solution);
        }
    }
    solution
}

impl InputData {
    fn allergen_to_possible_ingredients(&self) -> HashMap<&Allergen, HashSet<&Ingredient>> {
        let mut map: HashMap<&Allergen, HashSet<&Ingredient>> = HashMap::new();
        self.rows.iter().for_each(|row| {
            row.allergens.iter().for_each(|allergen| {
                match map.get(allergen) {
                    None => {
                        map.insert(allergen, row.ingredients.iter().collect());
                    }
                    Some(ingredients) => {
                        let common_ingredients = ingredients.
                            intersection(&row.ingredients.iter().collect()).
                            map(|s| *s).
                            collect();
                        map.insert(allergen, common_ingredients);
                    }
                }
            });
        });
        map
    }

    fn all_ingredients(&self) -> HashSet<&Ingredient> {
        self.rows.iter().fold(HashSet::new(), |acc, row| {
            acc.union(&row.ingredients.iter().collect()).map(|s| *s).collect()
        })
    }
}

fn parse_input() -> InputData {
    let content = fs::read_to_string("inputs/day21.txt").unwrap();
    let stuff_regex = Regex::new(r"^(?P<ingredients>[a-z ]+) \(contains (?P<allergens>[a-z, ]+)\)$").unwrap();
    let rows = content.split("\n").map(|line| {
        let caps = match stuff_regex.captures(line) {
            None => panic!("Couldn't parse line {}", line),
            Some(c) => c,
        };
        let ingredients = caps.name("ingredients").unwrap().as_str().split(" ").map(|s| s.to_string()).collect();
        let allergens = caps.name("allergens").unwrap().as_str().split(", ").map(|s| s.to_string()).collect();

        Row {
            ingredients: ingredients,
            allergens: allergens,
        }
    }).collect();
    InputData {
        rows: rows,
    }
}

