use enum_iterator::IntoEnumIterator;
use std::{array::IntoIter, error::Error, fmt::Display, fs};

use itertools::Itertools;

#[derive(Debug, IntoEnumIterator, PartialEq, Clone, Copy)]

enum PodType {
    Amber,
    Bronce,
    Copper,
    Desert,
}

type Pos = (usize, usize);
#[derive(Debug, PartialEq, Clone, Copy)]
struct PodState {
    position: Pos,
    podtype: PodType,
}

impl PodState {
    fn move_cost(&self) -> u64 {
        match self.podtype {
            PodType::Amber => 1,
            PodType::Bronce => 10,
            PodType::Copper => 100,
            PodType::Desert => 1000,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
struct State {
    pods: [PodState; 8],
    map: Vec<Vec<char>>,
    cost: u64,
    transient: Option<usize>,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pod in self.pods {
            writeln!(f, "{:?} {},{}", pod.podtype, pod.position.0, pod.position.1)?;
        }
        for line in &self.map {
            for c in line {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "cost: {} transient: {:?}", self.cost, self.transient)?;
        Ok(())
    }
}
impl State {
    fn from_data(data: Vec<Vec<char>>) -> Self {
        let mut pods = [PodState {
            podtype: PodType::Amber,
            position: (0, 0),
        }; 8];
        for y in 0..2 {
            for x in 0..4 {
                let xp = 3 + 2 * x;
                let yp = 2 + y;
                let c = data[yp][xp];
                pods[y * 4 + x] = PodState {
                    podtype: match c {
                        'A' => PodType::Amber,
                        'B' => PodType::Bronce,
                        'C' => PodType::Copper,
                        'D' => PodType::Desert,
                        _ => panic!("unexpected char {c} @ {xp} {yp}"),
                    },
                    position: (xp, yp),
                };
            }
        }
        println!("pods: {:?}", pods);
        State {
            pods,
            map: data.clone(),
            cost: 0,
            transient: None,
        }
    }
    fn fork_move(&self, pod_index: usize, pod_target: Pos, cost: u64) -> Self {
        let mut pods = self.pods.clone();
        let mut map = self.map.clone();
        let mut pod = &mut pods[pod_index];
        map[pod.position.1][pod.position.0] = '.';
        map[pod_target.1][pod_target.0] = match pod.podtype {
            PodType::Amber => 'A',
            PodType::Bronce => 'B',
            PodType::Copper => 'C',
            PodType::Desert => 'D',
        };
        pod.position = pod_target;

        State {
            pods,
            map,
            cost: self.cost + cost,
            transient: if State::is_forbidden(pod_target) {
                Some(pod_index)
            } else {
                None
            },
        }
    }

    fn is_forbidden(pos: Pos) -> bool {
        match pos {
            (3,1) | (5,1) | (7,1) | (9,1) => true,
            _ => false,
        }
    }

    fn room_column(col: usize) -> Option<PodType> {
        match col {
            3 => Some(PodType::Amber),
            5 => Some(PodType::Bronce),
            7 => Some(PodType::Copper),
            9 => Some(PodType::Desert),
            _ => None
        }
    }
    fn pod_moves<'a>(&'a self, pod_index: usize) -> Vec<((usize, usize), u64)> {
        let pod = self.pods[pod_index];
        [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(|delta| {
                let x = (pod.position.0 as isize + delta.0) as usize;
                let y = (pod.position.1 as isize + delta.1) as usize;
                if pod.position.y == 1 {
                    match pod.position.x {
                        3 =>
                    }
                } else {
                    match self.map[y][x] {
                        '.' => Some(((x, y), pod.move_cost())),
                        _ => None,
                    }
                }
            })
            .collect_vec()
    }

    fn possible_states(&self) -> Vec<State> {
        if let Some(pod_index) = self.transient {
            self.pod_moves(pod_index)
                .iter()
                .map(|m| self.fork_move(pod_index, m.0, m.1))
                .collect_vec()
        } else {
            (0..8)
                .flat_map(|pi| {
                    self.pod_moves(pi)
                        .iter()
                        .map(|m| self.fork_move(pi, m.0, m.1))
                        .collect_vec()
                })
                .collect_vec()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("day23/input_simple")?
        .lines()
        .map(|s| s.chars().collect_vec())
        .collect_vec();
    let initial = State::from_data(content);
    let mut work_queue = initial.possible_states();
    while let Some(w) = work_queue.pop() {
        println!("{}", w);
        let mut new = w.possible_states();
        work_queue.append(&mut new);
    }
    Ok(())
}
