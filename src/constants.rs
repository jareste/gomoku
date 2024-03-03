use crate::game::Piece;

pub const DEPTH: i8 = 3;
pub const WINNING_BONUS: i32 = 10_000_000;
pub const LOSING_PENALTY: i32 = -10_000_000;
pub const THREATENING_BONUS: i32 = 100_000;
pub const DIRECTIONS: [(i8,i8); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];


pub const DEVELOPING_TWO: [[Piece; 5]; 20] = [
    [Piece::Player1, Piece::Player1, Piece::Empty, Piece::Empty, Piece::Empty], // X X - - -
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Empty], // - X X - -
    [Piece::Empty, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty], // - - X X -
    [Piece::Empty, Piece::Empty, Piece::Empty, Piece::Player1, Piece::Player1], // - - - X X
    [Piece::Player1, Piece::Empty, Piece::Player1, Piece::Empty, Piece::Empty], // X - X - -
    [Piece::Player1, Piece::Empty, Piece::Empty, Piece::Player1, Piece::Empty], // X - - X -
    [Piece::Player1, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Player1], // X - - - X
    [Piece::Empty, Piece::Player1, Piece::Empty, Piece::Empty, Piece::Player1], // - X - - X
    [Piece::Empty, Piece::Empty, Piece::Player1, Piece::Empty, Piece::Player1], // - - X - X
    [Piece::Empty, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Empty], // - X - X -
    
    [Piece::Player2, Piece::Player2, Piece::Empty, Piece::Empty, Piece::Empty], // O O - - -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Empty], // - O O - -
    [Piece::Empty, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty], // - - O O -
    [Piece::Empty, Piece::Empty, Piece::Empty, Piece::Player2, Piece::Player2], // - - - O O
    [Piece::Player2, Piece::Empty, Piece::Player2, Piece::Empty, Piece::Empty], // O - O - -
    [Piece::Player2, Piece::Empty, Piece::Empty, Piece::Player2, Piece::Empty], // O - - O -
    [Piece::Player2, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Player2], // O - - - O
    [Piece::Empty, Piece::Player2, Piece::Empty, Piece::Empty, Piece::Player2], // - O - - O
    [Piece::Empty, Piece::Empty, Piece::Player2, Piece::Empty, Piece::Player2], // - - O - O
    [Piece::Empty, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Empty], // - O - O -
];


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

pub const DEVELOPING_THREE: [[Piece; 5]; 14] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1], // - X X - X
    [Piece::Empty, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1], // - X - X X
    [Piece::Empty, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1], // - - X X X
    [Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Empty], // X X X - -
    [Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Empty], // X X - X -
    [Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Empty], // X - X X -
    [Piece::Player1, Piece::Empty, Piece::Empty, Piece::Player1, Piece::Player1], // X - - X X

    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2], // - O O - O
    [Piece::Empty, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2], // - X - X X
    [Piece::Empty, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2], // - - X X X
    [Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Empty], // X X X - -
    [Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Empty], // X X - X -
    [Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Empty], // X - X X -
    [Piece::Player2, Piece::Empty, Piece::Empty, Piece::Player2, Piece::Player2], // X - - X X

];

pub const FREE_FOUR: [[Piece; 6]; 2] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty], // - X X X X -

    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty], // - O O O O -
];	

pub const DEVELOPING_FOUR: [[Piece; 5]; 10] = [
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1], // - X X X X
    [Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1], // X X X - X
    [Piece::Player1, Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1], // X X - X X
    [Piece::Player1, Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player1], // X - X X X
    [Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Empty], // X X X X -

    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2], // - O O O O
    [Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2], // O O O - O
    [Piece::Player2, Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2], // O O - O O
    [Piece::Player2, Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player2], // O - O O O
    [Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Empty], // O O O O -
];

pub const FIVE_IN_A_ROW: [[Piece; 5]; 2] = [
    [Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1, Piece::Player1], // X X X X X

    [Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2, Piece::Player2], // O O O O O
];

pub const POSSIBLE_CAPTURE: [[Piece; 4]; 4] = [
    [Piece::Player1, Piece::Player2, Piece::Player2, Piece::Empty], // X O O -
    [Piece::Empty, Piece::Player2, Piece::Player2, Piece::Player1], // - O O X
    [Piece::Player2, Piece::Player1, Piece::Player1, Piece::Empty], // O X X -
    [Piece::Empty, Piece::Player1, Piece::Player1, Piece::Player2], // - X X O
];

pub const CAPTURE: [[Piece; 4]; 2] = [
    [Piece::Player1, Piece::Player2, Piece::Player2, Piece::Player1], // X O O X
    [Piece::Player2, Piece::Player1, Piece::Player1, Piece::Player2], // O X X O
];

// PatternsValue = {
//     Patterns.POTENTIAL_CAPTURE        : 1,
//     Patterns.AX_DEVELOPING_TO_2       : 10,
//     Patterns.AX_DEVELOPING_TO_3       : 100,
//     Patterns.CAPTURE                  : 2000,
//     Patterns.AX_DEVELOPING_TO_4       : 10000,
//     Patterns.FREE_3                   : 100000,
//     Patterns.FREE_4                   : 1000000,
//     Patterns.FIVE_IN_A_ROW            : float('inf'),
// }