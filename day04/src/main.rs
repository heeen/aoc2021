use std::fs;

#[derive(Copy, Clone, Debug)]
struct Field {
    pub value: i32,
    pub marked: bool,
}
#[derive(Debug)]
struct Bingoboard {
    pub fields: [[Field; 5]; 5],
    pub row_marked: [usize; 5],
    pub col_marked: [usize; 5],
    pub won: bool,
}

fn main() {
    let contents = fs::read_to_string("day04/input").expect("could not read input");
    let mut lines = contents.lines();
    let moves: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|e| e.parse::<i32>().unwrap())
        .collect();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        let mut board = Bingoboard {
            fields: [[Field {
                value: 0,
                marked: false,
            }; 5]; 5],
            row_marked: [0; 5],
            col_marked: [0; 5],
            won: false,
        };
        for y in 0..5 {
            let line = lines.next();
            let row = line.unwrap().split_ascii_whitespace().enumerate();
            for entry in row {
                board.fields[y][entry.0].value = entry.1.parse().unwrap()
            }
        }
        println!("board: {:?}", board);
        boards.push(board);
    }

    let mut won_count = 0;
    for draw in moves {
        println!("drawn: {}", draw);
        for (bi, board) in boards.iter_mut().enumerate() {
            if board.won {
                continue;
            }
            for y in 0..5 {
                for x in 0..5 {
                    if board.fields[y][x].value == draw {
                        //println!("board {} hit {} {}", bi, x, y);
                        board.fields[y][x].marked = true;
                        board.col_marked[x] += 1;
                        board.row_marked[y] += 1;
                        if board.col_marked[x] == 5 || board.row_marked[y] == 5 {
                            let score = board.get_score(draw);
                            println!("BINGO board {}: score {}", bi, score);
                            board.won = true;
                            won_count += 1;
                        }
                    }
                }
            }
        }
        if won_count == boards.len() {
            break;
        }
    }
}
impl Bingoboard {
    pub(crate) fn get_score(&self, last: i32) -> i32 {
        let mut score = 0;
        /*    The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.*/
        for y in 0..5 {
            for x in 0..5 {
                if !self.fields[y][x].marked {
                    score += self.fields[y][x].value
                }
            }
        }
        score * last
    }
}
