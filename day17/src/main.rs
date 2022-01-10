use std::fs;

use regex::Regex;

fn main() {
    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();

    let content = fs::read_to_string("day17/input").unwrap();
    let vals = re.captures_iter(&content).next().unwrap();
    let vals = vec![&vals[1], &vals[2], &vals[3], &vals[4]]
        .iter()
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let (x1, x2, y1, y2) = (vals[0], vals[1], vals[2], vals[3]);
    println!("input: {} {}  {} {}", x1, x2, y1, y2);

    let v0 = y1.abs() - 1;
    let v_terminal = v0 + 1;
    let max_height = (v0 * v_terminal) / 2;

    println!("v0: {} max_height: {}", v0, max_height);

    let v_x_min = ((2 * x1 - 1) as f64).sqrt() as i32;
    let v_x_max = x2 + 1;
    let v_y_min = -v_terminal - 1;
    let v_y_max = v_terminal + 1;
    let mut count = 0;
    let mut total = 0;
    for v_x in v_x_min..v_x_max {
        for v_y in v_y_min..v_y_max {
            total += 1;
            if simulate(v_x, v_y, x1, x2, y1, y2) {
                count += 1;
            }
        }
    }
    println!(
        "count: {} of {} x({}..{}) y({}..{})",
        count, total, v_x_min, v_x_max, v_y_min, v_y_max
    );
    println!("input: {} {}  {} {}", x1, x2, y1, y2);
}

fn simulate(v_x0: i32, v_y0: i32, x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    let (mut x, mut y) = (0, 0);
    let (mut vel_x, mut vel_y) = (v_x0, v_y0);
    loop {
        x += vel_x;
        y += vel_y;
        vel_x -= vel_x.signum();
        vel_y -= 1;
        if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
            return true;
        }
        if x > x2 || y < y1 {
            return false;
        }
    }
}
