
// use crate::constants::{DEPTH, WINNING_BONUS, LOSING_PENALTY, DIRECTIONS, DEVELOPING_TWO, DEVELOPING_THREE, FREE_FOUR, DEVELOPING_FOUR, FIVE_IN_A_ROW};
// use crate::constants::{POSSIBLE_CAPTURE, CAPTURE, FREE_THREE_FIVE, FREE_THREE_SIX};
use crate::game::{Game, Piece};

use crate::constants;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::sync::Mutex;

fn evaluate_pattern(pattern: &[Piece]) -> i128 {
    let mut score = 0;

    if pattern.len() == 4 {
        match pattern[1] {
            Piece::Player2 => {
                for &possible_capture in constants::POSSIBLE_CAPTURE_X.iter() {
                    score += 1;
                }
                for &capture in constants::CAPTURE_X.iter() {
                    score += 2_000;
                }
            },
            Piece::Player1 => {
                for &possible_capture in constants::POSSIBLE_CAPTURE_O.iter() {
                    score -= 1;
                }
                for &capture in constants::CAPTURE_O.iter() {
                    score -= 2_000;
                }
            },
            _ => (),
        }
    }
    if pattern.len() == 5 {
        match pattern[1] {
            Piece::Player1 => {
                for &developing_two in constants::DEVELOPING_TWO_X.iter() {
                    if developing_two == pattern {
                        score += 10;
                    }
                }
                for &five_in_a_row in constants::FIVE_IN_A_ROW_X.iter() {
                    if five_in_a_row == pattern {
                        i64::MAX;
                    }
                }
                for &developing_four in constants::DEVELOPING_FOUR_X.iter() {
                    if developing_four == pattern {
                        score += 10_000;
                    }
                }
                for &developing_three in constants::DEVELOPING_THREE_X.iter() {
                    if developing_three == pattern{
                        score += 100;
                    }
                }
                for &free_three_five in constants::FREE_THREE_FIVE_X.iter() {
                    if free_three_five == pattern {
                        // println!("free5X");
                        score += 100_000;
                    }
                }
            },
            Piece::Player2 => {
                for &developing_two in constants::DEVELOPING_TWO_O.iter() {
                    if developing_two == pattern {
                        score -= 10;
                    }
                }
                for &five_in_a_row in constants::FIVE_IN_A_ROW_O.iter() {
                    if five_in_a_row == pattern {
                        i64::MIN;
                    }
                }
                for &developing_four in constants::DEVELOPING_FOUR_O.iter() {
                    if developing_four == pattern {
                        score -= 10_000;
                    }
                }
                for &developing_three in constants::DEVELOPING_THREE_O.iter() {
                    if developing_three == pattern{
                        score -= 100;
                    }
                }
                for &free_three_five in constants::FREE_THREE_FIVE_O.iter() {
                    if free_three_five == pattern {
                        // println!("free5O");
                        score -= 100_000;
                    }
                }
            },
            _ => (),
        }
    } else if pattern.len() == 6 {
        match pattern[1] {
            Piece::Player1 => {
                for &free_three_six in constants::FREE_THREE_SIX_X.iter() {
                    if free_three_six == pattern {
                        score += 100_000;
                    }
                }
                for &free_four in constants::FREE_FOUR_X.iter() {
                    if free_four == pattern {
                        // println!("free3X");
                        score += 1_000_000;
                    }
                }
            },
            Piece::Player2 => {
                for &free_three_six in constants::FREE_THREE_SIX_O.iter() {
                    if free_three_six == pattern {
                        score -= 100_000;
                    }
                }
                for &free_four in constants::FREE_FOUR_O.iter() {
                    if free_four == pattern {
                        // println!("free4O");
                        score -= 1_000_000;
                    }
                }
            },
            _ => (),
        }
    }

    score
}

pub fn generate_patterns(map: [[Piece; 19]; 19]) -> i128 {
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

    (0..19).flat_map(|i| (0..19).map(move |j| (i, j)))
        .par_bridge()
        .map(|(i, j)| {
            if map[i][j] == Piece::Empty {
                return 0;
            }
            let mut score = 0;
            for &direction in &directions {
                for length in 4..=6 {
                    if let Some(pattern) = get_pattern(&map, (i, j), direction, length) {
                        score += evaluate_pattern(&pattern);
                    }
                }
            }
            score
        })
        .sum()
}

fn get_pattern(map: &[[Piece; 19]; 19], start: (usize, usize), direction: (isize, isize), length: usize) -> Option<Vec<Piece>> {
    let mut pattern = Vec::new();

    for i in -1..length as isize {
        let x = start.0 as isize + direction.0 * i as isize;
        let y = start.1 as isize + direction.1 * i as isize;

        if x < 0 || x >= 19 || y < 0 || y >= 19 {
            return None;
        }

        pattern.push(map[x as usize][y as usize]);
    }

    Some(pattern)
}