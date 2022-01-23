use std::fs;

fn main() {
    let contents = fs::read_to_string("day06/input").expect("could not read input");
    let ages: Vec<_> = contents
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    println!("input ages {:?}", ages);


    let mut age_bins = vec![0u64; 9];
    for age in ages {
        age_bins[age as usize] += 1;
    }
    println!("age counts {:?}", age_bins);
    for day in 0..257 {
        let mut new_bins = vec![0u64; 9];
        new_bins[..8].clone_from_slice(&age_bins[1..9]);
        new_bins[6] += age_bins[0];
        new_bins[8] = age_bins[0];
        println!("day {day} age counts {:?} new counts {:?} sum {}", age_bins, new_bins, age_bins.iter().sum::<u64>());
        age_bins = new_bins;
    }
}
