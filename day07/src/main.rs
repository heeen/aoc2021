use std::fs;

fn main() {
    let contents = fs::read_to_string("day07/input").expect("could not read input");
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let start_posiitons: Vec<_> = contents
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| {
            let v = s.parse::<i32>().unwrap();
            min = min.min(v);
            max = max.max(v);
            v
        })
        .collect();
//    println!("input positions {:?}", start_posiitons);

    let costFn = |position: i32| {
        start_posiitons.iter().fold(0, |a, crab| {
            let dist = (position - crab).abs();
            a + (dist * (dist + 1)) / 2
        })
    };

    let mut last_pos = min;
    let mut last_cost = costFn(min);
    let mut step = (min + max) / 2;

    let mut next_pos = last_pos + step;
    let mut next_cost = costFn(next_pos);
    loop {
        if next_cost > last_cost {
            step = -step / 2;
        }
        if step == 0 {
            break;
        }
        last_pos = next_pos;
        last_cost = next_cost;
        next_pos = last_pos + step;
        next_cost = costFn(next_pos);
    }

    println!("position {} cost {}", last_pos, last_cost);
}
