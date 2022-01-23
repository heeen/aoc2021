//use bit_vec::BitVec;
use itertools::Itertools;
use std::fs;

fn to_bit_set(wires: &str) -> u8 {
    wires
        .chars()
        .fold(0u8, |a, c| a | 1 << ((c as usize) - ('a' as usize)))
}

struct PatternDef {
    one_cf: u8,
    four_bcdf: u8,
    seven_acf: u8,
    bd: u8,
}

impl PatternDef {
    fn discriminate(&self, pattern: u8) -> Option<u8> {
        let ones = pattern.count_ones();
        if pattern == self.one_cf {
            Some(1)
        } else if pattern == self.seven_acf {
            Some(7)
        } else if pattern == self.four_bcdf {
            Some(4)
        } else if ones == 7 {
            Some(8)
        } else if ones == 5 {
            if pattern & self.one_cf == self.one_cf {
                Some(3)
            } else if pattern & self.bd == self.bd {
                Some(5)
            } else {
                Some(2)
            }
        } else if ones == 6 {
            let nine = self.one_cf | self.bd;
            if pattern & nine == nine {
                Some(9)
            } else if pattern & self.bd == self.bd {
                Some(6)
            } else {
                Some(0)
            }
        } else {
            None
        }
    }
}

fn main() {
    let contents = fs::read_to_string("day08/input").expect("could not read input");

    let mut counts = vec![0; 10];

    let data: Vec<_> = contents
        .lines()
        .map(|s| {
            let (patterns, values) = s.split_once('|').unwrap();
            let patterns: Vec<_> = patterns
                .split_ascii_whitespace()
                .sorted_by(|a, b| a.len().cmp(&b.len()))
                .map(to_bit_set)
                .collect();

            let def = PatternDef {
                one_cf: patterns[0],
                seven_acf: patterns[1],
                four_bcdf: patterns[2],
                bd: patterns[2] & !patterns[0],
            };
            let values: Vec<_> = values
                .split_ascii_whitespace()
                .map(|wires| {
                    let pattern = to_bit_set(wires);
                    def.discriminate(pattern)
                })
                .collect();
            (patterns, values)
        })
        .collect();

    let mut sum = 0u64;
    for line in data {
        line.1
            .iter()
            .for_each(|digit| counts[digit.unwrap() as usize] += 1);
        let value = line
            .1
            .iter()
            .fold(0u64, |a, digit| a * 10 + (digit.unwrap() as u64));
        print!("patterns:");
        for (i, pattern) in line.0.iter().enumerate() {
            print!("{i}:{pattern:08b} (#{}), ", pattern.count_ones());
        }
        println!(" values {:?} {}", line.1, value);
        sum += value;
    }
    println!(
        "counts: {:?} 1478s:{} sum {}",
        counts,
        counts[1] + counts[4] + counts[7] + counts[8],
        sum
    );
}
