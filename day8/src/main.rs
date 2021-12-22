use std::{error::Error, collections::{HashMap, HashSet}, vec};

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

        let two_on = observed_values.iter().find(|x| x.len() == 2).unwrap();
        let four_on = observed_values.iter().find(|x| x.len() == 4).unwrap();
        let seven_on = observed_values.iter().find(|x| x.len() == 3).unwrap();
        let eight_on = observed_values.iter().find(|x| x.len() == 7).unwrap();
        let six_on: Vec<HashSet<char>> = observed_values.iter().filter(|x| x.len() == 6).map(|x|HashSet::from_iter(x.chars())).collect();
        let five_on: Vec<HashSet<char>> = observed_values.iter().filter(|x| x.len() == 5).map(|x|HashSet::from_iter(x.chars())).collect();

        let segment_cf: HashSet<char> = HashSet::from_iter(two_on.chars());
        let segment_bcdf: HashSet<char> = HashSet::from_iter(four_on.chars());
        let segment_acf: HashSet<char> = HashSet::from_iter(seven_on.chars());
        let segment_abcdefg: HashSet<char> = HashSet::from_iter(eight_on.chars());

        let segment_a = segment_acf.difference(&segment_cf).next().unwrap().clone();

        let segment_bd: HashSet<char> = segment_bcdf.difference(&segment_cf).map(|x| x.clone()).collect();

        let mut segment_cde: HashSet<char> = segment_abcdefg.clone();

        for c in segment_abcdefg.iter() {
            if six_on.iter().fold(true, |acc, six_char| acc && six_char.contains(&c)) {
                segment_cde.remove(&c);
            }
        }

        let mut segment_bcef: HashSet<char> = segment_abcdefg.clone();
        for c in segment_abcdefg.iter() {
            if five_on.iter().fold(true, |acc, six_char| acc && six_char.contains(&c)) {
                segment_bcef.remove(&c);
            } 
        }


        let segment_b = segment_bd.difference(&segment_cde).next().unwrap().clone();
        let segment_d = segment_bd.iter().filter(|c| **c != segment_b).next().unwrap().clone();

        let segment_f = segment_cf.difference(&segment_cde).next().unwrap().clone();
        let segment_c = segment_cf.iter().filter(|c| **c != segment_f).next().unwrap().clone();

        let mut segment_eg = segment_abcdefg.clone();
        segment_eg.remove(&segment_a);
        segment_eg.remove(&segment_b);
        segment_eg.remove(&segment_c);
        segment_eg.remove(&segment_d);
        segment_eg.remove(&segment_f);

        let segment_g = segment_eg.difference(&segment_bcef).next().unwrap().clone();
        let segment_e = segment_eg.iter().filter(|c| **c != segment_g).next().unwrap().clone();

        solver.mapping.insert(vec![segment_a, segment_b, segment_c, segment_e, segment_f, segment_g].iter().sorted().collect(), 0);
        solver.mapping.insert(vec![segment_c, segment_f].iter().sorted().collect(), 1);
        solver.mapping.insert(vec![segment_a, segment_c, segment_d, segment_e, segment_g].iter().sorted().collect(), 2);
        solver.mapping.insert(vec![segment_a, segment_c, segment_d, segment_f, segment_g].iter().sorted().collect(), 3);
        solver.mapping.insert(vec![segment_b, segment_c, segment_d, segment_f].iter().sorted().collect(), 4);
        solver.mapping.insert(vec![segment_a, segment_b, segment_d, segment_f, segment_g].iter().sorted().collect(), 5);
        solver.mapping.insert(vec![segment_a, segment_b, segment_d, segment_e, segment_f, segment_g].iter().sorted().collect(), 6);
        solver.mapping.insert(vec![segment_a, segment_c, segment_f].iter().sorted().collect(), 7);
        solver.mapping.insert(vec![segment_a, segment_b, segment_c, segment_d, segment_e, segment_f, segment_g].iter().sorted().collect(), 8);
        solver.mapping.insert(vec![segment_a, segment_b, segment_c, segment_d, segment_f, segment_g].iter().sorted().collect(), 9);
        

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
    let mut decoded_values: Vec<i32> = vec![];

    for (observed_values, to_decode) in unpacked_input_lines {
        let solver = SevenSegmentSolver::new(observed_values);

        let mut value = 0;

        for (pos, digit) in to_decode.iter().rev().enumerate() {
            match solver.parse(digit) {
                Some(x) =>  {
                    digit_occurrances[x as usize] += 1;
                    value += 10i32.pow(pos.try_into().unwrap()) * x as i32;
                },
                None => continue,
            }
        }

        decoded_values.push(value);
    }

    let sum: i32 = decoded_values.iter().sum();

    println!("Amount of digits with unique output values: {}", digit_occurrances[1] + digit_occurrances[4] + digit_occurrances[7] + digit_occurrances[8]);
    println!("Sum of decoded values: {}", sum);
}

fn unpack_input_line(line: &str) -> (Vec<&str>, Vec<&str>){
    let (observed, digits) = line.split_once(" | ").unwrap();
    return (observed.split(" ").collect(), digits.split(" ").collect());
}


#[cfg(test)]
const TEST_INPUT: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

#[cfg(test)]
const TRIVIAL_INPUT: &str = "abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg | cf cf cf cf";

#[test]
fn test_cheap_digit_identification() {
    let (observed, to_parse) = unpack_input_line(TEST_INPUT);

    let solver = SevenSegmentSolver::new(observed);

    assert_eq!(solver.parse("ab").unwrap(), 1);
    assert_eq!(solver.parse("ba").unwrap(), 1);
    
    assert_eq!(solver.parse("gcdfa").unwrap(), 2);
    assert_eq!(solver.parse("fbcad").unwrap(), 3);
    assert_eq!(solver.parse("eafb").unwrap(), 4);
    assert_eq!(solver.parse("cdfbe").unwrap(), 5);
    assert_eq!(solver.parse("cdfgeb").unwrap(), 6);
    assert_eq!(solver.parse("dab").unwrap(), 7);
    assert_eq!(solver.parse("acedgfb").unwrap(), 8);
    assert_eq!(solver.parse("cefabd").unwrap(), 9);
    assert_eq!(solver.parse("cagedb").unwrap(), 0);
}

#[test]
fn test_on_identity() {
    let (observed, to_parse) = unpack_input_line(TRIVIAL_INPUT);

    let solver = SevenSegmentSolver::new(observed);
}