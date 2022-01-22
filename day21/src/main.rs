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
fn main() {
    let simple = false;
    let mut player1_pos = 1u32 - 1;
    let mut player2_pos = 3u32 - 1;
    if simple {
        player1_pos = 4u32 - 1;
        player2_pos = 8u32 - 1;
    }
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
impl DeterministicDie {
    pub(crate) fn new() -> Self {
        DeterministicDie { roll_count: 0 }
    }
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