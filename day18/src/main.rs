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
    fn explode(&mut self, level: u32) -> (bool, Option<u32>, Option<u32>) {
        match self {
            SnailNum::Just(_) => (false, None, None),
            SnailNum::Pair(self_pair) if level < 4 => {
                let (exp, l, r) = self_pair.left.explode(level + 1);
                if exp {
                    if let Some(v) = r {
                        self_pair.right.add_left(v);
                    }
                    return (true, l, None);
                }

                let (exp, l, r) = self_pair.right.explode(level + 1);
                if exp {
                    if let Some(v) = l {
                        self_pair.left.add_right(v);
                    }
                    return (true, None, r);
                }
                (false, None, None)
            }
            SnailNum::Pair(pair) => {
                let ret = match (&pair.left, &pair.right) {
                    (SnailNum::Just(l), SnailNum::Just(r)) => (true, Some(*l), Some(*r)),
                    (_, _) => panic!("expected simple value pair, not nested pair"),
                };
                *self = SnailNum::Just(0);
                return ret;
            }
        }
    }

    pub(crate) fn split(&mut self) -> bool {
        match self {
            SnailNum::Just(v) if *v > 9 => {
                let l = (*v as f32 / 2.0).floor() as u32;
                let r = (*v as f32 / 2.0).ceil() as u32;
                let num = SnailNum::Pair(Box::new(SnailPair {
                    left: SnailNum::Just(l),
                    right: SnailNum::Just(r),
                }));
                *self = num;
                true
            }
            SnailNum::Just(_) => false,
            SnailNum::Pair(self_pair) => self_pair.left.split() || self_pair.right.split(),
        }
    }

    pub(crate) fn add_left(&mut self, a: u32) {
        match self {
            SnailNum::Just(v) => *self = SnailNum::Just(*v + a),
            SnailNum::Pair(self_pair) => self_pair.left.add_left(a),
        }
    }

    pub(crate) fn add_right(&mut self, a: u32) {
        match self {
            SnailNum::Just(v) => *self = SnailNum::Just(*v + a),
            SnailNum::Pair(self_pair) => self_pair.right.add_right(a),
        }
    }
    fn magnitude(&self) -> u64 {
        match self {
            SnailNum::Just(v) => *v as u64,
            SnailNum::Pair(self_pair) => {
                self_pair.left.magnitude() * 3 + self_pair.right.magnitude() * 2
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
        loop {
            let (exp, _, _) = num.explode(0);
            if exp {
                continue;
            }
            if !num.split() {
                break;
            }
        }
        num
    }
}

fn main() {
    let content = fs::read_to_string("day18/input").unwrap();

    let numbers: Vec<_> = content
        .lines()
        .map(|e| parse_snailfish_number(e, 0).0)
        .collect();

    let mut sum = numbers[0].clone();
    for (i, number) in (&numbers[1..]).iter().enumerate() {
        println!("{}", number);
        sum = &sum + number;
        println!("add {}", number);
        println!("sum {} mag {}", sum, sum.magnitude());
    }

    let mut max = 0;
    for (i1, num1) in numbers.iter().enumerate() {
        for (i2, num2) in numbers.iter().enumerate() {
            let sum = num1 + num2;
            let mag = sum.magnitude();
            if mag > max {
                max = mag;
                println!("new max: {}+{} {}", num1, num2, mag);
            }
        }
    }
}
