use crate::game::Piece;

pub const DEPTH: i8 = 7;
pub const WINNING_BONUS: i32 = 10_000_000;
pub const LOSING_PENALTY: i32 = -10_000_000;
pub const THREATENING_BONUS: i32 = 100_000;

pub const FREE_THREE: [[Piece; 6]; 10] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Empty], // - X X X -
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1], // - X X X -
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player2], // - X X X -
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Empty], // - X X - X -
    [Piece::Empty, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty], // - X - X X -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Empty], // - O O O -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player1], // - O O O -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2], // - O O O -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Empty], // - O O - O -
    [Piece::Empty, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty], // - O - O O -
];

pub const FREE_FOUR: [[Piece; 6]; 2] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty], // - X X X X -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty], // - O O O O -
];	

pub const DEVELOPING_FOUR: [[Piece; 7]; 10] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Empty], // - X X X - X -
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty], // - X X - X X -
    [Piece::Empty, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty], // - X - X X X -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Empty], // - O O O - O -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty], // - O O - O O -
    [Piece::Empty, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty], // - O - O O O -
];

pub const FIVE_IN_A_ROW: [[Piece; 5]; 2] = [
    [Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1], // X X X X X
    [Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2], // O O O O O
];