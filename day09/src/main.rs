use std::fs;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}
trait Heightmap {
    fn get(&self, x: i32, y: i32) -> Option<i32>;
    fn set(&mut self, x: i32, y: i32);
    fn get_candidates(&mut self, stack: &mut Vec<Point>, x: i32, y: i32);
}

impl Heightmap for Vec<Vec<i32>> {
    fn get(&self, x: i32, y: i32) -> Option<i32> {
        if y < 0 || y as usize >= self.len() {
            None
        } else {
            let row = &self[y as usize];
            if x < 0 || x as usize >= row.len() {
                None
            } else {
                Some(row[x as usize])
            }
        }
    }

    fn set(&mut self, x: i32, y: i32) {
        self[y as usize][x as usize] = 9;
    }

    fn get_candidates(&mut self, stack: &mut Vec<Point>, x: i32, y: i32) {
        if let Some(v) = self.get(x, y) {
            if v < 9 {
                self.set(x, y);
                stack.push(Point { x, y });
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("day09/input").expect("could not read input");
    let mut heightmap: Vec<Vec<i32>> = contents
        .lines()
        .map(|l| l.chars().map(|c| c as i32 - '0' as i32).collect())
        .collect();

    let mut score = 0;

    let mut seeds = Vec::new();
    for y in 0..heightmap.len() as i32 {
        for x in 0..heightmap[y as usize].len() as i32 {
            let val = heightmap.get(x, y).unwrap();
            let top = heightmap.get(x, y - 1).unwrap_or(i32::MAX);
            let bot = heightmap.get(x, y + 1).unwrap_or(i32::MAX);
            let left = heightmap.get(x - 1, y).unwrap_or(i32::MAX);
            let right = heightmap.get(x + 1, y).unwrap_or(i32::MAX);
            if top > val && bot > val && left > val && right > val {
                println!("low x {}, y {} {}", x, y, val);
                score += val + 1;
                seeds.push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    println!("score {}", score);

    let mut basin_scores = Vec::new();
    for seed in seeds {
        let mut basin_score = 0;
        let mut stack = vec![seed];
        loop {
            let p = stack.pop();
            match p {
                Some(p) => {
                    basin_score += 1;
                    heightmap.set(p.x, p.y);

                    heightmap.get_candidates(&mut stack, p.x, p.y - 1);
                    heightmap.get_candidates(&mut stack, p.x, p.y + 1);
                    heightmap.get_candidates(&mut stack, p.x - 1, p.y);
                    heightmap.get_candidates(&mut stack, p.x + 1, p.y);
                }
                None => {
                    basin_scores.push(basin_score);
                    break;
                }
            }
        }
    }

    basin_scores.sort_by(|a, b| b.cmp(a));

    println!(
        "basins: {:?} mult {}",
        basin_scores,
        basin_scores.iter().take(3).fold(1, |a, v| a * v)
    );
}
