extern crate clap;

use std::error::Error;

use clap::{App, Arg};

struct FishDay {
    fishes_in_day: [u64; 9],
}

impl FishDay {
    fn get_next_day(&self) -> Self {
        let reproducing_fish = self.fishes_in_day[0];

        let mut next_day = [0u64; 9];
        next_day[..8].clone_from_slice(&self.fishes_in_day[1..]);
        next_day[8] = reproducing_fish;
        next_day[6] += reproducing_fish;

        return FishDay {
            fishes_in_day: next_day
        };
    }

    fn get_living_fish_count(&self) -> u64 {
        return self.fishes_in_day.iter().sum();
    }
}

impl TryFrom<&str> for FishDay {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut day = FishDay {
            fishes_in_day: [0u64; 9]
        };

        for parsed_day in value.split(',').map(|x| x.parse::<usize>()) {
            match parsed_day {
                Ok(fish_in_day) => day.fishes_in_day[fish_in_day] += 1,
                Err(e) => return Err(e.into()),
            }
        }

        Ok(day)
    }
}

fn main() {
    let commandline_matches = App::new("Advent of Code Day 6")
                    .arg(Arg::with_name("INPUT")
                        .help("Lanternfish file to parse.")
                        .required(true)
                        .index(1))
                    .arg(Arg::with_name("DAYS")
                        .help("Days to simulate.")
                        .required(true)
                        .index(2))
                    .get_matches();

    let input_file_path = commandline_matches.value_of("INPUT").unwrap();
    let days_to_simulate = commandline_matches.value_of("DAYS").unwrap().parse::<usize>().unwrap();

    let file_contents = std::fs::read_to_string(input_file_path)
        .expect("Failed to open the readings file");

    let mut fish_state: FishDay = file_contents
        .split('\n')
        .into_iter()
        .next()
        .unwrap()
        .try_into()
        .unwrap();
    
    
    for _ in 0..days_to_simulate {
        fish_state = fish_state.get_next_day();
    }

    println!("Living fish after {} days: {}", days_to_simulate, fish_state.get_living_fish_count());
}

#[cfg(test)]
const TEST_INPUT: &str = "3,4,3,1,2";

#[cfg(test)]
const AFTER_18_DAYS: &str = "6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8";

#[test]
fn test_parsing() {
    let day: FishDay = TEST_INPUT.try_into().unwrap();
    assert_eq!(day.fishes_in_day, [0u64, 1, 1, 2, 1, 0, 0, 0, 0]);
}

#[test]
fn test_sample_input() {
    let mut days: Vec<FishDay> = vec![TEST_INPUT.try_into().unwrap()];

    let expected: FishDay = AFTER_18_DAYS.try_into().unwrap();

    for _ in 0..18 {
        days.push(days[days.len() - 1].get_next_day())
    }

    assert_eq!(days[18].fishes_in_day, expected.fishes_in_day);
}