use std::{collections::HashMap, fs};

fn substitute<'a>(
    input: &'a str,
    patterns: &[(&str, &str)],
    depth: usize,
    cache: &'a mut HashMap<(String, usize), HashMap<char, usize>>,
) -> HashMap<char, usize> {
    if let Some(entry) = cache.get(&(input.to_string(), depth)) {
        return entry.clone();
    }

    let mut result_counts = HashMap::new();
    for (pattern, insertion) in patterns {
        if input == *pattern {
            let result = String::new() + &input[0..1] + *insertion + &input[1..];
            *result_counts
                .entry(insertion.chars().next().unwrap())
                .or_default() += 1;
            if depth != 0 {
                merge(
                    &mut result_counts,
                    substitute(&result[0..2], patterns, depth - 1, cache),
                );
                merge(
                    &mut result_counts,
                    substitute(&result[1..3], patterns, depth - 1, cache),
                );
            }

            break;
        }
    }
    cache.insert((input.to_string(), depth), result_counts.clone());
    result_counts
}

fn merge(a: &mut HashMap<char, usize>, b: HashMap<char, usize>) {
    for (ch, cnt) in b {
        *a.entry(ch).or_default() += cnt;
    }
}

fn main() {
    let contents = fs::read_to_string("day14/input").expect("could not read input");
    let mut lines = contents.lines();
    let template = lines.next().unwrap().to_owned();
    lines.next();
    let patterns = lines
        .map(|l| l.split_once(" -> ").unwrap())
        .collect::<Vec<_>>();
    println!("template: {}", template);
    println!("patterns: {:?}", patterns);

    let depth = 40;
    let mut counts = template.chars().fold(HashMap::new(), |mut a, c| {
        *a.entry(c).or_default() += 1;
        a
    });
    let mut cache = HashMap::new();
    for i in 0..template.len() - 1 {
        merge(
            &mut counts,
            substitute(&template[i..i + 2], &patterns, depth - 1, &mut cache),
        );
    }
    println!("counts {:?}", counts);

    let mut min = ('\0', usize::MAX);
    let mut max = ('\0', usize::MIN);
    for e in counts {
        if e.1 > max.1 {
            max = e;
        }
        if e.1 < min.1 {
            min = e;
        }
    }
    println!("min: {:?} max {:?} diff {}", min, max, max.1 - min.1);
}
