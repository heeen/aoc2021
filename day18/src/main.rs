use std::{fmt, fs};

#[derive(Debug, Clone)]
enum SnailNum {
    Just(u32),
    Pair(Box<SnailPair>),
}

impl fmt::Display for SnailNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailNum::Just(v) => write!(f, "{}", v),
            SnailNum::Pair(p) => write!(f, "{}", p),
        }
    }
}

#[derive(Debug, Clone)]
struct SnailPair {
    left: SnailNum,
    right: SnailNum,
}

impl fmt::Display for SnailPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl SnailPair {
    fn explode(&mut self, level: u32) -> (Option<u32>, Option<u32>) {
        let (mut ret_l, mut ret_r) = (None, None);
        if level < 4 {
            if let SnailNum::Pair(pair) = &mut self.left {
                let (ll, lr) = pair.explode(level + 1);
                if let Some(v) = lr {
                    pair.add_left(v);
                }
                ret_l = ll;
            }
            if let SnailNum::Pair(pair) = &mut self.right {
                let (rl, rr) = pair.explode(level + 1);
                if let Some(v) = rl {
                    pair.add_right(v);
                }
                ret_r = rr;
            }
        } else {
            if let SnailNum::Just(v) = self.left {
                ret_l = Some(v)
            }
            if let SnailNum::Just(v) = self.right {
                ret_r = Some(v)
            }
        }
        println!(
            "level {} {} exploding into {:?}, {:?}",
            level, self, ret_l, ret_r
        );
        (ret_l, ret_r)
    }
    fn add_right(&mut self, v: u32) {
        match &mut self.right {
            SnailNum::Just(o) => self.left = SnailNum::Just(*o + v),
            SnailNum::Pair(pair) => pair.add_right(v),
        }
    }
    fn add_left(&mut self, v: u32) {
        match &mut self.left {
            SnailNum::Just(o) => self.left = SnailNum::Just(*o + v),
            SnailNum::Pair(pair) => pair.add_left(v),
        }
    }
}
fn parse_snailfish_number(input: &str, mut cursor: usize) -> (SnailNum, usize) {
    let mut ch = input.chars().nth(cursor).unwrap();
    if let Some(d) = ch.to_digit(10) {
        return (SnailNum::Just(d), cursor + 1);
    }

    assert!(ch == '[');
    cursor += 1;

    let (left, mut cursor) = parse_snailfish_number(input, cursor);
    ch = input.chars().nth(cursor).unwrap();
    assert!(ch == ',');
    cursor += 1;
    let (right, mut cursor) = parse_snailfish_number(input, cursor);
    ch = input.chars().nth(cursor).unwrap();
    assert!(ch == ']');
    cursor += 1;

    (SnailNum::Pair(Box::new(SnailPair { left, right })), cursor)
}

impl SnailNum {
    pub(crate) fn reduce(&mut self) {
        match self {
            SnailNum::Just(_) => {}
            SnailNum::Pair(pair) => {
                pair.explode(0);
            }
        }
    }
}

impl std::ops::Add for &SnailNum {
    type Output = SnailNum;

    fn add(self, rhs: Self) -> Self::Output {
        let mut num = SnailNum::Pair(Box::new(SnailPair {
            left: self.clone(),
            right: rhs.clone(),
        }));
        println!("Adding {} + {}: {}", self, rhs, num);
        num.reduce();
        println!("after reduction:{}", num);
        num
    }
}
fn main() {
    println!("Hello, world!");
    let content = fs::read_to_string("day18/input_simple2").unwrap();

    let numbers: Vec<_> = content
        .lines()
        .map(|e| parse_snailfish_number(e, 0).0)
        .collect();
    for number in &numbers {
        println!("{}", number);
    }

    let addition = &numbers[0] + &numbers[1];
    println!("addition {}", addition);
}
