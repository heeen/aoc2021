use std::{collections::HashMap, fs};
fn main() {
    let contents = fs::read_to_string("day14/input_simple").expect("could not read input");
    let mut lines = contents.lines();
    let mut template = lines.next().unwrap().to_owned();
    lines.next();
    let patterns = lines
        .map(|l| l.split_once(" -> ").unwrap())
        .collect::<Vec<_>>();
    println!("template: {}", template);
    println!("patterns: {:?}", patterns);

    for step in 0..40 {
        let mut insertions = Vec::new();
        for (pattern, insertion) in &patterns {
            let mut gpos = 0;
            loop {
                if let Some(pos) = &template[gpos..].find(pattern) {
                    insertions.push((gpos + pos + 1, insertion));
                    gpos += pos + 1;
                } else {
                    break;
                }
            }
        }
        insertions.sort_by(|a, b| a.0.cmp(&b.0));
        //println!("insertions {:?}", insertions);
        let mut result = String::new();
        let mut start = 0usize;
        for insertion in insertions {
            /*
            println!(
                "> {} + {} + {}",
                result,
                &template[start..insertion.0],
                insertion.1
            );
            */
            result = result + &template[start..insertion.0] + insertion.1;
            start = insertion.0;
        }
        result = result + &template[start..];
        //println!("step {} result: {}", step, result);
        template = result;
    }

    let frequencies = template.chars().fold(HashMap::new(), |mut a, c| {
        *a.entry(c).or_default() += 1;
        a
    });

    let mut min = ('\0', usize::MAX);
    let mut max = ('\0', usize::MIN);
    for e in frequencies {
        if e.1 > max.1 {
            max = e;
        }
        if e.1 < min.1 {
            min = e;
        }
    }
    println!("min: {:?} max {:?} diff {}", min, max, max.1 - min.1);
}
