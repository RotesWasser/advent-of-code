use std::{error::Error, cmp::{min, max}};

use clap::{App, Arg};
use regex::{self, Regex};

struct Point2D {
    pub x: i32,
    pub y: i32
}

struct Line2D {
    pub from: Point2D,
    pub to: Point2D
}

impl Line2D {
    fn new(from: Point2D, to: Point2D) -> Self {
        return Line2D {
            from,
            to
        };
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
                Line2D::new(Point2D{
                        x: caps["x1"].parse::<i32>().unwrap(),
                        y: caps["y1"].parse::<i32>().unwrap(),
                    },
                    Point2D {
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

    let axis_aligned_lines: Vec<Line2D> = lines.into_iter().filter(|line| line.from.x == line.to.x || line.from.y == line.to.y).collect();

    println!("Number of Points with overlaps: {}", calculate_intersections(axis_aligned_lines));
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
        if line.from.x == line.to.x {
            // vertical
            let y_start = min(line.from.y, line.to.y) - y_min;
            let y_end = max(line.from.y, line.to.y) -y_min;

            for y in y_start..(y_end + 1) {
                let areax: usize = (line.from.x - x_min).try_into().unwrap();
                let areay: usize = y.try_into().unwrap();

                area[areax][areay] += 1;
            }
        } else if line.from.y == line.to.y {
            // horizontal
            let x_start = min(line.from.x, line.to.x) - x_min;
            let x_end = max(line.from.x, line.to.x) - x_min;

            for x in x_start..(x_end + 1) {
                let areax: usize = x.try_into().unwrap();
                let areay: usize = (line.from.y - y_min).try_into().unwrap();

                area[areax][areay] += 1;
            }
        } else {
            panic!("Can't deal with non-axis-aligned lines.");
        }
    }

    return area.iter().flatten().filter(|x| (**x) > 1).count();
}

#[test]
fn test_simple() {
    let l1: Line2D = "-1,0 -> 1,0".try_into().unwrap();
    let l2: Line2D = "0,-1 -> 0,1".try_into().unwrap();

    let isects = calculate_intersections(vec![l1, l2]);
    assert_eq!(isects, 1);
}

#[test]
fn test_overlapping() {
    let l1: Line2D = "-1,0 -> 1,0".try_into().unwrap();
    let l2: Line2D = "-1,0 -> 1,0".try_into().unwrap();

    let isects = calculate_intersections(vec![l1, l2]);
    assert_eq!(isects, 3);
}

#[test]
fn test_example_input() {
    let lines: Vec<Line2D> = r"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.try_into().unwrap())
        .collect();

    let axis_aligned_lines: Vec<Line2D> = lines.into_iter().filter(|line| line.from.x == line.to.x || line.from.y == line.to.y).collect();

    let isects = calculate_intersections(axis_aligned_lines);

    assert_eq!(isects, 5);
}

#[test]
fn test_parsing() {
    let res: Line2D = "-1,-2 -> -3,-4".try_into().unwrap();

    assert_eq!(res.from.x, -1);
    assert_eq!(res.from.y, -2);
    assert_eq!(res.to.x, -3);
    assert_eq!(res.to.y, -4);
}