use std::{collections::HashSet, fs};
fn main() {
    let contents = fs::read_to_string("day14/input_simple").expect("could not read input");
    let lines = contents.lines();
    let seed = lines.next().unwrap();
    lines.next();
    let patterns = lines.map(|l| l.split_once('->').unwrap())
    for line in lines {
    }
    println!("seed: {}", seed);
    println!("patterns: {}", patterns);
}
