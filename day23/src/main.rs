use enum_iterator::IntoEnumIterator;
use itertools::Itertools;
use std::hash::{Hash, Hasher};
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    error::Error,
    fmt::Display,
    fs,
};

#[derive(Debug, IntoEnumIterator, PartialEq, Clone, Copy, Hash)]
enum PodType {
    Amber,
    Bronce,
    Copper,
    Desert,
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
    fn room_index(&self) -> usize {
        match self {
            PodType::Amber => 0,
            PodType::Bronce => 1,
            PodType::Copper => 2,
            PodType::Desert => 3,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Room {
    Empty,
    Bottom(PodType),
    BottomSecond(PodType, PodType),
    BottomSecondThird(PodType, PodType, PodType),
    Full(PodType, PodType, PodType, PodType),
}

impl Room {
    fn move_out(&mut self) -> Option<(PodType, u64)> {
        match *self {
            Room::Empty => None,
            Room::Bottom(pod) => {
                *self = Room::Empty;
                Some((pod, 4))
            }
            Room::BottomSecond(bottom, top) => {
                *self = Room::Bottom(bottom);
                Some((top, 3))
            }
            Room::BottomSecondThird(bottom, second, third) => {
                *self = Room::BottomSecond(bottom, second);
                Some((third, 2))
            }
            Room::Full(bottom, second, third, fourth) => {
                *self = Room::BottomSecondThird(bottom, second, third);
                Some((fourth, 1))
            }
        }
    }

    fn move_in(&mut self, pod: PodType) -> Option<u64> {
        match self {
            Room::Empty => {
                *self = Room::Bottom(pod);
                Some(4)
            }
            Room::Bottom(bottom) => {
                *self = Room::BottomSecond(*bottom, pod);
                Some(3)
            }
            Room::BottomSecond(bottom, second) => {
                *self = Room::BottomSecondThird(*bottom, *second, pod);
                Some(2)
            }

            Room::BottomSecondThird(bottom, second, third) => {
                *self = Room::Full(*bottom, *second, *third, pod);
                Some(1)
            }
            Room::Full(_, _, _, _) => None,
        }
    }

    fn count(&self) -> usize {
        match self {
            Room::Empty => 0,
            Room::Bottom(_) => 1,
            Room::BottomSecond(_, _) => 2,
            Room::BottomSecondThird(_, _, _) => 3,
            Room::Full(_, _, _, _) => 4,
        }
    }

    fn array(&self) -> [Option<PodType>; 4] {
        match *self {
            Room::Empty => [None, None, None, None],
            Room::Bottom(bot) => [None, None, None, Some(bot)],
            Room::BottomSecond(bot, sec) => [None, None, Some(sec), Some(bot)],
            Room::BottomSecondThird(bot, sec, thrd) => [None, Some(thrd), Some(sec), Some(bot)],
            Room::Full(bot, sec, thrd, top) => [Some(top), Some(thrd), Some(sec), Some(bot)],
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    rooms: [Room; 4],
    spots: [Option<PodType>; 11],
    cost: u64,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, spot) in self.spots.iter().enumerate() {
            match spot {
                Some(PodType::Amber) => write!(f, "A"),
                Some(PodType::Bronce) => write!(f, "B"),
                Some(PodType::Copper) => write!(f, "C"),
                Some(PodType::Desert) => write!(f, "D"),
                None => write!(f, "."),
            }?;
        }
        write!(f, "\n  ")?;

        for i in 0..4 {
            for room in self.rooms {
                match room.array()[i] {
                    Some(PodType::Amber) => write!(f, "A "),
                    Some(PodType::Bronce) => write!(f, "B "),
                    Some(PodType::Copper) => write!(f, "C "),
                    Some(PodType::Desert) => write!(f, "D "),
                    None => write!(f, ". "),
                }?;
            }
            write!(f, "\n  ")?;
        }
        writeln!(f, "cost {} ", self.cost)?;
        Ok(())
    }
}

impl Ord for WorkQueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.state.cost.cmp(&self.state.cost)
    }
}

impl PartialOrd for WorkQueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.state.cost.partial_cmp(&self.state.cost)
    }
}

impl PartialEq for WorkQueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.state.rooms == other.state.rooms && self.state.spots == other.state.spots
    }
}

impl Eq for WorkQueueEntry {}
impl Hash for WorkQueueEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.rooms.hash(state);
        self.state.spots.hash(state);
    }
}

#[derive(Debug)]
struct WorkQueueEntry {
    state: State,
}

impl State {
    fn room_mut(&mut self, room: PodType) -> &mut Room {
        &mut self.rooms[room.room_index()]
    }

    fn room(&self, room: PodType) -> Room {
        self.rooms[room.room_index()]
    }

    fn path_clear_to_spot(&self, room: PodType, spot_index: usize) -> Option<u64> {
        let room_spot = 2 + 2 * room.room_index();
        let (range, distance) = if spot_index < room_spot {
            (spot_index..room_spot, room_spot - spot_index)
        } else {
            (room_spot..spot_index + 1, spot_index - room_spot)
        };

        if self.spots[range.clone()].iter().all(|s| s.is_none()) {
            Some(distance as u64)
        } else {
            None
        }
    }
    fn path_clear_to_room(&self, spot_index: usize, room: PodType) -> Option<u64> {
        let room_spot = 2 + 2 * room.room_index();
        let (range, distance) = if spot_index < room_spot {
            (spot_index + 1..room_spot, room_spot - spot_index)
        } else {
            (room_spot..spot_index, spot_index - room_spot)
        };

        if self.spots[range.clone()].iter().all(|s| s.is_none()) {
            Some(distance as u64)
        } else {
            None
        }
    }

    fn move_from_room(&self, room_type: PodType, spot_index: usize) -> Result<Self, ()> {
        let room = self.room(room_type);

        if room == Room::Bottom(room_type)
            || room == Room::BottomSecond(room_type, room_type)
            || room == Room::BottomSecondThird(room_type, room_type, room_type)
            || room == Room::Full(room_type, room_type, room_type, room_type)
        {
            return Err(());
        }
        let path_cost = self.path_clear_to_spot(room_type, spot_index);
        if path_cost.is_none() {
            return Err(());
        }
        let mut ret = self.clone();
        let candidate = ret.room_mut(room_type).move_out();

        if let Some((pod, room_cost)) = candidate {
            ret.spots[spot_index] = Some(pod);
            ret.cost += (room_cost + path_cost.unwrap()) * pod.move_cost();
            ret.sanity_check();
            Ok(ret)
        } else {
            Err(())
        }
    }

    fn move_into_room(&self, spot_index: usize, room_type: PodType) -> Result<Self, ()> {
        if self.spots[spot_index] != Some(room_type) {
            return Err(());
        }

        let path_cost = self.path_clear_to_room(spot_index, room_type);
        if path_cost.is_none() {
            return Err(());
        }
        let mut ret = self.clone();
        let room_cost = ret.room_mut(room_type).move_in(room_type);
        if let Some(rcost) = room_cost {
            ret.spots[spot_index] = None;
            ret.cost += (path_cost.unwrap() + rcost) * room_type.move_cost();
            ret.sanity_check();
            Ok(ret)
        } else {
            Err(())
        }
    }

    fn generate_moves(&self) -> Vec<Self> {
        let rooms_x_spots = [
            PodType::Amber,
            PodType::Bronce,
            PodType::Copper,
            PodType::Desert,
        ]
        .iter()
        .cartesian_product([0, 1, 3, 5, 7, 9, 10]);

        let from_rooms = rooms_x_spots
            .clone()
            .filter_map(|(room, spot)| self.move_from_room(*room, spot).ok());

        let into_rooms = rooms_x_spots
            .clone()
            .filter_map(|(room, spot)| self.move_into_room(spot, *room).ok());

        into_rooms.chain(from_rooms).collect()
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
        State {
            rooms: [
                Room::Full(pods[4], PodType::Desert, PodType::Desert, pods[0]),
                Room::Full(pods[5], PodType::Copper, PodType::Bronce, pods[1]),
                Room::Full(pods[6], PodType::Bronce, PodType::Amber, pods[2]),
                Room::Full(pods[7], PodType::Amber, PodType::Copper, pods[3]),
            ],
            spots: [None; 11],
            cost: 0,
        }
    }

    fn solved_state() -> Self {
        State {
            rooms: [
                Room::Full(
                    PodType::Amber,
                    PodType::Amber,
                    PodType::Amber,
                    PodType::Amber,
                ),
                Room::Full(
                    PodType::Bronce,
                    PodType::Bronce,
                    PodType::Bronce,
                    PodType::Bronce,
                ),
                Room::Full(
                    PodType::Copper,
                    PodType::Copper,
                    PodType::Copper,
                    PodType::Copper,
                ),
                Room::Full(
                    PodType::Desert,
                    PodType::Desert,
                    PodType::Desert,
                    PodType::Desert,
                ),
            ],
            spots: [None; 11],
            cost: 0,
        }
    }
    fn sanity_check(&self) {
        let spots = self.spots.iter().filter_map(|s| s.as_ref()).count();
        let rooms = self.rooms.iter().map(|r| r.count()).sum::<usize>();
        if spots+rooms != 16 {
            panic!("{spots}+{rooms} != 16\n{self}");
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("day23/input_simple")?
        .lines()
        .map(|s| s.chars().collect_vec())
        .collect_vec();
    let initial = State::from_data(content);

    println!("initial:\n{initial}");

    let mut known_costs = HashMap::new();
    let mut work_queue = BinaryHeap::new();

    work_queue.push(WorkQueueEntry {
        state: initial.clone(),
    });

    while let Some(w) = work_queue.pop() {
        let known_cost = known_costs.get(&w);
        let w = w.state;
        if known_cost.is_none() || known_cost.unwrap() > &w.cost {
            known_costs.insert(WorkQueueEntry { state: w.clone() }, w.cost);
            let new = w.generate_moves();
            for state in new {
                work_queue.push(WorkQueueEntry { state });
            }
        }
    }

    let solved = State::solved_state();
    println!(
        "------\n{}\ncost {:?}",
        solved,
        known_costs.get(&WorkQueueEntry {
            state: solved.clone()
        })
    );
    Ok(())
}
