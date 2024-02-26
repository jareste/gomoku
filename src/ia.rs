use std::collections::HashSet;
use std::thread;
use crate::game::{Game, Piece};
// use crate::game;


struct Move {
    index: (i8, i8),
    score: i32,
}

pub trait IA{
    fn dfs_check_movement(&mut self, x: i8, y: i8, squares_to_check: i8) -> bool;
    fn get_possible_moves(&mut self) -> Vec<(i8, i8)>;
    fn get_consequtive_pieces_score(&mut self, player: Piece) -> i32;
    fn get_heuristic(&mut self) -> i32;
    fn minimax(&mut self, depth: i8, alpha: i32, beta: i32, is_maximizing_player: bool) -> Move;
    fn best_move(&mut self) -> (i8, i8);
    fn is_part_of_line(&mut self, x: usize, y: usize, player: Piece) -> Vec<(isize, isize)>;
    fn evaluate_move(&mut self, moves: (i8, i8), player: Piece) -> i32;
}


impl IA for Game {
    // this function will check the squares around the possible movement and return false if there is no piece in squares_to_check around the movement
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
                    let player = self.map[x][y];
                    for &(dx, dy) in &directions {
                        for i in 1..=2 {
                            let nx = x as isize + i * dx;
                            let ny = y as isize + i * dy;
                            if nx >= 0 && ny >= 0 && nx < 19 && ny < 19 && self.map[nx as usize][ny as usize] == Piece::Empty  {
                                self.map[nx as usize][ny as usize] = player;
                                if !self.find_free_threes(player, (nx as i8, ny as i8), 1) {
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
        // let directions: [(isize, isize); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
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

    fn get_heuristic(&mut self) -> i32 {
        let mut score = 0;
        score += self.get_consequtive_pieces_score(Piece::Player1);
        score -= self.get_consequtive_pieces_score(Piece::Player2);
        score
    }

    fn evaluate_move(&mut self, moves: (i8, i8), player: Piece) -> i32 {
        let (x, y) = moves;
        let mut score = 0;

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
            score += consecutive_pieces;
        }

        self.map[x as usize][y as usize] = Piece::Empty;

        score
    }

    fn minimax(&mut self, depth: i8, mut alpha: i32, mut beta: i32, is_maximizing_player: bool) -> Move {
        if depth == 0 {
            return Move { index: (0, 0), score: self.get_heuristic() };
        }
        let game_state = self.to_string();

        if let Some(score) = self.transposition_table.get(&game_state) {
            return Move { index: (0, 0), score: *score };
        }
        let mut best_move = (0, 0);
        let mut best_score = if is_maximizing_player { i32::MIN } else { i32::MAX };
        let mut possible_moves = self.get_possible_moves();
        possible_moves.sort_by_key(|&moves| -self.evaluate_move(moves, Piece::Player1));
        for &moves in possible_moves.iter() {
            self.map[moves.0 as usize][moves.1 as usize] = if is_maximizing_player { Piece::Player1 } else { Piece::Player2 };
            // this will check if the direct next movement is a winning movement so it stops.
            if depth == 3 { 
                let score = self.get_consequtive_pieces_score(if is_maximizing_player { Piece::Player1 } else { Piece::Player2 });
                if score >= 1_000_000 {
                    self.map[moves.0 as usize][moves.1 as usize] = Piece::Empty;
                    return Move { index: moves, score };
                }
            }
            let mut score = self.minimax(depth - 1, alpha, beta, !is_maximizing_player).score;
            self.map[moves.0 as usize][moves.1 as usize] = Piece::Empty;
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
}

// fn capture_direction(&mut self, x: isize, y: isize, dx: isize, dy: isize, piece: Piece, o_piece: Piece) -> bool {
//     if (1..3).all(|i| self.map.get((x + i * dx) as usize).and_then(|row| row.get((y + i * dy) as usize)) == Some(&o_piece))
//         && self.map.get((x + 3 * dx) as usize).and_then(|row| row.get((y + 3 * dy) as usize)) == Some(&piece) {
//         return true;
//     }
//     false
// }

// fn capture(&mut self, x: usize, y: usize, piece: Piece, o_piece: Piece) -> bool {
//     let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
//     for &(dx, dy) in &directions {
//         if capture_direction(&self.map, x as isize, y as isize, dx, dy, piece, o_piece) {
//             return true;
//         }
//         if capture_direction(&self.map, x as isize, y as isize, -dx, -dy, piece, o_piece) {
//             return true;
//         }
//     }
//     false
// }
