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
    let (p1_wins, p2_wins) = quantum_game(player1_pos, player2_pos);
    if simple {
        assert_eq!(p1_wins, 444356092776315);
        assert_eq!(p2_wins, 341960390180808);
    }
    println!("more wins: {}", p1_wins.max(p2_wins));
}

#[derive(Debug, Clone, Copy)]
struct PlayerState {
    pos: u32,
    score: u32,
    choices: u64,
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

fn win_table(start: u32, first: bool) -> HashMap<i32, (Vec<PlayerState>, Vec<PlayerState>)> {
    let mut table = HashMap::new();
    let mut workqueue = vec![PlayerState {
        pos: start,
        score: 0,
        choices: if first { 1 } else { 27 },
    }];
    let mut round = 0;
    while !workqueue.is_empty() {
        println!("--- round {round} ---");
        let (wins, mut cont): (Vec<PlayerState>, Vec<PlayerState>) = workqueue
            .iter()
            .fold(Vec::new(), |mut a, s| {
                quantum_rolls(s, &mut a);
                a
            })
            .iter()
            .partition(|e| e.score >= 21);

        for s in &wins {
            println!("win pos {} score {} choices {}", s.pos, s.score, s.choices)
        }
        table.insert(round, (wins, cont.clone()));

        for s in &mut cont {
            println!("pos {} score {} choices {}", s.pos, s.score, s.choices);
        }
        workqueue = cont;
        round += 1;
    }
    table
}
fn quantum_game(player1_pos: u32, player2_pos: u32) -> (u64, u64) {
    let p1_results = win_table(player1_pos, true);
    let p2_results = win_table(player2_pos, false);
    let mut p1_win_sum = 0;
    let mut p2_win_sum = 0;

    let mut p1_loss_choices = 1;
    let mut p2_loss_choices = 1;
    for i in 0..p1_results.len() as i32 {
        let (p1_wins, p1_losses) = &p1_results[&i];
        let (p2_wins, p2_losses) = &p2_results[&i];
        let p1_win_choices = p1_wins.iter().map(|r| r.choices).sum::<u64>();

        p1_win_sum += p2_loss_choices * p1_win_choices;
        p1_loss_choices = p1_losses.iter().map(|r| r.choices).sum::<u64>();
        let p2_win_choices = p2_wins.iter().map(|r| r.choices).sum::<u64>();
        p2_win_sum += p1_loss_choices * p2_win_choices;
        p2_loss_choices = p2_losses.iter().map(|r| r.choices).sum::<u64>();

        println!(
            "{} p1 w{}/l{} p2 w{}/l{} p1 wins {} / p2 wins {}",
            i,
            p1_win_choices,
            p1_loss_choices,
            p2_win_choices,
            p2_loss_choices,
            p1_win_sum,
            p2_win_sum
        );
    }

    (p1_win_sum / 27, p2_win_sum / 27)
}
