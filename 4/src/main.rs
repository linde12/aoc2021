use std::{collections::VecDeque, str::FromStr};

struct Piece {
    value: u32,
    marked: bool,
}

struct Board {
    rows: Vec<Vec<Piece>>,
}

impl Board {
    fn has_won(&self) -> bool {
        let horizontal =
            (0..self.rows.len()).any(|row_idx| self.rows[row_idx].iter().all(|p| p.marked));
        let vertical =
            (0..self.rows.len()).any(|col_idx| self.rows.iter().all(|col| col[col_idx].marked));
        horizontal || vertical
    }

    fn mark_number(&mut self, number: u32) {
        for column in self.rows.iter_mut() {
            for piece in column.iter_mut() {
                if piece.value == number {
                    piece.marked = true
                }
            }
        }
    }

    fn get_unmarked(&self) -> Vec<u32> {
        self.rows
            .iter()
            .flatten()
            .filter(|piece| !piece.marked)
            .map(|p| p.value)
            .collect()
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: Vec<String> = s
            .lines()
            .map(|mut line| {
                if line.starts_with(" ") {
                    line = &line[1..];
                }
                line.replace("  ", " ")
            })
            .collect();

        let rows: Vec<Vec<Piece>> = board
            .iter()
            .map(|line| {
                line.split(" ")
                    .map(|n| {
                        let value: u32 = n.parse().unwrap();
                        Piece {
                            value,
                            marked: false,
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(Self { rows })
    }
}

fn main() {
    let input = include_str!("./input.txt").trim();
    let mut parts: VecDeque<&str> = input.split("\n\n").collect();
    let order = parts.pop_front().unwrap();
    let numbers: Vec<u32> = order.split(",").map(|n| n.parse().unwrap()).collect();

    let mut boards: Vec<Board> = parts
        .into_iter()
        .map(FromStr::from_str)
        .collect::<Result<Vec<Board>, ()>>()
        .unwrap();
    let mut winners = vec![];

    for number in numbers {
        // mark the number in every board
        boards
            .iter_mut()
            .for_each(|board| board.mark_number(number));

        // filter away boards that have won and push the winning
        // product to `winners`
        boards = boards
            .into_iter()
            .filter(|board| {
                if board.has_won() {
                    let sum: u32 = board.get_unmarked().iter().sum();
                    winners.push(sum * number);
                    return false;
                }
                true
            })
            .collect();

        if boards.len() == 0 {
            break;
        }
    }

    println!("part 1: {}", winners.first().unwrap());
    println!("part 2: {}", winners.last().unwrap());
}
