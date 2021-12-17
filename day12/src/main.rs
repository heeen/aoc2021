use std::collections::{HashMap, HashSet};
use std::fs;
#[derive(Debug)]
struct Node<'a> {
    links: Vec<&'a str>,
}

trait Visitor<'a> {
    fn traverse(&'a self, current_path: Vec<&'a str>) -> usize;
}
impl<'a> Visitor<'a> for HashMap<&'a str, Node<'a>> {
    fn traverse(&'a self, current_path: Vec<&'a str>) -> usize {
        let node = *current_path.iter().last().unwrap();
        if node == "end"  {
            println!("========> {:?}", current_path);
            return 1;
        }
        let mut count = 0;
        let links = &self[node].links;
        for link in links {
            if link.chars().nth(0).unwrap().is_lowercase() {
                if let Some(found) = current_path.iter().find(|n| *n == link) {
                    continue;
                }
            }
            let mut new_path = current_path.to_owned();
            new_path.push(link);
            count += self.traverse(new_path);
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
    let count = nodes.traverse(vec!["start"]);
    println!("paths: {}", count);

}
