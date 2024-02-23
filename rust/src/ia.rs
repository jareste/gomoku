extern crate wasm_bindgen;
use web_sys::console;
use crate::game::Game;
// use crate::game;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Game {
    pub fn place_ia(&self) -> u8 {
        console::log_1(&"place_ia".into());
        0
    }
}

