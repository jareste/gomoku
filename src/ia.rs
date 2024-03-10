use rayon::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::game::{Game, Piece};
use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;
// use crate::constants::{DEPTH, WINNING_BONUS, LOSING_PENALTY, DIRECTIONS, DEVELOPING_TWO, DEVELOPING_THREE, FREE_FOUR, DEVELOPING_FOUR, FIVE_IN_A_ROW};
// use crate::constants::{POSSIBLE_CAPTURE, CAPTURE, FREE_THREE_FIVE, FREE_THREE_SIX};
use crate::heuristic::{generate_patterns, generate_patterns_single_move};
use crate::constants::DEPTH;
use std::sync::Mutex;
// use lazy_static::lazy_static;
// use std::error::Error;
// use csv::Writer;
// use csv::Reader;

// lazy_static! {
//     static ref TRANSPOSITION_TABLE: Mutex<HashMap<String, ((i8, i8), i128)>> = Mutex::new(HashMap::new());
// }

// pub fn load_transposition_table() -> Result<(), Box<dyn Error>> {
//     let mut rdr = Reader::from_path("transposition_table.csv")?;

//     let mut table = TRANSPOSITION_TABLE.lock().unwrap();
//     for result in rdr.records() {
//         let record = result?;
//         let state = record[0].to_string();
//         let move_index = (record[1].parse::<i8>()?, record[2].parse::<i8>()?);
//         let score = record[3].parse::<i128>()?;
//         table.insert(state, (move_index, score));
//     }

//     Ok(())
// }

// pub fn store_transposition_table() -> Result<(), Box<dyn Error>> {
//     let mut wtr = Writer::from_path("transposition_table.csv")?;

//     let table = TRANSPOSITION_TABLE.lock().unwrap();
//     for (state, &(move_index, score)) in table.iter() {
//         wtr.write_record(&[state, &move_index.0.to_string(), &move_index.1.to_string(), &score.to_string()])?;
//     }

//     wtr.flush()?;
//     Ok(())
// }


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Move {
    index: (i8, i8),
    score: i128,
}

// const DEPTH: i8 = 10;
// const WINNING_BONUS: f64 = 10_000_000.0;
// const LOSING_PENALTY: f64 = -11_000_000.0;
// const THREATENING_BONUS: f64 = 100_000.0;

pub trait IA{
    fn get_possible_moves(&mut self, is_maximizing_player: bool) -> Vec<(i8, i8)>;
    // fn minimax(&mut self, depth: i8, alpha: i128, beta: i128, is_maximizing_player: bool) -> Move;
    fn minimax(&mut self, depth: i8, alpha: i128, beta: i128, is_maximizing_player: bool) -> Move;
    fn best_move(&mut self) -> (i8, i8);
    fn distance(&self, a: (i8, i8), b: (i8, i8)) -> i8;
}


impl IA for Game {


    fn distance(&self, a: (i8, i8), b: (i8, i8)) -> i8 {
        ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as i8
    }

    // threading moves.
    fn get_possible_moves(&mut self, is_maximizing_player: bool) -> Vec<(i8, i8)> {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        let new_self = self.clone();
        let piece = if is_maximizing_player { Piece::Player1 } else { Piece::Player2 };
        let moves: Vec<(i8, i8)> = (0..19).into_par_iter().flat_map(move |x| {
            (0..19).into_par_iter().filter_map(move |y| {
                if new_self.map[x][y] != Piece::Empty {
                    let mut cell_moves = Vec::new();
                    for &(dx, dy) in &directions {
                        for i in 1..=1 {
                            let nx = x as isize + i * dx;
                            let ny = y as isize + i * dy;
                            if nx >= 0 && ny >= 0 && nx < 19 && ny < 19 && new_self.map[nx as usize][ny as usize] == Piece::Empty  {
                                if !new_self.find_free_threes((nx as i8, ny as i8), 1, piece) {
                                    cell_moves.push((nx as i8, ny as i8));
                                }
                            }
                        }
                    }
                    Some(cell_moves)
                } else {
                    None
                }
            }).flatten()
        }).collect();

        let mut vec_moves: Vec<_> = moves.into_iter().collect();
        let last_move1 = self.last_move1;
        let last_move2 = self.last_move2;

        vec_moves.sort_by(|a, b| {
            let ha = self.heat_map[a.0 as usize][a.1 as usize];
            let hb = self.heat_map[b.0 as usize][b.1 as usize];

            let da1 = ((a.0 as i32 - last_move1.0 as i32).pow(2) + (a.1 as i32 - last_move1.1 as i32).pow(2)) as f64;
            let da2 = ((a.0 as i32 - last_move2.0 as i32).pow(2) + (a.1 as i32 - last_move2.1 as i32).pow(2)) as f64;
            let db1 = ((b.0 as i32 - last_move1.0 as i32).pow(2) + (b.1 as i32 - last_move1.1 as i32).pow(2)) as f64;
            let db2 = ((b.0 as i32 - last_move2.0 as i32).pow(2) + (b.1 as i32 - last_move2.1 as i32).pow(2)) as f64;

            let da = da1.min(da2);
            let db = db1.min(db2);

            hb.partial_cmp(&ha)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal))
        });
        vec_moves
    }






    fn minimax(&mut self, depth: i8, alpha: i128, beta: i128, is_maximizing_player: bool) -> Move {
        if depth == 0 {
            return Move { index: (0, 0), score: generate_patterns(self.map.clone()) };
        }

        let mut possible_moves = self.get_possible_moves(is_maximizing_player);
        let (best_move, best_score) = possible_moves.par_iter()
            .map(|&moves| {
                let mut new_game = self.clone();
                new_game.place(moves.0 as usize, moves.1 as usize, if is_maximizing_player { Piece::Player1 } else { Piece::Player2 });
                if depth == DEPTH && new_game.check_win() == (true, Piece::Player1) {
                    return (moves, i128::MAX);
                }
                let score = new_game.minimax(depth - 1, alpha, beta, !is_maximizing_player).score;
                (moves, score)
            })
            .reduce_with(|(best_move1, best_score1), (best_move2, best_score2)| {
                if is_maximizing_player && best_score1 > best_score2 || !is_maximizing_player && best_score1 < best_score2 {
                    (best_move1, best_score1)
                } else {
                    (best_move2, best_score2)
                }
            }).unwrap_or(((0, 0), 0));

        Move { index: best_move, score: best_score }
    }

    fn best_move(&mut self) -> (i8, i8) {
        println!("heat map: {:?}", self.heat_map[9][9]);
        // println!("transposition table: {:?}", TRANSPOSITION_TABLE.lock().unwrap());
        self.minimax(DEPTH, i128::MIN, i128::MIN, true).index
    }

}


// transposition table minimax:
    // fn minimax(&mut self, depth: i8, alpha: i128, beta: i128, is_maximizing_player: bool) -> Move {
    //     let state = self.state_to_string();
    //     {
    //         let table = TRANSPOSITION_TABLE.lock().unwrap();
    //         if let Some(&(move_index, score)) = table.get(&state) {
    //             return Move { index: move_index, score };
    //         }
    //     }
    //     if depth == 0 {
    //         return Move { index: (0, 0), score: generate_patterns(self.map.clone()) };
    //     }
    
    //     let mut possible_moves = self.get_possible_moves(is_maximizing_player);
    //     let (best_move, best_score) = possible_moves.iter()
    //         .map(|&moves| {
    //             let mut new_game = self.clone();
    //             new_game.place(moves.0 as usize, moves.1 as usize, if is_maximizing_player { Piece::Player1 } else { Piece::Player2 });
    //             if depth == DEPTH && new_game.check_win() == (true, Piece::Player1) {
    //                 return (moves, i128::MAX);
    //             }
    //             let score = new_game.minimax(depth - 1, alpha, beta, !is_maximizing_player).score;
    //             (moves, score)
    //         })
    //         .fold(((0, 0), i128::MIN), |(best_move1, best_score1), (best_move2, best_score2)| {
    //             if is_maximizing_player && best_score1 > best_score2 || !is_maximizing_player && best_score1 < best_score2 {
    //                 (best_move1, best_score1)
    //             } else {
    //                 (best_move2, best_score2)
    //             }
    //         });
    
    //     let mut table = TRANSPOSITION_TABLE.lock().unwrap();
    //     table.insert(state, (best_move, best_score));
    //     Move { index: best_move, score: best_score }
    // }