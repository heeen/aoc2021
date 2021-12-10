use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn overlaps(lines: impl Iterator<Item = Line>, diagonals: bool) -> usize {
    let mut map: HashMap<Point, usize> = HashMap::new();
    let mut count: usize = 0;

    for line in lines {
        let dx = (line.p2.x - line.p1.x).signum();
        let dy = (line.p2.y - line.p1.y).signum();
        if dx != 0 && dy != 0 && !diagonals {
            continue;
        };
        let mut p = line.p1;
        loop {
            if let Some(v) = map.get(&p) {
                let v = *v;
                if v == 1 {
                    count += 1;
                }
                map.insert(p, v + 1);
            } else {
                map.insert(p, 1);
            }

            if p == line.p2 {
                break;
            }
            p.x += dx;
            p.y += dy;
        }
    }
    count
}

fn main() {
    let contents = fs::read_to_string("day05/input").expect("could not read input");

    let lines = contents.lines().map(|l| {
        let mut points = l.split(" -> ");
        let mut p1 = points.next().unwrap().split(',');
        let mut p2 = points.next().unwrap().split(',');
        Line {
            p1: Point {
                x: p1.next().unwrap().parse().unwrap(),
                y: p1.next().unwrap().parse().unwrap(),
            },
            p2: Point {
                x: p2.next().unwrap().parse().unwrap(),
                y: p2.next().unwrap().parse().unwrap(),
            },
        }
    });
    let count1 = overlaps(lines.clone(), true);
    let count2 = overlaps(lines.clone(), false);
    println!(
        "two or more: diagnoals {} without diagonals {}",
        count1, count2
    );
}
