mod lib;

use crate::lib::MarbleGame;

fn main() {
    const N_PLAYERS: u32 = 413;
    const N_MARBLES: usize = 71082;

    let mut marble_game = MarbleGame::new(N_PLAYERS, N_MARBLES);
    marble_game.simulate();
    let high_score = marble_game.get_high_score();

    println!("High score with {} players and {} marbles is: {}", N_PLAYERS, N_MARBLES, high_score);

    let mut marble_game = MarbleGame::new(N_PLAYERS, N_MARBLES * 100);
    marble_game.simulate();
    let high_score = marble_game.get_high_score();

    println!("High score with {} players and {} marbles is: {}", N_PLAYERS, N_MARBLES * 100, high_score);
}
