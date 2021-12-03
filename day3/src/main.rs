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

    for line in file_contents.split('\n').filter(|reading| !reading.is_empty()) {
            accumulator.add_line(line).unwrap()
        }

    println!("Gamma: {}, epsilon: {}, multiplied: {}", 
        accumulator.get_gamma(), 
        accumulator.get_epsilon(), 
        accumulator.get_gamma() * accumulator.get_epsilon()
    );
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

    acc.add_line("00100").unwrap();
    acc.add_line("11110").unwrap();
    acc.add_line("10110").unwrap();
    acc.add_line("10111").unwrap();
    acc.add_line("10101").unwrap();
    acc.add_line("01111").unwrap();
    acc.add_line("00111").unwrap();
    acc.add_line("11100").unwrap();
    acc.add_line("10000").unwrap();
    acc.add_line("11001").unwrap();
    acc.add_line("00010").unwrap();
    acc.add_line("01010").unwrap();

    assert_eq!(acc.get_gamma(), 0b10110);
    assert_eq!(acc.get_epsilon(), 0b01001)
}