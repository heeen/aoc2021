use itertools::Itertools;
use std::{collections::HashSet, fmt::Display, fs};

fn dump(data: &Vec<Vec<u8>>) {
    for row in data {
        let s = std::str::from_utf8(&row).unwrap();
        println!("{}", s);
    }
}
#[derive(Clone)]
struct TileImage {
    minX: i32,
    minY: i32,
    maxX: i32,
    maxY: i32,
    set: HashSet<(i32, i32)>,
}

impl TileImage {
    fn get(&self, x: i32, y: i32, voidValue: bool) -> bool {
        if x < self.minX || x > self.maxX || y < self.minY || y > self.maxY {
            voidValue
        } else {
            self.set.contains(&(x, y))
        }
    }
    fn get_9(&self, x: i32, y: i32, voidValue: bool) -> usize {
        let mut result = 0;
        for iy in y - 1..=y + 1 {
            for ix in x - 1..=x + 1 {
                result = (result << 1) | self.get(ix, iy, voidValue) as usize;
            }
        }
        result
    }
    fn set(&mut self, x: i32, y: i32) {
        self.set.insert((x, y));
        self.minX = self.minX.min(x);
        self.minY = self.minY.min(y);
        self.maxX = self.maxX.max(x);
        self.maxY = self.maxY.max(y);
    }

    fn new() -> Self {
        TileImage {
            minX: i32::MAX,
            minY: i32::MAX,
            maxX: i32::MIN,
            maxY: i32::MIN,
            set: HashSet::new(),
        }
    }
    fn fold(&self, program: &Vec<u8>, voidValue: bool) -> Self {
        let mut result_image = TileImage::new();

        const MARGIN: i32 = 2;
        for y in self.minY - MARGIN..=self.maxY + MARGIN {
            for x in self.minX - MARGIN..=self.maxX + MARGIN {
                let lookup = self.get_9(x, y, voidValue);
                if program[lookup] == '#' as u8 {
                    result_image.set(x, y)
                }
            }
        }
        result_image
    }
}
impl Display for TileImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.minY..=self.maxY {
            for x in self.minX..=self.maxX {
                write!(f, "{}", if self.get(x, y, false) { '#' } else { '.' });
            }
            write!(f, "\n");
        }
        write!(
            f,
            "\nx:{}..{} y:{}..{} count {}",
            self.minX,
            self.maxX,
            self.minY,
            self.maxY,
            self.set.len()
        )
    }
}
fn main() {
    let input = fs::read_to_string("day20/input").unwrap();

    let mut input = input.lines();
    let program = input.next().unwrap().chars().map(|c| c as u8).collect_vec();

    let program_inverts_void = program[0] == '#' as u8;
    let mut img = TileImage::new();
    let seed = input
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(l.chars().map(|c| c as u8).collect_vec())
            }
        })
        .collect_vec();

    for (y, row) in seed.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' as u8 {
                img.set(x as i32, y as i32);
            }
        }
    }
    println!("{}", img);
    println!("{}", std::str::from_utf8(&program).unwrap());

    let mut input = img.clone();
    for i in 0..2 {
        let result_image = input.fold(&program, program_inverts_void && i % 2 == 1);
        println!("result {}:\n{}", i, result_image);
        input = result_image;
    }
}
