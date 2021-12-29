extern crate clap;

use std::{collections::HashMap};

use clap::{App, Arg};
use itertools::Itertools;

struct Polymerizer {
    rules: HashMap<(usize, usize), usize>,
    characters_in_rules: Vec<char>
}

impl Polymerizer {
    fn new(rule_string: &str) -> Self {
        let mut constructed = Self {
            rules: HashMap::new(),
            characters_in_rules: vec![],
        };
        
        for line in rule_string.lines() {
            let (to_match, insertion) = line.split_once(" -> ").unwrap();
            let a = to_match.chars().nth(0).unwrap();
            let b = to_match.chars().nth(1).unwrap();
            let production = insertion.chars().nth(0).unwrap();

            constructed.add_rule((a,b), production);
        }

        return constructed;
    }

    fn add_rule(&mut self, (a, b): (char, char), production: char) {
        let idx_a = self.get_or_insert_char_idx(a);
        let idx_b = self.get_or_insert_char_idx(b);
        let idx_p = self.get_or_insert_char_idx(production);

        self.rules.insert((idx_a, idx_b), idx_p);
    }

    fn get_or_insert_char_idx(&mut self, value: char) -> usize {
        match self.characters_in_rules.iter().position(|x| *x == value) {
            Some(idx) => return idx,
            None => {
                self.characters_in_rules.push(value);
                return self.characters_in_rules.len() - 1;
            },
        }
    }

    fn character_frequencies_after_steps(&self, initial_polymer: &str, steps: u32) -> HashMap<char, usize> {
        let mut characters_in_polymer = self.characters_in_rules.clone();
        for c in initial_polymer.chars() {
            if characters_in_polymer.iter().position(|x| *x == c).is_none() {
                characters_in_polymer.push(c);
            }
        }


        let alphabet_size = characters_in_polymer.len();
        let mut cache: HashMap<(usize, usize, u32), Vec<usize>> = HashMap::new();
        let mut counts: Vec<usize> = vec![0; alphabet_size];


        for (a, b) in initial_polymer.chars().tuple_windows() {
            let idx_a = characters_in_polymer.iter().position(|x| *x == a).unwrap();
            let idx_b = characters_in_polymer.iter().position(|x| *x == b).unwrap();
            
            let res = self.calculate_frequencies(idx_a, idx_b, steps - 1, alphabet_size, &mut cache);

            for i in 0..alphabet_size {
                counts[i] += res[i];
            }
        }

        // also count the last character
        let last_char_idx = characters_in_polymer.iter().position(|x| *x == initial_polymer.chars().last().unwrap()).unwrap();
        counts[last_char_idx] += 1;

        // Transform result into expected hashmap form
        let mut result: HashMap<char, usize> = HashMap::new();
        for i in 0..alphabet_size {
            result.insert(characters_in_polymer[i], counts[i]);
        }

        result
    }

    fn calculate_frequencies(
            &self, 
            idx_a: usize, 
            idx_b: usize, 
            depth: u32,
            alphabet_size: usize,
            result_cache: &mut HashMap<(usize, usize, u32), Vec<usize>>
        ) -> Vec<usize> {
        if result_cache.contains_key(&(idx_a, idx_b, depth)) {
            return result_cache.get(&(idx_a, idx_b, depth)).unwrap().clone();
        } else {
            let mut result = vec![0; alphabet_size];
            if self.rules.contains_key(&(idx_a, idx_b)) {
                let idx_prod = *self.rules.get(&(idx_a, idx_b)).unwrap();
                
                if depth == 0 {
                    result[idx_a] += 1;
                    result[idx_prod] += 1;
                } else {
                    let left_recursion_result = self.calculate_frequencies(idx_a, idx_prod, depth - 1, alphabet_size, result_cache);
                    let right_recursion_result = self.calculate_frequencies(idx_prod, idx_b, depth - 1, alphabet_size, result_cache);

                    for i in 0..alphabet_size {
                        result[i] += left_recursion_result[i];
                        result[i] += right_recursion_result[i];
                    }
                }
            } else {
                // We don't have a rule, no recursion needed!
                result[idx_a] += 1;
            }

            // cache result
            result_cache.insert((idx_a, idx_b, depth), result.clone());
            return result;
        }
    }
}

fn main() {
    let commandline_matches = App::new("Advent of Code Day 14")
                    .arg(Arg::with_name("INPUT")
                        .help("Inputfile to parse.")
                        .required(true)
                        .index(1))
                    .arg(Arg::with_name("STEPS")
                        .help("Steps to simulate")
                        .required(true)
                        .index(2))
                    .get_matches();

    let input_file_path = commandline_matches.value_of("INPUT").unwrap();
    let steps: u32 = commandline_matches.value_of("STEPS").unwrap().parse().unwrap();

    let file_contents = std::fs::read_to_string(input_file_path)
        .expect("Failed to open the readings file");

    let (beginning_polymer, rule_string) = file_contents.trim_end().split_once("\n\n").unwrap();

    let polymerizer = Polymerizer::new(rule_string);
    let counters = polymerizer.character_frequencies_after_steps(beginning_polymer, steps);

    let most_frequent = counters.values().max().unwrap();
    let least_frequent = counters.values().min().unwrap();


    println!("Most common minus least common character frequency after {} steps: {}",steps, most_frequent - least_frequent);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = 
r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

#[test]
fn test_rule_parsing() {
    let (_, rule_string) = EXAMPLE_INPUT.trim_end().split_once("\n\n").unwrap();
    let polymerizer = Polymerizer::new(rule_string);

    assert_eq!(*polymerizer.rules.get(&(0, 1)).unwrap(), 2);
    assert_eq!(*polymerizer.rules.get(&(0, 3)).unwrap(), 0);
    assert_eq!(polymerizer.rules.len(), 16);
}

#[test]
fn test_example_depth_1() {
    let (beginning_polymer, rule_string) = EXAMPLE_INPUT.trim_end().split_once("\n\n").unwrap();
    let polymerizer = Polymerizer::new(rule_string);

    let character_counts = polymerizer.character_frequencies_after_steps(beginning_polymer, 1);

    assert_eq!(*character_counts.get(&'B').unwrap(), 2);
    assert_eq!(*character_counts.get(&'C').unwrap(), 2);
    assert_eq!(*character_counts.get(&'H').unwrap(), 1);
    assert_eq!(*character_counts.get(&'N').unwrap(), 2);
}

#[test]
fn test_example_depth_10() {
    let (beginning_polymer, rule_string) = EXAMPLE_INPUT.trim_end().split_once("\n\n").unwrap();
    let polymerizer = Polymerizer::new(rule_string);

    let character_counts = polymerizer.character_frequencies_after_steps(beginning_polymer, 10);

    assert_eq!(*character_counts.get(&'B').unwrap(), 1749);
    assert_eq!(*character_counts.get(&'C').unwrap(), 298);
    assert_eq!(*character_counts.get(&'H').unwrap(), 161);
    assert_eq!(*character_counts.get(&'N').unwrap(), 865);
}