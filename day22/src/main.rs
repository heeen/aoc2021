use itertools::Itertools;
use std::{error::Error, fs, ops::RangeInclusive};

#[derive(Debug, Clone, PartialEq)]
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

        let x = xmin.parse().unwrap()..=xmax.parse().unwrap();
        let y = ymin.parse().unwrap()..=ymax.parse().unwrap();
        let z = zmin.parse().unwrap()..=zmax.parse().unwrap();
        Self { x, y, z }
    }
    fn count(&self) -> usize {
        (self.x.end() - self.x.start() + 1) as usize
            * (self.y.end() - self.y.start() + 1) as usize
            * (self.z.end() - self.z.start() + 1) as usize
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
            let mut result = Vec::new();
            let ex_x = self.x.except(&other.x);
            for r in ex_x {
                result.push(Range3 {
                    x: r,
                    y: self.y.clone(),
                    z: self.z.clone(),
                });
            }
            let ex_y = self.y.except(&other.y);
            for r in ex_y {
                result.push(Range3 {
                    x: intersection.x.clone(),
                    y: r,
                    z: self.z.clone(),
                })
            }
            let ex_z = self.z.except(&other.z);
            for r in ex_z {
                result.push(Range3 {
                    x: intersection.x.clone(),
                    y: intersection.y.clone(),
                    z: r,
                })
            }
            result
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

    fn except(&self, other: &Self) -> Vec<Self> {
        if let Some(intersection) = self.intersection(other) {
            if intersection == *self {
                vec![]
            } else if intersection.end() == self.end() {
                vec![*self.start()..=intersection.start() - 1]
            } else if intersection.start() == self.start() {
                vec![intersection.end() + 1..=*self.end()]
            } else {
                vec![
                    *self.start()..=intersection.start() - 1,
                    intersection.end() + 1..=*self.end(),
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

#[test]
fn except3_test() {
    let r1 = Range3 {
        x: 0..=2,
        y: 0..=2,
        z: 0..=2,
    };
    println!("r1 {:?} count {}", r1, r1.count());
    let candidates = vec![
        Range3 {
            x: 1..=1,
            y: 1..=1,
            z: 1..=1,
        },
        r1.clone(),
        Range3 {
            x: 0..=2,
            y: 1..=1,
            z: 1..=1,
        },
        Range3 {
            x: 0..=2,
            y: 1..=1,
            z: 0..=2,
        },
        Range3 {
            x: -1..=1,
            y: -1..=1,
            z: -1..=1,
        },
        Range3 {
            x: -2..=-1,
            y: -2..=-1,
            z: -2..=-1,
        },
        Range3 {
            x: -2..=-1,
            y: 0..=2,
            z: 0..=2,
        },
    ];

    for r2 in candidates {
        println!("testing candidate {:?} count {}", r2, r2.count());
        let ex = r1.except(&r2);
        let ex_count = ex.iter().map(|r| r.count()).sum::<usize>();
        if let Some(is) = r1.intersection(&r2) {
            println!("  intersection {:?} count {}", is, is.count());
            assert!(r1.contains_range(&is));
            assert!(r2.contains_range(&is));

            for (i, ex_sub) in ex.iter().enumerate() {
                println!(
                    "    checking except part {:?} {} with {:?}",
                    ex_sub,
                    ex_sub.count(),
                    is
                );
                for j in 0..i {
                    assert!(ex_sub.intersection(&ex[j]).is_none());
                }
                assert!(ex_sub.intersection(&is).is_none());
            }
            assert!(ex_count <= r1.count());
            assert_eq!(ex_count, r1.count() - is.count());
        } else {
            assert_eq!(ex_count, r1.count());
            assert!(ex[0] == r1);
        }
    }
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

    let p1 = operate(
        content
            .iter()
            .filter_map(|(op, r)| match part1_limit.intersection(&r) {
                None => None,
                Some(i) => Some((*op, i)),
            }),
    );
    let p1_count = p1.iter().map(|r| r.count()).sum::<usize>();
    println!("{:?}", p1_count);
    let p2 = operate(content);
    let p2_count = p2.iter().map(|r| r.count()).sum::<usize>();
    println!("{:?}", p2_count);
    Ok(())
}

fn operate<I>(ops: I) -> Vec<Range3>
where
    I: IntoIterator<Item = (bool, Range3)>,
{
    let mut on_ranges = Vec::new();
    for (op, r) in ops {
        if op {
            let mut remainder = vec![r];
            for already_on in &on_ranges {
                let mut next = Vec::new();
                for rem in remainder {
                    next.append(&mut rem.except(&already_on));
                }
                remainder = next;
            }
            on_ranges.append(&mut remainder);
        } else {
            let mut still_on = Vec::new();
            for on in on_ranges {
                still_on.append(&mut on.except(&r));
            }

            on_ranges = still_on;
        }
    }

    on_ranges
}
