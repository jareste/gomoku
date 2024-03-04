use rayon::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use crate::game::{Game, Piece};
use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Move {
    index: (i8, i8),
    score: i32,
}

pub struct KillerMove {
    depth: i8,
    killer: Move,
}

const DEPTH: i8 = 3;
const WINNING_BONUS: i32 = 10_000_000;
const LOSING_PENALTY: i32 = -11_000_000;
const THREATENING_BONUS: i32 = 100_000;

pub trait IA{
    fn get_possible_moves(&mut self, is_maximizing_player: bool) -> Vec<(i8, i8)>;
    fn get_consequtive_pieces_score(&mut self, player: Piece) -> i32;
    fn get_heuristic(&mut self) -> i32;
    fn minimax(&mut self, depth: i8, alpha: i32, beta: i32, is_maximizing_player: bool) -> Move;
    fn best_move(&mut self) -> (i8, i8);
    fn is_part_of_line(&mut self, x: usize, y: usize, player: Piece) -> Vec<(isize, isize)>;
    fn distance(&self, a: (i8, i8), b: (i8, i8)) -> i8;


    fn get_transposition_table(&mut self) -> &mut HashMap<String, Move>;
    fn set_transposition_table(&mut self, transposition_table: HashMap<String, Move>);
    // fn iddfs(&mut self, max_depth: i8) -> Move;
}


impl IA for Game {

    fn get_transposition_table(&mut self) -> &mut HashMap<String, Move> {
        &mut self.transposition_table
    }

    fn set_transposition_table(&mut self, transposition_table: HashMap<String, Move>) {
        self.transposition_table = transposition_table;
    }


    fn distance(&self, a: (i8, i8), b: (i8, i8)) -> i8 {
        ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as i8
    }

    fn get_possible_moves(&mut self, is_maximizing_player: bool) -> Vec<(i8, i8)> {
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
        let mut vec_moves: Vec<_> = moves.into_iter().collect();
        vec_moves.sort_by(|a, b| {
            let da1 = self.distance(*a, self.last_move_p1);
            let da2 = self.distance(*a, self.last_move_p2);
            let db1 = self.distance(*b, self.last_move_p1);
            let db2 = self.distance(*b, self.last_move_p2);
            let da = da1.min(da2);
            let db = db1.min(db2);
            da.cmp(&db)
        });
    


        let second_half_start = vec_moves.len() / 2;
        let mut second_half: Vec<_> = vec_moves.split_off(second_half_start);
        let num_to_remove = (second_half.len() as f64 * 0.20).round() as usize;

        let rng = &mut rand::thread_rng();
        let indices_to_remove: Vec<_> = (0..second_half.len()).choose_multiple(rng, num_to_remove);
        second_half = second_half.into_iter().enumerate().filter(|(i, _)| !indices_to_remove.contains(i)).map(|(_, item)| item).collect();

        vec_moves.extend(second_half);



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
    fn get_consequtive_pieces_score(&mut self, player: Piece) -> i32 {
        let mut score = 0;
        for x in 2..16 {
            for y in 2..16 {
                if self.map[x][y] == player {
                    let directions = self.is_part_of_line(x, y, player);
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
                            score += i16::MAX as i32;
                        }
                        if open_line > 0 {
                            score += match consequtive_pieces {
                                4 => 100_000,
                                3 => 10_000,
                                2 => 1_000,
                                1 => 100,
                                _ => 0,
                            } * open_line;
                        }
                    }
                }
            }
        }
        score
    }

    // should be reviewed but it's working.
    fn get_heuristic(&mut self) -> i32 {
        match self.check_win() {
            (true,Piece::Player1) => return WINNING_BONUS,
            (true,Piece::Player2) => return LOSING_PENALTY,
            _ => (),
        }
        let mut score = 0;
        score += self.get_consequtive_pieces_score(Piece::Player1);
        score -= (self.get_consequtive_pieces_score(Piece::Player2));
        if self.captured1 > 0 {
            score += self.captured1 as i32 * 100;
        }
        if self.captured2 > 0 {
            score -= self.captured2 as i32 * 100;
        }
        if self.captured1 > self.captured2 + 2 {
            score += 1_000;
        }
        // score -= ((self.get_consequtive_pieces_score(Piece::Player2) as f32) * 1.2) as i32;
        score
    }


    fn minimax(&mut self, depth: i8, mut alpha: i32, mut beta: i32, is_maximizing_player: bool) -> Move {

        let mut possible_moves = self.get_possible_moves(is_maximizing_player);
        if depth == 0 {
            return Move { index: (0, 0), score: self.get_heuristic() };
        }
        let winner = self.check_win();
        if winner == (true, Piece::Player1) || winner == (true, Piece::Player2) {
            // println!("depth: {}, winner: {}", depth, winner.1);
            return Move { index: (0,0), score: if (winner == (true, Piece::Player1)) { i32::MAX } else { i32::MIN }};
        }
        let state_string = self.state_to_string();
        if let Some(cached_move) = self.get_transposition_table().get(&state_string) {
            return *cached_move;
        }
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

        let best_move = Move { index: best_move, score: best_score };
        self.get_transposition_table().insert(state_string, best_move);
        best_move
    }

    fn best_move(&mut self) -> (i8, i8) {
        // let mut best_score = i32::MIN;
        // let mut best_index = (0, 0);
        // let mut best_move = Move { index: (0, 0), score: i32::MIN };

        // for i in 1..5 {
        //     self.get_transposition_table().clear();
        //     best_move = self.minimax(i, i32::MIN, i32::MAX, true);
        //     println!("depth: {}, best:score: {}", i, best_move.score);
        //     if best_move.score > best_score {
        //         best_score = best_move.score;
        //         best_index = best_move.index;
        //     }
        //     // Update the transposition table with the best move found at this depth
        //     let state_string = self.state_to_string();
        //     self.get_transposition_table().insert(state_string, best_move);
        // }

        // best_index
        self.minimax(3, i32::MIN, i32::MAX, true).index
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
