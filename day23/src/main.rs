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

impl PodState {}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Room {
    Empty,
    Bottom(PodType),
    Two(PodType, PodType),
}

impl Room {
    fn move_out(&mut self) -> Option<(PodType, u64)> {
        match *self {
            Room::Empty => None,
            Room::Bottom(pod) => {
                *self = Room::Empty;
                Some((pod, 3))
            }
            Room::Two(bottom, top) => {
                *self = Room::Bottom(bottom);
                Some((top, 2))
            }
        }
    }

    fn move_in(&mut self, pod: PodType) -> Option<u64> {
        match self {
            Room::Empty => {
                *self = Room::Bottom(pod);
                Some(3)
            }
            Room::Bottom(bottom) => {
                *self = Room::Two(pod, *bottom);
                Some(2)
            }
            Room::Two(_, _) => None,
        }
    }
}

enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
struct State {
    rooms: [Room; 4],
    // 01 2 3 4 56
    //   A B C D
    spots: [Option<PodType>; 7],
    cost: u64,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for spot in self.spots {
            match spot {
                Some(PodType::Amber) => write!(f, "A"),
                Some(PodType::Bronce) => write!(f, "B"),
                Some(PodType::Copper) => write!(f, "C"),
                Some(PodType::Desert) => write!(f, "D"),
                None => write!(f, "."),
            };
        }
        write!(f, "\n  ");
        for room in self.rooms {
            match room {
                Room::Bottom(_) | Room::Empty => write!(f, ". "),
                Room::Two(_, PodType::Amber) => write!(f, "A "),
                Room::Two(_, PodType::Bronce) => write!(f, "B "),
                Room::Two(_, PodType::Copper) => write!(f, "C "),
                Room::Two(_, PodType::Desert) => write!(f, "D "),
            };
        }
        write!(f, "\n  ");
        for room in self.rooms {
            match room {
                Room::Bottom(PodType::Amber) | Room::Two(PodType::Amber, _) => write!(f, "A "),
                Room::Bottom(PodType::Bronce) | Room::Two(PodType::Bronce, _) => write!(f, "B "),
                Room::Bottom(PodType::Copper) | Room::Two(PodType::Copper, _) => write!(f, "C "),
                Room::Bottom(PodType::Desert) | Room::Two(PodType::Desert, _) => write!(f, "D "),
                Room::Empty => write!(f, "."),
            };
        }
        Ok(())
    }
}

impl State {
    fn room(&mut self, room: PodType) -> &mut Room {
        match room {
            PodType::Amber => &mut self.rooms[0],
            PodType::Bronce => &mut self.rooms[1],
            PodType::Copper => &mut self.rooms[2],
            PodType::Desert => &mut self.rooms[3],
        }
    }

    fn room_exit_destinations(&self, room: PodType) -> (usize, usize) {
        match room {
            PodType::Amber => (1, 2),
            PodType::Bronce => (2, 3),
            PodType::Copper => (3, 4),
            PodType::Desert => (5, 6),
        }
    }

    fn move_from_room(&mut self, room_type: PodType, direction: Direction) {
        let destinations = self.room_exit_destinations(room_type);
        let destination = match direction {
            Direction::Left => &mut self.spots[destinations.0],
            Direction::Right => &mut self.spots[destinations.1],
        };
        let room = self.room(room_type);
        if let Some((pod, cost)) = room.move_out() {
            *destination = Some(pod);
            self.cost += cost * pod.move_cost();
        }
    }

    fn move_from_spot(&mut self, from_index: usize, direction: Direction) {
        let spot = &mut self.spots[from_index];
        let to = match direction {
            Direction::Left => &mut self.spots[from_index - 1],
            Direction::Right => &mut self.spots[from_index + 1],
        };
        *to = *spot;
        *spot = None;
    }

    fn from_data(data: Vec<Vec<char>>) -> Self {
        let mut pods = [PodType::Amber; 8];
        for y in 0..=1 {
            for x in 0..4 {
                let xp = 3 + 2 * x;
                let yp = 2 + y;
                let c = data[yp][xp];
                pods[y * 4 + x] = match c {
                    'A' => PodType::Amber,
                    'B' => PodType::Bronce,
                    'C' => PodType::Copper,
                    'D' => PodType::Desert,
                    _ => panic!("unexpected char {c} @ {xp} {yp}"),
                };
            }
        }
        println!("pods: {:?}", pods);
        State {
            rooms: [
                Room::Two(pods[0], pods[4]),
                Room::Two(pods[1], pods[5]),
                Room::Two(pods[2], pods[6]),
                Room::Two(pods[3], pods[7]),
            ],
            spots: [None; 7],
            cost: 0,
        }
    }
    fn room_column(col: usize) -> Option<PodType> {
        match col {
            3 => Some(PodType::Amber),
            5 => Some(PodType::Bronce),
            7 => Some(PodType::Copper),
            9 => Some(PodType::Desert),
            _ => None,
        }
    }
    fn possible_states(&self) -> Vec<State> {
        vec![]
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
impl PodType {
    fn move_cost(&self) -> u64 {
        match self {
            PodType::Amber => 1,
            PodType::Bronce => 10,
            PodType::Copper => 100,
            PodType::Desert => 1000,
        }
    }
}
