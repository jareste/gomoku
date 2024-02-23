extern crate wasm_bindgen;
use web_sys::console;
use crate::game::{Game, Piece};
// use crate::game;

use wasm_bindgen::prelude::*;

pub fn get_possible_moves(map: &[[Piece; 19]; 19]) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    for x in 0..19 {
        for y in 0..19 {
            if map[x][y] == Piece::Empty {
                moves.push((x as i8, y as i8));
            }
        }
    }
    moves
}



