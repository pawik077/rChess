use chess::{Board, BoardStatus, ChessMove, Color};
use std::str::FromStr;

/// Represents the status of the game.
pub enum Status {
   Ongoing,
   Checkmate(Color),
   Stalemate 
}

/// Represents a chess game state.
/// 
/// Holds the current board state, current turn and move/board history.
pub struct Game {
    board: Board,
    turn: Color,
    history: Vec<(Board, Color)>,
    moves: Vec<ChessMove>,
}

impl Game {
    /// Creates a new Game instance with standard starting position.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let game = Game::new();
    /// assert_eq!(game.turn, Color::White);
    /// ```
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            turn: Color::White,
            history: Vec::new(),
            moves: Vec::new(),
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
    /// let game = Game::new();
    /// game.display_board();
    /// ```
    pub fn display_board(&self) {
        let mut board_str = String::new();
        let (rank_range, file_range): (Vec<usize>, Vec<usize>) = match self.turn {
            Color::White => ((0..8).rev().collect(), (0..8).collect()),
            Color::Black => ((0..8).collect(), (0..8).rev().collect())
        }; //a hack to make up for the lack of 8..0 in rust

        for rank in &rank_range {
            board_str.push_str(&format!("{}  ", rank + 1));
            for file in &file_range {
                let square = chess::Square::make_square(chess::Rank::from_index(*rank), chess::File::from_index(*file));
                let piece = self.board.piece_on(square);
                let color = self.board.color_on(square);

                let symbol = match (piece, color) {
                    (Some(chess::Piece::Pawn), Some(chess::Color::White)) => '♙',
                    (Some(chess::Piece::Pawn), Some(chess::Color::Black)) => '♟',
                    (Some(chess::Piece::Knight), Some(chess::Color::White)) => '♘',
                    (Some(chess::Piece::Knight), Some(chess::Color::Black)) => '♞',
                    (Some(chess::Piece::Rook), Some(chess::Color::White)) => '♖',
                    (Some(chess::Piece::Rook), Some(chess::Color::Black)) => '♜',
                    (Some(chess::Piece::Bishop), Some(chess::Color::White)) => '♗',
                    (Some(chess::Piece::Bishop), Some(chess::Color::Black)) => '♝',
                    (Some(chess::Piece::Queen), Some(chess::Color::White)) => '♕',
                    (Some(chess::Piece::Queen), Some(chess::Color::Black)) => '♛',
                    (Some(chess::Piece::King), Some(chess::Color::White)) => '♔',
                    (Some(chess::Piece::King), Some(chess::Color::Black)) => '♚',
                    _ => '.'
                };
                board_str.push(symbol);
                board_str.push(' ');
            }
            board_str.push('\n');
        }
        board_str.push_str(match self.turn {
            Color::White => "   a b c d e f g h\n",
            Color::Black => "   h g f e d c b a\n"
        });
        println!("{}", board_str);
    }

    /// Attempts to make a move from the given inputstring.
    /// 
    /// Depending on the `uci` flag, the function expects the input either
    /// in UCI format or in Standard Algebraic Notation (SAN). If the move
    /// is valid and legal it is applied to the game state. Otherwise an error 
    /// message is returned.
    /// 
    /// # Arguments
    /// 
    /// * `input` - A &str representing the move
    /// * `uci` - A bool indicating whether the input is in UCI format
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - the input format is invalid,
    /// - the move is illegal.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // Assuming the game starts at the standard position:
    /// let mut game = Game::new();
    /// // This should succeed (for UCI input)
    /// assert!(game.make_move("e2e4", true).is_ok());
    /// ```
    pub fn make_move(&mut self, input: &str, uci: bool) -> Result<(), String> {
        if uci {
            match ChessMove::from_str(input) {
                Ok(chess_move) => {
                    if self.board.legal(chess_move) {
                        self.history.push((self.board, self.turn));
                        self.board = self.board.make_move_new(chess_move);
                        self.turn = !self.turn;
                        self.moves.push(chess_move);
                        Ok(())
                    } else {
                        Err("Illegal move!".into())
                    }
                }
                Err(_) => Err("Invalid input format!".into()),
            }
        } else {
            match ChessMove::from_san(&self.board, input) {
                Ok(chess_move) => {
                    self.history.push((self.board, self.turn));
                    self.board = self.board.make_move_new(chess_move);
                    self.turn = !self.turn;
                    self.moves.push(chess_move);
                    Ok(())
                }
                Err(_) => Err("Invalid input!".into())
            }
        }
    }

    /// Undoes the last move, reverting the board to its previous state.
    /// 
    /// Pops the last state from the undo history stack and restores
    /// both the board and the turn. If no moves have been made, 
    /// returns an error.
    pub fn undo(&mut self) -> Result<(), String> {
        if let Some((prev_board, prev_turn)) = self.history.pop() {
            self.board = prev_board;
            self.turn = prev_turn;
            self.moves.pop();
            Ok(())
        } else {
            Err("No moves to undo!".into())
        }
    }

    /// Prints the history of moves played so far.
    /// 
    /// Moves are displayed in pairs using UCI notation, along
    /// with their number - first the white move, then black.
    /// If black hasn't made their move in the last turn,
    /// only white move is printed.
    pub fn print_move_history(&self) {
        println!("Move history:");
        for (i, chunk) in self.moves.chunks(2).enumerate() {
            let white_move = chunk.get(0).unwrap();
            match chunk.get(1) {
                Some(black_move) => println!("{}. {} {}", i + 1, white_move, black_move),
                None => println!("{}. {}", i + 1, white_move),
            }
        }
        println!();
    }

    /// Returns the status of the game.
    /// Checks the board state and maps the chess crate's `BoardStatus`
    /// to the custom [`Status`] enum.
    /// 
    /// # Returns
    /// 
    /// - [`Status::Ongoing`] if the game is still in progress
    /// - [`Status::Stalemate`] if there are no legal moves for the current player but the player is not in check
    /// - [`Status::Checkmate`] if the current player is in check and there are no legal moves available. Also returns the winner of the game.
    /// 
    /// # Example
    /// 
    /// ```
    /// let game = Game::new();
    /// assert_eq!(game.status(), Status::Ongoing);
    /// ```
    pub fn status(&self) -> Status { 
        match self.board.status() {
            BoardStatus::Ongoing => Status::Ongoing,
            BoardStatus::Checkmate => Status::Checkmate(!self.turn),
            BoardStatus::Stalemate => Status::Stalemate
        }
    }
}
