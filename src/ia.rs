use std::collections::HashSet;
use crate::game::{Game, Piece};

pub struct Move {
    pub x: i8,
    pub y: i8,
    pub score: i8,
}

pub trait IA{
    // fn get_possible_moves(&mut self) -> Vec<(i8, i8, i128)>;
    fn get_possible_moves(&mut self, distance: i8) -> Vec<(i8, i8, i128)>;
    fn minmax(&mut self, depth: i8, is_maximizing: bool, alpha: i128, beta: i128) -> (i128, (i8, i8));
    fn best_move(&mut self) -> (i8, i8);
    fn check_consequtives(&self, x: i8, y: i8, dx: i8, dy: i8, player: Piece) -> i8;
    // fn evaluate(&mut self, player: Piece, new_move: (i8, i8)) -> i128;
    fn get_consequtive_pieces_score(&mut self, player: Piece) -> i128;
}

impl IA for Game {
    let score_cache: HashMap<((i8, i8), Piece), i128> = HashMap::new();

    fn get_possible_moves(&mut self, distance: i8) -> Vec<(i8, i8, i128)> {
        let mut possible_moves = Vec::new();
        let movements: HashSet<((i8, i8), Piece)> = self.movements.iter().cloned().collect();
        for moves in movements.iter() {
            let (x, y) = moves.0;
            for dx in -distance..=distance {
                for dy in -distance..=distance {
                    let new_pos = ((x + dx) as i8, (y + dy) as i8);
                    if !self.find_free_threes(new_pos, 1) {
                        if !self.movements.contains(&((new_pos.0, new_pos.1), Piece::Player1)) &&
                        !self.movements.contains(&((new_pos.0, new_pos.1), Piece::Player2)) && 
                        new_pos.0 >= 0 && new_pos.0 < 19 && new_pos.1 >= 0 && new_pos.1 < 19 &&
                        self.map[new_pos.0 as usize][new_pos.1 as usize] == Piece::Empty {
                            let evaluation = self.get_consequtive_pieces_score(Piece::Player1) - self.get_consequtive_pieces_score(Piece::Player2);
                            possible_moves.push((new_pos.0, new_pos.1, evaluation));
                        }
                    }
                }
            }
        }
        possible_moves
    }

    fn check_consequtives(&self, x: i8, y: i8, dx: i8, dy: i8, player: Piece) -> i8 {
        let mut count = 0;
        let mut new_x = x;
        let mut new_y = y;
        while new_x >= 0 && new_x < 19 && new_y >= 0 && new_y < 19 && self.map[new_x as usize][new_y as usize] == player {
            count += 1;
            new_x += dx;
            new_y += dy;
        }
        count
    }

    fn get_consequtive_pieces_score(&mut self, player: Piece) -> i128 {
        let key = (player, self.movements.len());
        if let Some(score) = self.score_cache.get(&key) {
            return *score;
        }
        let mut score = 0;
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    
        // Convert the HashSet to a Vec and filter by player
        let player_movements: Vec<((i8, i8), Piece)> = self.movements.iter().cloned().filter(|&(_, piece)| piece == player).collect();
    
        for &((x, y), _) in &player_movements {
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
                        if self.movements.contains(&((nx as i8, ny as i8), player)) {
                            consequtive_pieces += 1;
                        } else if !self.movements.contains(&((nx as i8, ny as i8), Piece::Empty)) {
                            open_line -= 1;
                        }
                    } else if !self.movements.contains(&((nx as i8, ny as i8), Piece::Empty)) {
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
        self.score_cache.insert(key, score);
        score
    }



    // fn evaluate(&mut self, player: Piece, new_move: (i8, i8)) -> i128 {
    //     let mut score = 0;
    //     let directions = [(0, 1), (1, 0), (1, 1), (1, -1), (0, -1), (-1, 0), (-1, -1), (-1, 1)];
    //     if self.map[new_move.0 as usize][new_move.1 as usize] != Piece::Empty {
    //         println!("position: {:?} is not empty, has piece : {:?}", new_move, self.map[new_move.0 as usize][new_move.1 as usize]);
    //         return 0;
    //     }
    //     self.map[new_move.0 as usize][new_move.1 as usize] = player;
    //     for moves in &self.movements {
    //         let (x, y) = moves.0;
    //         for (dx, dy) in &directions {
    //             let count = self.check_consequtives(x, y, *dx, *dy, player);
    //         }
    //     }
    //     self.map[new_move.0 as usize][new_move.1 as usize] = Piece::Empty;
    //     score
    // }

    fn minmax(&mut self, depth: i8, is_maximizing: bool, mut alpha: i128, mut beta: i128) -> (i128, (i8, i8)) {
        // let game_state = self.to_string();
        // if let Some(score) = self.transposition_table.get(&game_state) {
        //     return (score.0, (0, 0));
        // }
        let possible_moves = self.get_possible_moves(1);
        
        if depth == 0 {
            return (0, (0, 0));
        }
        let mut best_move = possible_moves[0];
        let mut best_score = if is_maximizing { i128::MIN } else { i128::MAX };
        for moves in possible_moves {
            let mut new_game = self.clone();
            new_game.map[moves.0 as usize][moves.1 as usize] = if is_maximizing { Piece::Player1 } else { Piece::Player2 };
            new_game.movements.insert(((moves.0, moves.1), if is_maximizing { Piece::Player1 } else { Piece::Player2 }));
            let result = new_game.minmax(depth - 1, !is_maximizing, alpha, beta);
            let new_game_state = new_game.to_string();
            // self.transposition_table.insert(new_game_state, (result.0, result.1, depth));
            match is_maximizing {
                true => {
                    if result.0 > best_score {
                        best_score = result.0;
                        best_move = moves;
                    }
                    alpha = alpha.max(result.0);
                },
                false => {
                    if result.0 < best_score {
                        best_score = result.0;
                        best_move = moves;
                    }
                    beta = beta.min(result.0);
                }
            }
            if beta <= alpha {
                break;
            }
        }
        (best_score, (best_move.0, best_move.1))
    }

    fn best_move(&mut self) -> (i8, i8) {
        self.minmax(3, true, i128::MIN, i128::MAX).1
    }
}