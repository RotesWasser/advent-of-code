use std::{error::Error, collections::HashSet, hash::Hash};

use clap::{App, Arg};

#[derive(Debug, PartialEq, Eq, Clone)]
struct DumboCave {
    // indexed y, x
    data: Vec<Vec<DumboOctopus>>,
    width: usize,
    height: usize,
    flashes: usize
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point2D {
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DumboOctopus {
    position: Point2D,
    charge: u8,
}

impl Point2D {
    fn new(x: usize, y: usize) -> Self {
        return Self {x, y}
    }
}

impl DumboOctopus {
    fn new(x: usize, y: usize, charge: u8) -> Self {
        return Self {
            position: Point2D::new(x, y), charge
        }
    }
}

impl DumboCave {
    fn step(&self) -> Self {
        let mut working_copy = self.data.clone();
        let mut might_flash: HashSet<Point2D> = HashSet::new();
        let mut flashed: HashSet<Point2D> = HashSet::new();

        // increase charge level and find initial flash candidates
        for octopus in working_copy.iter_mut().flatten() {
            octopus.charge += 1;
            if octopus.charge > 9 {
                might_flash.insert(octopus.position);
            }
        }

        // flash all candidates
        while !might_flash.is_empty() {
            let candidate_pos = *might_flash.iter().next().unwrap();
            might_flash.remove(&candidate_pos);

            let candidate = &working_copy[candidate_pos.y][candidate_pos.x];
            
            if candidate.charge > 9 {
                flashed.insert(candidate_pos);
                let neighbours: HashSet<Point2D> = HashSet::from_iter(self.get_neighbour_positions(&candidate_pos).into_iter());
                let unflashed_neighbours = neighbours.difference(&flashed);

                for unflashed in unflashed_neighbours {
                    working_copy[unflashed.y][unflashed.x].charge += 1;
                    might_flash.insert(*unflashed);
                }
            }
        }

        // reset flashed octopi
        for flashed_pos in flashed.iter() {
            working_copy[flashed_pos.y][flashed_pos.x].charge = 0;
        }

        return Self {
            data: working_copy,
            width: self.width,
            height: self.height,
            flashes: self.flashes + flashed.len()
        }
    }

    fn get_first_synchronized_round(&self) -> usize {
        let mut step_count = 0;
        let mut stepped = (*self).clone();

        loop {
            let prev_flashes = stepped.flashes;
            stepped = stepped.step();
            step_count += 1;

            if stepped.flashes - prev_flashes == self.width * self.height {
                break;
            }
        }

        step_count
    }

    fn get_neighbour_positions(&self, position: &Point2D) -> Vec<Point2D>{
        let mut neighbours = vec![];

        if position.x != 0 {
            // left
            neighbours.push(Point2D::new(position.x-1, position.y));
        }

        if position.x != self.width - 1 {
            // right
            neighbours.push(Point2D::new(position.x + 1, position.y));
        }

        if position.y != 0 {
            // bottom
            neighbours.push(Point2D::new(position.x, position.y - 1));
        }

        if position.y != self.height - 1 {
            // top
            neighbours.push(Point2D::new(position.x, position.y + 1));
        }

        if position.x != 0 && position.y != 0 {
            // bottom-left
            neighbours.push(Point2D::new(position.x - 1, position.y - 1));
        }

        if position.x != 0 && position.y != self.height - 1 {
            // top-left
            neighbours.push(Point2D::new(position.x - 1, position.y + 1));
        }

        if position.x != self.width - 1 && position.y != 0 {
            // bottom-left
            neighbours.push(Point2D::new(position.x + 1, position.y - 1));
        }

        if position.x != self.width - 1 && position.y != self.height - 1 {
            // bottom-right
            neighbours.push(Point2D::new(position.x + 1, position.y + 1));
        }

        return neighbours;
    }
}

impl TryFrom<&str> for DumboCave {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let discovered_width: usize = match value.lines().next() {
            Some(first_line) => first_line.len(),
            None => return Err("Can't work on an empty string!".into()),
        };

        for (y, line) in value.lines().enumerate() {
            let mut line_heights = vec![];

            for (x, height_value) in line.chars().enumerate() {
                 if height_value.is_numeric() {
                    line_heights.push(
                        DumboOctopus::new(
                            x, 
                            y, 
                            height_value.to_digit(10).unwrap() as u8
                    ));
                 }
            }

            if line_heights.len() != discovered_width {
                return Err(
                    format!("Length of line {} doesn't match length {} of the first line!", line_heights.len(), discovered_width).into()
                );
            }

            data.push(line_heights);
        }

        return Ok(
            DumboCave {
                height: data.len(),
                width: discovered_width,
                data,
                flashes: 0
            }
        );
    }
}

fn main() {
    let matches = App::new("Advent of Code Day 11")
                    .arg(Arg::with_name("INPUT")
                        .help("Input file to parse.")
                        .required(true)
                        .index(1))
                    .arg(Arg::with_name("STEPS")
                        .help("Steps to simulate")
                        .required(true)
                        .index(2))
                    .get_matches();

    let input = std::fs::read_to_string(&matches.value_of("INPUT").unwrap())
        .expect("Failed to open the input file");

    let steps_to_simulate: usize = matches.value_of("STEPS").unwrap().parse().unwrap();

    let initial_cave: DumboCave = input.trim_end().try_into().unwrap();

    let mut stepped_cave = initial_cave.clone();
    for _ in 0..steps_to_simulate {
        stepped_cave = stepped_cave.step();
    }

    println!("Flashes after {} steps: {}", steps_to_simulate, stepped_cave.flashes);
    println!("First synchronized round: {}", initial_cave.get_first_synchronized_round());
}

#[cfg(test)]
const EXAMPLE_INPUT: &str =
r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[cfg(test)]
const EXAMPLE_AFTER_ONE_STEP: &str =
r"6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637";

#[cfg(test)]
const EXAMPLE_AFTER_TEN_STEPS: &str =
r"0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000";

#[test]
fn test_parsing() {
    let cave: DumboCave = EXAMPLE_INPUT.try_into().unwrap();

    assert_eq!(cave.data[0][0].charge, 5);
    assert_eq!(cave.data[9][9].charge, 6);
    assert_eq!(cave.width, 10);
    assert_eq!(cave.width, 10);
}

#[test]
fn test_single_step() {
    let cave: DumboCave = EXAMPLE_INPUT.try_into().unwrap();
    let expected: DumboCave = EXAMPLE_AFTER_ONE_STEP.try_into().unwrap();

    assert_eq!(cave.step(), expected);
}

#[test]
fn test_ten_steps() {
    let mut steps: Vec<DumboCave> = vec![EXAMPLE_INPUT.try_into().unwrap()];
    let mut expected: DumboCave = EXAMPLE_AFTER_TEN_STEPS.try_into().unwrap();
    expected.flashes = 204;

    for _ in 0..10 {
        steps.push(steps.last().unwrap().step());
    }

    assert_eq!(*steps.last().unwrap(), expected);
}

#[test]
fn test_finding_first_synchronized_round() {
    let cave: DumboCave = EXAMPLE_INPUT.try_into().unwrap();

    assert_eq!(cave.get_first_synchronized_round(), 195);
}