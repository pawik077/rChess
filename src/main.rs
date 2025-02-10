mod game;
use game::Game;
use std::io::{self, Write};

fn main() {
    let mut game = Game::new();
    loop {
        game.display_board();
        print!("Enter move (e.g., e2e4): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "quit" => break,
            _ => {
                if let Err(e) = game.make_move(input) {
                    println!("{}", e);
                    continue;
                }
            }
        }

        if let Err(e) = game.make_move(input) {
            println!("{}", e);
            continue;
        }

        if game.is_game_over() {
            println!("Game Over");
            break;
        }
    }
}
