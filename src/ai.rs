use chess::{Board, BoardStatus, ChessMove, Color, MoveGen, Piece, ALL_SQUARES};

pub fn evaluate(board: &Board, perspective: Color) -> i32 {
    let mut score = 0;
    for square in ALL_SQUARES {
        if let Some(piece) = board.piece_on(square) {
            let piece_value = match piece {
                Piece::Pawn => 1,
                Piece::Knight => 3,
                Piece::Bishop => 3,
                Piece::Rook => 5,
                Piece::Queen => 9,
                Piece::King => 20,
            };
            if board.color_on(square) == Some(perspective) {
                score += piece_value;
            } else {
                score -= piece_value;
            }
        }
    }
    score
}

pub fn minimax(
    board: &Board,
    depth: u32,
    maximizing: bool,
    perspective: Color,
    mut alpha: i32,
    mut beta: i32,
) -> (i32, Option<ChessMove>) {
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        return (evaluate(board, perspective), None);
    }

    let mut best_move = None;
    let mut best_eval = if maximizing { i32::MIN } else { i32::MAX };

    for m in MoveGen::new_legal(board) {
        let new_board = board.make_move_new(m);
        let (eval, _) = minimax(
            &new_board,
            depth - 1,
            !maximizing,
            !perspective,
            alpha,
            beta,
        );
        if maximizing {
            if eval > best_eval {
                best_eval = eval;
                best_move = Some(m);
            }
            alpha = alpha.max(eval);
        } else {
            if eval < best_eval {
                best_eval = eval;
                best_move = Some(m);
            }
            beta = beta.min(eval);
        }
        if beta <= alpha {
            break;
        }
    }
    (best_eval, best_move)
}

