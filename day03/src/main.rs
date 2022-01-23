use core::slice::Iter;
use std::fs;

fn main() {
    let contents = fs::read_to_string("day03/input").expect("could not read input");
    let values: Vec<_> = contents
        .lines()
        .map(|l| u64::from_str_radix(l, 2).unwrap())
        .collect();

    let width = contents.lines().last().unwrap().len();

    let ones = count_bits(values.iter(), width);

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    let mut bit = 1 << (width - 1);
    for one in ones {
        if one * 2 > values.len() {
            gamma += bit;
        } else {
            epsilon += bit;
        }
        bit >>= 1;
    }
    println!("day 3 part 1: {gamma} {epsilon} {}", gamma * epsilon);

    let mut oxygen = 0;
    let mut filtered_o2 = values.clone();
    let mut bit = 1 << (width - 1);
    for i in 0..width {
        let ones = count_bits(filtered_o2.iter(), width);

        let more_ones = ones[i] * 2 >= filtered_o2.len();
        filtered_o2 = filtered_o2
            .into_iter()
            .filter(|l| more_ones == (l & bit == bit))
            .collect();
        if filtered_o2.len() == 1 {
            oxygen = filtered_o2[0];
            break;
        }
        bit >>= 1;
    }

    let mut scrubber = 0;
    let mut filtered_scrubber = values;
    let mut bit = 1 << (width - 1);
    for i in 0..width {
        let ones = count_bits(filtered_scrubber.iter(), width);
        let more_ones = ones[i] * 2 >= filtered_scrubber.len();
        filtered_scrubber = filtered_scrubber
            .into_iter()
            .filter(|l| more_ones != (l & bit == bit))
            .collect();
        if filtered_scrubber.len() == 1 {
            scrubber = filtered_scrubber[0];
            break;
        }
        bit >>= 1;
    }

    println!(
        "day 3 part 1: {oxygen} {scrubber} {}",
        oxygen * scrubber
    );
}

fn count_bits(values: Iter<u64>, width: usize) -> Vec<usize> {
    values.fold(vec![0usize; width], |mut accum, l| {
        let mut bit = 1 << (width - 1);
        for a in accum.iter_mut() {
            if l & bit == bit {
                *a += 1;
            }
            bit >>= 1;
        }
        accum
    })
}
