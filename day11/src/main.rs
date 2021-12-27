use std::fs;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}
trait FlashPropagator {
    fn age_phase(&mut self, stack: &mut Vec<Point>);
    fn flash_phase(&mut self, stack: Vec<Point>) -> usize;
    fn dump(&self);
}

impl FlashPropagator for Vec<Vec<i32>> {
    fn age_phase(&mut self, stack: &mut Vec<Point>) {
        for (y, row) in self.iter_mut().enumerate() {
            for (x, fish) in row.iter_mut().enumerate() {
                *fish += 1;
                if *fish > 9 {
                    stack.push(Point { x, y });
                    *fish = 0;
                }
            }
        }
    }

    fn flash_phase(&mut self, stack: Vec<Point>) -> usize {
        let mut stack = stack;
        let mut counter = 0;
        loop {
            match stack.pop() {
                Some(p) => {
                    counter += 1;
                    for y in 0.max(p.y as i32 - 1) as usize..self.len().min(p.y + 2) {
                        let row = &mut self[y];
                        for x in 0.max(p.x as i32 - 1) as usize..row.len().min(p.x + 2) {
                            if row[x] == 0 {
                                continue;
                            }
                            row[x] += 1;
                            if row[x] > 9 {
                                row[x] = 0;
                                stack.push(Point { x, y })
                            }
                        }
                    }
                }
                None => break,
            }
        }
        counter
    }

    fn dump(&self) {
        for row in self {
            println!("{:?}", row);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("day11/input").expect("could not read input");
    let input: Vec<Vec<i32>> = contents
        .lines()
        .map(|l| l.chars().map(|c| c as i32 - '0' as i32).collect())
        .collect();

    let mut counter = 0usize;
    let mut workset = input.clone();
    for _cycle in 0..100 {
        let mut stack = Vec::new();
        workset.age_phase(&mut stack);
        counter += workset.flash_phase(stack);
    }
    println!("flashes {}", counter);

    let mut workset = input;
    let mut cycle = 0;
    loop {
        let mut stack = Vec::new();
        workset.age_phase(&mut stack);
        let count = workset.flash_phase(stack);
        cycle += 1;
        println!("stepw {} flashes {}", cycle, count);
        if count == workset.len() * workset[0].len() {
            break;
        }
    }
}
