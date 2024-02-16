#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Empty,
    Player1,
    Player2,
}

pub struct Game {
    map: [[Option<Piece>; 19]; 19],
    captured1: i8,
    captured2: i8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            map: [[Some(Piece::Empty); 19]; 19],
            captured1: 0,
            captured2: 0,
        }
    }

    fn print_map(&self) {
        for i in 0..19 {
            for j in 0..19 {
                match self.map[i][j] {
                    Some(Piece::Empty) => print!("-"),
                    Some(Piece::Player1) => print!("X"),
                    Some(Piece::Player2) => print!("O"),
                    None => print!(" "),
                }
            }
            println!();
        }
    }

    fn check_valid(&self, x: usize, y: usize) -> bool {
        if x < 0 || x > 18 || y < 0 || y > 18 {
            return false;
        }
        if self.map[x][y].is_some() && self.map[x][y] != Some(Piece::Empty) {
            return false;
        }
        true
    }

    pub fn place(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        if self.map[x][y].is_some() && self.map[x][y] != Some(Piece::Empty){
            return false;
        }
        self.map[x][y] = Some(piece);
        self.print_map();
        true
    }

    // pub fn capture(&mut self, x: usize, y: usize, piece: Piece) {
    //     let mut captured = 0;

    //     if x > 0 && self.map[x - 1][y] == Some(piece) {
    //         captured += 1;
    //     }
    //     if x < 18 && self.map[x + 1][y] == Some(piece) {
    //         captured += 1;
    //     }
    //     if y > 0 && self.map[x][y - 1] == Some(piece) {
    //         captured += 1;
    //     }
    //     if y < 18 && self.map[x][y + 1] == Some(piece) {
    //         captured += 1;
    //     }

    //     if captured == 4 {
    //         self.map[x][y] = Some(Piece::Empty);

    //         if piece == Piece::Player1 {
    //             self.captured1 += 1;
    //         } else {
    //             self.captured2 += 1;
    //         }
    //     }
    // }

}


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
        println!("input: {:?}", input);
        numbers = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

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
        println!("numbers: {:?}", numbers);
        numbers.clear();
        println!("numbers: {:?}", numbers);
        movements += 1;
    }
}