use std::error::Error;

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
        .collect();

    

}

#[test]
fn test_parsing() {
    let res: Line2D = "-1,-2 -> -3,-4".try_into().unwrap();

    assert_eq!(res.from.x, -1);
    assert_eq!(res.from.y, -2);
    assert_eq!(res.to.x, -3);
    assert_eq!(res.to.y, -4);
}