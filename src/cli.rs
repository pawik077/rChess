use chess::Color;
use super::game::{Game, Status};
use std::io::{self, Write};

pub fn intro() {
    println!("WELCOME TO CHESS!!");
    let valid_inputs = ["quit", "single", "multi"];
    let input: String  = loop {
        print!("Select game mode (single or multi, quit to exit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input, please try again.");
            continue;
        }
        let input = input.trim().to_lowercase();
        if valid_inputs.contains(&input.as_str()) {
            break input;
        } else {
            eprintln!("Illegal input, please try again.");
        }
    };
    match input.as_str() {
        "quit" => (),
        "single" => single_player(),
        "multi" => two_player(),
        _ => unreachable!(),
    }
}

fn two_player() {
    let mut game = Game::new_multi();
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
                if let Err(e) = game.make_move_from_str(input, false) {
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

fn single_player() {
    let input: String  = loop {
        print!("Select your color (white or black, random to choose randomly): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input, please try again.");
            continue;
        }
        let input = input.trim().to_lowercase();
        if input == "white" || input == "black" || input == "random" {
            break input;
        } else {
            eprintln!("Illegal input, please try again.");
        }
    };
    let player_color = match input.as_str() {
        "white" => Color::White,
        "black" => Color::Black,
        "random" => unimplemented!(),
        _ => unreachable!(),
    };
    let mut game = Game::new_single(player_color, 7);

    loop {
        game.display_board();
        if game.turn() == player_color {
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
                    if let Err(e) = game.make_move_from_str(input, false) {
                        println!("{}", e);
                        continue;
                    }
                }
            }
        } else {
            match game.get_ai_move() {
                Ok(mv) => game.make_move(mv),
                Err(e) => println!("{}", e)
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