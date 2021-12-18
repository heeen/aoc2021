use std::collections::{HashMap, HashSet};
use std::fs;
#[derive(Debug)]
struct Node<'a> {
    links: Vec<&'a str>,
}

trait Visitor<'a> {
    fn traverse(&'a self, second_visit: bool) -> usize;
}
impl<'a> Visitor<'a> for HashMap<&'a str, Node<'a>> {
    fn traverse(&'a self, second_visit: bool) -> usize {
        let mut work_queue = vec![(vec!["start"], HashSet::from([("start")]), None)];
        let mut count = 0;

        loop {
            match work_queue.pop() {
                Some((current_path, current_visited, visited_twice)) => {
                    let node = *current_path.iter().last().unwrap();
                    if node == "end" {
                        println!(
                            "========> {} twice: {:?}",
                            current_path.join(","),
                            visited_twice
                        );
                        count += 1;
                        continue;
                    }
                    for link in &self[node].links {
                        if *link == "start" {
                            continue;
                        }
                        let mut new_visited_twice: Option<&str> = visited_twice;
                        if link.chars().nth(0).unwrap().is_lowercase() {
                            if current_visited.contains(link) {
                                if second_visit && visited_twice.is_none() {
                                    new_visited_twice = Some(link)
                                } else {
                                    continue;
                                }
                            }
                        }
                        let mut new_path = current_path.to_owned();
                        new_path.push(link);
                        let mut new_counts = current_visited.to_owned();
                        new_counts.insert(node);
                        work_queue.push((new_path, new_counts, new_visited_twice));
                    }
                }
                None => {
                    break;
                }
            }
        }

        count
    }
}
fn main() {
    let mut nodes = HashMap::new();
    let contents = fs::read_to_string("day12/input").expect("could not read input");
    for line in contents.lines() {
        let (from, to) = line.split_once('-').unwrap();
        let node = nodes.entry(from).or_insert(Node { links: Vec::new() });
        node.links.push(to);
        let node = nodes.entry(to).or_insert(Node { links: Vec::new() });
        node.links.push(from);
    }
    println!("{:?}", nodes);
    let count = nodes.traverse(false);
    println!("paths 1: {}", count);
    let count = nodes.traverse(true);
    println!("paths 2: {}", count);
}
