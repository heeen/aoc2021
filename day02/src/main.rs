use std::fs;

fn main() {
    let contents = fs::read_to_string("day02/input").expect("could not read input");
    let values: Vec<_> = contents
        .lines()
        .map(|l| {
            let parts = l.split_once(' ').unwrap();
            (parts.0, parts.1.parse::<i32>().unwrap())
        })
        .collect();

    let position = values
        .iter()
        .fold((0, 0), |accum: (i32, i32), value| match value.0 {
            "forward" => (accum.0 + value.1, accum.1),
            "up" => (accum.0, accum.1 - value.1),
            "down" => (accum.0, accum.1 + value.1),
            _ => panic!("unexpected movement"),
        });

    println!(
        "day 2 part 1: {} {} {}",
        position.0,
        position.1,
        position.0 * position.1
    );

    let position = values
        .iter()
        .fold((0, 0, 0), |accum, value| {
            let (pos, depth, aim) = accum;
            let (command, value) = *value;
            match command {
            "forward" => (pos + value, depth + aim * value, aim),
            "up" => (pos, depth, aim - value),
            "down" => (pos, depth, aim + value),
            _ => panic!("unexpected movement"),
        }});

    println!(
        "day 2 part 2: {} {} {}",
        position.0,
        position.1,
        position.0 * position.1
    );
}
