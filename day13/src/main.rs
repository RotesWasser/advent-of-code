extern crate clap;

use std::{error::Error, cmp::max, fmt::Display};

use clap::{App, Arg};

struct TransparentPaper {
    dots: Vec<Vec<bool>>,
}

impl TransparentPaper {
    /// Fold left
    fn fold_x(&self, fold_at: usize) -> Self {
        if fold_at < self.get_width() / 2 {
            todo!("Implement over-folding, ommitted currently");
        }

        let mut folded = TransparentPaper {
            dots: vec![vec![false; self.get_height()]; fold_at]
        };

        for y in 0..self.get_height() {
            // copy over non-folded dots
            for x in 0..fold_at {
                folded.dots[x][y] |= self.dots[x][y];
            }
        
            // copy over folded_dots
            for x in 0..fold_at {
                folded.dots[x][y] |= self.dots[self.get_width() - 1 - x][y];
            }
        }
        
        folded
    }

    /// Fold right
    fn fold_y(&self, y_coord: usize) -> Self {
        return self.transpose().fold_x(y_coord).transpose();
    }

    fn transpose(&self) -> Self {
        let mut res = Self {
            dots: vec![vec![false; self.get_width()]; self.get_height()],
        };

        for x in 0..self.get_width() {
            for y in 0..self.get_height() {
                res.dots[y][x] = self.dots[x][y];
            }
        }

        res
    }

    fn get_width(&self) -> usize {
        return self.dots.len();
    }

    fn get_height(&self) -> usize {
        return self.dots[0].len()
    }

    fn get_dots(&self) -> usize {
        return self.dots.iter().flatten().fold(0, |acc, dot| if *dot {acc + 1} else {acc});
    }
}

impl Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.get_height() {
            for x in 0..self.get_width() {
                write!(f,"{}", if self.dots[x][y] {"#"} else {"."})?;
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}

impl TryFrom<&str> for TransparentPaper {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let coordinates = value.split("\n").map(
            |line|
            match line.split_once(",") {
                Some((x,y)) => Ok((x.parse().unwrap(), y.parse().unwrap())),
                None => Err(format!("Failed to parse line: \"{}\"", line).into()),
            }
        ).collect::<Result<Vec<(usize, usize)>, Self::Error>>()?;

        let (max_x, max_y) = coordinates.iter().fold((0,0), |(acc_x,acc_y), (p_x, p_y)| (max(acc_x, *p_x), max(acc_y, *p_y)));

        let mut paper = TransparentPaper {
            dots: vec![vec![false; max_y + 1]; max_x + 1]
        };

        for (x,y) in coordinates {
            paper.dots[x][y] = true;
        }

        return Ok(paper);
    }
}



fn main() {
    let commandline_matches = App::new("Advent of Code Day 13")
                    .arg(Arg::with_name("INPUT")
                        .help("Inputfile to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input_file_path = commandline_matches.value_of("INPUT").unwrap();

    let file_contents = std::fs::read_to_string(input_file_path)
        .expect("Failed to open the readings file");

    let (dot_string, command_string) = file_contents.split_once("\n\n").unwrap();

    let command_sequence = command_string
        .split("\n")
        .filter(|reading| !reading.is_empty())
        .map(|x| x.split_once("=").unwrap())
        .map(|(cmd, at)| (cmd, at.parse::<usize>().unwrap()));

    let paper: TransparentPaper = dot_string.try_into().unwrap();
    let mut folds: Vec<TransparentPaper> = vec![paper];
    
    for (cmd, at) in command_sequence {

        match cmd {
            "fold along x" => {
                folds.push(folds[folds.len() - 1].fold_x(at));
            }
            "fold along y" => {
                folds.push(folds[folds.len() - 1].fold_y(at));

            }
            _ => panic!("Unsupported command in file: {}", cmd)
        }
    }

    println!("Dots after first fold: {}", folds[1].get_dots());
    println!("Final dot pattern\n{}", folds[folds.len() - 1]);
}

#[cfg(test)]
const TEST_INPUT: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

#[cfg(test)]
const SIMPLE_TEST_BOARD: &str = r"1,1
0,1
0,0
2,0";

#[test]
fn test_parsing_simple() {
    let paper: TransparentPaper = SIMPLE_TEST_BOARD.try_into().unwrap();

    assert_eq!(paper.dots[0][0], true);
    assert_eq!(paper.dots[0][1], true);
    assert_eq!(paper.dots[1][0], false);
    assert_eq!(paper.dots[1][1], true);
    assert_eq!(paper.dots[2][0], true);
    assert_eq!(paper.dots[2][1], false);
}

#[test]
fn test_fold_simple() {
    let paper: TransparentPaper = SIMPLE_TEST_BOARD.try_into().unwrap();
    let folded = paper.fold_x(1);

    assert_eq!(folded.get_width(), 1);
    assert_eq!(folded.get_height(), 2);

    assert_eq!(folded.dots[0][0], true);
    assert_eq!(folded.dots[0][1], true);
}

#[test]
fn test_transpose() {
    let paper: TransparentPaper = SIMPLE_TEST_BOARD.try_into().unwrap();
    let transposed = paper.transpose();

    assert_eq!(transposed.dots[0][0], true);
    assert_eq!(transposed.dots[1][0], true);
    assert_eq!(transposed.dots[0][1], false);
    assert_eq!(transposed.dots[1][1], true);
    assert_eq!(transposed.dots[0][2], true);
    assert_eq!(transposed.dots[1][2], false);
}

#[test]
fn test_example() {
    let paper: TransparentPaper = TEST_INPUT.split_once("\n\n").unwrap().0.try_into().unwrap();
    println!("Original Paper\n{}", paper);
    
    let folded_y_7 = paper.fold_y(7);
    println!("Fold y=7 \n{}", folded_y_7);
    assert_eq!(folded_y_7.get_dots(), 17);

    let folded_x_5 = folded_y_7.fold_x(5);
    println!("Fold x=5 \n{}", folded_x_5);
    assert_eq!(folded_x_5.get_dots(), 16);
}