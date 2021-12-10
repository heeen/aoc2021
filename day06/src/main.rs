use std::fs;

fn main() {
    let contents = fs::read_to_string("day06/input").expect("could not read input");
    let mut ages: Vec<_> = contents
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    println!("input ages {:?}", ages);
    for day in (0..80) {
        let mut new_spawns = 0;
        ages.iter_mut().for_each(|f| match (*f) {
            0 => {
                *f = 6;
                new_spawns += 1
            }
            _ => *f -= 1,
        });
        for i in 0..new_spawns {
            ages.push(8);
        }
    }
    println!("day 80 ages {:?}", ages.len());
}
