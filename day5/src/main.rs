use std::{error::Error, collections::{BTreeMap, HashMap, hash_map::Entry}, cmp::{min, max}};

use clap::{App, Arg};
use regex::{self, Regex};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point2D {
    pub x: i32,
    pub y: i32
}

#[derive(PartialEq, Eq)]
struct Line2D {
    pub from: Point2D,
    pub to: Point2D
}

impl Line2D {
    fn get_multiplicity_at(&self, position: &Point2D) -> u32 {
        return 1;
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Intersection {
    pub position: Point2D,
    pub count: u32
}

#[derive(PartialEq, Eq)]
struct LineEvent<'a> {
    pub event_type: LineEventType,
    pub x: i32,
    pub line: &'a Line2D
}

#[derive(PartialEq, Eq)]
enum LineEventType {
    HorizontalStart,
    HorizontalEnd,
    VerticalLine
}

impl<'a> LineEvent<'a> {
    fn new(event_type: LineEventType, x: i32, line: &'a Line2D) -> LineEvent {
        LineEvent {
            event_type,
            x,
            line,
        }
    }
}

impl<'a> PartialOrd for LineEvent<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.x.partial_cmp(&other.x);
    }
}

impl<'a> Ord for LineEvent<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.x.cmp(&other.x);
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
                Line2D {
                    from: Point2D{
                        x: caps["x1"].parse::<i32>().unwrap(),
                        y: caps["y1"].parse::<i32>().unwrap(),
                    },
                    to: Point2D {
                        x: caps["x2"].parse::<i32>().unwrap(),
                        y: caps["y2"].parse::<i32>().unwrap(),
                    }
                }
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
        .filter(|line: &Line2D| line.from.x == line.to.x || line.from.y == line.to.y)
        .collect();

    let intersections = get_intersections_of_vertical_and_horizontal_lines(lines);

    let more_than_one_isect_count = intersections.iter().filter(|i| i.count > 1).count();

    println!("Amount of intersections with more than one crossing line: {}", more_than_one_isect_count);
}


fn get_intersections_of_vertical_and_horizontal_lines(input_lines: Vec<Line2D>) -> Vec<Intersection> {
    let mut living_lines: BTreeMap<i32, &Line2D> = BTreeMap::new();
    let mut line_events: Vec<LineEvent> = vec![];
    let mut intersections: HashMap<Point2D, Intersection> = HashMap::new();

    // TODO: Preprocess Lines to identify horizontal and vertical ones that overlap.
    // TODO: Extend the Line2D implementation to allow for segments with overlap-counts.

    for line in input_lines.iter() {
        if line.from.x == line.to.x {
            line_events.push(LineEvent::new(LineEventType::VerticalLine, line.from.x, line))
        } else if line.from.y == line.to.y {
            line_events.push(LineEvent::new(LineEventType::HorizontalStart, min(line.from.x, line.to.x), line));
            line_events.push(LineEvent::new(LineEventType::HorizontalEnd, max(line.from.x, line.to.x), line));
        } else {
            panic!("Cannot deal with non-axis-aligned lines.")
        }
    }

    line_events.sort();

    for event in line_events {
        match event.event_type {
            LineEventType::HorizontalStart => {
                if living_lines.insert(event.line.from.y, event.line) != None {
                    panic!("Tried to have two overlapping living lines.")
                }
            },
            LineEventType::HorizontalEnd => {
                living_lines.remove(&event.line.from.y);
            },
            LineEventType::VerticalLine => {
                let vertical_line = event.line;
                
                let line_start = min(vertical_line.from.y, vertical_line.to.y);
                let line_end = max(vertical_line.from.y, vertical_line.to.y);
                

                for (y, horizontal_line) in living_lines.range(line_start..line_end) {
                    let x = event.line.from.x;
                    let intersection = Point2D {x, y: *y};

                    let amount_of_lines = horizontal_line.get_multiplicity_at(&intersection) + vertical_line.get_multiplicity_at(&intersection);

                    // Gaussian sum formula up to n-1
                    let crossing_multiplicity = ((amount_of_lines - 1).pow(2) + (amount_of_lines - 1)) / 2;

                    intersections.insert(intersection.clone(), Intersection {
                        position: intersection,
                        count: crossing_multiplicity
                    });
                }
                
                
            },
        }
    }

    // We now know the vertical x horizontal intersections
    // Iterate the lines to insert possible intersections 

    return intersections.into_values().collect();
}

#[test]
fn test_parsing() {
    let res: Line2D = "-1,-2 -> -3,-4".try_into().unwrap();

    assert_eq!(res.from.x, -1);
    assert_eq!(res.from.y, -2);
    assert_eq!(res.to.x, -3);
    assert_eq!(res.to.y, -4);
}

#[test]
fn intersection_simple() {
    let l1: Line2D = "-1,0 -> 1,0".try_into().unwrap();
    let l2: Line2D = "0,-1 -> 0,1".try_into().unwrap();

    let intersections = get_intersections_of_vertical_and_horizontal_lines(vec![l1, l2]);

    assert_eq!(intersections.len(), 1);
    assert_eq!(intersections.contains(&Intersection { position: Point2D {x:  0, y: 0}, count: 1 }), true);
}

#[test]
fn intersection_overlapping() {
    let l1: Line2D = "-1,0 -> 1,0".try_into().unwrap();
    let l2: Line2D = "-1,0 -> 1,0".try_into().unwrap();

    let intersections = get_intersections_of_vertical_and_horizontal_lines(vec![l1, l2]);

    assert_eq!(intersections.len(), 3);
    assert_eq!(intersections.contains(&Intersection { position: Point2D {x: -1, y: 0}, count: 1 }), true);
    assert_eq!(intersections.contains(&Intersection { position: Point2D {x:  0, y: 0}, count: 1 }), true);
    assert_eq!(intersections.contains(&Intersection { position: Point2D {x:  1, y: 0}, count: 1 }), true);
}