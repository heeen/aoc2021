use std::collections::{HashMap, HashSet};

use itertools::Itertools;
struct DeterministicDie {
    roll_count: u32,
}

trait Die {
    fn roll(&mut self) -> u32;
    fn roll_count(&self) -> u32;
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> u32 {
        let ret = (self.roll_count % 100) + 1;
        self.roll_count += 1;
        ret
    }

    fn roll_count(&self) -> u32 {
        self.roll_count
    }
}

impl DeterministicDie {
    pub(crate) fn new() -> Self {
        DeterministicDie { roll_count: 0 }
    }
}

fn deterministic_game(mut player1_pos: u32, mut player2_pos: u32) {
    let mut player1_score = 0u32;
    let mut player2_score = 0u32;
    let mut die = DeterministicDie::new();
    while player1_score < 1000 && player2_score < 1000 {
        let rolls = (0..3).map(|i| die.roll()).collect_vec();
        player1_pos = (player1_pos + rolls.iter().sum::<u32>()) % 10;
        player1_score += player1_pos + 1;
        println!(
            "player 1 rolled {rolls:?} and moves to {} for a score of {player1_score}",
            player1_pos + 1
        );
        if player1_score >= 1000 {
            break;
        }
        let rolls = (0..3).map(|i| die.roll()).collect_vec();
        player2_pos = (player2_pos + rolls.iter().sum::<u32>()) % 10;
        player2_score += player2_pos + 1;
        println!(
            "player 2 rolled {rolls:?} and moves to {} for a score of {player2_score}",
            player2_pos + 1
        );
    }
    println!(
        "product: {}",
        die.roll_count() * player1_score.min(player2_score)
    );
}

fn main() {
    let simple = false;
    let mut player1_pos = 1u32 - 1;
    let mut player2_pos = 3u32 - 1;
    if simple {
        player1_pos = 4u32 - 1;
        player2_pos = 8u32 - 1;
    }
    deterministic_game(player1_pos, player2_pos);
    quantum_game(player1_pos, player2_pos);
}

#[derive(Debug, Clone, Copy)]
struct PlayerState {
    pos: u32,
    score: u32,
    choices: u64,
}

fn quantum_rolls(state: &PlayerState, out: &mut Vec<PlayerState>) {
    out.append(
        &mut [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
            .iter()
            .map(|(offset, choices)| {
                let pos = (state.pos + offset) % 10;
                PlayerState {
                    pos,
                    choices: state.choices * choices,
                    score: state.score + pos + 1,
                }
            })
            .collect(),
    );
}

fn win_table(start: u32) -> HashMap<i32, Vec<PlayerState>> {
    let mut table = HashMap::new();
    let mut workqueue = vec![PlayerState {
        pos: start,
        score: 0,
        choices: 1,
    }];
    let mut round = 0;
    while !workqueue.is_empty() {
        println!("--- round {round} ---");
        let (wins, cont) = workqueue
            .iter()
            .fold(Vec::new(), |mut a, s| {
                quantum_rolls(s, &mut a);
                a
            })
            .iter()
            .partition(|e| e.score >= 21);
        workqueue = cont;

        for s in &wins {
            println!("win pos {} score {} choices {}", s.pos, s.score, s.choices)
        }
        table.insert(round, wins);
        for s in &workqueue {
            println!("pos {} score {} choices {}", s.pos, s.score, s.choices)
        }
        round += 1;
    }
   table
}
fn quantum_game(player1_pos: u32, player2_pos: u32) {
    let p1_wins = win_table(player1_pos);
    let p2_wins = win_table(player2_pos);

}

/*
sum combinations
3 	1
4 	3
5 	6
6 	7
7 	6
8 	3
9 	1
-----
    27
*/
