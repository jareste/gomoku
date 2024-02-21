use std::collections::HashSet;
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
        if piece == Piece::Player1 {
            self.capture(x, y, piece, Piece::Player2);
        } else {
            self.capture(x, y, piece, Piece::Player1);
        }
        // if self.check_actual_free_three(x, y, piece) {
        //     self.map[x][y] = Some(Piece::Empty);
        //     return false;
        // }
        if self.find_free_threes(piece) {
            self.map[x][y] = Some(Piece::Empty);
            return false;
        }
        self.print_map();
        let (win, message) = self.check_win();
        if win {
            println!("{}", message);
            println!("Game over!");
            exit(0);
        }
        true
    }

    // terminal game HELPER FUNCTION
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

    // NEW CHECK WIN FUNCTION MAYBE NOT WORKING AS EXPECTED
    fn check_win(&self) -> (bool, String) {
        match (self.captured1 >= 10, self.captured2 >= 10) {
            (true, _) => return (true, "Player 1 wins!".to_string()),
            (_, true) => return (true, "Player 2 wins!".to_string()),
            _ => (),
        }

        for i in 0..19 {
            for j in 0..19 {
                match self.map[i][j] {
                    Some(Piece::Player1) => {
                        if i < 15 && (1..=4).all(|k| self.map[i + k][j] == Some(Piece::Player1)) {
                            return (true, "Player 1 wins!".to_string());
                        }
                        if j < 15 && self.map[i][j + 1..=j + 4].iter().all(|&x| x == Some(Piece::Player1)) {
                            return (true, "Player 1 wins!".to_string());
                        }
                        if i < 15 && j < 15 && (1..=4).all(|k| self.map[i + k][j + k] == Some(Piece::Player1)) {
                            return (true, "Player 1 wins!".to_string());
                        }
                        if i < 15 && j > 3 && (1..=4).all(|k| self.map[i + k][j - k] == Some(Piece::Player1)) {
                            return (true, "Player 1 wins!".to_string());
                        }
                    }
                    Some(Piece::Player2) => {
                        if i < 15 && (1..=4).all(|k| self.map[i + k][j] == Some(Piece::Player2)) {
                            return (true, "Player 2 wins!".to_string());
                        }
                        if j < 15 && (1..=4).all(|k| self.map[i][j + k] == Some(Piece::Player2)) {
                            return (true, "Player 2 wins!".to_string());
                        }
                        if i < 15 && j < 15 && (1..=4).all(|k| self.map[i + k][j + k] == Some(Piece::Player2)) {
                            return (true, "Player 2 wins!".to_string());
                        }
                        if i < 15 && j > 3 && (1..=4).all(|k| self.map[i + k][j - k] == Some(Piece::Player2)) {
                            return (true, "Player 2 wins!".to_string());
                        }
                    }
                    _ => (),
                }
            }
        }
        (false, "".to_string())
    }

    // NEW CAPTURE FUNCTIONS MAYBE NOT WORKING AS EXPECTED
    fn capture_direction(&mut self, x: isize, y: isize, dx: isize, dy: isize, piece: Piece, o_piece: Piece) {
        if (0..3).all(|i| self.map.get((x + i * dx) as usize).and_then(|row| row.get((y + i * dy) as usize)) == Some(&Some(o_piece)))
            && self.map.get((x + 3 * dx) as usize).and_then(|row| row.get((y + 3 * dy) as usize)) == Some(&Some(piece)) {
                for i in 1..=2 {
                if let Some(row) = self.map.get_mut((x + i * dx) as usize) {
                    if let Some(cell) = row.get_mut((y + i * dy) as usize) {
                        *cell = Some(Piece::Empty);
                    }
                }
            }
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

    
   
     
    // free three are three pieces that if another one is added got no counterplay.
    // example: - X X - X -
    // example: - X - X X -
    // example: - X X X -
    // first i must check if the actual movement creates a free three
    //  *****************************************************************************************************************************
    // i could generate a vector with the pieces and match them with the actuals to check if the movement creates a free three
    //  i have to check all directions from a given point.
    // AIXO NO ESTA ACTIVAT I DE MOMENT NO TIRA
    fn check_actual_free_three(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        if x == 0 || x == 18 || y == 0 || y == 18 {
            return false;
        }

        match piece {
            Piece::Player1 => {
                println!("checking free three pl1");
                // check up this checks - x X X - where the lower x is the actual position
                // println!("pieces checked: {:?} {:?} {:?} {:?} {:?}", self.map[x - 1][y] , self.map[x][y], self.map[x + 1][y], self.map[x + 2][y], self.map[x + 3][y]);
                if x > 2 && [1, -3].iter().all(|&i| self.map[(x as isize + i) as usize][y] == Some(Piece::Empty)) && (1..=2).all(|i| self.map[x - i][y] == Some(Piece::Player1)) {
                    println!("******************************************************************************************************");
                    // check for another free three;
                }
                // check up this checks - X x X - where the lower x is the actual position
                if x > 1 && x < 17 && [2, -2].iter().all(|&i| self.map[(x as isize + i) as usize][y] == Some(Piece::Empty)) && [1, -1].iter().all(|&i| self.map[(x as isize + i) as usize][y] == Some(Piece::Player1)) {
                    println!("******************************************************************************************************");
                    // check for another free three;
                }
                // check up this checks - X X x - where the lower x is the actual position
                if x < 16 && [3, -1].iter().all(|&i| self.map[(x as isize + i) as usize][y] == Some(Piece::Empty)) && (1..=2).all(|i| self.map[x + i][y] == Some(Piece::Player1)) {
                    println!("******************************************************************************************************");
                    // check for another free three;
                }

                // NEW BLOCK FOR HORIZONTAL CHECKS
                //  check horizontal this checks for - x X X - where the lower x is the actual position
                if y > 2 && [1, -3].iter().all(|&i| self.map[x][(y as isize + i) as usize] == Some(Piece::Empty)) && (1..=2).all(|i| self.map[x][y - i] == Some(Piece::Player1)) {
                    println!("------------------------------------------------------------------------------------------------------");;
                    // check for another free three;
                }
                // check horizontal this checks for - X x X - where the lower x is the actual position
                if y > 1 && y < 17 && [2, -2].iter().all(|&i| self.map[x][(y as isize + i) as usize] == Some(Piece::Empty)) && [1, -1].iter().all(|&i| self.map[x][(y as isize + i) as usize] == Some(Piece::Player1)) {
                    println!("------------------------------------------------------------------------------------------------------");;
                    // check for another free three;
                }
                println!("hola");
                // check horizontal this checks for - X X x - where the lower x is the actual position
                if y < 16 && [3, -1].iter().all(|&i| self.map[x][(y as isize + i) as usize] == Some(Piece::Empty)) && (1..=2).all(|i| self.map[x][y + i] == Some(Piece::Player1)) {
                    println!("------------------------------------------------------------------------------------------------------");;
                    // check for another free three;
                }


                // // NEW BLOCK FOR DIAGONAL UP RIGHTCHECKS

                // // check diagonal this checks for - x X X - where the lower x is the actual position
                // if x > 2 && y > 2 && [1, -3].iter().all(|&i| self.map[(x as isize + i) as usize][(y as isize + i) as usize] == Some(Piece::Empty)) && (1..=2).all(|i| self.map[x - i][y - i] == Some(Piece::Player1)) {
                //     println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
                //     // check for another free three;
                // }
                // // check diagonal this checks for - X x X - where the lower x is the actual position
                // if x > 1 && x < 17 && y > 1 && y < 17 && [2, -2].iter().all(|&i| self.map[(x as isize + i) as usize][(y as isize + i) as usize] == Some(Piece::Empty)) && [1, -1].iter().all(|&i| self.map[(x as isize + i) as usize][(y as isize + i) as usize] == Some(Piece::Player1)) {
                //     println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
                //     // check for another free three;
                // }
                // // check diagonal this checks for - X X x - where the lower x is the actual position
                // if x < 16 && y < 16 && [3, -1].iter().all(|&i| self.map[(x as isize + i) as usize][(y as isize + i) as usize] == Some(Piece::Empty)) && (1..=2).all(|i| self.map[x + i][y + i] == Some(Piece::Player1)) {
                //     println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
                //     // check for another free three;
                // }

                // // NEW BLOCK FOR HORIZONTAL DOWN CHECKS
                // // check diagonal down this checks for - x X X - where the lower x is the actual position
                // if x < 16 && y > 2 && [1, -3].iter().all(|&i| self.map[(x as isize + i) as usize][(y as isize - i) as usize] == Some(Piece::Empty)) && (1..=2).all(|i| self.map[x + i][y - i] == Some(Piece::Player1)) {
                //     println!("=======================================================================================================");
                //     // check for another free three;
                // }
                // // check diagonal down this checks for - X x X - where the lower x is the actual position
                // if x < 17 && x > 1 && y < 17 && y > 1 && [2, -2].iter().all(|&i| self.map[(x as isize + i) as usize][(y as isize - i) as usize] == Some(Piece::Empty)) && [1, -1].iter().all(|&i| self.map[(x as isize + i) as usize][(y as isize - i) as usize] == Some(Piece::Player1)) {
                //     println!("=======================================================================================================");
                //     // check for another free three;
                // }
                // // check diagonal down this checks for - X X x - where the lower x is the actual position
                // println!("check {}", (y as isize -1) as usize);
                // if x > 2 && y < 16 && [3, -1].iter().all(|&i| self.map[(x as isize + i) as usize][(y as isize - i) as usize] == Some(Piece::Empty)) && (1..=2).all(|i| self.map[x - i][y + i] == Some(Piece::Player1)) {
                //     println!("=======================================================================================================");
                //     // check for another free three;
                // }

                // check diagonal up left this checks for - x X X - where the lower x is the actual position

                // // check up this checks - x X X - where the lower x is the actual position
                // if x > 2 && [1, -3].iter().all(|&i| self.map[(x as isize + i) as usize][y] == Some(Piece::Empty)) && (1..=2).all(|i| self.map[x - i][y] == Some(Piece::Player1)) {
                //     if self.map[x - 1][y] == Some(Piece::Player1) && self.map[x - 2][y] == Some(Piece::Player1) && self.map[x - 3][y] == Some(Piece::Empty) {
                //         free_three += 1;
                //     }
                // }
            }
            Piece::Player2 => {
                if y > 2 {
                    if self.map[x][y - 1] == Some(Piece::Player2) && self.map[x][y - 2] == Some(Piece::Player2) && self.map[x][y - 3] == Some(Piece::Empty) {
                    // check for another free three;
                    }
                }
                if y < 15 {
                    if self.map[x][y + 1] == Some(Piece::Player2) && self.map[x][y + 2] == Some(Piece::Player2) && self.map[x][y + 3] == Some(Piece::Empty) {
                    // check for another free three;
                    }
                }
            }
            _ => (),
        }
        false
    }


    // function to check all the free threes in the board for a selected player and keep in memory positions of the actuals one that have been visited.
    // idea of doing it with a match as them are the only possible pieces
    // i want to check always from the first piece of the sequence so then i got no issuues with finding multiples at same time
    fn find_free_threes(&mut self, piece: Piece) -> bool {
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
                if self.map[x][y] == Some(piece) {
                    // println!("no petardea");
                    // checking X vertical up
                    if let [Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)] = [
                        self.map[x - 1][y],
                        self.map[x][y],
                        self.map[x + 1][y],
                        self.map[x + 2][y],
                        self.map[x + 3][y],
                        if x + 4 < 19 { self.map[x + 4][y] } else { Some(Piece::Empty) },
                    ] {
                        let sequence = (a, b, c, d, e, f);
                        // println!("sequence: {:?}", sequence);
                        if posibilities.contains(&sequence) {
                            println!("free three found!");
                            match piece {
                                Piece::Player1 => free_three_p1 += 1,
                                Piece::Player2 => free_three_p2 += 1,
                                _ => (),
                            }
                        }
                    }
                    // checking Y horizontal right
                    if let [Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)] = [
                        self.map[x][y - 1],
                        self.map[x][y],
                        self.map[x][y + 1],
                        self.map[x][y + 2],
                        self.map[x][y + 3],
                        if y + 4 < 19 { self.map[x][y + 4] } else { Some(Piece::Empty) },
                    ] {
                        let sequence = (a, b, c, d, e, f);
                        // println!("sequence: {:?}", sequence);
                        if posibilities.contains(&sequence) {
                            println!("free three found!");
                            match piece {
                                Piece::Player1 => free_three_p1 += 1,
                                Piece::Player2 => free_three_p2 += 1,
                                _ => (),
                            }
                        }
                    }
                    // checking diagonal up right /
                    if let [Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)] = [
                        self.map[x - 1][y - 1],
                        self.map[x][y],
                        self.map[x + 1][y + 1],
                        self.map[x + 2][y + 2],
                        self.map[x + 3][y + 3],
                        if x + 4 < 19 && y + 4 < 19 { self.map[x + 4][y + 4] } else { Some(Piece::Empty) },
                    ] {
                        let sequence = (a, b, c, d, e, f);
                        // println!("sequence: {:?}", sequence);
                        if posibilities.contains(&sequence) {
                            println!("free three found!");
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
                    if let [Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)] = [
                        self.map[x + 1][y - 1],
                        self.map[x][y],
                        self.map[x - 1][y + 1],
                        self.map[x - 2][y + 2],
                        self.map[x - 3][y + 3],
                        if x > 3 && y + 4 < 19 { self.map[x - 4][y + 4] } else { Some(Piece::Empty) },
                    ] {
                        let sequence = (a, b, c, d, e, f);
                        // println!("sequence: {:?}", sequence);
                        if posibilities.contains(&sequence) {
                            println!("free three found!");
                            match self.map[x][y] {
                                Some(Piece::Player1) => free_three_p1 += 1,
                                Some(Piece::Player2) => free_three_p2 += 1,
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        println!("free three p1: {:?} | free three p2: {:?}", free_three_p1, free_three_p2);
        if free_three_p1 > 1 || free_three_p2 > 1 {
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
    }
}


// -----------------------------------------------------DEPRECATED FUNCTIONS:

// fn check_win(&self) -> (bool, String) {
//     if self.captured1 >= 10 {
//         return (true, "Player 1 wins!".to_string());
//     }
//     if self.captured2 >= 10 {
//         return (true, "Player 2 wins!".to_string());
//     }
//     for i in 0..19 {
//         for j in 0..19 {
//             if self.map[i][j] == Some(Piece::Player1) {
//                 if i < 15 {
//                     if self.map[i + 1][j] == Some(Piece::Player1) && self.map[i + 2][j] == Some(Piece::Player1) && self.map[i + 3][j] == Some(Piece::Player1) && self.map[i + 4][j] == Some(Piece::Player1) {
//                         return (true, "Player 1 wins!".to_string());
//                     }
//                 }
//                 if j < 15 {
//                     if self.map[i][j + 1] == Some(Piece::Player1) && self.map[i][j + 2] == Some(Piece::Player1) && self.map[i][j + 3] == Some(Piece::Player1) && self.map[i][j + 4] == Some(Piece::Player1) {
//                         return (true, "Player 1 wins!".to_string());
//                     }
//                 }
//                 if i < 15 && j < 15 {
//                     if self.map[i + 1][j + 1] == Some(Piece::Player1) && self.map[i + 2][j + 2] == Some(Piece::Player1) && self.map[i + 3][j + 3] == Some(Piece::Player1) && self.map[i + 4][j + 4] == Some(Piece::Player1) {
//                         return (true, "Player 1 wins!".to_string());
//                     }
//                 }
//                 if i < 15 && j > 3 {
//                     if self.map[i + 1][j - 1] == Some(Piece::Player1) && self.map[i + 2][j - 2] == Some(Piece::Player1) && self.map[i + 3][j - 3] == Some(Piece::Player1) && self.map[i + 4][j - 4] == Some(Piece::Player1) {
//                         return (true, "Player 1 wins!".to_string());
//                     }
//                 }
//             }
//             if self.map[i][j] == Some(Piece::Player2) {
//                 if i < 15 {
//                     if self.map[i + 1][j] == Some(Piece::Player2) && self.map[i + 2][j] == Some(Piece::Player2) && self.map[i + 3][j] == Some(Piece::Player2) && self.map[i + 4][j] == Some(Piece::Player2) {
//                         return (true, "Player 1 wins!".to_string());
//                     }
//                 }
//                 if j < 15 {
//                     if self.map[i][j + 1] == Some(Piece::Player2) && self.map[i][j + 2] == Some(Piece::Player2) && self.map[i][j + 3] == Some(Piece::Player2) && self.map[i][j + 4] == Some(Piece::Player2) {
//                         return (true, "Player 1 wins!".to_string());
//                     }
//                 }
//                 if i < 15 && j < 15 {
//                     if self.map[i + 1][j + 1] == Some(Piece::Player2) && self.map[i + 2][j + 2] == Some(Piece::Player2) && self.map[i + 3][j + 3] == Some(Piece::Player2) && self.map[i + 4][j + 4] == Some(Piece::Player2) {
//                         return (true, "Player 1 wins!".to_string());
//                     }
//                 }
//                 if i < 15 && j > 3 {
//                     if self.map[i + 1][j - 1] == Some(Piece::Player2) && self.map[i + 2][j - 2] == Some(Piece::Player2) && self.map[i + 3][j - 3] == Some(Piece::Player2) && self.map[i + 4][j - 4] == Some(Piece::Player2) {
//                         return (true, "Player 1 wins!".to_string());
//                     }
//                 }
//             }
//         }
//     }
//     (false, "".to_string())
// }


// //i must check like expanding from the actual position to check captures or possible ones right now not working
// fn capture(&mut self, x: usize, y: usize, piece: Piece, o_piece: Piece) {
//     println!("x: {:?} y: {:?}", x, y);
//     //capturo hacia arriba
//     if x > 2 && self.map[x - 1][y] == Some(o_piece) && self.map[x - 2][y] == Some(o_piece) && self.map[x - 3][y] == Some(piece) {
//             self.map[x - 1][y] = Some(Piece::Empty);
//             self.map[x - 2][y] = Some(Piece::Empty);
//             if piece == Piece::Player1 {
//                 self.captured1 += 2;
//             } else {
//                 self.captured2 += 2;
//             }
//         }
//     //capturo hacia abajo
//     if x < 16 && self.map[x + 1][y] == Some(o_piece) && self.map[x + 2][y] == Some(o_piece) && self.map[x + 3][y] == Some(piece) {
//         self.map[x + 1][y] = Some(Piece::Empty);
//         self.map[x + 2][y] = Some(Piece::Empty);
//         if piece == Piece::Player1 {
//             self.captured1 += 2;
//         } else {
//             self.captured2 += 2;
//         }
//     }
//     //capturo hacia la izquierda
//     if y > 2 && self.map[x][y - 1] == Some(o_piece) && self.map[x][y - 2] == Some(o_piece) && self.map[x][y - 3] == Some(piece) {
//         self.map[x][y - 1] = Some(Piece::Empty);
//         self.map[x][y - 2] = Some(Piece::Empty);
//         if piece == Piece::Player1 {
//             self.captured1 += 2;
//         } else {
//             self.captured2 += 2;
//         }
//     }
//     //capturo hacia la derecha
//     if y < 16 && self.map[x][y + 1] == Some(o_piece) && self.map[x][y + 2] == Some(o_piece) && self.map[x][y + 3] == Some(piece) {
//         self.map[x][y + 1] = Some(Piece::Empty);
//         self.map[x][y + 2] = Some(Piece::Empty);
//         if piece == Piece::Player1 {
//             self.captured1 += 2;
//         } else {
//             self.captured2 += 2;
//         }
//     }
//     //capturo diagonal arriba izquierda
//     if x > 2 && y > 2 && self.map[x - 1][y - 1] == Some(o_piece) && self.map[x - 2][y - 2] == Some(o_piece) && self.map[x - 3][y - 3] == Some(piece) {
//         self.map[x - 1][y - 1] = Some(Piece::Empty);
//         self.map[x - 2][y - 2] = Some(Piece::Empty);
//         if piece == Piece::Player1 {
//             self.captured1 += 2;
//         } else {
//             self.captured2 += 2;
//         }
//     }
//     // capturo diagonal arriba derecha
//     if x>2 && y < 16 && self.map[x - 1][y + 1] == Some(o_piece) && self.map[x - 2][y + 2] == Some(o_piece) && self.map[x - 3][y + 3] == Some(piece) {
//         self.map[x - 1][y + 1] = Some(Piece::Empty);
//         self.map[x - 2][y + 2] = Some(Piece::Empty);
//         if piece == Piece::Player1 {
//             self.captured1 += 2;
//         } else {
//             self.captured2 += 2;
//         }
//     }
//     //capturo diagonal abajo izquierda
//     if x < 16 && y > 2 && self.map[x + 1][y - 1] == Some(o_piece) && self.map[x + 2][y - 2] == Some(o_piece) && self.map[x + 3][y - 3] == Some(piece) {
//         self.map[x + 1][y - 1] = Some(Piece::Empty);
//         self.map[x + 2][y - 2] = Some(Piece::Empty);
//         if piece == Piece::Player1 {
//             self.captured1 += 2;
//         } else {
//             self.captured2 += 2;
//         }
//     }
//     //capturo diagonal abajo derecha
//     if x < 16 && y < 16 && self.map[x + 1][y + 1] == Some(o_piece) && self.map[x + 2][y + 2] == Some(o_piece) && self.map[x + 3][y + 3] == Some(piece) {
//         self.map[x + 1][y + 1] = Some(Piece::Empty);
//         self.map[x + 2][y + 2] = Some(Piece::Empty);
//         if piece == Piece::Player1 {
//             self.captured1 += 2;
//         } else {
//             self.captured2 += 2;
//         }
//     }
// }