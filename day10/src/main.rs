use core::panic;
use std::{collections::HashMap, fs};

enum ParseResult {
    Illegal(char),
    Incomplete(Vec<char>)
}

trait Parser {
    fn parse_braces(&self) -> ParseResult;
}

impl Parser for str {
    fn parse_braces(&self) -> ParseResult {
        let mut stack: Vec<char> = Vec::new();
        println!("parsing: {:?}", self);
        for c in self.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                }
                _ => {
                    if let Some(opened) = stack.pop() {
                        let expected = match opened {
                            '(' => ')',
                            '[' => ']',
                            '{' => '}',
                            '<' => '>',
                            _ => panic!("unexpected stack content {}", opened),
                        };
                        if c != expected {
                            println!("Expected {}, found {}", expected, c);
                            return ParseResult::Illegal(c);
                        }
                    }
                }
            }
        }
        println!("remaining stack {:?}", stack);
        ParseResult::Incomplete(stack)
    }
}

fn main() {
    let contents = fs::read_to_string("day10/input").expect("could not read input");
    let input = contents.lines();
    let mut completion_scores = Vec::new();
    let mut scores: HashMap<char, i32> = HashMap::new();
    for line in input {
        match line.parse_braces() {
            ParseResult::Illegal(ch) => {
                let e = scores.entry(ch).or_default();
                *e += 1;
            }
            ParseResult::Incomplete(stack) => {
                let completion_score = stack.iter().rev().fold(0u64, |a, c| {
                    a * 5
                        + match c {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => panic!("unexpected stack value {}", c),
                        }
                });
                println!("completion score {}", completion_score);
                completion_scores.push(completion_score);
            }
        }
    }
    let score = scores.iter().fold(0, |a, e| {
        a + e.1
            * match e.0 {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            }
    });

    completion_scores.sort();
    println!("scores: {:?} {} completion score {:?}", scores, score, completion_scores[completion_scores.len()/2]);
}
