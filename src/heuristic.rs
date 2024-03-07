
// use crate::constants::{DEPTH, WINNING_BONUS, LOSING_PENALTY, DIRECTIONS, DEVELOPING_TWO, DEVELOPING_THREE, FREE_FOUR, DEVELOPING_FOUR, FIVE_IN_A_ROW};
// use crate::constants::{POSSIBLE_CAPTURE, CAPTURE, FREE_THREE_FIVE, FREE_THREE_SIX};
use crate::game::{Game, Piece};

use crate::constants;

fn evaluate_pattern(pattern: &[Piece]) -> i128 {
    let mut score = 0;

    if pattern.len() == 4 {
        for &possible_capture in constants::POSSIBLE_CAPTURE_X.iter() {
            score += 1;
        }
        for &capture in constants::CAPTURE_X.iter() {
            score += 2_000;
        }
        for &possible_capture in constants::POSSIBLE_CAPTURE_O.iter() {
            score -= 1;
        }
        for &capture in constants::CAPTURE_O.iter() {
            score -= 2_000;
        }
    }
    if pattern.len() == 5 {
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
                score += 100_000;
            }
        }
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
                score -= 100_000;
            }
        }
    } else if pattern.len() == 6 {
        for &free_three_six in constants::FREE_THREE_SIX_X.iter() {
            if free_three_six == pattern {
                score += 100_000;
            }
        }
        for &free_four in constants::FREE_FOUR_X.iter() {
            if free_four == pattern {
                score += 1_000_000;
            }
        }
        for &free_three_six in constants::FREE_THREE_SIX_O.iter() {
            if free_three_six == pattern {
                score -= 100_000;
            }
        }
        for &free_four in constants::FREE_FOUR_O.iter() {
            if free_four == pattern {
                score -= 1_000_000;
            }
        }
    }

    score
}

pub fn generate_patterns(map: [[Piece; 19]; 19]) -> i128 {
    let mut patterns = Vec::new();
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];


    for i in 0..19 {
        for j in 0..19 {
            for &direction in &directions {
                for length in 4..=6 {
                    if let Some(pattern) = get_pattern(&map, (i, j), direction, length) {
                        patterns.push(pattern);
                    }
                }
            }
        }
    }
    let mut score = 0;
    for pattern in patterns {
        score += evaluate_pattern(&pattern);
    }
    if map[6][8] == Piece::Player1 {
        println!("score: {}", score);
    }
    // patterns
    score
}

fn get_pattern(map: &[[Piece; 19]; 19], start: (usize, usize), direction: (isize, isize), length: usize) -> Option<Vec<Piece>> {
    let mut pattern = Vec::new();

    for i in 0..length {
        let x = start.0 as isize + direction.0 * i as isize;
        let y = start.1 as isize + direction.1 * i as isize;

        if x < 0 || x >= 19 || y < 0 || y >= 19 {
            return None;
        }

        pattern.push(map[x as usize][y as usize]);
    }

    Some(pattern)
}