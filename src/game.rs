use super::ai::minimax;
use chess::{Board, BoardStatus, ChessMove, Color};
use std::str::FromStr;

/// Represents the status of the game.
#[derive(PartialEq, Debug)]
pub enum Status {
    Ongoing,
    Checkmate(Color),
    Stalemate,
}

/// Represents the game mode.
pub enum GameMode {
    TwoPlayer,
    SinglePlayer(Color),
}

/// Represents a chess game state.
///
/// Holds the current board state, current turn and move/board history.
pub struct Game {
    board: Board,
    turn: Color,
    game_mode: GameMode,
    recursion_depth: Option<u32>,
    history: Vec<(Board, Color)>,
    moves: Vec<ChessMove>,
}

impl Game {
    /// Creates a new two-player Game instance with standard starting position.
    ///
    /// # Examples
    ///
    /// ```
    /// let game = Game::new_multi();
    /// assert_eq!(game.turn, Color::White);
    /// ```
    pub fn new_multi() -> Self {
        Self {
            board: Board::default(),
            turn: Color::White,
            game_mode: GameMode::TwoPlayer,
            recursion_depth: None,
            history: Vec::new(),
            moves: Vec::new(),
        }
    }

    /// Creates a new single-player Game instance with standard starting position.
    ///
    /// # Arguments
    ///
    /// * player_color - a chess::Color instance representing the player color
    /// * recursion_depth - An u32 representing the recursion depth for AI
    ///
    /// # Example
    ///
    /// ```
    /// let game = Game::new_single(Color::White, 5);
    /// game.display_board();
    /// ```
    pub fn new_single(player_color: Color, recursion_depth: u32) -> Self {
        Self {
            board: Board::default(),
            turn: Color::White,
            game_mode: GameMode::SinglePlayer(player_color),
            recursion_depth: Some(recursion_depth),
            history: Vec::new(),
            moves: Vec::new(),
        }
    }

    /// Attempts to generate a ChessMove from the given inputstring.
    ///
    /// Depending on the `uci` flag, the function expects the input either
    /// in UCI format or in Standard Algebraic Notation (SAN).
    ///
    /// # Arguments
    ///
    /// * `input` - A &str representing the move
    /// * `uci` - A bool indicating whether the input is in UCI format
    ///
    /// # Returns
    ///
    /// * `Ok(ChessMove)` - If the input is valid and the move is legal
    /// * `Err(String)` - If the input is invalid or the move is illegal
    ///
    /// # Examples
    ///
    /// ```
    /// let game = Game::new_multi();
    /// assert!(game.parse_move("e2e4", true).is_ok());
    /// assert!(game.parse_move("e4", false).is_ok());
    /// ```
    fn parse_move(&self, input: &str, uci: bool) -> Result<ChessMove, String> {
        if uci {
            match ChessMove::from_str(input) {
                Ok(mv) => {
                    if self.board.legal(mv) {
                        Ok(mv)
                    } else {
                        Err("Illegal move!".into())
                    }
                }
                Err(_) => Err("Invalid input format!".into()),
            }
        } else {
            match ChessMove::from_san(&self.board, input) {
                Ok(mv) => Ok(mv),
                Err(_) => Err("Invalid input!".into()),
            }
        }
    }

    /// Makes a move on the board.
    ///
    /// # Arguments
    ///
    /// * `mv` - a ChessMove instance
    ///
    /// # Examples
    ///
    /// ```
    /// let mut game = Game::new_multi();
    /// let mv = parse_move("e2e4", true).unwrap();
    /// game.make_move(mv);
    /// ```
    pub fn make_move(&mut self, mv: ChessMove) {
        self.history.push((self.board, self.turn));
        self.board = self.board.make_move_new(mv);
        self.turn = !self.turn;
        self.moves.push(mv);
    }

    /// Attempts to make a move from the given inputstring.
    /// Parses the input using `self.parse_move()` method.
    /// If it succeeds, makes the move using `self.make_move()`.
    /// Otherwise, returns an error.
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
    /// let mut game = Game::new_multi();
    /// // This should succeed (for UCI input)
    /// assert!(game.make_move_from_str("e2e4", true).is_ok());
    /// ```
    pub fn make_move_from_str(&mut self, input: &str, uci: bool) -> Result<(), String> {
        match self.parse_move(input, uci) {
            Ok(mv) => {
                self.make_move(mv);
                Ok(())
            }
            Err(e) => Err(e),
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
    /// let game = Game::new_multi();
    /// assert_eq!(game.status(), Status::Ongoing);
    /// ```
    pub fn status(&self) -> Status {
        match self.board.status() {
            BoardStatus::Ongoing => Status::Ongoing,
            BoardStatus::Checkmate => Status::Checkmate(!self.turn),
            BoardStatus::Stalemate => Status::Stalemate,
        }
    }

    /// Returns the current turn
    pub fn turn(&self) -> Color {
        self.turn
    }

    /// Returns the current board state
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Returns the move history of the game
    pub fn moves(&self) -> &Vec<ChessMove> {
        &self.moves
    }

    /// Gets the best move generated by AI.
    ///
    /// # Returns
    ///
    /// * `Ok(ChessMove)` if there is a legal move
    /// * `Err()` if there are no legal moves
    ///
    /// # Example
    ///
    /// ```
    /// let mut game = Game::new_single(Color:Black, 3);
    /// match game.get_ai_move() {
    ///     Ok(mv) => game.make_move(mv),
    ///     Err(e) => println!("{}", e)
    /// }
    /// ```
    pub fn get_ai_move(&self) -> Result<ChessMove, String> {
        let ai_color = match self.game_mode {
            GameMode::SinglePlayer(player_color) => !player_color,
            GameMode::TwoPlayer => return Err("AI can only be used in single player mode".into()),
        };
        let (_eval, best_move) = minimax(
            &self.board,
            self.recursion_depth.unwrap(),
            true,
            ai_color,
            i32::MIN,
            i32::MAX,
        );
        match best_move {
            Some(m) => Ok(m),
            None => Err("No legal moves for AI available".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_status_ongoing() {
        let game = Game::new_multi();
        assert_eq!(game.status(), Status::Ongoing)
    }

    #[test]
    fn is_status_checkmate_for_black() {
        let mut game = Game::new_multi();
        // Fool's mate
        game.make_move_from_str("f3", false).unwrap();
        game.make_move_from_str("e5", false).unwrap();
        game.make_move_from_str("g4", false).unwrap();
        game.make_move_from_str("Qh4", false).unwrap();
        assert_eq!(game.status(), Status::Checkmate(chess::Color::Black));
    }

    #[test]
    fn is_status_checkmate_for_white() {
        let mut game = Game::new_multi();
        // Reverse Fool's mate
        game.make_move_from_str("e4", false).unwrap();
        game.make_move_from_str("f6", false).unwrap();
        game.make_move_from_str("d4", false).unwrap();
        game.make_move_from_str("g5", false).unwrap();
        game.make_move_from_str("Qh5", false).unwrap();
        assert_eq!(game.status(), Status::Checkmate(chess::Color::White));
    }

    #[test]
    fn is_status_stalemate() {
        let mut game = Game::new_multi();
        // Sam Loyd's stalemate in 10 moves
        game.make_move_from_str("e3", false).unwrap();
        game.make_move_from_str("a5", false).unwrap();
        game.make_move_from_str("Qh5", false).unwrap();
        game.make_move_from_str("Ra6", false).unwrap();
        game.make_move_from_str("Qxa5", false).unwrap();
        game.make_move_from_str("h5", false).unwrap();
        game.make_move_from_str("Qxc7", false).unwrap();
        game.make_move_from_str("Rah6", false).unwrap();
        game.make_move_from_str("h4", false).unwrap();
        game.make_move_from_str("f6", false).unwrap();
        game.make_move_from_str("Qxd7+", false).unwrap();
        game.make_move_from_str("Kf7", false).unwrap();
        game.make_move_from_str("Qxb7", false).unwrap();
        game.make_move_from_str("Qd3", false).unwrap();
        game.make_move_from_str("Qxb8", false).unwrap();
        game.make_move_from_str("Qh7", false).unwrap();
        game.make_move_from_str("Qxc8", false).unwrap();
        game.make_move_from_str("Kg6", false).unwrap();
        game.make_move_from_str("Qe6", false).unwrap();
        assert_eq!(game.status(), Status::Stalemate);
    }
}
