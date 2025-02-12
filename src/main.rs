mod game;
use game::{Game, Status};
use std::io::{self, Write};

fn main() {
    let mut game = Game::new();
    loop {
        game.display_board();
        print!("Enter move: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "quit" => break,
            "undo" => {
                if let Err(e) = game.undo() {
                    println!("{}", e);
                    continue;
                }
            },
            "print" => game.print_move_history(),
            _ => {
                if let Err(e) = game.make_move(input, false) {
                    println!("{}", e);
                    continue;
                }
            }
        }

        match game.status() {
            Status::Checkmate(color)=> {
                println!("Game Over: {:?} wins!", color);
                break;
            }
            Status::Stalemate => {
                println!("Stalemate");
                break;
            }
            Status::Ongoing => ()
        }
    }
}
