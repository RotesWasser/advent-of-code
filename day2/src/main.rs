extern crate clap;

use std::{error::Error};

use clap::{App, Arg};

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Forward,
    Down,
    Up
}

#[derive(PartialEq, Eq, Debug)]
struct SubmarineCommand {
    pub direction: Direction,
    pub distance: i32
}

impl TryFrom<&str> for SubmarineCommand {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((direction, distance)) = value.split_once(' ') {
            let direction = match direction {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                unmatched => {
                    return Err(format!("Unknown submarine command: {}", unmatched).into());
                }
            };

            let distance = match distance.parse::<i32>() {
                Ok(x) => x,
                Err(_) => return Err("Couldn't parse distance".into())
            };

            return Ok(SubmarineCommand {
                direction,
                distance
            });

        } else {
            return Err("Couldn't split by space".into());
        }
    }
}

fn main() {
    let commandline_matches = App::new("Advent of Code Day 2")
                    // .arg(Arg::with_name("sliding-window")
                    //     .help("Use sliding window as required by the second challenge.")
                    //     .long("sliding-window"))
                    .arg(Arg::with_name("INPUT")
                        .help("Submarine command file to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input_file_path = commandline_matches.value_of("INPUT").unwrap();

    let file_contents = std::fs::read_to_string(input_file_path)
        .expect("Failed to open the readings file");

    let extracted_positions = file_contents
        .split('\n')
        .filter(|reading| !reading.is_empty())
        .map(|reading| {
            TryInto::<SubmarineCommand>::try_into(reading).unwrap()
        });
    
    let (horizontal, depth) = calculate_final_position_task_one(extracted_positions);

    println!("Final horizontal: {}, final depth: {}, multiplied: {}", horizontal, depth, horizontal * depth);
}

fn calculate_final_position_task_one<I>(commands: I) -> (i32, i32)
where
    I: IntoIterator<Item = SubmarineCommand>
{
    let final_position = commands.into_iter().fold((0,0), |(horizontal, depth), command| {
        match command.direction {
            Direction::Forward => return (horizontal + command.distance, depth),
            Direction::Down => return (horizontal, depth + command.distance),
            Direction::Up => return (horizontal, depth - command.distance),
        }
    });

    return final_position;
}

#[test]
fn test_submarine_command_parsing() {
    assert_eq!(TryInto::<SubmarineCommand>::try_into("forward 5").unwrap(), SubmarineCommand {
        direction: Direction::Forward,
        distance: 5
    });
}

#[test]
fn test_position_calculation_simple() {
    let commands: Vec<SubmarineCommand> = vec![
        "forward 5".try_into().unwrap(), 
        "down 4".try_into().unwrap(), 
        "up 1".try_into().unwrap()
    ];

    let (horizontal, depth) = calculate_final_position_task_one(commands);

    assert_eq!(horizontal, 5);
    assert_eq!(depth, 3);
}