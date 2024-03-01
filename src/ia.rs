use rayon::prelude::*;
use std::collections::HashSet;
use crate::game::{Game, Piece};

#[derive(Clone, Copy, Debug)]
pub struct Move {
    index: (i8, i8),
    score: i32,
}

pub struct KillerMove {
    depth: i8,
    killer: Move,
}

const TEN: i8 = 1;
const WINNING_BONUS: i32 = 1_000_000;
const LOSING_PENALTY: i32 = -1_000_000;
const THREATENING_BONUS: i32 = 100_000;

pub trait IA{
    fn dfs_check_movement(&mut self, x: i8, y: i8, squares_to_check: i8) -> bool;
    fn get_possible_moves(&mut self) -> Vec<(i8, i8)>;
    // fn get_consequtive_pieces_score(&mut self, player: Piece) -> i32;
    // fn get_heuristic(&mut self) -> i32;
    fn minimax(&mut self, depth: i8, alpha: i32, beta: i32, is_maximizing_player: bool) -> Move;
    fn best_move(&mut self) -> (i8, i8);
    // fn is_part_of_line(&mut self, x: usize, y: usize, player: Piece) -> Vec<(isize, isize)>;
    // fn evaluate_move(&mut self, moves: (i8, i8), player: Piece) -> i32;
    // fn get_heuristic_moves(&mut self, possible_moves: &Vec<(i8, i8)>, is_maximizing_player: bool) -> Vec<Move>;
    // fn iddfs(&mut self, max_depth: i8) -> Move;

    fn evaluate_move(&mut self, last_move: (i8, i8), player: Piece);
    fn count_empty_spaces(&self, last_move: (i8, i8), dx: i8, dy: i8) -> i32;
    fn count_opponent_threats(&self, last_move: (i8, i8), player: Piece, dx: i8, dy: i8) -> i32;
    fn count_consecutive_pieces(&self, last_move: (i8, i8), player: Piece, dx: i8, dy: i8) -> i32;
}


impl IA for Game {
    fn dfs_check_movement(&mut self, x: i8, y: i8, squares_to_check: i8) -> bool {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        for &(dx, dy) in &directions {
            for i in 1..=squares_to_check {
                let nx = x + i * dx;
                let ny = y + i * dy;
                if nx < 0 || ny < 0 || nx >= 19 || ny >= 19 {
                    break;
                }
                if self.map[nx as usize][ny as usize] != Piece::Empty {
                    return true;
                }
            }
        }
        false
    }

    fn get_possible_moves(&mut self) -> Vec<(i8, i8)> {
        let mut moves = HashSet::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        for x in 0..19 {
            for y in 0..19 {
                if self.map[x][y] != Piece::Empty {
                    for &(dx, dy) in &directions {
                        for i in 1..=1 {
                            let nx = x as isize + i * dx;
                            let ny = y as isize + i * dy;
                            if nx >= 0 && ny >= 0 && nx < 19 && ny < 19 && self.map[nx as usize][ny as usize] == Piece::Empty  {
                                self.map[nx as usize][ny as usize] = self.map[x][y];
                                if !self.find_free_threes((nx as i8, ny as i8), 1) {
                                    moves.insert((nx as i8, ny as i8));
                                }
                                self.map[nx as usize][ny as usize] = Piece::Empty;
                            }
                        }
                    }
                }
            }
        }
        moves.into_iter().collect()
    }

    fn evaluate_move(&mut self, last_move: (i8, i8), player: Piece) {
        let mut score = 0;
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        for &(dx, dy) in &directions {
            score += self.count_consecutive_pieces(last_move, player, dx, dy) * 10;
        }
        for &(dx, dy) in &directions {
            score += self.count_opponent_threats(last_move, player, dx, dy) * 10; //maybe more value to it(?)
        }
        for &(dx, dy) in &directions {
            score += self.count_empty_spaces(last_move, dx, dy) * 5;
        }
        match player {
            Piece::Player1 => self.heuristic += (score + self.captured1 as i32 * 15),
            Piece::Player2 => self.heuristic -= (score + self.captured2 as i32 * 15),
            _ => (),
        }
    }

    fn count_empty_spaces(&self, last_move: (i8, i8), dx: i8, dy: i8) -> i32 {
        let mut count = 0;
        let mut x = last_move.0 + dx;
        let mut y = last_move.1 + dy;
        while x >= 0 && y >= 0 && x < 19 && y < 19 && self.map[x as usize][y as usize] == Piece::Empty {
            count += 1;
            x += dx;
            y += dy;
        }
        count
    }

    fn count_opponent_threats(&self, last_move: (i8, i8), player: Piece, dx: i8, dy: i8) -> i32 {
        let mut count = 0;
        let mut x = last_move.0 + dx;
        let mut y = last_move.1 + dy;
        let opponent = match player {
            Piece::Player1 => Piece::Player2,
            Piece::Player2 => Piece::Player1,
            _ => Piece::Empty,
        };

        while x >= 0 && y >= 0 && x < 19 && y < 19 && self.map[x as usize][y as usize] == opponent {
            count += 1;
            x += dx;
            y += dy;
        }

        count
    }

    fn count_consecutive_pieces(&self, last_move: (i8, i8), player: Piece, dx: i8, dy: i8) -> i32 {
        let mut count = 0;
        let mut x = last_move.0 + dx;
        let mut y = last_move.1 + dy;

        while x >= 0 && y >= 0 && x < 19 && y < 19 && self.map[x as usize][y as usize] == player {
            count += 1;
            x += dx;
            y += dy;
        }

        count
    }

    fn minimax(&mut self, depth: i8, mut alpha: i32, mut beta: i32, is_maximizing_player: bool) -> Move {
        let mut possible_moves = self.get_possible_moves();
        if depth == 0 {
            return Move { index: (0, 0), score: self.heuristic };
        }
        // let game_state = self.to_string();

        // if let Some(score) = self.transposition_table.get(&game_state) {
        //     return Move { index: (0, 0), score: *score };
        // }
        let mut best_move = (0, 0);
        let mut best_score = if is_maximizing_player { i32::MIN } else { i32::MAX };

        for &moves in possible_moves.iter() {
            let mut new_game = self.clone();
            if !new_game.place(moves.0 as usize, moves.1 as usize, if is_maximizing_player { Piece::Player1 } else { Piece::Player2 }) {
                continue;
            }
            let score = new_game.minimax(depth - 1, alpha, beta, !is_maximizing_player).score;
            println!("Score: {}", score);
            match is_maximizing_player {
                true => {
                    if score > best_score { 
                    best_score = score;
                    best_move = moves;
                    }
                    alpha = std::cmp::max(alpha, score);
                },
                false => {
                    if score < best_score {
                        best_score = score;
                        best_move = moves;

                    }
                    beta = std::cmp::min(beta, score);
                },
            }
            if beta <= alpha {
                break;
            }
        }
        Move { index: best_move, score: best_score }
    }


     fn best_move(&mut self) -> (i8, i8) {
        let best_move =  self.minimax(TEN, i32::MIN, i32::MAX, true);
        println!("Best move: {:?}", best_move);
        best_move.index
     }

  /*  fn best_move(&mut self) -> (i8, i8) {
        let mut best_move = Move { index: (0, 0), score: i32::MIN };
        for depth in 1..=1 {
            let moves = self.minimax(depth, i32::MIN, i32::MAX, true);
            // println!("Depth: {} Score: {}, index: {:?}", depth, moves.score, moves.index);
            if moves.score > best_move.score {
                best_move = moves;
            }
        }
        best_move.index
    } */
}


// fn minimax(&mut self, depth: i8, mut alpha: i32, mut beta: i32, is_maximizing_player: bool) -> Move {
//     let mut possible_moves = self.get_possible_moves();
//     if depth == 0 {
//         return Move { index: (0, 0), score: self.heuristic };
//     }
//     let game_state = self.to_string();

//     if let Some(score) = self.transposition_table.get(&game_state) {
//         return Move { index: (0, 0), score: *score };
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
//     // self.transposition_table.insert(game_state, best_score);
//     Move { index: best_move, score: best_score }
// }