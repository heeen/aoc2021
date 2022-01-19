use enum_iterator::IntoEnumIterator;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl std::ops::Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Sub for Pos {
    type Output = Pos;
    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, IntoEnumIterator, PartialEq, Clone, Copy)]
enum UpAxis {
    X,
    NegativeX,
    Y,
    NegativeY,
    Z,
    NegativeZ,
}

#[derive(Debug, IntoEnumIterator, PartialEq, Clone, Copy)]
enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg279,
}

impl Pos {
    fn transformed(&self, orientation: (UpAxis, Rotation)) -> Self {
        let (x, y, z) = (self.x, self.y, self.z);
        let (x, y, z) = match orientation.0 {
            UpAxis::X => (x, y, z),
            UpAxis::NegativeX => (-x, y, -z),
            UpAxis::Y => (y, -x, z),
            UpAxis::NegativeY => (-y, x, z),
            UpAxis::Z => (z, y, -x),
            UpAxis::NegativeZ => (-z, y, x),
        };
        let (x, y, z) = match orientation.1 {
            Rotation::Deg0 => (x, y, z),
            Rotation::Deg90 => (x, z, -y),
            Rotation::Deg180 => (x, -y, -z),
            Rotation::Deg279 => (x, -z, y),
        };
        Pos { x, y, z }
    }
}

#[test]
fn test_transform() {
    let mut results = HashSet::new();
    for orientation in UpAxis::into_enum_iter().cartesian_product(Rotation::into_enum_iter()) {
        let p = Pos { x: 1, y: 2, z: 3 }.transformed(orientation);
        assert!(!results.contains(&p));
        results.insert(p);
    }
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Pos>,
    octants: [Vec<usize>; 8],
    constellations: Vec<Constellation>,
    position: Pos,
}

#[derive(Copy, Clone, Debug)]
struct Constellation {
    distance: i32,
    indices: (usize, usize),
    scanner: usize,
}

fn main() {
    let mut scanners = fs::read_to_string("day19/input")
        .unwrap()
        .lines()
        .fold(Vec::new(), |mut a, line| {
            if line.starts_with("---") {
                a.push(Scanner::new());
            } else if !line.is_empty() {
                let mut parts = line.split(',').map(|p| p.parse::<i32>().unwrap());
                let last = a.last_mut().unwrap();
                let (x, y, z) = (
                    parts.next().unwrap(),
                    parts.next().unwrap(),
                    parts.next().unwrap(),
                );

                let octant = if x.signum() > 0 { 1 } else { 0 }
                    + if y.signum() > 0 { 2 } else { 0 }
                    + if z.signum() > 0 { 4 } else { 0 };
                last.octants[octant].push(last.beacons.len());
                last.beacons.push(Pos { x, y, z });
            }
            a
        });

    let mut constellations_scanners = HashMap::new();

    for (scanner_index, scanner) in scanners.iter_mut().enumerate() {
        println!("--- scanner {scanner_index} ---");
        for (octant, beacons) in scanner.octants.iter().enumerate() {
            //            println!("octant {octant} {beacons:?}");
            let min_dist = beacons
                .iter()
                .tuple_combinations()
                .map(|(c_a, c_b)| {
                    let b_a = scanner.beacons[*c_a];
                    let b_b = scanner.beacons[*c_b];
                    let d = (b_a.x - b_b.x, b_a.y - b_b.y, b_a.z - b_b.z);
                    let dist_sq = d.0 * d.0 + d.1 * d.1 + d.2 * d.2;
                    (dist_sq, *c_a, *c_b)
                })
                .min_by_key(|e| e.0)
                .unwrap();

            let constellation = Constellation {
                distance: min_dist.0,
                indices: (min_dist.1, min_dist.2),
                scanner: scanner_index,
            };
            scanner.constellations.push(constellation);
            constellations_scanners
                .entry(constellation.distance)
                .or_insert_with(Vec::new)
                .push(constellation);
        }
    }

    let mut workqueue: Vec<_> = vec![0];
    let mut unified_beacons: HashSet<Pos> = scanners[0].beacons.iter().cloned().collect();
    let mut visited_scanners = HashSet::new();
    while let Some(scanner_index) = workqueue.pop() {
        if visited_scanners.contains(&scanner_index) {
            continue;
        }
        println!("----------- popped scanner {scanner_index}");
        let constellations = scanners[scanner_index].constellations.clone();

        let mut successful_links = HashSet::new();
        for links in constellations
            .iter()
            .map(|c| &constellations_scanners[&c.distance])
        {
            if links.len() < 2 {
                continue;
            }
            for (c_a, c_b) in links
                .iter()
                .filter(|c| !visited_scanners.contains(&c.scanner))
                .tuple_combinations()
            {
                let (c_self, c_other, other) = if c_a.scanner == scanner_index {
                    (c_a, c_b, c_b.scanner)
                } else if c_b.scanner == scanner_index {
                    (c_b, c_a, c_a.scanner)
                } else {
                    continue;
                };
                if successful_links.contains(&other) {
                    continue;
                }

                if let Some((orientation, dist, transformed_b_beacons)) =
                    check_match(&scanners, c_self, c_other)
                {
                    assert_eq!(scanners[other].beacons.len(), transformed_b_beacons.len());
                    println!("connection {scanner_index} - {other}: {orientation:?} dist {dist:?}");
                    scanners[other].beacons = transformed_b_beacons;
                    scanners[other].position = dist;

                    unified_beacons = unified_beacons
                        .union(&scanners[other].beacons.iter().cloned().collect())
                        .cloned()
                        .collect();
                    workqueue.push(other);
                    successful_links.insert(other);
                }
            }
        }
        visited_scanners.insert(scanner_index);
    }
    assert_eq!(visited_scanners.len(), scanners.len());
    println!("unified beacons: {} ", unified_beacons.len());
    let max_dist = scanners
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((a_i, a), (b_i, b))| {
            let d = a.position - b.position;
            (
                d.x.abs() + d.y.abs() + d.z.abs(),
                a_i,
                b_i,
                a.position,
                b.position,
            )
        })
        .max_by_key(|e| e.0)
        .unwrap();

    println!("max distance: {:?}", max_dist);
    assert!(max_dist.0 < 39680);
    assert_eq!(unified_beacons.len(), 405);
}

fn check_match(
    scanners: &[Scanner],
    c_a: &Constellation,
    c_b: &Constellation,
) -> Option<((UpAxis, Rotation), Pos, Vec<Pos>)> {
    let (scanner_a, scanner_b) = (&scanners[c_a.scanner], &scanners[c_b.scanner]);
    let (a_0, a_1) = (
        scanner_a.beacons[c_a.indices.0],
        scanner_a.beacons[c_a.indices.1],
    );
    let (b_0, b_1) = (
        scanner_b.beacons[c_b.indices.0],
        scanner_b.beacons[c_b.indices.1],
    );

    let v_a = a_1 - a_0;
    let v_a_inv = a_0 - a_1;
    let v_b = b_1 - b_0;

    let mut scanner_a_beacons = HashSet::new();
    for beacon in scanner_a.beacons.iter() {
        scanner_a_beacons.insert(*beacon);
    }

    for orientation in UpAxis::into_enum_iter().cartesian_product(Rotation::into_enum_iter()) {
        let transformed = v_b.transformed(orientation);
        let dist = if transformed == v_a {
            a_0 - b_0.transformed(orientation)
        } else if transformed == v_a_inv {
            a_0 - b_1.transformed(orientation)
        } else {
            continue;
        };

        let mut scanner_b_beacons = Vec::new();
        let mut overlap = 0usize;
        for beacon in scanner_b.beacons.iter() {
            let p = beacon.transformed(orientation) + dist;
            if scanner_a_beacons.contains(&p) {
                overlap += 1;
            }
            scanner_b_beacons.push(p);
        }
        if overlap < 12 {
            continue;
        }
        return Some((orientation, dist, scanner_b_beacons));
    }

    None
}
impl Scanner {
    fn new() -> Self {
        Scanner {
            beacons: Vec::new(),
            octants: [(); 8].map(|_| Vec::new()),
            constellations: Vec::new(),
            position: Pos { x: 0, y: 0, z: 0 },
        }
    }
}
