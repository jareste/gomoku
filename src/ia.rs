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
    fn evaluate_move(&mut self, moves: (i8, i8), player: Piece) -> i32;
    fn get_heuristic_moves(&mut self, possible_moves: &Vec<(i8, i8)>, is_maximizing_player: bool) -> Vec<Move>;
    fn distance(&self, a: (i8, i8), b: (i8, i8)) -> i8;

    // fn iddfs(&mut self, max_depth: i8) -> Move;
}


impl IA for Game {
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
        let last_move = match is_maximizing_player {
            true => self.last_move_p1,
            false => self.last_move_p2,
        };
        vec_moves.sort_by(|a, b| {
            let da = self.distance(*a, last_move);
            let db = self.distance(*b, last_move);
            da.cmp(&db)
        });
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
        for x in 0..19 {
            for y in 0..19 {
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
                        if open_line > 0 {
                            score += match consequtive_pieces {
                                5 => 1_000_000,
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

    fn evaluate_move(&mut self, moves: (i8, i8), player: Piece) -> i32 {
        let (x, y) = moves;
        let mut score = 0;

        self.map[x as usize][y as usize] = player;

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        for &(dx, dy) in &directions {
            let mut consecutive_pieces = 0;
            for i in -4..=4 { // Change this line
                let nx = x as isize + i * dx;
                let ny = y as isize + i * dy;
                if nx >= 0 && ny >= 0 && nx < 19 && ny < 19 && self.map[nx as usize][ny as usize] == player {
                    consecutive_pieces += 1;
                }
            }
            match player {
                Piece::Empty => (),
                Piece::Player1 => score += consecutive_pieces,
                Piece::Player2 => score -= consecutive_pieces,
            }
        }
        self.map[x as usize][y as usize] = Piece::Empty;
        score
    }

    fn get_heuristic_moves(&mut self, possible_moves: &Vec<(i8, i8)>, is_maximizing_player: bool) -> Vec<Move> {
        let mut heuristic_moves = Vec::new();
        for &moves in possible_moves.iter() {
            let mut ia = self.clone();
            ia.map[moves.0 as usize][moves.1 as usize] = if is_maximizing_player { Piece::Player1 } else { Piece::Player2 };
            let score = ia.evaluate_move(moves, if is_maximizing_player { Piece::Player1 } else { Piece::Player2 });
            let moves = Move { index: moves, score: score };
            heuristic_moves.push(moves);
        }
        heuristic_moves.sort_by_key(|&k| -k.score);
        heuristic_moves.into_iter().collect()
    }


    fn minimax(&mut self, depth: i8, mut alpha: i32, mut beta: i32, is_maximizing_player: bool) -> Move {
        let mut possible_moves = self.get_possible_moves(is_maximizing_player);
        if depth == 0 {
            return Move { index: (0, 0), score: self.get_heuristic() };
        }
        let mut best_move = (0, 0);
        let mut best_score = if is_maximizing_player { i32::MIN } else { i32::MAX };
        
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
        if best_move == (5, 11) {
            println!("score: {:?}", best_score);
        }
        Move { index: best_move, score: best_score }
    }

    fn best_move(&mut self) -> (i8, i8) {
        let best_move = self.minimax(3, i32::MIN, i32::MAX, true);
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