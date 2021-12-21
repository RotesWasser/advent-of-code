use std::{error::Error};

use clap::{App, Arg};

struct CrabArea {
    starting_positions: Vec<i32>,
    min_pos: i32,
    max_pos: i32,
}

struct BlastLocation {
    position: i32,
    cost: i32,
}

impl BlastLocation {
    fn new(position: i32, cost: i32) -> Self {
        return BlastLocation { position, cost }
    }
}

impl TryFrom<&str> for CrabArea {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let starting_positions: Vec<i32> = 
        value
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

        let min_pos: i32 = starting_positions.iter().min().unwrap().clone();
        let max_pos: i32 = starting_positions.iter().max().unwrap().clone();

        Ok(
            CrabArea {
                starting_positions,
                min_pos,
                max_pos
            }
        )
    }
}

impl CrabArea {
    fn calculate_blast_position(&self,cost_function: impl Fn(i32) -> i32) -> BlastLocation {
        let mut best_blast_pos = self.min_pos;
        let mut best_cost = i32::MAX;

        for blast_pos in self.min_pos..self.max_pos {
            let fuel_cost: i32 = self.starting_positions.iter().map(|crab_pos| cost_function((blast_pos - crab_pos).abs())).sum();

            if fuel_cost < best_cost {
                best_cost = fuel_cost;
                best_blast_pos = blast_pos;
            } 
        }

        return BlastLocation::new(best_blast_pos, best_cost);
    }
}

fn constant_fuel_cost_function(distance: i32) -> i32 {
    return distance;
}

fn sum_fuel_cost_function(distance: i32) -> i32 {
    // Gaussian Sum formula
    return (distance.pow(2) + distance) / 2;
}

fn main() {
    let matches = App::new("Advent of Code Day 7")
                    .arg(Arg::with_name("INPUT")
                        .help("Input file to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input = std::fs::read_to_string(&matches.value_of("INPUT").unwrap())
        .expect("Failed to open the readings file");

    let crabs: CrabArea = input
        .split('\n')
        .next()
        .unwrap()
        .try_into()
        .unwrap();

    let constant_cost_blast = crabs.calculate_blast_position(constant_fuel_cost_function);
    let sum_cost_blast = crabs.calculate_blast_position(sum_fuel_cost_function);

    println!("Constant Fuel Burn: blast position {}, cost {}", constant_cost_blast.position, constant_cost_blast.cost);
    println!("Summed Fuel Burn: blast position {}, cost {}", sum_cost_blast.position, sum_cost_blast.cost);
}


#[cfg(test)]
const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

#[test]
fn test_example_constant_fuel_burn() {
    let crab_area: CrabArea = TEST_INPUT.try_into().unwrap();
    let blast = crab_area.calculate_blast_position(constant_fuel_cost_function);
    assert_eq!(blast.position, 2);
    assert_eq!(blast.cost, 37);
}

#[test]
fn test_example_sum_fuel_burn() {
    let crab_area: CrabArea = TEST_INPUT.try_into().unwrap();
    let blast = crab_area.calculate_blast_position(sum_fuel_cost_function);
    assert_eq!(blast.position, 5);
    assert_eq!(blast.cost, 168);
}
