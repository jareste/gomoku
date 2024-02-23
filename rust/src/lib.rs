extern crate cfg_if;
extern crate wasm_bindgen;
mod game;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// #[wasm_bindgen]
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum Piece {
//     Empty,
//     Player1,
//     Player2,
// }

// #[wasm_bindgen]
// pub struct Game {
//     map: [[Option<Piece>; 19]; 19],
//     captured1: i8,
//     captured2: i8,
// }

// #[wasm_bindgen]
// impl Game {
//     pub fn new() -> Self {
//         Self {
//             map: [[Some(Piece::Empty); 19]; 19],
//             captured1: 0,
//             captured2: 0,
//         }
//     }
    
//     pub fn place(&mut self, x: usize, y: usize, piece: Piece) -> bool {
//         if self.map[x][y].is_some() && self.map[x][y] != Some(Piece::Empty){
//             return false;
//         }
//         self.map[x][y] = Some(piece);
//         if piece == Piece::Player1 {
//             self.capture(x, y, piece, Piece::Player2);
//         } else {
//             self.capture(x, y, piece, Piece::Player1);
//         }
//         if self.check_actual_free_three(x, y, piece) {
//             self.map[x][y] = Some(Piece::Empty);
//             return false;
//         }
//         true
//     }

//     // NEW CHECK WIN FUNCTION MAYBE NOT WORKING AS EXPECTED
//     pub fn check_win(&self) -> bool {
//         match (self.captured1 >= 10, self.captured2 >= 10) {
//             (true, _) => return true,
//             (_, true) => return true,
//             _ => (),
//         }

//         for i in 0..19 {
//             for j in 0..19 {
//                 match self.map[i][j] {
//                     Some(Piece::Player1) => {
//                         if i < 15 && (1..=4).all(|k| self.map[i + k][j] == Some(Piece::Player1)) {
//                             return true
//                         }
//                         if j < 15 && self.map[i][j + 1..=j + 4].iter().all(|&x| x == Some(Piece::Player1)) {
//                             return true
//                         }
//                         if i < 15 && j < 15 && (1..=4).all(|k| self.map[i + k][j + k] == Some(Piece::Player1)) {
//                             return true
//                         }
//                         if i < 15 && j > 3 && (1..=4).all(|k| self.map[i + k][j - k] == Some(Piece::Player1)) {
//                             return true
//                         }
//                     }
//                     Some(Piece::Player2) => {
//                         if i < 15 && (1..=4).all(|k| self.map[i + k][j] == Some(Piece::Player2)) {
//                             return true
//                         }
//                         if j < 15 && (1..=4).all(|k| self.map[i][j + k] == Some(Piece::Player2)) {
//                             return true
//                         }
//                         if i < 15 && j < 15 && (1..=4).all(|k| self.map[i + k][j + k] == Some(Piece::Player2)) {
//                             return true
//                         }
//                         if i < 15 && j > 3 && (1..=4).all(|k| self.map[i + k][j - k] == Some(Piece::Player2)) {
//                             return true
//                         }
//                     }
//                     _ => (),
//                 }
//             }
//         }
//         false
//     }

//     // NEW CAPTURE FUNCTIONS MAYBE NOT WORKING AS EXPECTED
//     fn capture_direction(&mut self, x: isize, y: isize, dx: isize, dy: isize, piece: Piece, o_piece: Piece) {
//         if (0..3).all(|i| self.map.get((x + i * dx) as usize).and_then(|row| row.get((y + i * dy) as usize)) == Some(&Some(o_piece)))
//             && self.map.get((x + 3 * dx) as usize).and_then(|row| row.get((y + 3 * dy) as usize)) == Some(&Some(piece)) {
//                 for i in 1..=2 {
//                 if let Some(row) = self.map.get_mut((x + i * dx) as usize) {
//                     if let Some(cell) = row.get_mut((y + i * dy) as usize) {
//                         *cell = Some(Piece::Empty);
//                     }
//                 }
//             }
//             if piece == Piece::Player1 {
//                 self.captured1 += 2;
//             } else {
//                 self.captured2 += 2;
//             }
//         }
//     }

//     // NEW CAPTURE FUNCTIONS MAYBE NOT WORKING AS EXPECTED
//     fn capture(&mut self, x: usize, y: usize, piece: Piece, o_piece: Piece) {
//         let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
//         for &(dx, dy) in &directions {
//             self.capture_direction(x as isize, y as isize, dx, dy, piece, o_piece);
//             self.capture_direction(x as isize, y as isize, -dx, -dy, piece, o_piece);
//         }
//     }

    
   
                    
//     // free three are three pieces that if another one is added got no counterplay.
//     // example: - X X - X -
//     // example: - X - X X -
//     // example: - X X X -
//     // first i must check if the actual movement creates a free three
//     //  *****************************************************************************************************************************
//     fn check_actual_free_three(&mut self, x: usize, y: usize, piece: Piece) -> bool {
//         let mut free_three = 0;

//         // check horizontal
//         match piece {
//             Piece::Player1 => {
//                 if y > 2 {
//                     if self.map[x][y - 1] == Some(Piece::Player1) && self.map[x][y - 2] == Some(Piece::Player1) && self.map[x][y - 3] == Some(Piece::Empty) {
//                         free_three += 1;
//                     }
//                 }
//                 if y < 15 {
//                     if self.map[x][y + 1] == Some(Piece::Player1) && self.map[x][y + 2] == Some(Piece::Player1) && self.map[x][y + 3] == Some(Piece::Empty) {
//                         free_three += 1;
//                     }
//                 }
//             }
//             Piece::Player2 => {
//                 if y > 2 {
//                     if self.map[x][y - 1] == Some(Piece::Player2) && self.map[x][y - 2] == Some(Piece::Player2) && self.map[x][y - 3] == Some(Piece::Empty) {
//                         free_three += 1;
//                     }
//                 }
//                 if y < 15 {
//                     if self.map[x][y + 1] == Some(Piece::Player2) && self.map[x][y + 2] == Some(Piece::Player2) && self.map[x][y + 3] == Some(Piece::Empty) {
//                         free_three += 1;
//                     }
//                 }
//             }
//             _ => (),
//         }


//         false
//     }
                    
// }