use core::panic;
use std::{collections::HashMap, fs};

trait Parser {
    fn parse(&mut self, line: Vec<char>) -> Option<char>;
}

impl Parser for Vec<char> {
    fn parse(&mut self, line: Vec<char>) -> Option<char> {
//        println!("parsing: {:?}", line);
        for c in line {
            match c {
                '(' | '[' | '{' | '<' => {
                    self.push(c);
                }
                _ => {
                    if let Some(opened) = self.pop() {
                        let expected = match opened {
                            '(' => ')',
                            '[' => ']',
                            '{' => '}',
                            '<' => '>',
                            _ => panic!("unexpected stack content {}", opened),
                        };
                        if c != expected {
                            println!("Expected {}, found {}", expected, c);
                            return Some(c);
                        }
                    }
                }
            }
        }
        println!("remaining stack {:?}", self);
        None
    }
}

fn main() {
    let contents = fs::read_to_string("day10/input_simple").expect("could not read input");
    let input = contents.lines().map(|l| l.chars().collect());

    let mut scores: HashMap<char, i32> = HashMap::new();
    for line in input {
        let mut stack: Vec<char> = Vec::new();
        if let Some(illegal) = stack.parse(line) {
            let e = scores.entry(illegal).or_default();
            *e += 1;
        }
    }
    let score = scores.iter().fold(0, |a, e| a + e.1 * match e.0 {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    });
    println!("scores: {:?} {}", scores, score);
}
