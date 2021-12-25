use std::{error::Error};

use clap::{App, Arg};

struct Heightmap {
    // indexed y, x
    data: Vec<Vec<u32>>,
    width: usize,
    height: usize
}

struct LowPoint {
    position: (usize, usize),
    height: u32,
}

impl LowPoint {
    fn new(x: usize, y: usize, height: u32) -> Self {
        return Self {
            position: (x, y),
            height
        };
    }
}

impl Heightmap {
    fn get_low_points(&self) -> Vec<LowPoint> {
        let mut low_points: Vec<LowPoint> = vec![];

        for x in 0..self.width {
            for y in 0..self.height {
                let mut is_minimum = true;

                if x != 0 {
                    is_minimum &= self.data[y][x - 1] > self.data[y][x];
                }

                if x != self.width - 1 {
                    is_minimum &= self.data[y][x + 1] > self.data[y][x];
                }

                if y != 0 {
                    is_minimum &= self.data[y - 1][x] > self.data[y][x];
                }

                if y != self.height - 1 {
                    is_minimum &= self.data[y + 1][x] > self.data[y][x];
                }

                
                if is_minimum {
                    low_points.push(LowPoint::new(x, y, self.data[y][x]));
                }
            }
        }

        low_points
    }
}

impl TryFrom<&str> for Heightmap {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let discovered_width: usize = match value.lines().next() {
            Some(first_line) => first_line.len(),
            None => return Err("Can't work on an empty string!".into()),
        };

        for line in value.lines() {
            let mut line_heights = vec![];

            for height_value in line.chars() {
                 if height_value.is_numeric() {
                    line_heights.push(height_value.to_digit(10).unwrap());
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
            Heightmap {
                height: data.len(),
                width: discovered_width,
                data,
            }
        );
    }
}


fn main() {
    let matches = App::new("Advent of Code Day 9")
                    .arg(Arg::with_name("INPUT")
                        .help("Input file to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input = std::fs::read_to_string(&matches.value_of("INPUT").unwrap())
        .expect("Failed to open the input file");

    let map: Heightmap = input
        .trim_end()
        .try_into()
        .unwrap();

    let low_points = map.get_low_points();

    let low_point_risk_sum = low_points.iter().fold(0, |acc, p| acc + p.height + 1);

    println!("Sum of risk levels of all lowpoints: {}", low_point_risk_sum);
}

#[cfg(test)]
const EXAMPLE_STRING: &str =
r"2199943210
3987894921
9856789892
8767896789
9899965678";

#[test]
fn test_map_parsing() {
    let map: Heightmap = EXAMPLE_STRING.try_into().unwrap();

    assert_eq!(map.width, 10);
    assert_eq!(map.height, 5);
    assert_eq!(map.data[0][0], 2);
    assert_eq!(map.data[4][9], 8);
}

#[test]
fn test_example() {
    let map: Heightmap = EXAMPLE_STRING.try_into().unwrap();

    let low_points = map.get_low_points();
    let sum = low_points.iter().fold(0, |acc, p| acc + p.height + 1);

    assert_eq!(sum, 15);
}