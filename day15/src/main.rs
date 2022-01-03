use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct PointQueueEntry {
    pos: Point,
    cost_sum: u32,
}

#[derive(Debug)]
struct PointCostEntry {
    via: Point,
    cost_sum: u32,
}

impl Ord for PointQueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost_sum.cmp(&self.cost_sum)
    }
}
impl PartialEq for PointQueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cost_sum == other.cost_sum
    }
}
impl PartialOrd for PointQueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost_sum.partial_cmp(&self.cost_sum)
    }
}

impl Eq for PointQueueEntry {}
trait Grid<T> {
    fn getpos(&self, pos: Point) -> Option<T>;
}

impl Grid<u32> for Vec<Vec<u32>> {
    fn getpos(&self, pos: Point) -> Option<u32> {
        let repeat_x = 5;
        let repeat_y = 5;
        let height = self.len() * repeat_y;
        let width = self[0].len() * repeat_x;
        if pos.x < 0 || pos.y < 0 || pos.x as usize >= width || pos.y as usize >= height {
            None
        } else {
            let tile_x: u32 = pos.x as u32 / self[0].len() as u32;
            let tile_y: u32 = pos.y as u32 / self.len() as u32;
            let cost = (self[pos.y as usize % self.len()][pos.x as usize % self[0].len()]
                + tile_x
                + tile_y
                - 1)
                % 9
                + 1;
            Some(cost)
        }
    }
}
fn test_move(
    input: &dyn Grid<u32>,
    known_costs: &mut HashMap<Point, PointCostEntry>,
    cur: &PointQueueEntry,
    to: Point,
    work_queue: &mut BinaryHeap<PointQueueEntry>,
) {
    if let Some(enter_cost) = input.getpos(to) {
        let cost_sum = cur.cost_sum + enter_cost;
        let known_cost = known_costs.get(&to);
        if known_cost.is_none() || known_cost.unwrap().cost_sum > cost_sum {
            known_costs.insert(
                to,
                PointCostEntry {
                    cost_sum,
                    via: cur.pos,
                },
            );
            work_queue.push(PointQueueEntry { cost_sum, pos: to });
        }
    }
}

fn main() {
    let contents = fs::read_to_string("day15/input").expect("could not read input");
    let input: Vec<Vec<u32>> = contents
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let start = Point { x: 0, y: 0 };

    let dest = Point {
        y: input.len() as i32 * 5 - 1,
        x: input[0].len() as i32 * 5 - 1,
    };

    println!("dest={:?}", dest);
    let mut known_costs = HashMap::new();
    let mut work_queue = BinaryHeap::new();
    work_queue.push(PointQueueEntry {
        pos: start,
        cost_sum: 0,
    });
    known_costs.insert(
        start,
        PointCostEntry {
            cost_sum: 0,
            via: start,
        },
    );
    loop {
        if let Some(point) = work_queue.pop() {
            let pos = point.pos;
            //println!("\npopped {:?}", point);
            //println!("workqueue {:?}", work_queue);
            if pos == dest {
                println!("reached with cost {:?}", point);
                break;
            }
            let left = Point {
                x: pos.x - 1,
                y: pos.y,
            };
            let right = Point {
                x: pos.x + 1,
                y: pos.y,
            };
            let up = Point {
                x: pos.x,
                y: pos.y - 1,
            };
            let down = Point {
                x: pos.x,
                y: pos.y + 1,
            };
            test_move(&input, &mut known_costs, &point, left, &mut work_queue);
            test_move(&input, &mut known_costs, &point, right, &mut work_queue);
            test_move(&input, &mut known_costs, &point, up, &mut work_queue);
            test_move(&input, &mut known_costs, &point, down, &mut work_queue);
        } else {
            println!("empty");
            break;
        }
        /*
        for y in 0..input.len() * 5 {
            for x in 0..input[0].len() * 5 {
                print!(
                    "{:3} ",
                    known_costs
                        .get(&Point {
                            x: x as i32,
                            y: y as i32
                        })
                        .unwrap_or(&999)
                )
            }
            println!("");
        }
        */
    }
/*
    let mut path_set = HashSet::new();
    println!("end");
    let mut node = known_costs.get(&dest).unwrap();
        println!("{:?}", node);
    while node.via != start {
        node = known_costs.get(&node.via).unwrap();
        println!("{:?}", node);
        path_set.insert(node.via);
    }
    for y in 0..input.len() * 5 {
        for x in 0..input[0].len() * 5 {
            if path_set.contains(&Point {
                x: x as i32,
                y: y as i32,
            }) {
                print!(
                    "[{}]",
                    input
                        .getpos(Point {
                            x: x as i32,
                            y: y as i32
                        })
                        .unwrap_or(0)
                )
            } else {
                print!(
                    " {} ",
                    input
                        .getpos(Point {
                            x: x as i32,
                            y: y as i32
                        })
                        .unwrap_or(0)
                )
            }
        }
        println!("");
    }
    */
}
