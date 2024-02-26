mod game;
mod ia;
use game::terminal_game;
use game::terminal_game_ia;


fn main() {
    let game = 0;
    if game == 1 {
        terminal_game();
        return;
    }
    else if game == 0 {
        terminal_game_ia();
        return;
    }
}