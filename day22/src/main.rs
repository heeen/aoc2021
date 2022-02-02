use itertools::Itertools;
use std::{
    error::Error,
    fs,
    ops::{Range, RangeInclusive},
};

#[derive(Debug, Clone)]
struct Range3 {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl Range3 {
    fn from_str(data: &str) -> Self {
        let (xstr, ystr, zstr) = data.splitn(3, ',').collect_tuple().unwrap();
        let (xmin, xmax) = xstr[2..].split_once("..").unwrap();
        let (ymin, ymax) = ystr[2..].split_once("..").unwrap();
        let (zmin, zmax) = zstr[2..].split_once("..").unwrap();

        println!("{} {}", xmin, xmax);
        let x = xmin.parse().unwrap()..=xmax.parse().unwrap();
        let y = ymin.parse::<i32>().unwrap()..=ymax.parse::<i32>().unwrap();
        let z = zmin.parse::<i32>().unwrap()..=zmax.parse::<i32>().unwrap();
        Self { x, y, z }
    }
}

trait RangeBooleanOps
where
    Self: Sized,
{
    fn intersection(&self, other: &Self) -> Option<Self>;
    fn contains_range(&self, other: &Self) -> bool;
    fn except(&self, other: &Self) -> Vec<Self>;

}

impl RangeBooleanOps for Range3 {
    fn intersection(&self, other: &Self) -> Option<Self> {
        match (
            self.x.intersection(&other.x),
            self.y.intersection(&other.y),
            self.z.intersection(&other.z),
        ) {
            (Some(x), Some(y), Some(z)) => Some(Self { x, y, z }),
            _ => None,
        }
    }

    fn contains_range(&self, other: &Self) -> bool {
        self.x.contains_range(&other.x)
            && self.y.contains_range(&other.y)
            && self.z.contains_range(&other.z)
    }

    fn except(&self, other: &Self) -> Vec<Self> {
        if let Some(intersection) = self.intersection(other) {
            vec![]
        } else {
            vec![self.clone()]
        }
    }
}

impl RangeBooleanOps for RangeInclusive<i32> {
    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.end() < other.start() || other.end() < self.start() {
            None
        } else {
            Some(*self.start().max(other.start())..=*self.end().min(other.end()))
        }
    }

    fn contains_range(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    fn except(&self, other: &Self)-> Vec<Self> {
        if let Some(intersection) = self.intersection(other) {
            if intersection == *self {
                vec![]
            } else if intersection.end() == self.end() {
                vec![*self.start()..=*intersection.start()]
            } else if intersection.start() == self.start() {
                vec![*intersection.end()..=*self.end()]
            } else {
                vec![
                    *self.start()..=intersection.start() - 1,
                    intersection.end()+1..=*self.end()
                ]
            }
        } else {
            vec![self.clone()]
        }
    }
}

#[test]
fn intersection_test() {
    let r1 = Range3 {
        x: -10..=0,
        y: -10..=0,
        z: -10..=0,
    };
    let r2 = Range3 {
        x: 0..=10,
        y: 0..=10,
        z: 0..=10,
    };
    let intersection = r1.intersection(&r2);
    println!("intersection: {:?}", intersection);
    for i in intersection.unwrap().x {
        print!("x{i}")
    }
    println!();
    let r2 = Range3 {
        x: 1..=10,
        y: 1..=10,
        z: 2..=10,
    };
    let intersection = r1.intersection(&r2);
    println!("intersection: {:?}", intersection);

    let r2 = Range3 {
        x: -10..=0,
        y: 0..=10,
        z: 0..=10,
    };
    let intersection = r1.intersection(&r2);
    println!("intersection: {:?}", intersection);
}

#[test]
fn except_test() {
    let e = (0..=10).except(&(4..=5));
    println!("{:?}", e);
    let e = (0..=10).except(&(-1..=11));
    println!("{:?}", e);
    let e = (0..=10).except(&(-1..=5));
    println!("{:?}", e);
    let e = (0..=10).except(&(5..=10));
    println!("{:?}", e);
}


fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("day22/input_simple")?
        .lines()
        .map(|l| {
            let (cmd, range) = l.split_once(' ').unwrap();
            return (cmd == "on", Range3::from_str(range));
        })
        .collect_vec();
    let part1_limit = Range3 {
        x: -50..=50,
        y: -50..=50,
        z: -50..=50,
    };

//    let mut on_ranges = Vec::new();
    for (op,r) in content
        .iter()
        .filter_map(|(op, r)| match part1_limit.intersection(&r) {
            None => None,
            Some(i) => Some((*op, i)),
        })
    {
        if op {
        }else {

        }
        println!("{:?} {:?}", op, r);
    }
    Ok(())
}
