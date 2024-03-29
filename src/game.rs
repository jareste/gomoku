use std::time::Instant;
use crate::ia::IA;
use std::collections::HashMap;
use std::process::exit;
use bevy::prelude::*;
use rand::Rng;
use std::fmt;
use crate::ia::Move;
use crate::constants::POSSIBILITIES;
use std::sync::Mutex;
use lazy_static::lazy_static;


use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

lazy_static! {
    static ref TIMES: Mutex<Vec<f64>> = Mutex::new(Vec::new());
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    Empty,
    Player1,
    Player2,
}

pub fn string_to_map(input: &str) -> [[Piece; 19]; 19] {
    let lines = input.lines().collect::<Vec<_>>();
    let mut map = [[Piece::Empty; 19]; 19];

    for (i, line) in lines.iter().enumerate() {
        for (j, character) in line.split_whitespace().enumerate() {
            map[i][j] = match character {
                "X" => Piece::Player1,
                "O" => Piece::Player2,
                _ => Piece::Empty,
            };
        }
    }

    map
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

#[derive(Resource, Debug, Component, PartialEq, Clone, Copy)]
pub struct Game {
    pub map: [[Piece; 19]; 19],
    pub heat_map: [[f32; 19]; 19],
    pub captured1: i8,
    pub captured2: i8,
    pub movements: i16,
    pub last_move1: (i8, i8),
    pub last_move2: (i8, i8),
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: [[Piece::Empty; 19]; 19],
            heat_map: [[0.0; 19]; 19],
            captured1: 0,
            captured2: 0,
            movements: 0,
            last_move1: (0, 0),
            last_move2: (0, 0),
        }
    }

    pub fn restart(&mut self) {
        self.map = [[Piece::Empty; 19]; 19];
        self.heat_map = [[0.0; 19]; 19];
        self.captured1 = 0;
        self.captured2 = 0;
        self.movements = 0;
        self.last_move1 = (0, 0);
        self.last_move2 = (0, 0);
    }
    
    pub fn update_heat_map(&mut self, last_move: (i8, i8)) {
        let (x, y) = last_move;
        for dx in -2..=2 {
            for dy in -2..=2 {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && nx < 19 && ny >= 0 && ny < 19 {
                    let distance = dx.abs().max(dy.abs());
                    let heat = 3 - distance;
                    let heat = heat as f32 * (1.0 + self.movements as f32 / 100.0);
                    self.heat_map[nx as usize][ny as usize] += heat;
                }
            }
        }
    }

    pub fn update_game_ia(&mut self, x: usize, y: usize) -> bool {
        if !self.validate_movement(x, y, Piece::Player2) {
            return false;
        }
        if !self.place(x, y, Piece::Player2) {
            return false;
        }
        self.update_heat_map((x as i8, y as i8));
        if self.check_win() == (true, Piece::Player2) {
            return true;
        }
        self.place_ia(1);
        true
    }

    fn validate_movement(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        if x < 0 || x > 18 || y < 0 || y > 18 {
            return false;
        }
        if self.map[x][y] != Piece::Empty {
            return false;
        }
        self.map[x][y] = piece;
        if self.find_free_threes((x as i8, y as i8), 1, piece) {
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
        self.update_heat_map((x as i8, y as i8));
        true
    }

    pub fn place(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        self.map[x][y] = piece;
        match piece {
            Piece::Player1 => {
                self.last_move1 = (x as i8, y as i8);
                self.capture(x, y, piece, Piece::Player2);
            },
            Piece::Player2 => {
                self.last_move2 = (x as i8, y as i8);
                self.capture(x, y, piece, Piece::Player1);
        },
            _ => (),
        }
        true
    }

    pub fn place_ia(&mut self, pl:i32) -> (usize, usize) {
        self.movements += 1;
        let start = Instant::now();
        let (x, y) = match pl {
            1 => self.best_move(),
            2 => self.worst_move(),
            _ => !panic!("Invalid player"),
        };
        let player = match pl {
            1 => Piece::Player1,
            2 => Piece::Player2,
            _ => !panic!("Invalid player"),
        };
        let oponent = match pl {
            1 => Piece::Player2,
            2 => Piece::Player1,
            _ => !panic!("Invalid player"),
        };
        let duration = start.elapsed();
        self.map[x as usize][y as usize] = player;
        self.capture(x as usize, y as usize, player, oponent);
        self.update_heat_map((x, y));

        let time = duration.as_secs_f64();
        let mut times = TIMES.lock().unwrap();
        times.push(time);

        times.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let median = if times.len() % 2 == 0 {
            (times[times.len() / 2 - 1] + times[times.len() / 2]) / 2.0
        } else {
            times[times.len() / 2]
        };

        println!("Time elapsed in placing the piece: {:?}", time);
        println!("Median time: {:?}", median);
        println!("movements: {:?}", self.movements);
        (x as usize, y as usize)
    }

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

    fn capture_direction(&mut self, x: isize, y: isize, dx: isize, dy: isize, piece: Piece, o_piece: Piece) {
        if (1..3).all(|i| self.map.get((x + i * dx) as usize).and_then(|row| row.get((y + i * dy) as usize)) == Some(&o_piece))
            && self.map.get((x + 3 * dx) as usize).and_then(|row| row.get((y + 3 * dy) as usize)) == Some(&piece) {

            self.map[(x + 1 * dx) as usize][(y + 1 * dy) as usize] = Piece::Empty;
            self.map[(x + 2 * dx) as usize][(y + 2 * dy) as usize] = Piece::Empty;
            if piece == Piece::Player1 {
                self.captured1 += 2;
            } else {
                self.captured2 += 2;
            }
        }
    }

    fn capture(&mut self, x: usize, y: usize, piece: Piece, o_piece: Piece) {
        let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
        for &(dx, dy) in &directions {
            self.capture_direction(x as isize, y as isize, dx, dy, piece, o_piece);
            self.capture_direction(x as isize, y as isize, -dx, -dy, piece, o_piece);
        }
    }

    pub fn start_ia(&mut self)
    {
        self.place(9, 9, Piece::Player1);
        self.update_heat_map((9, 9));
    }

    pub fn start_ia_random(&mut self)
    {
        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(0..19);
        let y: usize = rng.gen_range(0..19);
        self.place(x, y, Piece::Player1);
        self.update_heat_map((x as i8, y as i8));

    }

    pub fn find_free_threes(&self, last_move: (i8, i8), quantity: i8, piece: Piece) -> bool {
        let mut free_three_p1: i8 = 0;
        let mut free_three_p2: i8 = 0;
        let mut free_three_p1_positions: Vec<(i8, i8)> = Vec::new();
        let mut free_three_p2_positions: Vec<(i8, i8)> = Vec::new();
        for x in 1..16 {
            for y in 1..16 {
                if self.map[x][y] == piece {
                    if let [a, b, c, d, e, f] = [
                        self.map[x - 1][y],
                        piece,
                        self.map[x + 1][y],
                        self.map[x + 2][y],
                        self.map[x + 3][y],
                        if x + 4 < 19 { self.map[x + 4][y] } else { Piece::Empty },
                    ] {
                        let sequence = [a, b, c, d, e, f];
                        if POSSIBILITIES.contains(&sequence) {
                            match piece {
                                Piece::Player1 => {
                                    free_three_p1_positions.push(((x - 1) as i8, y as i8));
                                    free_three_p1_positions.push((x as i8, y as i8));
                                    free_three_p1_positions.push(((x + 1) as i8, y as i8));
                                    free_three_p1_positions.push(((x + 2) as i8, y as i8));
                                    free_three_p1_positions.push(((x + 3) as i8, y as i8));
                                    free_three_p1 += 1
                                },
                                Piece::Player2 => {
                                    free_three_p2_positions.push(((x - 1) as i8, y as i8));
                                    free_three_p2_positions.push((x as i8, y as i8));
                                    free_three_p2_positions.push(((x + 1) as i8, y as i8));
                                    free_three_p2_positions.push(((x + 2) as i8, y as i8));
                                    free_three_p2_positions.push(((x + 3) as i8, y as i8));
                                    free_three_p2 += 1
                                },
                                _ => (),
                            }
                        }
                    }
                    if let [a, b, c, d, e, f] = [
                        self.map[x][y - 1],
                        piece,
                        self.map[x][y + 1],
                        self.map[x][y + 2],
                        self.map[x][y + 3],
                        if y + 4 < 19 { self.map[x][y + 4] } else { Piece::Empty },
                    ] {
                        let sequence = [a, b, c, d, e, f];
                        if POSSIBILITIES.contains(&sequence) {
                            match piece {
                                Piece::Player1 => {
                                    free_three_p1_positions.push((x as i8, (y - 1) as i8));
                                    free_three_p1_positions.push((x as i8, y as i8));
                                    free_three_p1_positions.push((x as i8, (y + 1) as i8));
                                    free_three_p1_positions.push((x as i8, (y + 2) as i8));
                                    free_three_p1_positions.push((x as i8, (y + 3) as i8));
                                    free_three_p1 += 1
                                },
                                Piece::Player2 => {
                                    free_three_p2_positions.push((x as i8, (y - 1) as i8));
                                    free_three_p2_positions.push((x as i8, y as i8));
                                    free_three_p2_positions.push((x as i8, (y + 1) as i8));
                                    free_three_p2_positions.push((x as i8, (y + 2) as i8));
                                    free_three_p2_positions.push((x as i8, (y + 3) as i8));
                                    free_three_p2 += 1
                                },
                                _ => (),
                            }
                        }
                    }
                    if let [a, b, c, d, e, f] = [
                        self.map[x - 1][y - 1],
                        piece,
                        self.map[x + 1][y + 1],
                        self.map[x + 2][y + 2],
                        self.map[x + 3][y + 3],
                        if x + 4 < 19 && y + 4 < 19 { self.map[x + 4][y + 4] } else { Piece::Empty },
                    ] {
                        let sequence = [a, b, c, d, e, f];
                        if POSSIBILITIES.contains(&sequence) {
                            match piece {
                                Piece::Player1 => {
                                    free_three_p1_positions.push(((x - 1) as i8, (y - 1) as i8));
                                    free_three_p1_positions.push((x as i8, y as i8));
                                    free_three_p1_positions.push(((x + 1) as i8, (y + 1) as i8));
                                    free_three_p1_positions.push(((x + 2) as i8, (y + 2) as i8));
                                    free_three_p1_positions.push(((x + 3) as i8, (y + 3) as i8));
                                    free_three_p1 += 1
                                },
                                Piece::Player2 => {
                                    free_three_p2_positions.push(((x - 1) as i8, (y - 1) as i8));
                                    free_three_p2_positions.push((x as i8, y as i8));
                                    free_three_p2_positions.push(((x + 1) as i8, (y + 1) as i8));
                                    free_three_p2_positions.push(((x + 2) as i8, (y + 2) as i8));
                                    free_three_p2_positions.push(((x + 3) as i8, (y + 3) as i8));
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
                        piece,
                        self.map[x - 1][y + 1],
                        self.map[x - 2][y + 2],
                        self.map[x - 3][y + 3],
                        if x > 3 && y + 4 < 19 { self.map[x - 4][y + 4] } else { Piece::Empty },
                    ] {
                        let sequence = [a, b, c, d, e, f];
                        if POSSIBILITIES.contains(&sequence) {
                            match self.map[x][y] {
                                Piece::Player1 => {
                                    free_three_p1_positions.push(((x + 1) as i8, (y - 1) as i8));
                                    free_three_p1_positions.push((x as i8, y as i8));
                                    free_three_p1_positions.push(((x - 1) as i8, (y + 1) as i8));
                                    free_three_p1_positions.push(((x - 2) as i8, (y + 2) as i8));
                                    free_three_p1_positions.push(((x - 3) as i8, (y + 3) as i8));
                                    free_three_p1 += 1
                                },
                                Piece::Player2 => {
                                    free_three_p2_positions.push(((x + 1) as i8, (y - 1) as i8));
                                    free_three_p2_positions.push((x as i8, y as i8));
                                    free_three_p2_positions.push(((x - 1) as i8, (y + 1) as i8));
                                    free_three_p2_positions.push(((x - 2) as i8, (y + 2) as i8));
                                    free_three_p2_positions.push(((x - 3) as i8, (y + 3) as i8));
                                    free_three_p2 += 1
                                },
                                _ => (),
                            }
                        }
                    }
                }
            }
        };

        if free_three_p1 > quantity || free_three_p2 > quantity {
            if piece == Piece::Player1 {
                if free_three_p1_positions.contains(&last_move) {
                    return true;
                }
            }
            else {
                if free_three_p2_positions.contains(&last_move) {
                    return true;
                }
            }
        }
        false
    }

    pub fn hint(&mut self, player: i8) -> (i8, i8) {
        if player == 1 {
            self.best_move()
        } else {
            self.worst_move()
        }
    }
}
