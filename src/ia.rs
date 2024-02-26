use crate::game::{Game, Piece};
// use crate::game;


struct Move {
    index: (i8, i8),
    score: i32,
}

pub trait IA{
    fn dfs_check_movement(&mut self, x: i8, y: i8, squares_to_check: i8) -> bool;
    fn get_possible_moves(&mut self) -> Vec<(i8, i8)>;
    fn get_consequtive_pieces_score(&mut self, player: Piece) -> i32;
    fn get_heuristic(&mut self) -> i32;
    fn minimax(&mut self, depth: i8, alpha: i32, beta: i32, is_maximizing_player: bool) -> Move;
    fn best_move(&mut self) -> (i8, i8);
}

impl IA for Game {
    // this function will check the squares around the possible movement and return false if there is no piece in squares_to_check around the movement
    fn dfs_check_movement(&mut self, x: i8, y: i8, squares_to_check: i8) -> bool {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        for &(dx, dy) in &directions {
            for i in 1..=squares_to_check {
                let nx = x + i * dx;
                let ny = y + i * dy;
                if nx < 0 || ny < 0 || nx >= 19 || ny >= 19 {
                    break;
                }
                if self.map[nx as usize][ny as usize] != Piece::Empty {
                    return true;
                }
            }
        }
        false
    }

    fn get_possible_moves(&mut self) -> Vec<(i8, i8)> {
        let mut moves = Vec::new();
        for x in 0..18 {
            for y in 0..18 {
                if self.map[x][y] == Piece::Empty && self.dfs_check_movement(x as i8, y as i8, 2) &&
                    !self.find_free_threes(Piece::Player1, 1) {
                    moves.push((x as i8, y as i8));
                }
            }
        }
        moves
    }

    // rarete
    fn get_consequtive_pieces_score(&mut self, player: Piece) -> i32 {
        let directions: [(isize, isize); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
        let mut score = 0;
        for x in 0..19 {
            for y in 0..19 {
                if self.map[x][y] == player {
                    for &(dx, dy) in &directions {
                        let mut consequtive_pieces = 0;
                        let mut opponent_piece_found = false;
                        for i in 1..=4 {
                            let nx = x as isize + i * dx;
                            let ny = y as isize + i * dy;
                            if nx < 0 || ny < 0 || nx >= 19 || ny >= 19 {
                                break;
                            }
                            if self.map[nx as usize][ny as usize] == player {
                                consequtive_pieces += 1;
                            } else if self.map[nx as usize][ny as usize] == if player == Piece::Player1 { Piece::Player2 } else { Piece::Player1 } {
                                opponent_piece_found = true;
                                break; //here
                            }
                        }
                        if !opponent_piece_found {
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
        }
        score
    }

    // must be reviewed
    fn get_heuristic(&mut self) -> i32 {
        let mut score = 0;
        score += self.get_consequtive_pieces_score(Piece::Player1);
        if self.find_free_threes(Piece::Player1, 0) {
            score += 100000;
        }
        score -= self.get_consequtive_pieces_score(Piece::Player2);
        if self.find_free_threes(Piece::Player2, 0) {
            score -= 1000000;
        }
        score
    }

    fn minimax(&mut self, depth: i8, mut alpha: i32, mut beta: i32, is_maximizing_player: bool) -> Move {
        let possible_moves = self.get_possible_moves();

        if depth == 0 || possible_moves.is_empty() {
            return Move { index: (0, 0), score: self.get_heuristic() };
        }
        
        let mut best_move = (0, 0);
        let mut best_score = if is_maximizing_player { i32::MIN } else { i32::MAX };

        for &moves in possible_moves.iter() {
            let mut new_game = self.clone();

            new_game.map[moves.0 as usize][moves.1 as usize] = Piece::Player1;

            let mut score = new_game.minimax(depth - 1, alpha, beta, !is_maximizing_player).score;

            if score > best_score {
                best_score = score;
                best_move = moves;
            }

            match is_maximizing_player {
                true => alpha = std::cmp::max(alpha, score),
                false => beta = std::cmp::min(beta, score),
            }

            if beta <= alpha {
                break;
            }
        }
        Move { index: best_move, score: best_score }
    }

    fn best_move(&mut self) -> (i8, i8) {
        self.minimax(4, i32::MIN, i32::MAX, true).index
    }
}

// fn capture_direction(&mut self, x: isize, y: isize, dx: isize, dy: isize, piece: Piece, o_piece: Piece) -> bool {
//     if (1..3).all(|i| self.map.get((x + i * dx) as usize).and_then(|row| row.get((y + i * dy) as usize)) == Some(&o_piece))
//         && self.map.get((x + 3 * dx) as usize).and_then(|row| row.get((y + 3 * dy) as usize)) == Some(&piece) {
//         return true;
//     }
//     false
// }

// fn capture(&mut self, x: usize, y: usize, piece: Piece, o_piece: Piece) -> bool {
//     let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
//     for &(dx, dy) in &directions {
//         if capture_direction(&self.map, x as isize, y as isize, dx, dy, piece, o_piece) {
//             return true;
//         }
//         if capture_direction(&self.map, x as isize, y as isize, -dx, -dy, piece, o_piece) {
//             return true;
//         }
//     }
//     false
// }
