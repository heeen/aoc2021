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

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Room {
    Empty,
    Bottom(PodType),
    BottomTop(PodType, PodType),
}

impl Room {
    fn move_out(&mut self) -> Option<(PodType, u64)> {
        match *self {
            Room::Empty => None,
            Room::Bottom(pod) => {
                *self = Room::Empty;
                Some((pod, 2))
            }
            Room::BottomTop(bottom, top) => {
                *self = Room::Bottom(bottom);
                Some((top, 1))
            }
        }
    }

    fn move_in(&mut self, pod: PodType) -> Option<u64> {
        match self {
            Room::Empty => {
                *self = Room::Bottom(pod);
                Some(2)
            }
            Room::Bottom(bottom) => {
                *self = Room::BottomTop(*bottom, pod);
                Some(1)
            }
            Room::BottomTop(_, _) => None,
        }
    }

    fn count(&self) -> usize {
        match self {
            Room::Empty => 0,
            Room::Bottom(_) => 1,
            Room::BottomTop(_, _) => 2,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    rooms: [Room; 4],
    spots: [Option<PodType>; 11],
    cost: u64,
    estimation: u64,
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
        for room in self.rooms {
            match room {
                Room::Bottom(_) | Room::Empty => write!(f, ". "),
                Room::BottomTop(_, PodType::Amber) => write!(f, "A "),
                Room::BottomTop(_, PodType::Bronce) => write!(f, "B "),
                Room::BottomTop(_, PodType::Copper) => write!(f, "C "),
                Room::BottomTop(_, PodType::Desert) => write!(f, "D "),
            }?;
        }
        write!(f, "\n  ")?;
        for room in self.rooms {
            match room {
                Room::Bottom(PodType::Amber) | Room::BottomTop(PodType::Amber, _) => {
                    write!(f, "A ")
                }
                Room::Bottom(PodType::Bronce) | Room::BottomTop(PodType::Bronce, _) => {
                    write!(f, "B ")
                }
                Room::Bottom(PodType::Copper) | Room::BottomTop(PodType::Copper, _) => {
                    write!(f, "C ")
                }
                Room::Bottom(PodType::Desert) | Room::BottomTop(PodType::Desert, _) => {
                    write!(f, "D ")
                }
                Room::Empty => write!(f, ". "),
            }?;
        }
        writeln!(f, "cost {} estimation {}", self.cost, self.estimation)?;
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
    fn room(&mut self, room: PodType) -> &mut Room {
        &mut self.rooms[room.room_index()]
    }

    fn path_clear_to_spot(&self, room: PodType, spot_index: usize) -> Option<u64> {
        let room_spot = 2 + 2 * room.room_index();
        let (range,distance) = if spot_index < room_spot {
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
        let mut ret = self.clone();
        let room = *ret.room(room_type);

        if room == Room::Bottom(room_type) || room == Room::BottomTop(room_type, room_type) {
            return Err(());
        }
        let path_cost = ret.path_clear_to_spot(room_type, spot_index);
        if path_cost.is_none() {
            return Err(());
        }
        let candidate = ret.room(room_type).move_out();

        if let Some((pod, room_cost)) = candidate {
            ret.spots[spot_index] = Some(pod);
            ret.cost += (room_cost + path_cost.unwrap()) * pod.move_cost();
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
        if (path_cost.is_none()) {
            return Err(());
        }
        let mut ret = self.clone();
        let room_cost = ret.room(room_type).move_in(room_type);
        match (room_cost) {
            (Some(rcost)) => {
                ret.spots[spot_index] = None;
                ret.cost += (path_cost.unwrap() + rcost) * room_type.move_cost();
                //ret.estimate();
                Ok(ret)
            }
            _ => Err(()),
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
        let mut ret = State {
            estimation: 0,
            rooms: [
                Room::BottomTop(pods[4], pods[0]),
                Room::BottomTop(pods[5], pods[1]),
                Room::BottomTop(pods[6], pods[2]),
                Room::BottomTop(pods[7], pods[3]),
            ],
            spots: [None; 11],
            cost: 0,
        };
        //ret.estimate();
        ret
    }

    fn solved_state() -> Self {
        State {
            rooms: [
                Room::BottomTop(PodType::Amber, PodType::Amber),
                Room::BottomTop(PodType::Bronce, PodType::Bronce),
                Room::BottomTop(PodType::Copper, PodType::Copper),
                Room::BottomTop(PodType::Desert, PodType::Desert),
            ],
            spots: [None; 11],
            cost: 0,
            estimation: 0,
        }
    }

    fn estimate(&mut self) {
        self.estimation = 0;

        for (i, spot) in self.spots.iter().enumerate() {
            if let Some(t) = spot {
                let distance = 2 + (t.room_index() as i32 - i as i32).abs() as u64;
                self.estimation += t.move_cost() * distance
            }
        }
        for (i, room) in self.rooms.iter().enumerate() {
            let room_type = PodType::from_index(i);
            self.estimation += match room {
                Room::Empty => 0,
                Room::BottomTop(bot, top) if *bot == room_type && *top == room_type => 0,
                Room::Bottom(bot) if *bot == room_type => 0,
                Room::Bottom(bot) => {
                    bot.move_cost() * (4 + (i as i32 - bot.room_index() as i32).abs()) as u64
                }
                Room::BottomTop(bot, top) if *bot == room_type => {
                    top.move_cost() * (2 + (i as i32 - top.room_index() as i32).abs()) as u64
                }
                Room::BottomTop(bot, top) => {
                    bot.move_cost() * (4 + (i as i32 - bot.room_index() as i32).abs()) as u64
                        + top.move_cost() * (2 + (i as i32 - top.room_index() as i32).abs()) as u64
                }
            }
        }
    }
    fn sanity_check(&self) {
        let spots = self.spots.iter().filter_map(|s| s.as_ref()).count();
        let rooms = self.rooms.iter().map(|r| r.count()).sum::<usize>();
        assert_eq!(8, spots + rooms);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("day23/input")?
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
    //    known_costs.insert(WorkQueueEntry { state: initial }, 0);

    while let Some(w) = work_queue.pop() {
        let known_cost = known_costs.get(&w);
        let w = w.state;
        if known_cost.is_none() || known_cost.unwrap() > &w.cost {
//            println!("popped:\n{} {:?}", w, known_cost);
            known_costs.insert(WorkQueueEntry { state: w.clone() }, w.cost);
            let new = w.generate_moves();
//            println!("moves: {}", new.len());
            for state in new {
//                println!("{state}");
                work_queue.push(WorkQueueEntry { state });
            }
          //  break;
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

    fn from_index(i: usize) -> Self {
        match i {
            0 => PodType::Amber,
            1 => PodType::Bronce,
            2 => PodType::Copper,
            3 => PodType::Desert,
            _ => panic!("out of range"),
        }
    }
    fn room_exit_destinations(&self) -> (usize, usize) {
        match self {
            PodType::Amber => (1, 2),
            PodType::Bronce => (2, 3),
            PodType::Copper => (3, 4),
            PodType::Desert => (4, 5),
        }
    }
}
