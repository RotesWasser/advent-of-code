extern crate clap;

use std::{collections::{HashMap, hash_map::Entry}, iter};

use clap::{App, Arg};
use itertools::Itertools;

struct Polymerizer {
    rules: HashMap<(char, char), char>
}

impl Polymerizer {
    fn new(rule_string: &str) -> Self {
        let mut rules = HashMap::new();
        
        for line in rule_string.lines() {
            let (to_match, insertion) = line.split_once(" -> ").unwrap();
            rules.insert(
                (
                    to_match.chars().nth(0).unwrap(), 
                    to_match.chars().nth(1).unwrap()
                ), 
                insertion.chars().nth(0).unwrap());
        }

        return Self {
            rules
        };
    }

    fn polymerize<'a, I>(&self, to_polymerize: I) -> impl Iterator<Item = char>
    where
        I: IntoIterator<Item = char>
    {
        let mut it = to_polymerize.into_iter();

        let first = it.next().unwrap();
        let windows = iter::once(first).chain(it).tuple_windows();

        let rules = self.rules.clone();

        let result = 
            iter::once(vec![first])
            .chain(
                windows.map(move |(first, second)| {
                    let key: (char, char) = (first, second);
                    if rules.contains_key(&key) {
                        return vec![*rules.get(&key).unwrap(), second]
                    } else {
                        return vec![second]
                    }
                })
            ).flatten();
        
        result
    }

    fn calculate_character_frequencies_after_steps(&self, initial_polymer: &str, steps: u32) -> HashMap<char, usize> {

        let expected_polymer_length = initial_polymer.len() * 2usize.pow(steps);
        println!("Expected polymer length: {}", expected_polymer_length);

        let mut polymer_iter: Box<dyn Iterator<Item = char>> = Box::new(initial_polymer.chars().into_iter());
        for _ in 0..steps {
            polymer_iter = Box::new(self.polymerize(polymer_iter));
        }

        let mut counters: HashMap<char, usize> = HashMap::new();
        for (n, c) in polymer_iter.enumerate() {
            match counters.entry(c) {
                Entry::Occupied(mut occupied) => {
                    occupied.insert(occupied.get() + 1);
                },
                Entry::Vacant(vacant) => {
                    vacant.insert(1);
                },
            }

            if n % 100000000 == 0 {
                println!("Progress: {}%", (n as f64) / (expected_polymer_length as f64) * 100f64);
            }
        }

        return counters;
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
    let counters = polymerizer.calculate_character_frequencies_after_steps(beginning_polymer, steps);

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

    assert_eq!(*polymerizer.rules.get(&('C', 'H')).unwrap(), 'B');
    assert_eq!(*polymerizer.rules.get(&('C', 'N')).unwrap(), 'C');
    assert_eq!(polymerizer.rules.len(), 16);
}

#[test]
fn test_polymerization_without_rules() {
    let polymerizer = Polymerizer::new("");
    let once: String = polymerizer.polymerize("ABCDEF".chars()).collect();

    assert_eq!(once, "ABCDEF");
}

#[test]
fn test_example_explicit() {
    let (beginning_polymer, rule_string) = EXAMPLE_INPUT.trim_end().split_once("\n\n").unwrap();
    let polymerizer = Polymerizer::new(rule_string);

    let once: String = polymerizer.polymerize(beginning_polymer.chars()).collect();
    let twice: String = polymerizer.polymerize(polymerizer.polymerize(beginning_polymer.chars())).collect();
    assert_eq!(once,  "NCNBCHB");
    assert_eq!(twice, "NBCCNBBBCBHCB");
}

#[test]
fn test_example_counters() {
    let (beginning_polymer, rule_string) = EXAMPLE_INPUT.trim_end().split_once("\n\n").unwrap();
    let polymerizer = Polymerizer::new(rule_string);

    let character_counts = polymerizer.calculate_character_frequencies_after_steps(beginning_polymer, 10);

    assert_eq!(*character_counts.get(&'B').unwrap(), 1749);
    assert_eq!(*character_counts.get(&'C').unwrap(), 298);
    assert_eq!(*character_counts.get(&'H').unwrap(), 161);
    assert_eq!(*character_counts.get(&'N').unwrap(), 865);
}