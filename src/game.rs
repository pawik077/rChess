use chess::{Board, BoardStatus, ChessMove, Color};
use std::str::FromStr;

pub enum Status {
   Ongoing,
   Checkmate(Color),
   Stalemate 
}

pub struct Game {
    board: Board,
    turn: Color,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            turn: Color::White,
        }
    }

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

    pub fn make_move(&mut self, input: &str) -> Result<(), String> {
        match ChessMove::from_str(input) {
            Ok(chess_move) => {
                if self.board.legal(chess_move) {
                    self.board = self.board.make_move_new(chess_move);
                    self.turn = !self.turn;
                    Ok(())
                } else {
                    Err("Illegal move!".into())
                }
            }
            Err(_) => Err("Invalid input format!".into()),
        }
    }

    pub fn status(&self) -> Status { 
        match self.board.status() {
            BoardStatus::Ongoing => Status::Ongoing,
            BoardStatus::Checkmate => Status::Checkmate(!self.turn),
            BoardStatus::Stalemate => Status::Stalemate
        }
    }
}
