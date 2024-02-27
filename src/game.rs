// use std::process::exit;
use std::time::Instant;
use crate::ia::IA;
use std::collections::HashMap;
use crate::tess::math::Vector;
// use crate::ia::best_move;
use std::process::exit;
use bevy::prelude::*;
use std::fmt;
use std::collections::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub captured1: i8,
    pub captured2: i8,
    pub movements: HashSet<((i8, i8), Piece)>,
    pub transposition_table: HashMap<String, (i128, (i8, i8), i8)>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: [[Piece::Empty; 19]; 19],
            captured1: 0,
            captured2: 0,
            transposition_table: HashMap::new(),
            movements: HashSet::new(),
        }
    }
    
    pub fn to_string(&self) -> String {
        self.map.iter()
            .flat_map(|row| row.iter())
            .map(|piece| match piece {
                Piece::Empty => "0",
                Piece::Player1 => "1",
                Piece::Player2 => "2",
            })
            .collect()
    }

    pub fn place(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        println!("siiii");
        if self.map[x][y] != Piece::Empty {
            return false;
        }
        self.map[x][y] = piece;
        if piece == Piece::Player1 {
            self.capture(x, y, piece, Piece::Player2);
        } else {
            self.capture(x, y, piece, Piece::Player1);
        }
        if self.find_free_threes( (x as i8, y as i8), 1) {
            self.map[x][y] = Piece::Empty;
            return false;
        }
        // self.print_map();
        self.movements.insert(((x as i8, y as i8), piece));
        true
    }

    pub fn place_ia(&mut self) -> (usize, usize) {
        // self.map[9][9] = Piece::Player1;
        let start = Instant::now();
        let (x, y) = self.best_move();
        let duration = start.elapsed();
        self.map[x as usize][y as usize] = Piece::Player1;
        self.capture(x as usize, y as usize, Piece::Player1, Piece::Player2);

        // self.print_map();
        println!("Time elapsed in placing the piece: {:?}", duration.as_secs_f64());
        println!("IA placed at x: {} y: {}", x, y);
        self.movements.insert(((x, y), Piece::Player1));
        println!("Movements: {:?}", self.movements);
        (x as usize, y as usize)
    }

    // terminal game HELPER FUNCTION
    fn print_map(&self) {
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

    // NEW CHECK WIN FUNCTION MAYBE NOT WORKING AS EXPECTED
    pub fn check_win(&self) -> bool {
        match (self.captured1 >= 10, self.captured2 >= 10) {
            (true, _) => return true,
            (_, true) => return true,
            _ => (),
        }

        println!("Checking win");
        for i in 0..19 {
            for j in 0..19 {
                match self.map[i][j] {
                    Piece::Player1 => {
                        if i < 15 && (1..=4).all(|k| self.map[i + k][j] == Piece::Player1) {
                            self.print_map();
                            return true;
                        }
                        if j < 15 && self.map[i][j + 1..=j + 4].iter().all(|&x| x == Piece::Player1) {
                            self.print_map();
                            return true;
                        }
                        if i < 15 && j < 15 && (1..=4).all(|k| self.map[i + k][j + k] == Piece::Player1) {
                            self.print_map();
                            return true;
                        }
                        if i < 15 && j > 3 && (1..=4).all(|k| self.map[i + k][j - k] == Piece::Player1) {
                            self.print_map();
                            return true;
                        }
                    }
                    Piece::Player2 => {
                        if i < 15 && (1..=4).all(|k| self.map[i + k][j] == Piece::Player2) {
                            self.print_map();
                            return true;
                        }
                        if j < 15 && (1..=4).all(|k| self.map[i][j + k] == Piece::Player2) {
                            self.print_map();
                            return true;
                        }
                        if i < 15 && j < 15 && (1..=4).all(|k| self.map[i + k][j + k] == Piece::Player2) {
                            self.print_map();
                            return true;
                        }
                        if i < 15 && j > 3 && (1..=4).all(|k| self.map[i + k][j - k] == Piece::Player2) {
                            self.print_map();
                            return true;
                        }
                    }
                    _ => (),
                }
            }
        }
        self.print_map();
        false
    }

    // NEW CAPTURE FUNCTIONS MAYBE NOT WORKING AS EXPECTED
    fn capture_direction(&mut self, x: isize, y: isize, dx: isize, dy: isize, piece: Piece, o_piece: Piece) {
        if (1..3).all(|i| self.map.get((x + i * dx) as usize).and_then(|row| row.get((y + i * dy) as usize)) == Some(&o_piece))
            && self.map.get((x + 3 * dx) as usize).and_then(|row| row.get((y + 3 * dy) as usize)) == Some(&piece) {

            let pos1 = ((x + 1 * dx) as i8 , (y + 1 * dy) as i8);
            let pos2 = ((x + 2 * dx) as i8, (y + 2 * dy) as i8);

            let pos1_piece = self.movements.take(&((pos1.0, pos1.1), o_piece));
            let pos2_piece = self.movements.take(&((pos2.0, pos2.1), o_piece));

            if pos1_piece.is_some() {
                self.movements.remove(&pos1_piece.unwrap());
            }
            if pos2_piece.is_some() {
                self.movements.remove(&pos2_piece.unwrap());
            }
            self.map[(x + 1 * dx) as usize][(y + 1 * dy) as usize] = Piece::Empty;
            self.map[(x + 2 * dx) as usize][(y + 2 * dy) as usize] = Piece::Empty;
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
        for &(dx, dy) in &directions {
            self.capture_direction(x as isize, y as isize, dx, dy, piece, o_piece);
            self.capture_direction(x as isize, y as isize, -dx, -dy, piece, o_piece);
        }
    }

    pub fn start_ia(&mut self)
    {
        self.map[9][9] = Piece::Player1;
        self.movements.insert(((9, 9), Piece::Player1));
    }

    // function to check all the free threes in the board for a selected player and keep in memory positions of the actuals one that have been visited.
    // idea of doing it with a match as them are the only possible pieces
    // i want to check always from the first piece of the sequence so then i got no issuues with finding multiples at same time
    pub fn find_free_threes(&mut self, last_move: (i8, i8), quantity: i8) -> bool {
        let posibilities = [
        (Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Empty), // - X X X -
        (Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1), // - X X X -
        (Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player2), // - X X X -
        (Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Empty), // - O O O -
        (Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player1), // - O O O -
        (Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2), // - O O O -
        (Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Empty), // - X X - X -
        (Piece::Empty, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty), // - X - X X -
        (Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Empty), // - O O - O -
        (Piece::Empty, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty), // - O - O O -
        ];

        let mut free_three_p1: i8 = 0;
        let mut free_three_p2: i8 = 0;

        let (last_x, last_y) = (last_move.0 as usize, last_move.1 as usize);

        for direction in &[(1, 0), (0, 1), (1, 1), (-1, 1)] {
            let (dx, dy) = direction;
            if let Some(sequence) = self.get_sequence(last_x, last_y, *dx as isize, *dy as isize) {
                if posibilities.contains(&sequence) {
                    match self.map[last_x][last_y] {
                        Piece::Player1 => free_three_p1 += 1,
                        Piece::Player2 => free_three_p2 += 1,
                        _ => (),
                    }
                }
            }
        }

        if free_three_p1 > quantity || free_three_p2 > quantity {
            return true;
        }
        false
    }

        // Helper function to get a sequence starting from a position in a specific direction
        fn get_sequence(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<(Piece, Piece, Piece, Piece, Piece, Piece)> {
            let mut sequence = Vec::new();
            for i in 0..6 {
                let nx = x as isize + i * dx;
                let ny = y as isize + i * dy;
                if nx >= 0 && ny >= 0 && nx < 19 && ny < 19 {
                    sequence.push(self.map[nx as usize][ny as usize]);
                } else {
                    return None;
                }
            }
            Some((sequence[0], sequence[1], sequence[2], sequence[3], sequence[4], sequence[5]))
        }
                    
}

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
