use rayon::prelude::*;
use std::collections::HashSet;
use crate::game::{Game, Piece};
use crate::constants::{DEPTH, WINNING_BONUS, LOSING_PENALTY, DIRECTIONS, DEVELOPING_TWO, DEVELOPING_THREE, FREE_FOUR, DEVELOPING_FOUR, FIVE_IN_A_ROW};
use crate::constants::{POSSIBLE_CAPTURE, CAPTURE, FREE_THREE_FIVE, FREE_THREE_SIX};
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
    fn get_possible_moves(&mut self, is_maximizing_player: bool, depth: i8) -> Vec<((i8, i8), i128)>;
    fn minimax(&mut self, depth: i8, alpha: i128, beta: i128, is_maximizing_player: bool) -> Move;

    fn distance(&self, a: (i8, i8), b: (i8, i8)) -> i8;
    fn best_move(&mut self) -> (i8, i8);
    fn evaluate_map(&mut self) -> (bool, i128);
    fn generate_sequence(&self, x: i8, y: i8, dx: i8, dy: i8, range: (i8, i8)) -> Vec<Piece>;
    // fn iddfs(&mut self, max_depth: i8) -> Move;
}


impl IA for Game {
    fn distance(&self, a: (i8, i8), b: (i8, i8)) -> i8 {
        ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as i8
    }

    fn get_possible_moves(&mut self, is_maximizing_player: bool, depth: i8) -> Vec<((i8, i8), i128)> {
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
                                let (illegal, score) = self.evaluate_map();
                                self.map[nx as usize][ny as usize] = Piece::Empty;
                                if  !illegal {
                                    moves.insert(((nx as i8, ny as i8), score));
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut vec_moves: Vec<_> = moves.into_iter().collect();
        vec_moves.sort_by(|a, b| {
            if is_maximizing_player {
                // Sort in descending order for maximizing player
                b.1.cmp(&a.1)
            } else {
                // Sort in ascending order for minimizing player
                a.1.cmp(&b.1)
            }
        });
        let two_thirds_index = vec_moves.len() * 2 / 3;
        vec_moves.truncate(two_thirds_index);
        vec_moves
    }

    fn generate_sequence(&self, x: i8, y: i8, dx: i8, dy: i8, range: (i8, i8)) -> Vec<Piece> {
        let mut sequence = Vec::new();
        for i in range.0..=range.1 {
            let nx = x + i * dx;
            let ny = y + i * dy;
            if nx >= 0 && ny >= 0 && nx < 19 && ny < 19 {
                sequence.push(self.map[nx as usize][ny as usize]);
            }
        }
        sequence
    }

    // i'll be evaluating matches since the actual piece and the previous one as some situations need to evaluate the -1 aswell

    // i have to generate sequences for both, starting from -1 and from 0.
    fn evaluate_map(&mut self) -> (bool, i128) {
        // let piece = self.map[last_move.0 as usize][last_move.1 as usize];
        let mut score: i128 = 0;
        let mut free_three_p1: i8 = 0;
        let mut free_three_p2: i8 = 0;
        let directions = [(1, 0), (0, 1), (1, 1), (-1, 1)];
        let mut position_score: i128 = 0;
        for x in 0..18 {
            for y in 0..18 {
                if self.map[x][y] != Piece::Empty {
                    for direction in directions {
                        let sequence_four = self.generate_sequence(x as i8, y as i8, direction.0, direction.1, (-1, 3));
                        if sequence_four.len() == 4 {
                            let array_sequence = [sequence_four[0], sequence_four[1], sequence_four[2], sequence_four[3]];
                            // this needs to evaluate in the reverse as i'm checking since -1 so, the piece evaluate has been eaten.
                            if POSSIBLE_CAPTURE.contains(&array_sequence) {
                                position_score -= 1;
                            }
                            if CAPTURE.contains(&array_sequence) {
                                position_score -= 2_000;
                            }
                        }
                        let sequence_five = self.generate_sequence(x as i8, y as i8, direction.0, direction.1, (-1, 4));
                        if sequence_five.len() == 5 {
                            let array_sequence = [sequence_five[0], sequence_five[1], sequence_five[2], sequence_five[3], sequence_five[4]];
                            if FIVE_IN_A_ROW.contains(&array_sequence) {
                                position_score += i64::MAX as i128;
                            }
                            if DEVELOPING_FOUR.contains(&array_sequence) {
                                position_score += 10_000;
                            }
                            if DEVELOPING_THREE.contains(&array_sequence) {
                                position_score += 100;
                            }
                            if DEVELOPING_TWO.contains(&array_sequence) {
                                position_score += 10;
                            }
                            if FREE_THREE_FIVE.contains(&array_sequence) {
                                position_score += 100_000;
                                match self.map[x][y] {
                                    Piece::Player1 => free_three_p1 += 1,
                                    Piece::Player2 => free_three_p2 += 1,
                                    _ => (),
                                }
                                if free_three_p1 > 1 || free_three_p2 > 1 {
                                    return (true, 0); // break condition here as it's illegal to have more than 1 free three.
                                }
                            }

                        }
                        let sequence_six = self.generate_sequence(x as i8, y as i8, direction.0, direction.1, (-1, 5));
                        if sequence_six.len() == 6 {
                            let array_sequence = [sequence_six[0], sequence_six[1], sequence_six[2], sequence_six[3], sequence_six[4], sequence_six[5]];
                            if FREE_FOUR.contains(&array_sequence) {
                                position_score += 1_000_000;
                            }
                            if FREE_THREE_SIX.contains(&array_sequence) {
                                position_score += 100_000;
                                match self.map[x][y] {
                                    Piece::Player1 => free_three_p1 += 1,
                                    Piece::Player2 => free_three_p2 += 1,
                                    _ => (),
                                }
                                if free_three_p1 > 1 || free_three_p2 > 1 {
                                    return (true, 0); // break condition here as it's illegal to have more than 1 free three.
                                }
                            }
                        }
                        if score != 0 {
                            match self.map[x][y] {
                                Piece::Player1 => score += position_score,
                                Piece::Player2 => score -= position_score,
                                _ => (),
                            }
                        }

                    }
                }
            }
        };

        // println!("free three p1: {:?} | free three p2: {:?}", free_three_p1, free_three_p2);
        if free_three_p1 > 1 || free_three_p2 > 1 {
            return (true, 0);
        }
        (false, score)
    }

    fn minimax(&mut self, depth: i8, mut alpha: i128, mut beta: i128, is_maximizing_player: bool) -> Move {
        let mut possible_moves = self.get_possible_moves(is_maximizing_player, depth);
        if depth == 0 {
            return Move { index: possible_moves[0].0 , score: possible_moves[0].1 }; //placeholder
        }
        let mut best_move = (0, 0);
        let mut best_score = if is_maximizing_player { i128::MIN } else { i128::MAX };
        
        // possible_moves = self.get_heuristic_moves(&possible_moves, is_maximizing_player).iter().map(|&moves| moves.index).collect();

        for &((x, y), score) in possible_moves.iter() {
            let mut new_game = self.clone();
            if !new_game.place(x as usize, y as usize, if is_maximizing_player { Piece::Player1 } else { Piece::Player2 }) {
                continue;
            }
            let score = new_game.minimax(depth - 1, alpha, beta, !is_maximizing_player).score;
            // println!("Score: {}", score);
            match is_maximizing_player {
                true => {
                    if score > best_score { 
                    best_score = score;
                    best_move = (x, y);
                    }
                    alpha = std::cmp::max(alpha, score);
                },
                false => {
                    if score < best_score {
                        best_score = score;
                        best_move = (x, y);

                    }
                    beta = std::cmp::max(beta, score);
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