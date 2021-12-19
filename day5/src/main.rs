use std::{error::Error, cmp::{min, max}, ops::{Sub, AddAssign}};

use clap::{App, Arg};
use regex::{self, Regex};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vector2D {
    pub x: i32,
    pub y: i32
}

impl Vector2D {
    fn magnitude(&self) -> f64 {
        let fx: f64 = self.x.pow(2).try_into().unwrap();
        let fy: f64 = self.y.pow(2).try_into().unwrap();
        return (fx + fy).sqrt();
    }

    fn new(x: i32, y: i32) -> Self {
        return Vector2D {
            x,y
        };
    }
}

impl AddAssign<Vector2D> for Vector2D {
    fn add_assign(&mut self, rhs: Vector2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        return Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        };
    }
}

#[derive(Clone)]
struct Line2D {
    pub from: Vector2D,
    pub to: Vector2D
}

impl Line2D {
    fn new(from: Vector2D, to: Vector2D) -> Self {
        return Line2D {
            from,
            to
        };
    }

    fn get_integer_step_vector(&self) -> Vector2D {
        let dir_vector = self.to - self.from;
        
        return Vector2D {
            x: Line2D::calc_step_component(dir_vector.x, dir_vector.magnitude()),
            y: Line2D::calc_step_component(dir_vector.y, dir_vector.magnitude())
        };
    }

    fn calc_step_component(component: i32, magnitude: f64) -> i32 {
        let normalized = (component as f64) / magnitude;
        if normalized < 0f64 {
            return normalized.floor() as i32;
        } else if normalized > 0f64 {
            return normalized.ceil() as i32;
        } else {
            return 0;
        }
    }
}

impl TryFrom<&str> for Line2D {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let line_regex = Regex::new(r"^(?P<x1>[-]{0,1}\d+),(?P<y1>[-]{0,1}\d+) -> (?P<x2>[-]{0,1}\d+),(?P<y2>[-]{0,1}\d+)$").unwrap();

        if !line_regex.is_match(&value) {
            return Err(format!("Input didn't conform to regex").into());
        } else {
            let caps = line_regex.captures_iter(&value).next().unwrap();

            Ok(
                Line2D::new(Vector2D{
                        x: caps["x1"].parse::<i32>().unwrap(),
                        y: caps["y1"].parse::<i32>().unwrap(),
                    },
                    Vector2D {
                        x: caps["x2"].parse::<i32>().unwrap(),
                        y: caps["y2"].parse::<i32>().unwrap(),
                    }
                )
            )
        }
    }
}

fn main() {
    let matches = App::new("Advent of Code Day 5")
                    .arg(Arg::with_name("INPUT")
                        .help("Sonar readings file to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input = std::fs::read_to_string(&matches.value_of("INPUT").unwrap())
        .expect("Failed to open the readings file");

    let lines: Vec<Line2D> = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.try_into().unwrap())
        .collect();

    let axis_aligned_lines: Vec<Line2D> = lines.clone().into_iter().filter(|line| line.from.x == line.to.x || line.from.y == line.to.y).collect();

    println!("Number of Points with overlaps, axis aligned only: {}", calculate_intersections(axis_aligned_lines));
    println!("Number of Points with overlaps, including diagonals: {}", calculate_intersections(lines));
}

fn calculate_intersections(axis_aligned_lines: Vec<Line2D>) -> usize {
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;

    for line in axis_aligned_lines.iter() {
        x_min = min(x_min, min(line.from.x, line.to.x));
        x_max = max(x_max, max(line.from.x, line.to.x));
        y_min = min(y_min, min(line.from.y, line.to.y));
        y_max = max(y_max, max(line.from.y, line.to.y));
    }

    let area_width: usize = (x_max - x_min + 1).try_into().unwrap();
    let area_height: usize = (y_max - y_min + 1).try_into().unwrap();

    let mut area = vec![vec![0;area_height]; area_width];

    for line in axis_aligned_lines.iter() {
        let mut current_pos = Vector2D::new(line.from.x - x_min, line.from.y - y_min);
        let target_pos = Vector2D::new(line.to.x - x_min, line.to.y - y_min);
        let direction = line.get_integer_step_vector();
        
        area[current_pos.x as usize][current_pos.y as usize] += 1;
        loop {
            current_pos += direction;
            area[current_pos.x as usize][current_pos.y as usize] += 1;

            if current_pos == target_pos {
                break;
            }
        }
    }

    return area.iter().flatten().filter(|x| (**x) > 1).count();
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = r"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[test]
fn test_example_input_aa() {
    let lines: Vec<Line2D> = EXAMPLE_INPUT
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.try_into().unwrap())
        .collect();

    let axis_aligned_lines: Vec<Line2D> = lines.into_iter().filter(|line| line.from.x == line.to.x || line.from.y == line.to.y).collect();

    let isects = calculate_intersections(axis_aligned_lines);

    assert_eq!(isects, 5);
}

#[test]
fn test_example_input_with_diagonals_aa() {
    let lines: Vec<Line2D> = EXAMPLE_INPUT
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.try_into().unwrap())
        .collect();

    let isects = calculate_intersections(lines);

    assert_eq!(isects, 12);
}

#[test]
fn test_simple_aa() {
    let l1: Line2D = "-1,0 -> 1,0".try_into().unwrap();
    let l2: Line2D = "0,-1 -> 0,1".try_into().unwrap();

    let isects = calculate_intersections(vec![l1, l2]);
    assert_eq!(isects, 1);
}

#[test]
fn test_overlapping_aa() {
    let l1: Line2D = "-1,0 -> 1,0".try_into().unwrap();
    let l2: Line2D = "-1,0 -> 1,0".try_into().unwrap();

    let isects = calculate_intersections(vec![l1, l2]);
    assert_eq!(isects, 3);
}

#[test]
fn test_step_vector_calculation() {
    let lines: Vec<Line2D> = vec![
        "-5,-5 -> 5,5".try_into().unwrap(),
        "100,0 -> 0,0".try_into().unwrap()
    ];

    let expected: Vec<Vector2D> = vec![
        Vector2D::new(1,1),
        Vector2D::new(-1,0)
    ];

    for (line, expected) in lines.into_iter().zip(expected) {
        assert_eq!(line.get_integer_step_vector(), expected);
    }
}

#[test]
fn test_parsing() {
    let res: Line2D = "-1,-2 -> -3,-4".try_into().unwrap();

    assert_eq!(res.from.x, -1);
    assert_eq!(res.from.y, -2);
    assert_eq!(res.to.x, -3);
    assert_eq!(res.to.y, -4);
}