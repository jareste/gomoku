use rayon::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::game::{Game, Piece};
use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Move {
    index: (i8, i8),
    score: i128,
}

const DEPTH: i8 = 10;
const WINNING_BONUS: f64 = 10_000_000.0;
const LOSING_PENALTY: f64 = -11_000_000.0;
const THREATENING_BONUS: f64 = 100_000.0;

pub trait IA{
    fn get_possible_moves(&mut self, is_maximizing_player: bool) -> Vec<(i8, i8)>;
    fn get_consequtive_pieces_score(&mut self, player: Piece) -> f64;
    fn get_heuristic(&mut self) -> i128;
    fn minimax(&mut self, depth: i8, alpha: i128, beta: i128, is_maximizing_player: bool) -> Move;
    fn best_move(&mut self) -> (i8, i8);
    fn is_part_of_line(&mut self, x: usize, y: usize, player: Piece) -> Vec<(isize, isize)>;
    fn distance(&self, a: (i8, i8), b: (i8, i8)) -> i8;


    // fn get_transposition_table(&mut self) -> &mut HashMap<String, Move>;
    // fn set_transposition_table(&mut self, transposition_table: HashMap<String, Move>);
    // fn iddfs(&mut self, max_depth: i8) -> Move;
}


impl IA for Game {

    // fn get_transposition_table(&mut self) -> &mut HashMap<String, Move> {
    //     &mut self.transposition_table
    // }

    // fn set_transposition_table(&mut self, transposition_table: HashMap<String, Move>) {
    //     self.transposition_table = transposition_table;
    // }


    fn distance(&self, a: (i8, i8), b: (i8, i8)) -> i8 {
        ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as i8
    }

    fn get_possible_moves(&mut self, is_maximizing_player: bool) -> Vec<(i8, i8)> {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        let new_self = self.clone();
        let moves: Vec<(i8, i8)> = (0..19).into_par_iter().flat_map(|x| {
            (0..19).into_par_iter().filter_map(move |y| {
                if new_self.map[x][y] != Piece::Empty {
                    for &(dx, dy) in &directions {
                        for i in 1..=1 {
                            let nx = x as isize + i * dx;
                            let ny = y as isize + i * dy;
                            if nx >= 0 && ny >= 0 && nx < 19 && ny < 19 && new_self.map[nx as usize][ny as usize] == Piece::Empty  {
                                let mut map_clone = new_self.clone();
                                map_clone.map[nx as usize][ny as usize] = new_self.map[x][y];
                                if !map_clone.find_free_threes((nx as i8, ny as i8), 1) {
                                    return Some((nx as i8, ny as i8));
                                }
                            }
                        }
                    }
                }
                None
            })
        }).collect();
        let mut vec_moves: Vec<_> = moves.into_iter().collect();
        vec_moves.sort_by(|a, b| {
            let ha = self.heat_map[a.0 as usize][a.1 as usize];
            let hb = self.heat_map[b.0 as usize][b.1 as usize];
            hb.partial_cmp(&ha).unwrap_or(std::cmp::Ordering::Equal) // sort in descending order of heat
        });
        if self.movements > 7 {
            return vec_moves;
        }
        let second_half_start = vec_moves.len() / 2;
        let mut second_half: Vec<_> = vec_moves.split_off(second_half_start);

        // Calculate the percentage of pieces to remove based on the number of movements
        let percentage_to_remove = 0.10 + (self.movements as f64 / 100.0);
        let num_to_remove = (second_half.len() as f64 * percentage_to_remove).round() as usize;

        let rng = &mut rand::thread_rng();
        let indices_to_remove: Vec<_> = (0..second_half.len()).choose_multiple(rng, num_to_remove);
        second_half = second_half.into_iter().enumerate().filter(|(i, _)| !indices_to_remove.contains(i)).map(|(_, item)| item).collect();
        vec_moves.extend(second_half);
        // println!("{:?}", vec_moves);
        vec_moves
    }

    fn is_part_of_line(&mut self, x: usize, y: usize, player: Piece) -> Vec<(isize, isize)> {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        let mut valid_directions = Vec::new();
        for &(dx, dy) in &directions {
            let mut consecutive_pieces = 0;
            for i in -1..=1 {
                let nx = x as isize + i * dx;
                let ny = y as isize + i * dy;
                if nx >= 0 && ny >= 0 && nx < 19 && ny < 19 && self.map[nx as usize][ny as usize] == player {
                    consecutive_pieces += 1;
                }
            }
            if consecutive_pieces > 1 {
                valid_directions.push((dx, dy));
            }
        }
        valid_directions
    }
   

    // rarete
    fn get_consequtive_pieces_score(&mut self, player: Piece) -> f64 {
        let mut score = 0.0;
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        for x in 2..16 {
            for y in 2..16 {
                if self.map[x][y] == player {
                    if self.movements < 2 {
                        println!("x: {}, y: {}", x, y);
                    }
                    // let directions = self.is_part_of_line(x, y, player);
                    for &(dx, dy) in &directions {
                        let mut consequtive_pieces = 0;
                        let mut open_line = 2; // Assume line is open at both ends
                        for i in -1..=5 {
                            let nx = x as isize + i * dx;
                            let ny = y as isize + i * dy;
                            if nx < 0 || ny < 0 || nx >= 19 || ny >= 19 {
                                open_line -= 1;
                                continue;
                            }
                            if i >= 0 && i < 5 {
                                if self.map[nx as usize][ny as usize] == player {
                                    consequtive_pieces += 1;
                                } else if self.map[nx as usize][ny as usize] != Piece::Empty {
                                    open_line -= 1;
                                }
                            } else if self.map[nx as usize][ny as usize] != Piece::Empty {
                                open_line -= 1;
                            }
                        }
                        if consequtive_pieces == 5 {
                            score += f32::MAX as f64;
                        }
                        if open_line > 0 {
                            score += match consequtive_pieces {
                                4 => if open_line == 2 { 10_000_000.0 } else { 100_000.0 } ,
                                3 => if open_line == 2 { 100_000.0 } else { 1_000.0 },
                                2 => 100.0,
                                1 => 1.0,
                                _ => 0.0,
                            } * open_line as f64;
                        }
                    }
                    if self.movements < 2 {
                        println!("score: {}", score);
                    }
                }
            }
        }
        score
    }

    // should be reviewed but it's working.
    fn get_heuristic(&mut self) -> i128 {
        // match self.check_win() {
        //     (true,Piece::Player1) => return i32::MAX,
        //     (true,Piece::Player2) => return i32::MIN,
        //     _ => (),
        // }
        let mut score = 0.0;
        score += self.get_consequtive_pieces_score(Piece::Player1);
        score -= self.get_consequtive_pieces_score(Piece::Player2);
        if self.captured1 > 0 {
            score += self.captured1 as f64 * 20.0;
        }
        if self.captured2 > 0 {
            score -= self.captured2 as f64 * 20.0;
        }
        score as i128
    }


    fn minimax(&mut self, depth: i8, mut alpha: i128, mut beta: i128, is_maximizing_player: bool) -> Move {
        let mut possible_moves = self.get_possible_moves(is_maximizing_player);
        if depth == 0 {
            return Move { index: (0, 0), score: self.get_heuristic() };
        }
        let winner = self.check_win();
        if winner == (true, Piece::Player1) || winner == (true, Piece::Player2) {
            // println!("depth: {}, winner: {}", depth, winner.1);
            return Move { index: (0,0), score: if (winner == (true, Piece::Player1)) { i128::MAX } else { i128::MIN }};
        }
        // let state_string = self.state_to_string();
        // if let Some(cached_move) = self.get_transposition_table().get(&state_string) {
        //     return *cached_move;
        // }
        let (best_move, best_score) = possible_moves.par_iter()
        .map(|&moves| {
            let mut new_game = self.clone();
            new_game.place(moves.0 as usize, moves.1 as usize, if is_maximizing_player { Piece::Player1 } else { Piece::Player2 });
            let score = new_game.minimax(depth - 1, alpha, beta, !is_maximizing_player).score;
            (moves, score)
        })
        .reduce_with(|(best_move1, best_score1), (best_move2, best_score2)| {
            if is_maximizing_player && best_score1 > best_score2 || !is_maximizing_player && best_score1 < best_score2 {
                (best_move1, best_score1)
            } else {
                (best_move2, best_score2)
            }
        }).unwrap();
        if best_move == (7,10) {
            println!("depth: {}, best_move: {:?}, best_score: {}", depth, best_move, best_score);
        }
        if best_move == (9, 10){ 
            println!("depth: {}, best_move: {:?}, best_score: {}", depth, best_move, best_score);
        }
        let best_move = Move { index: best_move, score: best_score };
        // self.get_transposition_table().insert(state_string, best_move);
        best_move
    }

    fn best_move(&mut self) -> (i8, i8) {
        println!("heat map: {:?}", self.heat_map[9][9]);
        self.minimax(4, i128::MIN, i128::MAX, true).index
    }

}

// fn minimax(&mut self, depth: i8, mut alpha: i32, mut beta: i32, is_maximizing_player: bool) -> Move {
//     let mut possible_moves = self.get_possible_moves(is_maximizing_player);
//     if depth == 0 {
//         return Move { index: (0, 0), score: self.get_heuristic() };
//     }

//     let mut best_move = (0, 0);
//     let mut best_score = if is_maximizing_player { i32::MIN } else { i32::MAX };

//     let results: Vec<(i32, (i8, i8))> = possible_moves.par_iter().map(|&moves| {
//         let mut new_game = self.clone();
//         if !new_game.place(moves.0 as usize, moves.1 as usize, if is_maximizing_player { Piece::Player1 } else { Piece::Player2 }) {
//             return (i32::MIN, moves); // or some other default value
//         }
//         let score = new_game.minimax(depth - 1, alpha, beta, !is_maximizing_player).score;
//         (score, moves)
//     }).collect();

//     for (score, moves) in results {
//         match is_maximizing_player {
//             true => {
//                 if score > best_score { 
//                 best_score = score;
//                 best_move = moves;
//                 }
//                 alpha = std::cmp::max(alpha, score);
//             },
//             false => {
//                 if score < best_score {
//                     best_score = score;
//                     best_move = moves;

//                 }
//                 beta = std::cmp::min(beta, score);
//             },
//         }
//         if beta <= alpha {
//             break;
//         }
//     }
//     Move { index: best_move, score: best_score }
// }
