use std::collections::HashSet;
use crate::game::{Game, Piece};

pub struct Move {
    pub x: i8,
    pub y: i8,
    pub score: i8,
}

pub trait IA{
    fn get_possible_moves(&self) -> Vec<(i8, i8)>;
    fn minmax(&mut self, depth: i8, is_maximizing: bool) -> (i128, (i8, i8));
    fn best_move(&mut self) -> (i8, i8);
}

impl IA for Game {

    fn get_possible_moves(&self) -> Vec<(i8, i8)> {
        let mut possible_moves = Vec::new();
        for moves in &self.movements {
            let (x, y) = moves.0;
            for dx in -2..=2 {
                for dy in -2..=2 {
                    let new_pos = ((x + dx) as i8, (y + dy) as i8);
                    if !self.movements.iter().any(|&((x, y), _)| (x, y) == new_pos) {
                        possible_moves.push(new_pos);
                    }
                }
            }
             
        }
        println!("possible moves: {:?}", possible_moves);
        possible_moves
    }

    fn minmax(&mut self, depth: i8, is_maximizing: bool) -> (i128, (i8, i8)) {
        let mut best_score: i128 = if is_maximizing { -1000 } else { 1000 };
        let mut best_move: (i8, i8) = (0, 0);
        let mut moves = HashSet::new();
        for i in 0..3 {
            for j in 0..3 {
                if self.map[i][j] == Piece::Empty {
                    moves.insert((i, j));
                }
            }
        }
        for (i, j) in moves {
            if is_maximizing {
                self.map[i][j] = Piece::Player1;
                let score = self.minmax(depth - 1, false).0.into();
                self.map[i][j] = Piece::Empty;
                if score > best_score {
                    best_score = score;
                    best_move = (i as i8, j  as i8);
                }
            } else {
                self.map[i][j] = Piece::Player2;
                let score: i128 = self.minmax(depth - 1, true).0.into();
                self.map[i][j] = Piece::Empty;
                if score < best_score {
                    best_score = score;
                    best_move = (i  as i8, j  as i8);
                }
            }
        }
        (best_score, best_move)
    }

    fn best_move(&mut self) -> (i8, i8) {
        self.get_possible_moves();
        //self.minmax(3, true)
        (10, 10)
    }
}