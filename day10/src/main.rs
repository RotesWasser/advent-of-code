use std::{error::Error, collections::{HashSet, HashMap}, hash::Hash};

use clap::{App, Arg};



struct SyntaxError {
    position: usize,
    expected: Option<char>,
    offending_character: char
}

impl SyntaxError {    
    fn new(position: usize, offending_character: char, expected: Option<char>) -> Self {
        Self {
            position,
            expected,
            offending_character
        }
    }

}

struct SyntaxChecker {
    map_open_to_closing: HashMap<char,char>
}

impl SyntaxChecker {
    fn new() -> Self {
        let mut map_open_to_closing: HashMap<char, char> = HashMap::new();
        map_open_to_closing.insert('(', ')');
        map_open_to_closing.insert('[', ']');
        map_open_to_closing.insert('{', '}');
        map_open_to_closing.insert('<', '>');

        return Self {
            map_open_to_closing
        }
    }

    fn parse_line(&self, line: &str) -> Result<Vec<char>,SyntaxError>  {
        let mut parser_stack: Vec<char> = vec![];
        
        for (pos, read) in line.chars().enumerate() {
            if self.map_open_to_closing.contains_key(&read) {
                parser_stack.push(read);
            } else {
                let stack_top = match parser_stack.pop() {
                    Some(x) => x,
                    None => return Err(SyntaxError::new(pos, read, None)),
                };

                let expected = match self.map_open_to_closing.get(&stack_top) {
                    Some(x) => x,
                    None => return Err(SyntaxError::new(pos, read, None)),
                };

                if read != *expected {
                    return Err(SyntaxError::new(pos, read, Some(*expected)));
                }
            }
        }

        let completion = parser_stack.into_iter().rev().map(|x| self.map_open_to_closing.get(&x).unwrap()).copied().collect();

        Ok(completion)
    }
}

fn score_syntax_errors(errors: Vec<SyntaxError>) -> usize {
    errors
    .iter()
    .map(|e| match e.offending_character {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    })
    .sum()
}

fn score_completion(completion: &Vec<char>) -> usize {
    let mut score = 0;
    for character in completion {
        score *= 5;
        score += match character {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0
        }
    }

    score
}

fn score_completions(completions: Vec<Vec<char>>) -> usize {
    let mut scores: Vec<usize> = completions.iter().map(score_completion).collect();
    scores.sort();
    return scores[scores.len() / 2];
}

fn main() {
    let matches = App::new("Advent of Code Day 10")
                    .arg(Arg::with_name("INPUT")
                        .help("Input file to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input = std::fs::read_to_string(&matches.value_of("INPUT").unwrap())
        .expect("Failed to open the input file");

    let checker = SyntaxChecker::new();
    let mut errors: Vec<SyntaxError> = vec![];
    let mut completions: Vec<Vec<char>> = vec![];
    
    for line in input.lines() {
        match checker.parse_line(line) {
            Ok(completion) => {
                if completion.len() > 0 {
                    completions.push(completion);
                }
            },
            Err(e) =>  errors.push(e),
        }
    }

    println!("Error score: {}", score_syntax_errors(errors));
    println!("Completions score: {}", score_completions(completions));
}

#[cfg(test)]
const EXAMPLE_INPUT: &str =
r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

#[test]
fn test_example_errors() {
    let checker = SyntaxChecker::new();

    let errors: Vec<SyntaxError> = EXAMPLE_INPUT.lines()
        .map(|x| checker.parse_line(x))
        .filter(|x| x.is_err())
        .map(|x| x.unwrap_err())
        .collect();
    
    assert_eq!(score_syntax_errors(errors), 26397);
}

#[test]
fn test_completion_scoring() {
    assert_eq!(score_completion(&"}}]])})]".chars().collect()), 288957);
    assert_eq!(score_completion(&")}>]})".chars().collect()), 5566);
    assert_eq!(score_completion(&"}}>}>))))".chars().collect()), 1480781);
    assert_eq!(score_completion(&"]]}}]}]}>".chars().collect()), 995444);
    assert_eq!(score_completion(&"])}>".chars().collect()), 294);
}

#[test]
fn test_completion_vector_scoring() {
    let completions: Vec<Vec<char>> = vec![
        "}}]])})]", 
        ")}>]})", 
        "}}>}>))))", 
        "]]}}]}]}>", 
        "])}>"
    ].iter().map(|x| x.chars().collect()).collect();

    assert_eq!(score_completions(completions), 288957);


}