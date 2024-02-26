// use std::process::exit;
use std::time::Instant;
use crate::ia::IA;

// use crate::ia::best_move;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Empty,
    Player1,
    Player2,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Game {
    pub map: [[Piece; 19]; 19],
    pub captured1: i8,
    pub captured2: i8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: [[Piece::Empty; 19]; 19],
            captured1: 0,
            captured2: 0,
        }
    }
    
    pub fn place(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        if self.map[x][y] != Piece::Empty {
            return false;
        }
        self.map[x][y] = piece;
        if piece == Piece::Player1 {
            self.capture(x, y, piece, Piece::Player2);
        } else {
            self.capture(x, y, piece, Piece::Player1);
        }
        if self.find_free_threes(piece, 1) {
            self.map[x][y] = Piece::Empty;
            return false;
        }
        // self.print_map();
        true
    }

    pub fn place_ia(&mut self)  {
        // self.map[9][9] = Piece::Player1;
        let (x, y) = self.best_move();
        self.map[x as usize][y as usize] = Piece::Player1;
        self.capture(x as usize, y as usize, Piece::Player1, Piece::Player2);
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

        for i in 0..19 {
            for j in 0..19 {
                match self.map[i][j] {
                    Piece::Player1 => {
                        if i < 15 && (1..=4).all(|k| self.map[i + k][j] == Piece::Player1) {
                            return true;
                        }
                        if j < 15 && self.map[i][j + 1..=j + 4].iter().all(|&x| x == Piece::Player1) {
                            return true;
                        }
                        if i < 15 && j < 15 && (1..=4).all(|k| self.map[i + k][j + k] == Piece::Player1) {
                            return true;
                        }
                        if i < 15 && j > 3 && (1..=4).all(|k| self.map[i + k][j - k] == Piece::Player1) {
                            return true;
                        }
                    }
                    Piece::Player2 => {
                        if i < 15 && (1..=4).all(|k| self.map[i + k][j] == Piece::Player2) {
                            return true;
                        }
                        if j < 15 && (1..=4).all(|k| self.map[i][j + k] == Piece::Player2) {
                            return true;
                        }
                        if i < 15 && j < 15 && (1..=4).all(|k| self.map[i + k][j + k] == Piece::Player2) {
                            return true;
                        }
                        if i < 15 && j > 3 && (1..=4).all(|k| self.map[i + k][j - k] == Piece::Player2) {
                            return true;
                        }
                    }
                    _ => (),
                }
            }
        }
        false
    }

    // NEW CAPTURE FUNCTIONS MAYBE NOT WORKING AS EXPECTED
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

    // NEW CAPTURE FUNCTIONS MAYBE NOT WORKING AS EXPECTED
    fn capture(&mut self, x: usize, y: usize, piece: Piece, o_piece: Piece) {
        let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
        for &(dx, dy) in &directions {
            self.capture_direction(x as isize, y as isize, dx, dy, piece, o_piece);
            self.capture_direction(x as isize, y as isize, -dx, -dy, piece, o_piece);
        }
    }

    pub fn start_IA(&mut self)
    {
        self.map[9][9] = Piece::Player1;
    }

    // function to check all the free threes in the board for a selected player and keep in memory positions of the actuals one that have been visited.
    // idea of doing it with a match as them are the only possible pieces
    // i want to check always from the first piece of the sequence so then i got no issuues with finding multiples at same time
    pub fn find_free_threes(&mut self, piece: Piece, quantity: i8) -> bool {
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
                        let sequence = (a, b, c, d, e, f);
                        // println!("sequence: {:?}", sequence);
                        if posibilities.contains(&sequence) {
                            // println!("free three found!");
                            match piece {
                                Piece::Player1 => free_three_p1 += 1,
                                Piece::Player2 => free_three_p2 += 1,
                                _ => (),
                            }
                        }
                    }
                    // checking Y horizontal right
                    if let [a, b, c, d, e, f] = [
                        self.map[x][y - 1],
                        self.map[x][y],
                        self.map[x][y + 1],
                        self.map[x][y + 2],
                        self.map[x][y + 3],
                        if y + 4 < 19 { self.map[x][y + 4] } else { Piece::Empty },
                    ] {
                        let sequence = (a, b, c, d, e, f);
                        // println!("sequence: {:?}", sequence);
                        if posibilities.contains(&sequence) {
                            // println!("free three found!");
                            match piece {
                                Piece::Player1 => free_three_p1 += 1,
                                Piece::Player2 => free_three_p2 += 1,
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
                        let sequence = (a, b, c, d, e, f);
                        // println!("sequence: {:?}", sequence);
                        if posibilities.contains(&sequence) {
                            // println!("free three found!");
                            match piece {
                                Piece::Player1 => free_three_p1 += 1,
                                Piece::Player2 => free_three_p2 += 1,
                                _ => (),
                            }
                        }
                    }
                    if x < 3 {
                        continue;
                    }
                    // checking diagonal down right \
                    if let [a, b, c, d, e, f] = [
                        self.map[x + 1][y - 1],
                        self.map[x][y],
                        self.map[x - 1][y + 1],
                        self.map[x - 2][y + 2],
                        self.map[x - 3][y + 3],
                        if x > 3 && y + 4 < 19 { self.map[x - 4][y + 4] } else { Piece::Empty },
                    ] {
                        let sequence = (a, b, c, d, e, f);
                        // println!("sequence: {:?}", sequence);
                        if posibilities.contains(&sequence) {
                            match self.map[x][y] {
                                Piece::Player1 => free_three_p1 += 1,
                                Piece::Player2 => free_three_p2 += 1,
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        if free_three_p1 > quantity || free_three_p2 > quantity {
            return true;
        }
        false
    }
                    
}

// HELPER FUNCTION FOR TESTING
use core::panic;
use std::io::{self, Write};
pub fn terminal_game() {
    let mut game = Game::new();
    let mut input = String::new();
    let mut numbers: Vec<i32> = Vec::new();
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
        numbers = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        
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
        numbers.clear();
        movements += 1;
        game.print_map();
    }
}


pub fn terminal_game_IA() {
    let mut game = Game::new();
    let mut input = String::new();
    let mut numbers: Vec<i32> = Vec::new();
    let mut movements: usize = 0;
    game.start_IA();
    game.print_map();
    loop {
        println!("Player 2, please enter your move (x y): ");
        io::stdout().flush().unwrap(); // Make sure the prompt is immediately displayed
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        numbers = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        
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
        let start = Instant::now();
        game.place_ia();
        let duration = start.elapsed();
        numbers.clear();
        movements += 1;
        game.print_map();
        println!("Time elapsed in placing the piece: {:?}", duration.as_secs_f64());
        println!("movements: {:?}", movements);
    }
}
