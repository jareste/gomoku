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
                !find_free_threes(&map, Piece::Player1) {
                moves.push((x as i8, y as i8));
            }
        }
    }
    moves
}

fn get_consequtive_pieces_score(map: &[[Piece; 19]; 19], player: i8) -> i32 {
    let directions: [(isize, isize); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut score = 0;
    for x in 0..19 {
        for y in 0..19 {
            if map[x][y] == Piece::Player1 { //hardcoded player 1
                for &(dx, dy) in &directions {
                    let mut consequtive_pieces = 0;
                    for i in 1..=4 {
                        let nx = x as isize + i * dx;
                        let ny = y as isize + i * dy;
                        if nx < 0 || ny < 0 || nx >= 19 || ny >= 19 {
                            break;
                        }
                        if map[nx as usize][ny as usize] == Piece::Player1 { //hardcoded player 1
                            consequtive_pieces += 1;
                        } else {
                            break;
                        }
                    }
                    if consequtive_pieces == 5 {
                        score += 5000000;
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
    score
}


fn get_heuristic(map: &[[Piece; 19]; 19]) -> i32 {
    let mut score = 0;
    score += get_consequtive_pieces_score(map, 1);
    score
}

// fn minimax(map: &[[Piece; 19]; 19], depth: u32, is_maximizing_player: bool) -> Move {
//     if depth == 0 || get_possible_moves(&map).is_empty() {
//         return Move { index: (0, 0), score: get_heuristic(&map) };
//     }

//     // console::log_1(&format!("moves:\n{:?}", get_possible_moves(&map)).into());
//     if is_maximizing_player {
//         let mut best_score = i32::MIN;
//         let mut best_move = (0, 0);

//         for &moves in get_possible_moves(map).iter() {
//             let mut new_game = map.clone();
//             new_game[moves.0 as usize][moves.1 as usize] = Piece::Player1;
//             let score = minimax(&new_game, depth - 1, false).score;
//             if score > best_score {
//                 best_score = score;
//                 best_move = moves;
//             }
//         }
//         Move { index: best_move, score: best_score }
//     } else {
//         let mut best_score = i32::MAX;
    
//         let mut best_move = (0, 0);
//         for &moves in get_possible_moves(map).iter() {
//             let mut new_game = map.clone();
//             new_game[moves.0 as usize][moves.1 as usize] = Piece::Player2;
//             let score = minimax(&new_game,depth - 1, true).score;
//             if score < best_score {
//                 best_score = score;
//                 best_move = moves;
//             }
//         }
//         Move { index: best_move, score: best_score }
//     }
// }

// fn minimax(map: &[[Piece; 19]; 19], depth: u32, is_maximizing_player: bool) -> Move {
//     if depth == 0 || get_possible_moves(&map).is_empty() {
//         return Move { index: (0, 0), score: get_heuristic(&map) };
//     }

//     if is_maximizing_player {
//         let (best_move, best_score) = get_possible_moves(map).par_iter()
//             .map(|&moves| {
//                 let mut new_game = map.clone();
//                 new_game[moves.0 as usize][moves.1 as usize] = Piece::Player1;
//                 let score = minimax(&new_game, depth - 1, false).score;
//                 (moves, score)
//             })
//             .max_by_key(|&(_, score)| score)
//             .unwrap();

//         Move { index: best_move, score: best_score }
//     } else {
//         let (best_move, best_score) = get_possible_moves(map).par_iter()
//             .map(|&moves| {
//                 let mut new_game = map.clone();
//                 new_game[moves.0 as usize][moves.1 as usize] = Piece::Player2;
//                 let score = minimax(&new_game, depth - 1, true).score;
//                 (moves, score)
//             })
//             .min_by_key(|&(_, score)| score)
//             .unwrap();

//         Move { index: best_move, score: best_score }
//     }
// }

// pub fn best_move(map: &[[Piece; 19]; 19]) -> (i8, i8) {
//     minimax(map, 3,  true).index
// }

fn minimax(map: &[[Piece; 19]; 19], depth: u32, mut alpha: i32, mut beta: i32, is_maximizing_player: bool) -> Move {
    if depth == 0 || get_possible_moves(&map).is_empty() {
        return Move { index: (0, 0), score: get_heuristic(&map) };
    }

    if is_maximizing_player {
        let mut best_score = i32::MIN;
        let mut best_move = (0, 0);

        for &moves in get_possible_moves(map).iter() {
            let mut new_game = map.clone();
            new_game[moves.0 as usize][moves.1 as usize] = Piece::Player1;
            let score = minimax(&new_game, depth - 1, alpha, beta, false).score;
            if score > best_score {
                best_score = score;
                best_move = moves;
            }
            alpha = std::cmp::max(alpha, score);
            if beta <= alpha {
                break;
            }
        }

        Move { index: best_move, score: best_score }
    } else {
        let mut best_score = i32::MAX;
        let mut best_move = (0, 0);

        for &moves in get_possible_moves(map).iter() {
            let mut new_game = map.clone();
            new_game[moves.0 as usize][moves.1 as usize] = Piece::Player2;
            let score = minimax(&new_game, depth - 1, alpha, beta, true).score;
            if score < best_score {
                best_score = score;
                best_move = moves;
            }
            beta = std::cmp::min(beta, score);
            if beta <= alpha {
                break;
            }
        }

        Move { index: best_move, score: best_score }
    }
}

pub fn best_move(map: &[[Piece; 19]; 19]) -> (i8, i8) {
    minimax(map, 3, i32::MIN, i32::MAX, true).index
}




// this function is duplicated due to bein an implemetnation of game, if we get rid of wasm we can add this whole file to impl game.
fn check_sequence(sequence: (Piece, Piece, Piece, Piece, Piece, Piece), piece: Piece, posibilities: &[(Piece, Piece, Piece, Piece, Piece, Piece)], free_three_p1: &mut i8, free_three_p2: &mut i8) {
    if posibilities.contains(&sequence) {
        match piece {
            Piece::Player1 => *free_three_p1 += 1,
            Piece::Player2 => *free_three_p2 += 1,
            _ => (),
        }
    }
}

fn find_free_threes(map: &[[Piece; 19]; 19], piece: Piece) -> bool {
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
                check_sequence(
                    (map[x - 1][y], map[x][y], map[x + 1][y], map[x + 2][y], map[x + 3][y], if x + 4 < 19 { map[x + 4][y] } else { Piece::Empty }),
                    piece,
                    &posibilities,
                    &mut free_three_p1,
                    &mut free_three_p2,
                );
                check_sequence(
                    (map[x][y - 1], map[x][y], map[x][y + 1], map[x][y + 2], map[x][y + 3], if y + 4 < 19 { map[x][y + 4] } else { Piece::Empty }),
                    piece,
                    &posibilities,
                    &mut free_three_p1,
                    &mut free_three_p2,
                );
                check_sequence(
                    (map[x - 1][y - 1], map[x][y], map[x + 1][y + 1], map[x + 2][y + 2], map[x + 3][y + 3], if x + 4 < 19 && y + 4 < 19 { map[x + 4][y + 4] } else { Piece::Empty }),
                    piece,
                    &posibilities,
                    &mut free_three_p1,
                    &mut free_three_p2,
                );
                if x < 3 {
                    continue;
                }
                check_sequence(
                    (map[x + 1][y - 1], map[x][y], map[x - 1][y + 1], map[x - 2][y + 2], map[x - 3][y + 3], if x > 3 && y + 4 < 19 { map[x - 4][y + 4] } else { Piece::Empty }),
                    piece,
                    &posibilities,
                    &mut free_three_p1,
                    &mut free_three_p2,
                );
            }
        }
    }

    free_three_p1 > 1 || free_three_p2 > 1
}







// fn find_free_threes(map: &[[Piece; 19]; 19], piece: Piece) -> bool {
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
//         ];

//     let mut free_three_p1: i8 = 0;
//     let mut free_three_p2: i8 = 0;
//     for x in 1..16 {
//         for y in 1..16 {
//             if map[x][y] == piece {
//                 // println!("no petardea");
//                 // checking X vertical up
//                 let [a, b, c, d, e, f] = [
//                     map[x - 1][y],
//                     map[x][y],
//                     map[x + 1][y],
//                     map[x + 2][y],
//                     map[x + 3][y],
//                     if x + 4 < 19 { map[x + 4][y] } else { Piece::Empty },
//                 ];
//                 let sequence = (a, b, c, d, e, f);
//                 // println!("sequence: {:?}", sequence);
//                 if posibilities.contains(&sequence) {
//                     println!("free three found!");
//                     match piece {
//                         Piece::Player1 => free_three_p1 += 1,
//                         Piece::Player2 => free_three_p2 += 1,
//                         _ => (),
//                     }
//                 }

//                 // checking Y horizontal right
//                 let [a, b, c, d, e, f] = [
//                     map[x][y - 1],
//                     map[x][y],
//                     map[x][y + 1],
//                     map[x][y + 2],
//                     map[x][y + 3],
//                     if y + 4 < 19 { map[x][y + 4] } else { Piece::Empty },
//                 ];
//                 let sequence = (a, b, c, d, e, f);
//                 // println!("sequence: {:?}", sequence);
//                 if posibilities.contains(&sequence) {
//                     println!("free three found!");
//                     match piece {
//                         Piece::Player1 => free_three_p1 += 1,
//                         Piece::Player2 => free_three_p2 += 1,
//                         _ => (),
//                     }
//                 }
                
//                 // checking diagonal up right /
//                 let [a, b, c, d, e, f] = [
//                     map[x - 1][y - 1],
//                     map[x][y],
//                     map[x + 1][y + 1],
//                     map[x + 2][y + 2],
//                     map[x + 3][y + 3],
//                     if x + 4 < 19 && y + 4 < 19 { map[x + 4][y + 4] } else { Piece::Empty },
//                 ];
//                 let sequence = (a, b, c, d, e, f);
//                 // println!("sequence: {:?}", sequence);
//                 if posibilities.contains(&sequence) {
//                     println!("free three found!");
//                     match piece {
//                         Piece::Player1 => free_three_p1 += 1,
//                         Piece::Player2 => free_three_p2 += 1,
//                         _ => (),
//                     }
//                 }
            
//                 if x < 3 {
//                     continue;
//                 }
//                 // checking diagonal down right \
//                 let [a, b, c, d, e, f] = [
//                     map[x + 1][y - 1],
//                     map[x][y],
//                     map[x - 1][y + 1],
//                     map[x - 2][y + 2],
//                     map[x - 3][y + 3],
//                     if x > 3 && y + 4 < 19 { map[x - 4][y + 4] } else { Piece::Empty },
//                 ];
//                 let sequence = (a, b, c, d, e, f);
//                 // println!("sequence: {:?}", sequence);
//                 if posibilities.contains(&sequence) {
//                     println!("free three found!");
//                     match map[x][y] {
//                         Piece::Player1 => free_three_p1 += 1,
//                         Piece::Player2 => free_three_p2 += 1,
//                         _ => (),
//                     }
//                 }
            
//             }
//         }
//     }
//     println!("free three p1: {:?} | free three p2: {:?}", free_three_p1, free_three_p2);
//     if free_three_p1 > 1 || free_three_p2 > 1 {
//         return true;
//     }
//     false
// }