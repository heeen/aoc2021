use std::fs;

fn main() {
    let contents = fs::read_to_string("day01/input").expect("could not read input");
    let value = contents.lines().map(|l| l.parse::<i32>().unwrap()).fold(
        (None, 0),
        |prev_accum: (Option<i32>, i32), value: i32| match prev_accum {
            (None, _) => (Some(value), 0),
            (Some(prev), accum) => {
                if value > prev {
                    (Some(value), accum + 1)
                } else {
                    (Some(value), accum)
                }
            }
        },
    );

    println!("day 1 part 1: {}", value.1);

    let values: Vec<i32>  = contents.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let mut accum = 0;
    let mut sum = values[0] + values[1] + values[2];
    for i in 3..values.len() {
        let newsum = sum - values[i - 3] + values[i];
        if newsum > sum {
            accum += 1;
        }
        sum = newsum;
    }
    println!("day 1 part 2: {}", accum);
}
