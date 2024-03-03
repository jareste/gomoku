use rayon::prelude::*;
use std::collections::HashSet;
use crate::game::{Game, Piece};
use crate::constants::{DEPTH, WINNING_BONUS, LOSING_PENALTY, DIRECTIONS, DEVELOPING_TWO, FREE_THREE, DEVELOPING_THREE, FREE_FOUR, DEVELOPING_FOUR, FIVE_IN_A_ROW};
use crate::constants::{POSSIBLE_CAPTURE, CAPTURE};
use std::cmp::min;
use std::cmp::max;

#[derive(Clone, Copy, Debug)]
pub struct Move {
    index: (i8, i8),
    score: i128,
}

pub struct KillerMove {
    depth: i8,
    killer: Move,
}

pub trait IA{
    fn get_possible_moves(&mut self, is_maximizing_player: bool, depth: i8) -> Vec<(i8, i8)>;
    fn minimax(&mut self, depth: i8, alpha: i128, beta: i128, is_maximizing_player: bool) -> Move;

    fn distance(&self, a: (i8, i8), b: (i8, i8)) -> i8;
    fn best_move(&mut self) -> (i8, i8);
    fn get_best_movement(&mut self, is_maximizing_player: bool, possible_moves: Vec<(i8, i8)>) -> Move;
    fn evaluate_move(&self, is_maximizing_player: bool, last_move: (i8, i8)) -> i128;
    // fn iddfs(&mut self, max_depth: i8) -> Move;
}


impl IA for Game {
    fn distance(&self, a: (i8, i8), b: (i8, i8)) -> i8 {
        ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as i8
    }

    fn get_possible_moves(&mut self, is_maximizing_player: bool, depth: i8) -> Vec<(i8, i8)> {
        let mut moves = HashSet::new();
        for x in 0..19 as i8 {
            for y in 0..19 as i8 {
                if self.map[x as usize][y as usize] != Piece::Empty {
                    for &(dx, dy) in &DIRECTIONS {
                        for i in 1..=1 as i8 {
                            let nx = x + i * dx;
                            let ny = y + i * dy;
                            if nx >= 0 && ny >= 0 && nx < 19 && ny < 19 && self.map[nx as usize][ny as usize] == Piece::Empty  {
                                self.map[nx as usize][ny as usize] = self.map[x as usize][y as usize];
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
        let mut vec_moves: Vec<_> = moves.into_iter().collect();
        let (last_move, player) = match is_maximizing_player {
            true => (self.last_move_p1, Piece::Player1),
            false => (self.last_move_p2, Piece::Player2),
        };
        // println!("depth: {}", depth);
        // if depth - 4 <= 0 {
        //     let threshold = 49;
        //     vec_moves.retain(|&moves| self.evaluate_piece(moves.0 as usize, moves.1 as usize, player) >= threshold);
        // }
        vec_moves
    }
    // PatternsValue = {
//     Patterns.POTENTIAL_CAPTURE        : 1,
//     Patterns.AX_DEVELOPING_TO_2       : 10,
//     Patterns.AX_DEVELOPING_TO_3       : 100,
//     Patterns.CAPTURE                  : 2000,
//     Patterns.AX_DEVELOPING_TO_4       : 10_000,
//     Patterns.FREE_3                   : 100_000,
//     Patterns.FREE_4                   : 1_000_000,
//     Patterns.FIVE_IN_A_ROW            : float('inf'),
// }

// use crate::constants::{DEPTH, WINNING_BONUS, LOSING_PENALTY, DEVELOPING_TWO,\
//     FREE_THREE, DEVELOPING_THREE, FREE_FOUR, DEVELOPING_FOUR, FIVE_IN_A_ROW};


    fn evaluate_move(&self, is_maximizing_player: bool, last_move: (i8, i8)) -> i128 {
        let mut score: i128 = 0;
        let (player, opponent) = match is_maximizing_player {
            true => (Piece::Player1, Piece::Player2),
            false => (Piece::Player2, Piece::Player1),
        };
        let mut player_score = 0;
        let mut opponent_score = 0;
        for &(dx, dy) in &DIRECTIONS {
            let mut sequence = Vec::new();
            for i in -1..=5 as i8 {
                let nx = last_move.0 + i * dx;
                let ny = last_move.1 + i * dy;
                if nx >= 0 && nx < 19 && ny >= 0 && ny < 19 {
                    sequence.push(self.map[nx as usize][ny as usize]);
                }
            }
            if sequence.len() < 4 {
                continue;
            }
            if sequence.len() == 4 {
                let array_sequence = [sequence[0], sequence[1], sequence[2], sequence[3]];
                if POSSIBLE_CAPTURE.contains(&array_sequence) {
                    score += 1;
                }
                if CAPTURE.contains(&array_sequence) {
                    score += 2_000;
                }
            }
            if sequence.len() == 5 {
                let array_sequence = [sequence[0], sequence[1], sequence[2], sequence[3], sequence[4]];
                if FIVE_IN_A_ROW.contains(&array_sequence) {
                    score += i128::MAX;
                }
                if DEVELOPING_FOUR.contains(&array_sequence) {
                    score += 10_000;
                }
                if DEVELOPING_THREE.contains(&array_sequence) {
                    score += 100;
                }
                if DEVELOPING_TWO.contains(&array_sequence) {
                    score += 10;
                }
            }
            if sequence.len() == 6 {
                let array_sequence = [sequence[0], sequence[1], sequence[2], sequence[3], sequence[4], sequence[5]];
                if FREE_FOUR.contains(&array_sequence) {
                    score += 1_000_000;
                }
                if FREE_THREE.contains(&array_sequence) {
                    score += 100_000;
                }
            }
        }
        score
    }

    fn get_best_movement(&mut self, is_maximizing_player: bool, possible_moves: Vec<(i8, i8)>) -> Move {
        let mut best_move = (0,0);
        let mut best_score = i128::MIN;
        for &moves in possible_moves.iter() {
            self.map[moves.0 as usize][moves.1 as usize] = if is_maximizing_player { Piece::Player1 } else { Piece::Player2 };
            let score = self.evaluate_move(is_maximizing_player, moves);
            self.map[moves.0 as usize][moves.1 as usize] = Piece::Empty;
            if score > best_score {
                best_score = score;
                best_move = moves;
            }
        }
        Move { index: best_move, score: best_score }
    }

    fn minimax(&mut self, depth: i8, mut alpha: i128, mut beta: i128, is_maximizing_player: bool) -> Move {
        let mut possible_moves = self.get_possible_moves(is_maximizing_player, depth);
        if depth == 0 {
            return self.get_best_movement(is_maximizing_player, possible_moves);
        }
        let mut best_move = (0, 0);
        let mut best_score = if is_maximizing_player { i128::MIN } else { i128::MAX };
        
        // possible_moves = self.get_heuristic_moves(&possible_moves, is_maximizing_player).iter().map(|&moves| moves.index).collect();

        for &moves in possible_moves.iter() {
            let mut new_game = self.clone();
            if !new_game.place(moves.0 as usize, moves.1 as usize, if is_maximizing_player { Piece::Player1 } else { Piece::Player2 }) {
                continue;
            }
            let score = new_game.minimax(depth - 1, alpha, beta, !is_maximizing_player).score;
            // println!("Score: {}", score);
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
        let best_move = self.minimax(DEPTH, i128::MIN, i128::MAX, true);
        println!("best:score: {}", best_move.score);
        best_move.index
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