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

const WINNING_BONUS: i32 = 1_000_000;
const LOSING_PENALTY: i32 = -1_000_000;
const THREATENING_BONUS: i32 = 100_000;

pub trait IA{
    fn dfs_check_movement(&mut self, x: i8, y: i8, squares_to_check: i8) -> bool;
    fn get_possible_moves(&mut self) -> Vec<(i8, i8)>;
    fn get_consequtive_pieces_score(&mut self, player: Piece) -> i32;
    fn get_heuristic(&mut self) -> i32;
    fn minimax(&mut self, depth: i8, alpha: i32, beta: i32, is_maximizing_player: bool) -> Move;
    fn best_move(&mut self) -> (i8, i8);
    fn is_part_of_line(&mut self, x: usize, y: usize, player: Piece) -> Vec<(isize, isize)>;
    fn evaluate_move(&mut self, moves: (i8, i8), player: Piece) -> i32;
    fn get_heuristic_moves(&mut self, possible_moves: &Vec<(i8, i8)>, is_maximizing_player: bool) -> Vec<Move>;
    // fn iddfs(&mut self, max_depth: i8) -> Move;
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
                        for i in 1..=2 {
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
        let mut score = 0;
        score += self.get_consequtive_pieces_score(Piece::Player1);
        score -= (self.get_consequtive_pieces_score(Piece::Player2));
        match self.check_win() {
            (true,Piece::Player1) => score += WINNING_BONUS,
            (true,Piece::Player2) => score -= LOSING_PENALTY,
            _ => (),
        }
        // score -= ((self.get_consequtive_pieces_score(Piece::Player2) as f32) * 1.2) as i32;
        score
    }

    fn evaluate_move(&mut self, moves: (i8, i8), player: Piece) -> i32 {
        let (x, y) = moves;
        let mut score = 0;

        let state = self.map.iter().map(|row| row.iter().map(|cell| cell.to_string()).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("");
        if let Some(score) = self.transposition_table.get(&state) {
            return *score;
        }

        self.map[x as usize][y as usize] = player;

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        for &(dx, dy) in &directions {
            let mut consecutive_pieces = 0;
            for i in -1..=1 {
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
        self.transposition_table.insert(state, score);
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
        let mut possible_moves = self.get_possible_moves();
        if depth == 0 {
            return Move { index: (0, 0), score: self.get_heuristic() };
        }
        let game_state = self.to_string();

        if let Some(score) = self.transposition_table.get(&game_state) {
            return Move { index: (0, 0), score: *score };
        }
        let mut best_move = (0, 0);
        let mut best_score = if is_maximizing_player { i32::MIN } else { i32::MAX };
       // possible_moves.sort_by_key(|&moves| -self.evaluate_move(moves, Piece::Player1));
        for &moves in possible_moves.iter() {
            let mut new_game = self.clone();
            if !new_game.place(moves.0 as usize, moves.1 as usize, if is_maximizing_player { Piece::Player1 } else { Piece::Player2 }) {
                continue;
            }
            let score = new_game.minimax(depth - 1, alpha, beta, !is_maximizing_player).score;
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
        self.transposition_table.insert(game_state, best_score);
        Move { index: best_move, score: best_score }
    }
  
     fn best_move(&mut self) -> (i8, i8) {
         self.minimax(3, i32::MIN, i32::MAX, true).index
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
