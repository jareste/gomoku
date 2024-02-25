extern crate wasm_bindgen;
use web_sys::console;
use crate::game::{Game, Piece};
// use crate::game;

use wasm_bindgen::prelude::*;

struct Move {
    index: (i8, i8),
    score: i32,
}

// this function will check the squares around the possible movement and return false if there is no piece in squares_to_check around the movement
fn dfs_check_movement(map: &[[Piece; 19]; 19], x: i8, y: i8, squares_to_check: i8) -> bool {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    for &(dx, dy) in &directions {
        for i in 1..=squares_to_check {
            let nx = x + i * dx;
            let ny = y + i * dy;
            if nx < 0 || ny < 0 || nx >= 19 || ny >= 19 {
                break;
            }
            if map[nx as usize][ny as usize] != Piece::Empty {
                return true;
            }
        }
    }
    false
}

fn get_possible_moves(map: &[[Piece; 19]; 19]) -> Vec<(i8, i8)> {
    let mut moves = Vec::new();
    for x in 0..18 {
        for y in 0..18 {
            if map[x][y] == Piece::Empty && dfs_check_movement(&map, x as i8, y as i8, 2) &&
                !find_free_threes(&map, Piece::Player1, 1) {
                moves.push((x as i8, y as i8));
            }
        }
    }
    moves
}

fn get_consequtive_pieces_score(map: &[[Piece; 19]; 19], player: Piece) -> i32 {
    let directions: [(isize, isize); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut score = 0;
    for x in 0..19 {
        for y in 0..19 {
            if map[x][y] == player {
                for &(dx, dy) in &directions {
                    let mut consequtive_pieces = 0;
                    let mut opponent_piece_found = false;
                    for i in 1..=4 {
                        let nx = x as isize + i * dx;
                        let ny = y as isize + i * dy;
                        if nx < 0 || ny < 0 || nx >= 19 || ny >= 19 {
                            break;
                        }
                        if map[nx as usize][ny as usize] == player {
                            consequtive_pieces += 1;
                        } else if map[nx as usize][ny as usize] == if player == Piece::Player1 { Piece::Player2 } else { Piece::Player1 } {
                            opponent_piece_found = true;
                            break; //here
                        }
                    }
                    if !opponent_piece_found {
                        if consequtive_pieces == 5 {
                            score += 5000000;
                            if player == Piece::Player1 { score+=5000000 }
                        }
                        if consequtive_pieces == 4 {
                            let nx = x as isize - 1 * dx;
                            let ny = y as isize - 1 * dy;
                            let nnx = x as isize + 5 * dx;
                            let nny = y as isize + 5 * dy;
                            if nx >= 0 && nx <= 18 && ny >=0 && ny <= 18 && map[nx as usize][ny as usize] == Piece::Empty {
                                score += 1000000;
                            }
                            if nnx >= 0 && nnx <= 18 && nny >=0 && nny <= 18 && map[nnx as usize][nny as usize] == Piece::Empty {
                                score += 1000000;
                            }
                        }
                        if consequtive_pieces == 4 {
                            score += 1000;
                        }
                        if consequtive_pieces == 3 {
                            score += 100;
                        }
                        if consequtive_pieces == 2 {
                            score += 10;
                        }
                        if consequtive_pieces == 1 {
                            score += 1;
                        }
                    }
                }
            }
        }
    }
    score
}

fn find_fours(map: &[[Piece; 19]; 19], player: Piece) -> bool {
    let directions: [(isize, isize); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];

    for x in 0..19 {
        for y in 0..19 {
            if map[x][y] == player {
                for &(dx, dy) in &directions {
                    let mut consequtive_pieces = 0;
                    for i in 1..=4 {
                        let nx = x as isize + i * dx;
                        let ny = y as isize + i * dy;
                        if nx < 0 || ny < 0 || nx >= 19 || ny >= 19 {
                            break;
                        }
                        if map[nx as usize][ny as usize] == player {
                            consequtive_pieces += 1;
                        } else {
                            break;
                        }
                    }
                    if consequtive_pieces == 4 {
                        return true;
                    }
                }
            }
        }
    }
    false
}



fn get_heuristic(map: &[[Piece; 19]; 19]) -> i32 {
    let mut score = 0;
    score += get_consequtive_pieces_score(map, Piece::Player1);
    if find_free_threes(&map, Piece::Player1, 0) {
        score += 100000;
    }
    score -= get_consequtive_pieces_score(map, Piece::Player2);
    if find_free_threes(&map, Piece::Player2, 0) {
        score -= 1000000;
    }
    score
}

fn capture_direction(map: &[[Piece; 19]; 19], x: isize, y: isize, dx: isize, dy: isize, piece: Piece, o_piece: Piece) -> bool {
    if (1..3).all(|i| map.get((x + i * dx) as usize).and_then(|row| row.get((y + i * dy) as usize)) == Some(&o_piece))
        && map.get((x + 3 * dx) as usize).and_then(|row| row.get((y + 3 * dy) as usize)) == Some(&piece) {
        return true;
    }
    false
}

// NEW CAPTURE FUNCTIONS MAYBE NOT WORKING AS EXPECTED
fn capture(map: &[[Piece; 19]; 19], x: usize, y: usize, piece: Piece, o_piece: Piece) -> bool {
    let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
    for &(dx, dy) in &directions {
        if capture_direction(&map, x as isize, y as isize, dx, dy, piece, o_piece) {
            return true;
        }
        if capture_direction(&map, x as isize, y as isize, -dx, -dy, piece, o_piece) {
            return true;
        }
    }
    false
}

fn minimax(map: &[[Piece; 19]; 19], depth: i8, mut alpha: i32, mut beta: i32, is_maximizing_player: bool, captured1: i8, captured2: i8) -> Move {
    if depth == 0 || get_possible_moves(&map).is_empty() {
        return Move { index: (0, 0), score: get_heuristic(&map) };
    }

    if is_maximizing_player {
        let mut best_score = i32::MIN;
        let mut best_move = (0, 0);

        for &moves in get_possible_moves(&map).iter() {
            let mut new_game = map.clone();
            new_game[moves.0 as usize][moves.1 as usize] = Piece::Player1;
            let mut score = minimax(&new_game, depth - 1, alpha, beta, false, captured1, captured2).score;
            if capture(&mut new_game, moves.0 as usize, moves.1 as usize, Piece::Player1, Piece::Player2) {
                score += 1000000;
            }
            if captured1 > 5 {
                score += 1000;
            }
            if score > best_score {
                best_score = score;
                best_move = moves;
            }
            alpha = std::cmp::max(alpha, score);
            if beta <= alpha {
                break;
            }
        }
        if best_move == (9, 7) {
            console::log_1(&format!("9 7 MAX move score: {:?}", best_score).into());
        }
        if best_move == (9, 12) {
            console::log_1(&format!("9 12 MAX move score: {:?}", best_score).into());
        }        
        Move { index: best_move, score: best_score }
    } else {
        let mut best_score = i32::MAX;
        let mut best_move = (0, 0);

        for &moves in get_possible_moves(&map).iter() {
            let mut new_game = map.clone();
            new_game[moves.0 as usize][moves.1 as usize] = Piece::Player2;

            let mut score = minimax(&new_game, depth - 1, alpha, beta, true, captured1, captured2).score;            
            if capture(&mut new_game, moves.0 as usize, moves.1 as usize, Piece::Player2, Piece::Player1) {
                console::log_1(&"captureddsafdsgdsgawgwgadgdsgds".into());
                score -= 1000000;
            }
            if captured2 > 5 {
                score -= 1000;
            }
            if score < best_score {
                best_score = score;
                best_move = moves;
            }
            beta = std::cmp::min(beta, score);
            if beta <= alpha {
                break;
            }
        }
        if best_move == (9, 7) {
            console::log_1(&format!("9 7 MIN move score: {:?}", best_score).into());

        }
        if best_move == (9, 12) {
            console::log_1(&format!("9 12 MIN move score: {:?}", best_score).into());
        }
        Move { index: best_move, score: best_score }
    }
}

pub fn best_move(map: &[[Piece; 19]; 19], captured1: i8, captured2: i8) -> (i8, i8) {
    minimax(map, 3, i32::MIN, i32::MAX, true, captured1, captured2).index
}



// use std::sync::{Arc, mpsc};
// use std::thread;

// pub fn best_move(map: &[[Piece; 19]; 19]) -> (i8, i8) {
//     let possible_moves = get_possible_moves(&map);
//     if possible_moves.len() == 1 {
//         return possible_moves[0];
//     }

//     let (tx, rx) = mpsc::channel();

//     let mid = possible_moves.len() / 2;
//     let first_half = possible_moves[0..mid].to_vec();
//     let second_half = possible_moves[mid..].to_vec();

//     let map1 = map.clone();
//     let tx1 = tx.clone();
//     let handle1 = thread::spawn(move || {
//         for &moves in &first_half {
//             let mut new_game = map1.clone();
//             new_game[moves.0 as usize][moves.1 as usize] = Piece::Player1;
//             let result = minimax(&new_game, 2, i32::MIN, i32::MAX, false);
//             tx1.send(result).unwrap();
//             thread::sleep(std::time::Duration::from_secs(1));
//         }
//     });

//     let map2 = map.clone();
//     let handle2 = thread::spawn(move || {
//         for &moves in &second_half {
//             let mut new_game = map2.clone();
//             new_game[moves.0 as usize][moves.1 as usize] = Piece::Player1;
//             let result = minimax(&new_game, 2, i32::MIN, i32::MAX, false);
//             tx.send(result).unwrap();
//             thread::sleep(std::time::Duration::from_secs(1));
//         }
//     });

//     handle1.join().unwrap();
//     handle2.join().unwrap();

//     let mut best_result = i32::MIN;
//     let mut best_move = (0, 0);
//     for received in rx {
//         if received.score > best_result {
//             best_result = received.score;
//             best_move = received.index;
//         }
//     }
//     best_move
// }










// this function is duplicated due to bein an implemetnation of game, if we get rid of wasm we can add this whole file to impl game.
// fn check_sequence(sequence: (Piece, Piece, Piece, Piece, Piece, Piece), piece: Piece, posibilities: &[(Piece, Piece, Piece, Piece, Piece, Piece)], free_three_p1: &mut i8, free_three_p2: &mut i8) {
//     if posibilities.contains(&sequence) {
//         match piece {
//             Piece::Player1 => *free_three_p1 += 1,
//             Piece::Player2 => *free_three_p2 += 1,
//             _ => (),
//         }
//     }
// }

// fn find_free_threes(map: &[[Piece; 19]; 19], piece: Piece, quantity: i8) -> bool {
//     let posibilities = [
//         (Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Empty), // - X X X -
//         (Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1), // - X X X -
//         (Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player2), // - X X X -
//         (Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Empty), // - O O O -
//         (Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player1), // - O O O -
//         (Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2), // - O O O -
//         (Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Empty), // - X X - X -
//         (Piece::Empty, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty), // - X - X X -
//         (Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Empty), // - O O - O -
//         (Piece::Empty, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty), // - O - O O -
//     ];

//     let mut free_three_p1: i8 = 0;
//     let mut free_three_p2: i8 = 0;

//     for x in 1..16 {
//         for y in 1..16 {
//             if map[x][y] == piece {
//                 check_sequence(
//                     (map[x - 1][y], map[x][y], map[x + 1][y], map[x + 2][y], map[x + 3][y], if x + 4 < 19 { map[x + 4][y] } else { Piece::Empty }),
//                     piece,
//                     &posibilities,
//                     &mut free_three_p1,
//                     &mut free_three_p2,
//                 );
//                 check_sequence(
//                     (map[x][y - 1], map[x][y], map[x][y + 1], map[x][y + 2], map[x][y + 3], if y + 4 < 19 { map[x][y + 4] } else { Piece::Empty }),
//                     piece,
//                     &posibilities,
//                     &mut free_three_p1,
//                     &mut free_three_p2,
//                 );
//                 check_sequence(
//                     (map[x - 1][y - 1], map[x][y], map[x + 1][y + 1], map[x + 2][y + 2], map[x + 3][y + 3], if x + 4 < 19 && y + 4 < 19 { map[x + 4][y + 4] } else { Piece::Empty }),
//                     piece,
//                     &posibilities,
//                     &mut free_three_p1,
//                     &mut free_three_p2,
//                 );
//                 if x < 3 {
//                     continue;
//                 }
//                 check_sequence(
//                     (map[x + 1][y - 1], map[x][y], map[x - 1][y + 1], map[x - 2][y + 2], map[x - 3][y + 3], if x > 3 && y + 4 < 19 { map[x - 4][y + 4] } else { Piece::Empty }),
//                     piece,
//                     &posibilities,
//                     &mut free_three_p1,
//                     &mut free_three_p2,
//                 );
//             }
//         }
//     }

//     free_three_p1 > quantity || free_three_p2 > quantity
// }







fn find_free_threes(map: &[[Piece; 19]; 19], piece: Piece, quantity: i8) -> bool {
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
            if map[x][y] == piece {
                // println!("no petardea");
                // checking X vertical up
                let [a, b, c, d, e, f] = [
                    map[x - 1][y],
                    map[x][y],
                    map[x + 1][y],
                    map[x + 2][y],
                    map[x + 3][y],
                    if x + 4 < 19 { map[x + 4][y] } else { Piece::Empty },
                ];
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

                // checking Y horizontal right
                let [a, b, c, d, e, f] = [
                    map[x][y - 1],
                    map[x][y],
                    map[x][y + 1],
                    map[x][y + 2],
                    map[x][y + 3],
                    if y + 4 < 19 { map[x][y + 4] } else { Piece::Empty },
                ];
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
                
                // checking diagonal up right /
                let [a, b, c, d, e, f] = [
                    map[x - 1][y - 1],
                    map[x][y],
                    map[x + 1][y + 1],
                    map[x + 2][y + 2],
                    map[x + 3][y + 3],
                    if x + 4 < 19 && y + 4 < 19 { map[x + 4][y + 4] } else { Piece::Empty },
                ];
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
            
                if x < 3 {
                    continue;
                }
                // checking diagonal down right \
                let [a, b, c, d, e, f] = [
                    map[x + 1][y - 1],
                    map[x][y],
                    map[x - 1][y + 1],
                    map[x - 2][y + 2],
                    map[x - 3][y + 3],
                    if x > 3 && y + 4 < 19 { map[x - 4][y + 4] } else { Piece::Empty },
                ];
                let sequence = (a, b, c, d, e, f);
                // println!("sequence: {:?}", sequence);
                if posibilities.contains(&sequence) {
                    println!("free three found!");
                    match map[x][y] {
                        Piece::Player1 => free_three_p1 += 1,
                        Piece::Player2 => free_three_p2 += 1,
                        _ => (),
                    }
                }
            
            }
        }
    }
    println!("free three p1: {:?} | free three p2: {:?}", free_three_p1, free_three_p2);
    if free_three_p1 > quantity || free_three_p2 > quantity {
        return true;
    }
    false
}