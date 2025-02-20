#[cfg(test)]
use super::game::*;

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