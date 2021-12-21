use std::{error::Error, collections::{HashMap, HashSet}};

use clap::{App, Arg};
use itertools::Itertools;

struct SevenSegmentSolver {
    mapping: HashMap<String, u8>
}

impl SevenSegmentSolver {
    fn new(observed_values: Vec<&str>) -> Self {
        // Solve and store mapping

        let mut solver = SevenSegmentSolver {
            mapping: HashMap::new()
        };

        solver.mapping.insert(observed_values.iter().find(|x| x.len() == 2).unwrap().chars().sorted().collect(), 1);
        solver.mapping.insert(observed_values.iter().find(|x| x.len() == 4).unwrap().chars().sorted().collect(), 4);
        solver.mapping.insert(observed_values.iter().find(|x| x.len() == 3).unwrap().chars().sorted().collect(), 7);
        solver.mapping.insert(observed_values.iter().find(|x| x.len() == 7).unwrap().chars().sorted().collect(), 8);

        return solver;
    }

    fn parse(&self, digit_value: &str) -> Option<u8> {
        let sorted: String = digit_value.chars().sorted().collect();
        match self.mapping.get(&sorted) {
            Some(x) => Some((*x).clone()),
            None => None,
        }
    }
}

fn main() {
    let matches = App::new("Advent of Code Day 8")
                    .arg(Arg::with_name("INPUT")
                        .help("Input file to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input = std::fs::read_to_string(&matches.value_of("INPUT").unwrap())
        .expect("Failed to open the readings file");

    let unpacked_input_lines: Vec<(Vec<&str>, Vec<&str>)> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| unpack_input_line(line))
        .collect(); 
    
    let mut digit_occurrances = [0u32;10];

    for (observed_values, to_decode) in unpacked_input_lines {
        let solver = SevenSegmentSolver::new(observed_values);

        for digit in to_decode {
            match solver.parse(digit) {
                Some(x) => digit_occurrances[x as usize] += 1,
                None => continue,
            }
        }
    }

    println!("Amount of digits with unique output values: {}", digit_occurrances[1] + digit_occurrances[4] + digit_occurrances[7] + digit_occurrances[8]);
}

fn unpack_input_line(line: &str) -> (Vec<&str>, Vec<&str>){
    let (observed, digits) = line.split_once(" | ").unwrap();
    return (observed.split(" ").collect(), digits.split(" ").collect());
}


#[cfg(test)]
const TEST_INPUT: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

#[test]
fn test_cheap_digit_identification() {
    let (observed, to_parse) = unpack_input_line(TEST_INPUT);

    let solver = SevenSegmentSolver::new(observed);

    assert_eq!(solver.parse("ab").unwrap(), 1);
    assert_eq!(solver.parse("ba").unwrap(), 1);
    
    assert_eq!(solver.parse("dab").unwrap(), 7);
    assert_eq!(solver.parse("eafb").unwrap(), 4);
    assert_eq!(solver.parse("acedgfb").unwrap(), 8);
    
}