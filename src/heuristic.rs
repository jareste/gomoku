
// use crate::constants::{DEPTH, WINNING_BONUS, LOSING_PENALTY, DIRECTIONS, DEVELOPING_TWO, DEVELOPING_THREE, FREE_FOUR, DEVELOPING_FOUR, FIVE_IN_A_ROW};
// use crate::constants::{POSSIBLE_CAPTURE, CAPTURE, FREE_THREE_FIVE, FREE_THREE_SIX};
use crate::game::{Game, Piece};

use crate::constants;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::sync::Mutex;


static PATTERN_SCORES: Lazy<HashMap<u32, i128>> = Lazy::new(|| {
    let mut pattern_scores = HashMap::new();
    for &possible_capture in constants::POSSIBLE_CAPTURE_X.iter() {
        pattern_scores.insert(pattern_to_int_4(&possible_capture), 1);
    }
    for &capture in constants::CAPTURE_X.iter() {
        pattern_scores.insert(pattern_to_int_4(&capture), 2_000);
    }
    for &possible_capture in constants::POSSIBLE_CAPTURE_O.iter() {
        pattern_scores.insert(pattern_to_int_4(&possible_capture), -1);
    }
    for &capture in constants::CAPTURE_O.iter() {
        pattern_scores.insert(pattern_to_int_4(&capture), -2_000);
    }
    for &developing_two in constants::DEVELOPING_TWO_X.iter() {
        pattern_scores.insert(pattern_to_int_5(&developing_two), 10);
    }
    for &five_in_a_row in constants::FIVE_IN_A_ROW_X.iter() {
        pattern_scores.insert(pattern_to_int_5(&five_in_a_row), i64::MAX as i128);
    }
    for &developing_four in constants::DEVELOPING_FOUR_X.iter() {
        pattern_scores.insert(pattern_to_int_5(&developing_four), 10_000);
    }
    for &developing_three in constants::DEVELOPING_THREE_X.iter() {
        pattern_scores.insert(pattern_to_int_5(&developing_three), 100);
    }
    for &free_three_five in constants::FREE_THREE_FIVE_X.iter() {
        pattern_scores.insert(pattern_to_int_5(&free_three_five), 100_000);
    }
    for &developing_two in constants::DEVELOPING_TWO_O.iter() {
        pattern_scores.insert(pattern_to_int_5(&developing_two), -10);
    }
    for &five_in_a_row in constants::FIVE_IN_A_ROW_O.iter() {
        pattern_scores.insert(pattern_to_int_5(&five_in_a_row), i64::MIN as i128);
    }
    for &developing_four in constants::DEVELOPING_FOUR_O.iter() {
        pattern_scores.insert(pattern_to_int_5(&developing_four), -10_000);
    }
    for &developing_three in constants::DEVELOPING_THREE_O.iter() {
        pattern_scores.insert(pattern_to_int_5(&developing_three), -100);
    }
    for &free_three_five in constants::FREE_THREE_FIVE_O.iter() {
        pattern_scores.insert(pattern_to_int_5(&free_three_five), -100_000);
    }
    for &free_three_six in constants::FREE_THREE_SIX_X.iter() {
        pattern_scores.insert(pattern_to_int_6(&free_three_six), 100_000);
    }
    for &free_four in constants::FREE_FOUR_X.iter() {
        pattern_scores.insert(pattern_to_int_6(&free_four), 1_000_000);
    }
    for &free_three_six in constants::FREE_THREE_SIX_O.iter() {
        pattern_scores.insert(pattern_to_int_6(&free_three_six), -100_000);
    }
    for &free_four in constants::FREE_FOUR_O.iter() {
        pattern_scores.insert(pattern_to_int_6(&free_four), -1_000_000);
    }
    pattern_scores
});

fn pattern_to_int_4(pattern: &[Piece; 4]) -> u32 {
    let mut result = 0;
    for &piece in pattern {
        result *= 3;
        result += match piece {
            Piece::Empty => 0,
            Piece::Player1 => 1,
            Piece::Player2 => 2,
        };
    }
    result
}

fn pattern_to_int_5(pattern: &[Piece; 5]) -> u32 {
    let mut result = 0;
    for &piece in pattern {
        result *= 3;
        result += match piece {
            Piece::Empty => 0,
            Piece::Player1 => 1,
            Piece::Player2 => 2,
        };
    }
    result
}

fn pattern_to_int_6(pattern: &[Piece; 6]) -> u32 {
    let mut result = 0;
    for &piece in pattern {
        result *= 3;
        result += match piece {
            Piece::Empty => 0,
            Piece::Player1 => 1,
            Piece::Player2 => 2,
        };
    }
    result
}

fn evaluate_pattern(pattern_int:u32) -> i128 {
    *PATTERN_SCORES.get(&pattern_int).unwrap_or(&0)
}

pub fn generate_patterns(map: [[Piece; 19]; 19]) -> i128 {
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

    map.par_iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, &piece)| {
                    if piece != Piece::Empty {
                        let mut score = 0;
                        for &direction in &directions {
                            for length in 4..=6 {
                                score += get_pattern(&map, (i, j), direction, length);
                            }
                        }
                        Some(score)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .sum()
}

pub fn generate_patterns_single_move(map: [[Piece; 19]; 19], x: usize, y: usize) -> i128 {
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];
    let mut score = 0;
    for &direction in &directions {
        for length in 4..=6 {
            score += get_pattern(&map, (x, y), direction, length);
        }
    }
    score
}

fn get_pattern(map: &[[Piece; 19]; 19], start: (usize, usize), direction: (isize, isize), length: usize) -> i128 {
    let mut pattern = 0;

    for i in 0..length as isize {
        let x = start.0 as isize + direction.0 * i;
        let y = start.1 as isize + direction.1 * i;

        if x < 0 || x >= 19 || y < 0 || y >= 19 {
            return 0;
        }

        pattern *= 3;
        pattern += match map[x as usize][y as usize] {
            Piece::Empty => 0,
            Piece::Player1 => 1,
            Piece::Player2 => 2,
        };
    }
    evaluate_pattern(pattern)
}