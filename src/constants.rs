use crate::game::Piece;

pub const DEPTH: i8 = 2;
pub const WINNING_BONUS: i32 = 10_000_000;
pub const LOSING_PENALTY: i32 = -10_000_000;
pub const THREATENING_BONUS: i32 = 100_000;
pub const DIRECTIONS: [(i8,i8); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];

pub const POSSIBILITIES: [[Piece; 6]; 10] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Empty], // - X X X -
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1], // - X X X -
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player2], // - X X X -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Empty], // - O O O -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player1], // - O O O -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2], // - O O O -
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Empty], // - X X - X -
    [Piece::Empty, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty], // - X - X X -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Empty], // - O O - O -
    [Piece::Empty, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty], // - O - O O -
];

pub const DEVELOPING_TWO_X: [[Piece; 5]; 4] = [
    [Piece::Player1, Piece::Player1, Piece::Empty, Piece::Empty, Piece::Empty], // X X - - -
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Empty], // - X X - -
    [Piece::Empty, Piece::Player1, Piece::Empty, Piece::Empty, Piece::Player1], // - X - - X
    [Piece::Empty, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Empty], // - X - X -
];

pub const DEVELOPING_TWO_O: [[Piece; 5]; 4] = [
    [Piece::Player2, Piece::Player2, Piece::Empty, Piece::Empty, Piece::Empty], // O O - - -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Empty], // - O O - -
    [Piece::Empty, Piece::Player2, Piece::Empty, Piece::Empty, Piece::Player2], // - O - - O
    [Piece::Empty, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Empty], // - O - O -
];

pub const FREE_THREE_FIVE_X: [[Piece; 5]; 1] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty], // - X X X -
];

pub const FREE_THREE_FIVE_O: [[Piece; 5]; 1] = [
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty], // - O O O -
];

pub const FREE_THREE_SIX_X: [[Piece; 6]; 2] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Empty], // - X X - X -
    [Piece::Empty, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty], // - X - X X -

];

pub const FREE_THREE_SIX_O: [[Piece; 6]; 2] = [
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Empty], // - O O - O -
    [Piece::Empty, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty], // - O - O O -
];

pub const DEVELOPING_THREE_X: [[Piece; 5]; 4] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1], // - X X - X
    [Piece::Empty, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1], // - X - X X
    [Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Empty], // X X X - -
    [Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Empty], // X X - X -
];

pub const DEVELOPING_THREE_O: [[Piece; 5]; 4] = [   
[Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2], // - O O - O
[Piece::Empty, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2], // - X - X X
[Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Empty], // X X X - -
[Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Empty], // X X - X -
];

pub const FREE_FOUR_X: [[Piece; 6]; 1] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty], // - X X X X -

];	

pub const FREE_FOUR_O: [[Piece; 6]; 1] = [
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty], // - O O O O -

];

pub const DEVELOPING_FOUR_X: [[Piece; 5]; 5] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1], // - X X X X
    [Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1], // X X X - X
    [Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1], // X X - X X
    [Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1], // X - X X X
    [Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty], // X X X X -

];

pub const DEVELOPING_FOUR_O: [[Piece; 5]; 5] = [
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2], // - O O O O
    [Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2], // X X X - X
    [Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2], // X X - X X
    [Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2], // X - X X X
    [Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty], // X X X X -

];

pub const FIVE_IN_A_ROW_X: [[Piece; 5]; 1] = [
    [Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1], // X X X X X

];

pub const FIVE_IN_A_ROW_O: [[Piece; 5]; 1] = [
    [Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2], // O O O O O

];

pub const POSSIBLE_CAPTURE_X: [[Piece; 4]; 2] = [
    [Piece::Player1, Piece::Player2, Piece::Player2, Piece::Empty], // X O O -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player1], // - O O X

];

pub const POSSIBLE_CAPTURE_O: [[Piece; 4]; 2] = [
    [Piece::Player2, Piece::Player1, Piece::Player1, Piece::Empty], // O X X -
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player2], // - X X O

];

pub const CAPTURE_X: [[Piece; 4]; 1] = [
    [Piece::Player1, Piece::Player2, Piece::Player2, Piece::Player1], // X O O X
];

pub const CAPTURE_O: [[Piece; 4]; 1] = [
    [Piece::Player2, Piece::Player1, Piece::Player1, Piece::Player2], // O X X O
];
