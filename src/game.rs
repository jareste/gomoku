// use std::process::exit;
use std::time::Instant;
use crate::ia::IA;
use std::collections::HashMap;
// use crate::ia::best_move;
use std::process::exit;
use bevy::prelude::*;
use std::fmt;
use crate::constants::{FREE_THREE_FIVE, FREE_THREE_SIX, FIVE_IN_A_ROW};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Empty,
    Player1,
    Player2,
}


impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Piece::Empty => write!(f, "-"),
            Piece::Player1 => write!(f, "X"),
            Piece::Player2 => write!(f, "O"),
        }
    }
}

impl Piece {
    pub fn as_str(&self) -> &str {
        match self {
            Piece::Empty => "Empty",
            Piece::Player1 => "Player1",
            Piece::Player2 => "Player2",
        }
    }
}

#[derive(Resource, Debug, Component, PartialEq, Clone)]
pub struct Game {
    pub map: [[Piece; 19]; 19],
    pub values: [[i32; 19]; 19],
    pub score: i32,
    pub captured1: i8,
    pub captured2: i8,
    pub last_move_p1: (i8, i8),
    pub last_move_p2: (i8, i8),
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: [[Piece::Empty; 19]; 19],
            values: [[0; 19]; 19],
            score: 0,
            captured1: 0,
            captured2: 0,
            last_move_p1: (-1, -1),
            last_move_p2: (-1, -1),
        }
    }
    
    fn validate_movement(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        if x < 0 || x > 18 || y < 0 || y > 18 {
            return false;
        }
        if self.map[x][y] != Piece::Empty {
            return false;
        }
        self.map[x][y] = piece;
        if self.find_free_threes((x as i8, y as i8), 1) {
            self.map[x][y] = Piece::Empty;
            return false;
        }
        self.map[x][y] = Piece::Empty;
        true
    }

    pub fn update_game(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        if !self.validate_movement(x, y, piece) {
            return false;
        }
        if !self.place(x, y, piece) {
            return false;
        }
        true
    }

    pub fn update_game_ia(&mut self, x: usize, y: usize) -> bool {
        if !self.validate_movement(x, y, Piece::Player2) {
            return false;
        }
        if !self.place(x, y, Piece::Player2) {
            return false;
        }
        if self.check_win() == (true, Piece::Player2) {
            return true;
        }
        self.place_ia();
        // self.score = self.evaluate_map(Piece::Player1);
        // println!("scoremap before1 : {}", self.score);
        // self.score = self.evaluate_map(Piece::Player2);
        // println!("scoremap after2 : {}", self.score);
        true
    }

    pub fn place(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        self.map[x][y] = piece;
        match piece {
            Piece::Player1 => {
                self.capture(x, y, piece, Piece::Player2);
                self.last_move_p1 = (x as i8, y as i8);
            },
            Piece::Player2 => {
                self.capture(x, y, piece, Piece::Player1);
                self.last_move_p2 = (x as i8, y as i8);
        },
            _ => (),
        }
        true
    }

    pub fn place_ia(&mut self) -> (usize, usize) {
        let start = Instant::now();
        let (x, y) = self.best_move();
        let duration = start.elapsed();
        self.map[x as usize][y as usize] = Piece::Player1;
        self.capture(x as usize, y as usize, Piece::Player1, Piece::Player2);

        println!("Time elapsed in placing the piece: {:?}", duration.as_secs_f64());
        println!("IA placed at x: {} y: {}", x, y);
        (x as usize, y as usize)
    }

    // terminal game HELPER FUNCTION
    pub fn print_map(&self) {
        for i in 0..19 {
            for j in 0..19 {
                match self.map[i][j] {
                    Piece::Empty => print!("-  "),
                    Piece::Player1 => print!("X  "),
                    Piece::Player2 => print!("O  "),
                }
            }
            println!();
        }
        println!("Captured 1: {} | Captured2: {}", self.captured1, self.captured2);
    }

    pub fn check_five_in_a_row(&self, piece: Piece, start_x: usize, start_y: usize, dx: isize, dy: isize) -> bool {
        (0..5).all(|i| {
            let x = (start_x as isize + i * dx) as usize;
            let y = (start_y as isize + i * dy) as usize;
            self.map.get(x).and_then(|row| row.get(y)) == Some(&piece)
        })
    }

    pub fn check_win(&self) -> (bool, Piece) {
        match (self.captured1 >= 10, self.captured2 >= 10) {
            (true, _) => return (true, Piece::Player1),
            (_, true) => return (true, Piece::Player2),
            _ => (),
        }

        for i in 0..19 {
            for j in 0..19 {
                let piece = self.map[i][j];
                if piece == Piece::Empty {
                    continue;
                }

                let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];
                for &(dx, dy) in &directions {
                    if self.check_five_in_a_row(piece, i, j, dx, dy) {
                        return (true, piece);
                    }
                }
            }
        }

        (false, Piece::Empty)
    }

    // NEW CAPTURE FUNCTIONS MAYBE NOT WORKING AS EXPECTED
    fn capture_direction(&mut self, x: isize, y: isize, dx: isize, dy: isize, piece: Piece, o_piece: Piece) {
        if (1..3).all(|i| self.map.get((x + i * dx) as usize).and_then(|row| row.get((y + i * dy) as usize)) == Some(&o_piece))
            && self.map.get((x + 3 * dx) as usize).and_then(|row| row.get((y + 3 * dy) as usize)) == Some(&piece) {

            self.map[(x + 1 * dx) as usize][(y + 1 * dy) as usize] = Piece::Empty;
            self.values[(x + 1 * dx) as usize][(y + 1 * dy) as usize] = 0;
            self.map[(x + 2 * dx) as usize][(y + 2 * dy) as usize] = Piece::Empty;
            self.values[(x + 2 * dx) as usize][(y + 2 * dy) as usize] = 0;
            if piece == Piece::Player1 {
                self.captured1 += 2;
            } else {
                self.captured2 += 2;
            }
        }
    }

    // NEW CAPTURE FUNCTIONS MAYBE NOT WORKING AS EXPECTED
    fn capture(&mut self, x: usize, y: usize, piece: Piece, o_piece: Piece) {
        let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
        // let prev_capture1 = self.captured1;
        // let prev_capture2 = self.captured2;
        for &(dx, dy) in &directions {
            self.capture_direction(x as isize, y as isize, dx, dy, piece, o_piece);
            self.capture_direction(x as isize, y as isize, -dx, -dy, piece, o_piece);
        }
        // (self.captured1 - prev_capture1, self.captured2 - prev_capture2)
    }

    pub fn start_ia(&mut self)
    {
        self.map[9][9] = Piece::Player1;
    }

    pub fn find_free_threes(&mut self, last_move: (i8, i8), quantity: i8) -> bool {
        let piece = self.map[last_move.0 as usize][last_move.1 as usize];
        let mut free_three_p1: i8 = 0;
        let mut free_three_p2: i8 = 0;
        for x in 1..16 {
            for y in 1..16 {
                if self.map[x][y] == piece {
                    // println!("no petardea");
                    // checking X vertical up
                    if let [a, b, c, d, e, f] = [
                        self.map[x - 1][y],
                        self.map[x][y],
                        self.map[x + 1][y],
                        self.map[x + 2][y],
                        self.map[x + 3][y],
                        if x + 4 < 19 { self.map[x + 4][y] } else { Piece::Empty },
                    ] {
                        let sequence = [a, b, c, d, e, f];
                        // println!("sequence: {:?}", sequence);
                        if FREE_THREE_SIX.contains(&sequence) {
                            match piece {
                                Piece::Player1 => {
                                    if free_three_p1 > quantity {
                                        return true;
                                    }
                                    free_three_p1 += 1
                                },
                                Piece::Player2 => {
                                    if free_three_p2 > quantity {
                                        return true;
                                    }
                                    free_three_p2 += 1
                                },
                                _ => (),
                            }
                        }
                    }
                    if let [a, b, c, d, e, f] = [
                        self.map[x][y - 1],
                        self.map[x][y],
                        self.map[x][y + 1],
                        self.map[x][y + 2],
                        self.map[x][y + 3],
                        if y + 4 < 19 { self.map[x][y + 4] } else { Piece::Empty },
                    ] {
                        let sequence = [a, b, c, d, e, f];
                        if FREE_THREE_SIX.contains(&sequence) {
                            match piece {
                                Piece::Player1 => {
                                    if free_three_p1 > quantity {
                                        return true;
                                    }
                                    free_three_p1 += 1
                                },
                                Piece::Player2 => {
                                    if free_three_p2 > quantity {
                                        return true;
                                    }
                                    free_three_p2 += 1
                                },
                                _ => (),
                            }
                        }
                    }
                    // checking diagonal up right /
                    if let [a, b, c, d, e, f] = [
                        self.map[x - 1][y - 1],
                        self.map[x][y],
                        self.map[x + 1][y + 1],
                        self.map[x + 2][y + 2],
                        self.map[x + 3][y + 3],
                        if x + 4 < 19 && y + 4 < 19 { self.map[x + 4][y + 4] } else { Piece::Empty },
                    ] {
                        let sequence = [a, b, c, d, e, f];
                        if FREE_THREE_SIX.contains(&sequence) {
                            match piece {
                                Piece::Player1 => {
                                    if free_three_p1 > quantity {
                                        return true;
                                    }
                                    free_three_p1 += 1
                                },
                                Piece::Player2 => {
                                    if free_three_p2 > quantity {
                                        return true;
                                    }
                                    free_three_p2 += 1
                                },
                                _ => (),
                            }
                        }
                    }
                    if x < 3 {
                        continue;
                    }
                    if let [a, b, c, d, e, f] = [
                        self.map[x + 1][y - 1],
                        self.map[x][y],
                        self.map[x - 1][y + 1],
                        self.map[x - 2][y + 2],
                        self.map[x - 3][y + 3],
                        if x > 3 && y + 4 < 19 { self.map[x - 4][y + 4] } else { Piece::Empty },
                    ] {
                        let sequence = [a, b, c, d, e, f];
                        // println!("sequence: {:?}", sequence);
                        if FREE_THREE_SIX.contains(&sequence) {
                            match self.map[x][y] {
                                Piece::Player1 => {
                                    if free_three_p1 > quantity {
                                        return true;
                                    }
                                    free_three_p1 += 1
                                },
                                Piece::Player2 => {
                                    if free_three_p2 > quantity {
                                        return true;
                                    }
                                    free_three_p2 += 1
                                },
                                _ => (),
                            }
                        }
                    }
                }
            }
        };

        // println!("free three p1: {:?} | free three p2: {:?}", free_three_p1, free_three_p2);
        if free_three_p1 > quantity || free_three_p2 > quantity {
            return true;
        }
        false
    }
}






/*





// HELPER FUNCTION FOR TESTING
use std::io::{self, Write};
pub fn terminal_game() {
    let mut game = Game::new();
    let mut input = String::new();
    let mut movements: usize = 0;
    loop {
        if movements % 2 == 0 {
            println!("Player 1, please enter your move (x y): ");
        }
        else {
            println!("Player 2, please enter your move (x y): ");
        }
        io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        
        if numbers.len() != 2 {
            println!("numbers: {:?}", numbers);
            println!("You must enter exactly two integers!");
            continue;
        }
        if numbers[0] <0 || numbers[0] > 18 || numbers[1] < 0 || numbers[1] > 18 {
            println!("You must enter numbers between 0 and 18!");
            continue;
        }
        if movements % 2 == 0 {
            if !game.place(numbers[0] as usize, numbers[1] as usize, Piece::Player1) {
                println!("You can't place a piece there!");
                continue;
            }
        }
        else {
            if !game.place(numbers[0] as usize, numbers[1] as usize, Piece::Player2) {
                println!("You can't place a piece there!");
                continue;
            }
        }
        movements += 1;
        game.print_map();
    }
}


pub fn terminal_game_ia() {
    let mut game = Game::new();
    let mut input = String::new();
    // let mut numbers: Vec<i32> = Vec::new();
    let mut movements: usize = 1;
    game.start_ia();
    game.print_map();
    loop {
        println!("Player 2, please enter your move (x y): ");
        io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let numbers: Vec<i32> = input.split_whitespace().filter_map(|s| s.parse().ok()).collect();
        
        if numbers.len() != 2 {
            println!("numbers: {:?}", numbers);
            println!("You must enter exactly two integers!");
            continue;
        }
        if numbers[0] <0 || numbers[0] > 18 || numbers[1] < 0 || numbers[1] > 18 {
            println!("You must enter numbers between 0 and 18!");
            continue;
        }

        if !game.place(numbers[0] as usize, numbers[1] as usize, Piece::Player2) {
            println!("You can't place a piece there!");
            continue;
        }
        game.place_ia();
        // numbers.clear();
        movements += 1;
        println!("movements: {:?}", movements);
        if game.check_win() {
            break;
        }
    }
}
*/