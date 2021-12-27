use std::{collections::HashSet, fs};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Fold {
    X(i32),
    Y(i32),
}

fn main() {
    let contents = fs::read_to_string("day13/input").expect("could not read input");
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    for line in contents.lines() {
        if let Some(parts) = line.split_once(',') {
            points.insert(Point {
                x: parts.0.parse().unwrap(),
                y: parts.1.parse().unwrap(),
            });
        } else if let Some(parts) = line.split_once('=') {
            let v = parts.1.parse().unwrap();
            folds.push(match parts.0 {
                "fold along x" => Fold::X(v),
                "fold along y" => Fold::Y(v),
                _ => panic!("unexpected fold"),
            });
        }
    }
    for fold in folds {
        points = points
            .iter()
            .map(|p| match fold {
                Fold::X(x) => {
                    if p.x > x {
                        Point {
                            x: x - (p.x - x),
                            y: p.y,
                        }
                    } else {
                        *p
                    }
                }
                Fold::Y(y) => {
                    if p.y > y {
                        Point {
                            x: p.x,
                            y: y - (p.y - y),
                        }
                    } else {
                        *p
                    }
                }
            })
            .collect();
    }
    let mut printout: Vec<Vec<bool>> = Vec::new();
    for point in points {
        if point.y as usize >= printout.len() {
            printout.resize(point.y as usize + 1, Vec::new());
        }
        if point.x as usize >= printout[point.y as usize].len() {
            printout[point.y as usize].resize(point.x as usize + 1, false);
        }
        printout[point.y as usize][point.x as usize] = true;
    }

    for line in printout {
        println!(
            "{}",
            line.iter().map(|v| match v {
                true => "#",
                false => " ",
            }).collect::<String>()
        )
    }
}
