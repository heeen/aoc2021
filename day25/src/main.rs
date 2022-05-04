use itertools::Itertools;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    fmt::Display,
    fs,
};
fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("day25/input")?
        .lines()
        .map(|s| s.chars().collect_vec())
        .collect_vec();
    println!("{content:?}");

    let height = content.len();
    let width = content[0].len();
    println!("size: {width}x{height}");

    let mut east = HashSet::new();
    let mut south = HashSet::new();
    for (row, row_data) in content.iter().enumerate() {
        for (col, col_data) in row_data.iter().enumerate() {
            match col_data {
                '>' => {
                    east.insert((col, row));
                }
                'v' => {
                    south.insert((col, row));
                }
                _ => {}
            }
        }
    }
    let mut step = 0;
    loop {
        step += 1;
        let mut moved = false;
        let mut next_east = HashSet::new();
        let mut next_south = HashSet::new();
        for &(x, y) in east.iter() {
            let next = ((x + 1) % width, y);
            if east.contains(&next) || south.contains(&next) {
                next_east.insert((x, y));
            } else {
                moved = true;
                next_east.insert(next);
            }
        }
        east = next_east;
        for &(x, y) in south.iter() {
            let next = (x, (y + 1) % height);
            if east.contains(&next) || south.contains(&next) {
                next_south.insert((x, y));
            } else {
                moved = true;
                next_south.insert(next);
            }
        }
        south = next_south;

        println!("\nstep {step}:");
/*        for y in 0..height {
            for x in 0..width {
                if east.contains(&(x, y)) {
                    print!(">");
                } else if south.contains(&(x, y)) {
                    print!("v");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        */
        if !moved {
            break;
        }
    }

    Ok(())
}
