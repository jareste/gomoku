extern crate wasm_bindgen;
use web_sys::console;
use crate::game::{Game, Piece};
// use crate::game;

use wasm_bindgen::prelude::*;

struct Move {
    index: (i8, i8),
    score: i32,
}

fn get_possible_moves(map: &[[Piece; 19]; 19]) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    for x in 0..18 {
        for y in 0..18 {
            if map[x][y] == Piece::Empty {
                moves.push((x as i8, y as i8));
            }
        }
    }
    moves
}

// this function will check the squares around the possible movement and return false if there is no piece in squares_to_check around the movement
fn dfs_check_movement(map: &[[Piece; 19]; 19], x: i8, y: i8, squares_to_check: i8) -> bool {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    for &(dx, dy) in &directions {
        for i in 1..=squares_to_check {
            let nx = x + i * dx;
            let ny = y + i * dy;
            if nx < 0 || ny < 0 || nx >= 19 || ny >= 19 {
                break;
            }
            if map[nx as usize][ny as usize] != Piece::Empty {
                return true;
            }
        }
    }
    false
}


fn get_heuristic(map: &[[Piece; 19]; 19]) -> i32 {
    let mut score = 0;
    score
}


fn minimax(map: &[[Piece; 19]; 19], depth: u32, is_maximizing_player: bool) -> Move {
    if depth == 0 || get_possible_moves(map).is_empty() {
        return Move { index: (0, 0), score: get_heuristic(&map) };
    }

    if is_maximizing_player {
        let mut best_score = i32::MIN;
        let mut best_move = (0, 0);

        for &moves in get_possible_moves(map).iter() {
            if !dfs_check_movement(&map, moves.0, moves.1, 3) {
                continue;
            }
            let mut new_game = map.clone();
            new_game[moves.0 as usize][moves.1 as usize] = Piece::Player1;
            let score = minimax(&new_game, depth - 1, false).score;
            if score > best_score {
                best_score = score;
                best_move = moves;
            }
        }
        Move { index: best_move, score: best_score }
    } else {
        let mut best_score = i32::MAX;
    
        let mut best_move = (0, 0);
        for &moves in get_possible_moves(map).iter() {
            if !dfs_check_movement(&map, moves.0, moves.1, 3) {
                continue;
            }
            let mut new_game = map.clone();
            new_game[moves.0 as usize][moves.1 as usize] = Piece::Player2;
            let score = minimax(&new_game,depth - 1, true).score;
            if score < best_score {
                best_score = score;
                best_move = moves;
            }
        }
        Move { index: best_move, score: best_score }
    }
}

pub fn best_move(map: &[[Piece; 19]; 19]) -> (i8, i8) {
    minimax(map, 10,  true).index
}