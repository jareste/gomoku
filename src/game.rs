use std::process::exit;

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
    
    pub fn place(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        if self.map[x][y].is_some() && self.map[x][y] != Some(Piece::Empty){
            return false;
        }
        self.map[x][y] = Some(piece);
        self.capture(x, y, piece);
        self.print_map();
        let (win, message) = self.check_win();
        if win {
            println!("{}", message);
            println!("Game over!");
            exit(0);
        }
        true
    }

    // terminal game
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
        println!("Captured 1: {} | Captured2: {}", self.captured1, self.captured2);
    }

    fn check_win(&self) -> (bool, String) {
        if self.captured1 >= 10 {
            return (true, "Player 1 wins!".to_string());
        }
        if self.captured2 >= 10 {
            return (true, "Player 2 wins!".to_string());
        }
        for i in 0..19 {
            for j in 0..19 {
                if self.map[i][j] == Some(Piece::Player1) {
                    if i < 15 {
                        if self.map[i + 1][j] == Some(Piece::Player1) && self.map[i + 2][j] == Some(Piece::Player1) && self.map[i + 3][j] == Some(Piece::Player1) && self.map[i + 4][j] == Some(Piece::Player1) {
                            return (true, "Player 1 wins!".to_string());
                        }
                    }
                    if j < 15 {
                        if self.map[i][j + 1] == Some(Piece::Player1) && self.map[i][j + 2] == Some(Piece::Player1) && self.map[i][j + 3] == Some(Piece::Player1) && self.map[i][j + 4] == Some(Piece::Player1) {
                            return (true, "Player 1 wins!".to_string());
                        }
                    }
                    if i < 15 && j < 15 {
                        if self.map[i + 1][j + 1] == Some(Piece::Player1) && self.map[i + 2][j + 2] == Some(Piece::Player1) && self.map[i + 3][j + 3] == Some(Piece::Player1) && self.map[i + 4][j + 4] == Some(Piece::Player1) {
                            return (true, "Player 1 wins!".to_string());
                        }
                    }
                    if i < 15 && j > 3 {
                        if self.map[i + 1][j - 1] == Some(Piece::Player1) && self.map[i + 2][j - 2] == Some(Piece::Player1) && self.map[i + 3][j - 3] == Some(Piece::Player1) && self.map[i + 4][j - 4] == Some(Piece::Player1) {
                            return (true, "Player 1 wins!".to_string());
                        }
                    }
                }
                if self.map[i][j] == Some(Piece::Player2) {
                    if i < 15 {
                        if self.map[i + 1][j] == Some(Piece::Player2) && self.map[i + 2][j] == Some(Piece::Player2) && self.map[i + 3][j] == Some(Piece::Player2) && self.map[i + 4][j] == Some(Piece::Player2) {
                            return (true, "Player 1 wins!".to_string());
                        }
                    }
                    if j < 15 {
                        if self.map[i][j + 1] == Some(Piece::Player2) && self.map[i][j + 2] == Some(Piece::Player2) && self.map[i][j + 3] == Some(Piece::Player2) && self.map[i][j + 4] == Some(Piece::Player2) {
                            return (true, "Player 1 wins!".to_string());
                        }
                    }
                    if i < 15 && j < 15 {
                        if self.map[i + 1][j + 1] == Some(Piece::Player2) && self.map[i + 2][j + 2] == Some(Piece::Player2) && self.map[i + 3][j + 3] == Some(Piece::Player2) && self.map[i + 4][j + 4] == Some(Piece::Player2) {
                            return (true, "Player 1 wins!".to_string());
                        }
                    }
                    if i < 15 && j > 3 {
                        if self.map[i + 1][j - 1] == Some(Piece::Player2) && self.map[i + 2][j - 2] == Some(Piece::Player2) && self.map[i + 3][j - 3] == Some(Piece::Player2) && self.map[i + 4][j - 4] == Some(Piece::Player2) {
                            return (true, "Player 1 wins!".to_string());
                        }
                    }
                }
            }
        }
        (false, "".to_string())
    }

    //i must check like expanding from the actual position to check captures or possible ones right now not working
    fn capture(&mut self, x: usize, y: usize, piece: Piece) { 
        let mut captured = 0;

        if x > 0 && self.map[x - 1][y] == Some(piece) {
            captured += 1;
        }
        if x < 18 && self.map[x + 1][y] == Some(piece) {
            captured += 1;
        }
        if y > 0 && self.map[x][y - 1] == Some(piece) {
            captured += 1;
        }
        if y < 18 && self.map[x][y + 1] == Some(piece) {
            captured += 1;
        }

        if captured == 4 {
            self.map[x][y] = Some(Piece::Empty);

            if piece == Piece::Player1 {
                self.captured1 += 1;
            } else {
                self.captured2 += 1;
            }
        }
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
        numbers.clear();
        movements += 1;
    }
}