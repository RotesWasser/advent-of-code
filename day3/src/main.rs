use std::error::Error;

use clap::{App, Arg};


struct GammaEpsilonAccumulator {
    surplus_ones_at_position: Vec<i32>
}

impl GammaEpsilonAccumulator {
    fn new(word_size: usize) -> Self {

        return GammaEpsilonAccumulator {
            surplus_ones_at_position: vec![0; word_size],
        };
    }

    fn add_line(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if line.len() != self.surplus_ones_at_position.len() {
            return Err(
                format!("Can't process a line of mismatching size. Accumulator: {}, Line: {}", 
                    self.surplus_ones_at_position.len(), 
                    line.len())
                .into()
            );
        }

        for (pos, c) in line.char_indices() {
            match c {
                '0' => {
                    self.surplus_ones_at_position[pos] -= 1;
                }
                '1' => {
                    self.surplus_ones_at_position[pos] += 1;
                },
                _ => {
                    return Err(format!("Can't process a line containing invalid characters. Supported are '0' and '1', got: {}", c).into());
                }
            }
        }

        return Ok(());
    }

    fn get_gamma(&self) -> u64 {
        let res = self.surplus_ones_at_position.iter().fold(0, |acc, ones_surplus| (acc << 1) | (*ones_surplus > 0) as u64);

        return res;
    }

    fn get_epsilon(&self) -> u64 {
        return !self.get_gamma() & (u64::MAX >> (64 - self.surplus_ones_at_position.len()));
    }
}

fn main() {
    let commandline_matches = App::new("Advent of Code Day 2")
                    .arg(Arg::with_name("task-two")
                        .help("Use the aim calculation method required by the second challenge.")
                        .long("task-two"))
                    .arg(Arg::with_name("INPUT")
                        .help("Submarine command file to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input_file_path = commandline_matches.value_of("INPUT").unwrap();

    let file_contents = std::fs::read_to_string(input_file_path)
        .expect("Failed to open the readings file");

    let (first_line, _) = file_contents.split_once('\n').unwrap();

    let gamma_epsilon_word_size = first_line.len();

    let mut accumulator = GammaEpsilonAccumulator::new(gamma_epsilon_word_size);

    let lines: Vec<&str> = file_contents.split('\n').filter(|reading| !reading.is_empty()).collect();

    for line in lines.clone() {
        accumulator.add_line(line).unwrap()
    }

    let oxygen_rating = get_rating(lines.clone(), true);
    let co2_scrubber_rating = get_rating(lines, false);

    println!("Gamma: {}, epsilon: {}, multiplied: {}", 
        accumulator.get_gamma(), 
        accumulator.get_epsilon(), 
        accumulator.get_gamma() * accumulator.get_epsilon()
    );

    println!("Oxygen rating: {}, scrubber rating: {}, multiplied: {}",
        oxygen_rating, co2_scrubber_rating, oxygen_rating * co2_scrubber_rating);
}

// Terrible, but the first task set me up and I'm in a hurry.
fn filter_at_position(to_search: Vec<&str>, position: usize, use_geq: bool) -> Vec<&str> {
    let truncated: Vec<&str> = to_search.iter().map(|x| &(*x)[position..]).collect();

    let mut accumulator = GammaEpsilonAccumulator::new(truncated[0].len());
    for i in truncated {
        accumulator.add_line(i).unwrap();
    }

    let to_keep = if (use_geq && accumulator.surplus_ones_at_position[0] >= 0) || (!use_geq && accumulator.surplus_ones_at_position[0] < 0) {
        '1'
    } else { 
        '0'
    };

    return to_search.iter().filter(|x| x.chars().nth(position).unwrap() == to_keep).map(|x| x.clone()).collect();
}

fn get_rating(to_search: Vec<&str>, use_geq: bool) -> u64 {
    let mut remaining = to_search;
    let mut position = 0;

    while remaining.len() > 1 {
        remaining = filter_at_position(remaining, position, use_geq);
        position += 1;
    }

    return u64::from_str_radix(remaining[0], 2).unwrap();
}

const TEST_INPUTS: [&'static str; 12] = [
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];

#[test]
fn test_filter_at_position() {
    let mut results: Vec<Vec<&str>> = vec![TEST_INPUTS.to_vec()];

    for i in 0..5 {
        results.push(filter_at_position(results[i].clone(), i, true))
    }
    assert_eq!(results[1], vec!["11110", "10110", "10111", "10101", "11100", "10000", "11001"]);
    assert_eq!(results[2], vec!["10110", "10111", "10101", "10000"]);
    assert_eq!(results[3], vec!["10110", "10111", "10101"]);
    assert_eq!(results[4], vec!["10110", "10111"]);
    assert_eq!(results[5], vec!["10111"]);   
}

#[test]
fn test_readings() {
    assert_eq!(get_rating(TEST_INPUTS.to_vec(), true), 0b10111);
    assert_eq!(get_rating(TEST_INPUTS.to_vec(), false), 0b01010);
}


#[test]
fn test_mismatching_line_length() {
    let mut acc = GammaEpsilonAccumulator::new(2);

    let result = acc.add_line("0110").unwrap_err();
    let expected: Box<dyn Error> = "Can't process a line of mismatching size. Accumulator: 2, Line: 4".into();

    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn test_unsupported_character() {
    let mut acc = GammaEpsilonAccumulator::new(1);

    let result = acc.add_line("3").unwrap_err();
    let expected: Box<dyn Error> = "Can't process a line containing invalid characters. Supported are '0' and '1', got: 3".into();

    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn test_accumulation() {
    let mut acc = GammaEpsilonAccumulator::new(4);

    acc.add_line("1001").unwrap();
    acc.add_line("1100").unwrap();

    assert_eq!(acc.surplus_ones_at_position, vec![2, 0, -2, 0]);
}

#[test]
fn test_gamma_and_epsilon_calculation() {
    let mut acc = GammaEpsilonAccumulator::new(5);

    for line in TEST_INPUTS {
        acc.add_line(line).unwrap();
    }

    assert_eq!(acc.get_gamma(), 0b10110);
    assert_eq!(acc.get_epsilon(), 0b01001)
}