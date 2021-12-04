extern crate clap;

use std::{error::Error, cmp::min, vec};

use clap::{App, Arg};

struct BingoCell {
    number: u8,
    crossed: bool,
}

impl BingoCell {
    fn new(number: u8) -> Self {
        return BingoCell {
            number,
            crossed: false
        }
    }
}

struct BingoBoard {
    data: [[BingoCell;5];5]
}

impl BingoBoard {
    fn new(board_data: [[u8;5];5]) -> BingoBoard {
        return BingoBoard { 
            data: board_data.map(|row| row.map(|n| BingoCell::new(n))),
        }
    }

    fn cross_out_number(&mut self, drawn: u8) {
        for column in self.data.iter_mut() {
            for mut cell in column {
                if cell.number == drawn {
                    cell.crossed = true;
                }
            }
        }
    }

    fn is_won(&self) -> bool {
        for idx in 0..5 {
            if self.is_column_won(idx) || self.is_row_won(idx) {
                return true;
            }
        }

        return false;
    }

    fn is_column_won(&self, column: usize) -> bool {
        return self.data.iter().map(|row| &row[column]).fold(true, |acc, v| acc && v.crossed);
    }

    fn is_row_won(&self, row: usize) -> bool {
        return self.data[row].iter().fold(true, |acc, v| acc && (*v).crossed);
    }

    fn calculate_score(&self) -> u64 {
        return self.data.iter().flatten().fold(0, |acc, x| if x.crossed {acc} else {acc + x.number as u64});
    }
}

impl TryFrom<&str> for BingoBoard {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Box<dyn Error>> {

        let lines: Vec<&str> = value.split('\n').collect();
        
        if lines.len() != 5 {
            return Err("Only 5x5 bingo boards are supported.".into());
        }

        let mut result = [[0u8;5];5];

        for i in 0..lines.len() {
            result[i] = parse_board_line(lines[i])?;
        }

        return Ok(BingoBoard::new(result));
    }
}

fn parse_board_line(line: &str) -> Result<[u8;5], Box<dyn Error>> {
    let mut result = [0u8;5];

    for i in 0..5 {
        let string_offset = i * 3;
        let number = &line[string_offset..min(string_offset + 3, line.len())].trim();
        result[i] = number.parse::<u8>()?;
    }
    return Ok(result);
}

fn main() {
    let commandline_matches = App::new("Advent of Code Day 4")
                    .arg(Arg::with_name("INPUT")
                        .help("Bingo file to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input_file_path = commandline_matches.value_of("INPUT").unwrap();

    let file_contents = std::fs::read_to_string(input_file_path)
        .expect("Failed to open the readings file");

    let mut splits = file_contents.split("\n\n"); 

    let draw_sequence: Vec<u8> = splits.nth(0).unwrap().split(',').map(|x| x.parse::<u8>().unwrap()).collect();

    let mut unwon_boards: Vec<BingoBoard> = splits.map(|x | x.trim_end().try_into().unwrap()).collect();

    let mut winners: Vec<(u8, BingoBoard)> = vec![];

    for drawn in draw_sequence {

        let mut i = 0;
        while i < unwon_boards.len() {
            unwon_boards[i].cross_out_number(drawn);

            if unwon_boards[i].is_won() {
                let winner = unwon_boards.remove(i);
                
                winners.push((drawn, winner));
            } else {
                i += 1;
            }
        }
    }

    let (first_winner_number, first_winner) = winners.first().unwrap();
    let (last_winner_number, last_winner) = winners.last().unwrap();
    println!("First winning board with score {} at number {}. Multiplied: {}", 
        first_winner.calculate_score(), 
        first_winner_number, 
        first_winner.calculate_score() * (*first_winner_number) as u64);

    println!("Last winning board with score {} at number {}. Multiplied: {}", 
        last_winner.calculate_score(), 
        last_winner_number, 
        last_winner.calculate_score() * (*last_winner_number) as u64);
}

#[cfg(test)]
const TEST_BOARD: &str = 
r"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";

 #[test]
 fn test_column_win() {
     let mut board: BingoBoard = TEST_BOARD.try_into().unwrap();
 
     for n in [0u8, 24, 7, 5, 19] {
         board.cross_out_number(n);
     }
 
     assert_eq!(board.is_won(), true);
 }

#[test]
fn test_row_win() {
    let mut board: BingoBoard = TEST_BOARD.try_into().unwrap();

    for n in [22u8, 13, 17, 11, 0] {
        board.cross_out_number(n);
    }

    assert_eq!(board.is_won(), true);
}

#[test]
fn test_empty_board_is_not_won() {
    let board: BingoBoard = TEST_BOARD.try_into().unwrap();
    assert_eq!(board.is_won(), false);
}

#[test]
fn test_board_parsing() {
    let parsed: BingoBoard = TEST_BOARD.try_into().unwrap();

    assert_eq!(parsed.data[0].iter().map(|x| x.number).collect::<Vec<u8>>(), [22, 13, 17, 11,  0]);
    assert_eq!(parsed.data[1].iter().map(|x| x.number).collect::<Vec<u8>>(), [ 8,  2, 23,  4, 24]);
    assert_eq!(parsed.data[2].iter().map(|x| x.number).collect::<Vec<u8>>(), [21,  9, 14, 16,  7]);
    assert_eq!(parsed.data[3].iter().map(|x| x.number).collect::<Vec<u8>>(), [ 6, 10,  3, 18,  5]);
    assert_eq!(parsed.data[4].iter().map(|x| x.number).collect::<Vec<u8>>(), [ 1, 12, 20, 15, 19]);
}

#[test]
fn test_line_parsing() {
    let test_line = " 8  2 23  4 24";

    let parsed = parse_board_line(test_line).unwrap();
    assert_eq!(parsed, [8, 2, 23, 4, 24]);
}