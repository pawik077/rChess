use super::game::{Game, Status};
use chess::{Color, Piece};
use rand::random_bool;
use std::io::{self, Write};

pub fn intro() {
    println!("WELCOME TO CHESS!!");
    let valid_inputs = ["quit", "single", "multi"];
    let input: String = loop {
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
        display_board(&game);
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
            }
            "print" => print_move_history(&game),
            _ => {
                if let Err(e) = game.make_move_from_str(input, false) {
                    println!("{}", e);
                    continue;
                }
            }
        }

        match game.status() {
            Status::Checkmate(color) => {
                println!("Game Over: {:?} wins!", color);
                break;
            }
            Status::Stalemate => {
                println!("Stalemate");
                break;
            }
            Status::Ongoing => (),
        }
    }
}

fn single_player() {
    let input: String = loop {
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
        "random" => {
            if random_bool(0.5) {
                Color::White
            } else {
                Color::Black
            }
        }
        _ => unreachable!(),
    };
    println!("You're playing as {:?}", player_color);
    let mut game = Game::new_single(player_color, 7);

    loop {
        display_board(&game);
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
                }
                "print" => print_move_history(&game),
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
                Err(e) => println!("{}", e),
            }
        }
        match game.status() {
            Status::Checkmate(color) => {
                println!("Game Over: {:?} wins!", color);
                break;
            }
            Status::Stalemate => {
                println!("Stalemate");
                break;
            }
            Status::Ongoing => (),
        }
    }
}

/// Converts a chess piece and color into a Unicode character for display.
fn piece_symbol(piece: Piece, color: Color) -> char {
    match (piece, color) {
        (Piece::Pawn, Color::White) => '♙',
        (Piece::Pawn, Color::Black) => '♟',
        (Piece::Knight, Color::White) => '♘',
        (Piece::Knight, Color::Black) => '♞',
        (Piece::Rook, Color::White) => '♖',
        (Piece::Rook, Color::Black) => '♜',
        (Piece::Bishop, Color::White) => '♗',
        (Piece::Bishop, Color::Black) => '♝',
        (Piece::Queen, Color::White) => '♕',
        (Piece::Queen, Color::Black) => '♛',
        (Piece::King, Color::White) => '♔',
        (Piece::King, Color::Black) => '♚',
    }
}

/// Displays the current board state in a human-readable format.
///
/// The board is printed to the console with ranks and files labeled
/// and pieces represented by Unicode characters. The board is
/// automatically rotated based on the current player's turn.
///
/// # Example
///
/// ```
/// let game = Game::new_multi();
/// display_board(&game);
/// ```
fn display_board(game: &Game) {
    let mut board_str = String::new();

    let board = game.board();
    let turn = game.turn();

    let (rank_range, file_range): (Vec<usize>, Vec<usize>) = match turn {
        Color::White => ((0..8).rev().collect(), (0..8).collect()),
        Color::Black => ((0..8).collect(), (0..8).rev().collect()),
    }; //a hack to make up for the lack of 8..0 in rust

    for rank in &rank_range {
        board_str.push_str(&format!("{}  ", rank + 1));
        for file in &file_range {
            let square = chess::Square::make_square(
                chess::Rank::from_index(*rank),
                chess::File::from_index(*file),
            );
            let piece = board.piece_on(square);
            let color = board.color_on(square);

            let symbol = match (piece, color) {
                (Some(p), Some(c)) => piece_symbol(p, c),
                _ => '.',
            };
            board_str.push(symbol);
            board_str.push(' ');
        }
        board_str.push('\n');
    }
    board_str.push_str(match turn {
        Color::White => "   a b c d e f g h\n",
        Color::Black => "   h g f e d c b a\n",
    });
    println!("{}", board_str);
}

/// Prints the history of moves played so far.
///
/// Moves are displayed in pairs using UCI notation, along
/// with their number - first the white move, then black.
/// If black hasn't made their move in the last turn,
/// only white move is printed.
fn print_move_history(game: &Game) {
    println!("Move history:");
    for (i, chunk) in game.moves().chunks(2).enumerate() {
        match chunk {
            [w, b] => println!("{}. {} {}", i + 1, w, b),
            [w] => println!("{}. {}", i + 1, w),
            _ => unreachable!(),
        }
    }
    println!();
}
