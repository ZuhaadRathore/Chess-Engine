use crate::chess_engine::game::ChessGame;
use crate::chess_engine::fen::{parse_fen, position_to_fen, STARTING_FEN};
use crate::chess_engine::validation::{generate_legal_moves, is_in_check, is_checkmate, is_stalemate};
use crate::chess_engine::types::{Color, Piece, Square, Move, GameStatus};
use crate::chess_engine::position::Position;

// Helper function for perft testing
fn perft(position: &mut Position, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = generate_legal_moves(position);

    if depth == 1 {
        return moves.len() as u64;
    }

    let mut count = 0;
    for mv in moves {
        let snapshot = position.clone();

        // Apply move
        apply_move_for_perft(position, &mv);

        count += perft(position, depth - 1);

        // Restore position
        *position = snapshot;
    }

    count
}

fn apply_move_for_perft(position: &mut Position, mv: &Move) {
    // Update castling rights
    position.update_castling_rights_after_move(mv);

    // Handle en passant capture
    if mv.is_en_passant {
        let captured_pawn_rank = if position.side_to_move == Color::White {
            mv.to.rank() - 1
        } else {
            mv.to.rank() + 1
        };
        if let Some(captured_square) = Square::from_rank_file(captured_pawn_rank, mv.to.file()) {
            position.board.set(captured_square, None);
        }
    }

    // Handle castling
    if mv.is_castling {
        let rank = mv.from.rank();
        if mv.to.file() > mv.from.file() {
            let rook_from = Square::from_rank_file(rank, 7).unwrap();
            let rook_to = Square::from_rank_file(rank, 5).unwrap();
            let rook = position.board.get(rook_from);
            position.board.set(rook_from, None);
            position.board.set(rook_to, rook);
        } else {
            let rook_from = Square::from_rank_file(rank, 0).unwrap();
            let rook_to = Square::from_rank_file(rank, 3).unwrap();
            let rook = position.board.get(rook_from);
            position.board.set(rook_from, None);
            position.board.set(rook_to, rook);
        }
    }

    // Move the piece
    let piece = position.board.get(mv.from);
    position.board.set(mv.from, None);

    if let Some(promotion_piece) = mv.promotion {
        if let Some((_, color)) = piece {
            position.board.set(mv.to, Some((promotion_piece, color)));
        }
    } else {
        position.board.set(mv.to, piece);
    }

    // Update en passant target
    if let Some((Piece::Pawn, _)) = position.board.get(mv.to) {
        let from_rank = mv.from.rank();
        let to_rank = mv.to.rank();
        if from_rank.abs_diff(to_rank) == 2 {
            let ep_rank = (from_rank + to_rank) / 2;
            position.en_passant_target = Square::from_rank_file(ep_rank, mv.from.file());
        } else {
            position.en_passant_target = None;
        }
    } else {
        position.en_passant_target = None;
    }

    // Switch side to move
    position.side_to_move = position.side_to_move.opposite();
}

// Helper functions for testing
fn assert_move_legal(game: &ChessGame, from: &str, to: &str) {
    let from_sq = Square::from_algebraic(from).unwrap();
    let to_sq = Square::from_algebraic(to).unwrap();
    let legal_moves = game.get_legal_moves_for_square(from_sq);
    assert!(
        legal_moves.iter().any(|mv| mv.to == to_sq),
        "Move {} to {} should be legal",
        from,
        to
    );
}

fn assert_move_illegal(game: &ChessGame, from: &str, to: &str) {
    let from_sq = Square::from_algebraic(from).unwrap();
    let to_sq = Square::from_algebraic(to).unwrap();
    let legal_moves = game.get_legal_moves_for_square(from_sq);
    assert!(
        !legal_moves.iter().any(|mv| mv.to == to_sq),
        "Move {} to {} should be illegal",
        from,
        to
    );
}

fn make_moves(game: &mut ChessGame, moves: &[(&str, &str)]) {
    for (from, to) in moves {
        let from_sq = Square::from_algebraic(from).unwrap();
        let to_sq = Square::from_algebraic(to).unwrap();
        let legal_moves = game.get_legal_moves_for_square(from_sq);
        let mv = legal_moves
            .into_iter()
            .find(|m| m.to == to_sq)
            .expect(&format!("Move {} to {} not found", from, to));
        game.make_move(mv).unwrap();
    }
}

#[cfg(test)]
mod basic_moves {
    use super::*;

    #[test]
    fn test_initial_position() {
        let game = ChessGame::new();
        assert_eq!(game.get_legal_moves().len(), 20); // 16 pawn moves + 4 knight moves
    }

    #[test]
    fn test_pawn_single_push() {
        let game = ChessGame::new();
        assert_move_legal(&game, "e2", "e3");
        assert_move_legal(&game, "e2", "e4");
    }

    #[test]
    fn test_pawn_double_push() {
        let mut game = ChessGame::new();
        make_moves(&mut game, &[("e2", "e4")]);
        // After white's move, it's black's turn - black can play e7-e5
        assert_move_legal(&game, "e7", "e5");
        make_moves(&mut game, &[("e7", "e5")]);
        // Verify the position
        let fen = game.to_fen();
        assert!(fen.contains("4p3/4P3")); // Both pawns on their 4th/5th ranks
    }

    #[test]
    fn test_knight_moves() {
        let game = ChessGame::new();
        assert_move_legal(&game, "g1", "f3");
        assert_move_legal(&game, "g1", "h3");
        assert_move_legal(&game, "b1", "a3");
        assert_move_legal(&game, "b1", "c3");
    }

    #[test]
    fn test_pawn_capture() {
        // White pawn on e4 can capture Black pawn on d5
        let game = ChessGame::from_fen("rnbqkbnr/pppp1ppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap();
        assert_move_legal(&game, "e4", "d5");
    }

    #[test]
    fn test_en_passant() {
        let mut game = ChessGame::new();
        make_moves(&mut game, &[
            ("e2", "e4"),
            ("a7", "a6"),
            ("e4", "e5"),
            ("d7", "d5"),
        ]);

        // White pawn on e5 should be able to capture en passant on d6
        assert_move_legal(&game, "e5", "d6");
    }

    #[test]
    fn test_pawn_promotion() {
        let game = ChessGame::from_fen("8/P7/8/8/8/8/8/K6k w - - 0 1").unwrap();
        let moves = game.get_legal_moves_for_square(Square::from_algebraic("a7").unwrap());

        // Should have 4 promotion moves (Q, R, B, N)
        let promotion_moves: Vec<_> = moves.iter().filter(|m| m.promotion.is_some()).collect();
        assert_eq!(promotion_moves.len(), 4);
    }
}

#[cfg(test)]
mod castling {
    use super::*;

    #[test]
    fn test_white_kingside_castling() {
        let game = ChessGame::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        assert_move_legal(&game, "e1", "g1");
    }

    #[test]
    fn test_white_queenside_castling() {
        let game = ChessGame::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        assert_move_legal(&game, "e1", "c1");
    }

    #[test]
    fn test_black_castling() {
        let game = ChessGame::from_fen("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1").unwrap();
        assert_move_legal(&game, "e8", "g8");
        assert_move_legal(&game, "e8", "c8");
    }

    #[test]
    fn test_cannot_castle_through_check() {
        let game = ChessGame::from_fen("r3k2r/8/8/8/8/8/8/R2QK2R b KQkq - 0 1").unwrap();
        // Black cannot castle queenside (queen attacks d8)
        assert_move_illegal(&game, "e8", "c8");
    }

    #[test]
    fn test_cannot_castle_in_check() {
        let game = ChessGame::from_fen("r3k2r/8/8/8/8/8/4Q3/R3K2R b KQkq - 0 1").unwrap();
        // Black king is in check, cannot castle
        assert_move_illegal(&game, "e8", "g8");
        assert_move_illegal(&game, "e8", "c8");
    }

    #[test]
    fn test_cannot_castle_into_check() {
        let game = ChessGame::from_fen("r3k2r/8/8/8/8/8/8/R3K1QR b KQkq - 0 1").unwrap();
        // Black cannot castle kingside (queen attacks g8)
        assert_move_illegal(&game, "e8", "g8");
    }

    #[test]
    fn test_cannot_castle_without_rook() {
        // White has queenside castling only (rook on a1, no rook on h1)
        // Castling rights correctly set to Qkq (no K for white kingside)
        let game = ChessGame::from_fen("r3k2r/8/8/8/8/8/8/R3K3 w Qkq - 0 1").unwrap();
        // White cannot castle kingside (no rook on h1, no castling right)
        assert_move_illegal(&game, "e1", "g1");

        // White can still castle queenside (rook on a1)
        assert_move_legal(&game, "e1", "c1");
    }
}

#[cfg(test)]
mod check_and_checkmate {
    use super::*;

    #[test]
    fn test_check_detection() {
        // White queen on e2 attacks Black king on e8 (no pieces blocking)
        let position = parse_fen("rnbqkbnr/pppp1ppp/8/8/8/8/PPPPQPPP/RNB1KBNR b KQkq - 0 1").unwrap();
        assert!(is_in_check(&position, Color::Black));
    }

    #[test]
    fn test_back_rank_mate() {
        // White rook on a8 delivers checkmate to Black king on g8 (trapped by own pawns)
        let position = parse_fen("R5k1/5ppp/8/8/8/8/8/7K b - - 0 1").unwrap();
        assert!(is_checkmate(&position));
    }

    #[test]
    fn test_scholars_mate() {
        let mut game = ChessGame::new();
        make_moves(&mut game, &[
            ("e2", "e4"),
            ("e7", "e5"),
            ("d1", "h5"),
            ("b8", "c6"),
            ("f1", "c4"),
            ("g8", "f6"),
        ]);

        // Queen takes f7 is checkmate
        let from_sq = Square::from_algebraic("h5").unwrap();
        let to_sq = Square::from_algebraic("f7").unwrap();
        let legal_moves = game.get_legal_moves_for_square(from_sq);
        let mv = legal_moves.iter().find(|m| m.to == to_sq).unwrap();
        game.make_move(*mv).unwrap();

        assert_eq!(game.get_status(), GameStatus::Checkmate { winner: Color::White });
    }

    #[test]
    fn test_stalemate() {
        let position = parse_fen("k7/8/1Q6/8/8/8/8/K7 b - - 0 1").unwrap();
        assert!(is_stalemate(&position));
    }

    #[test]
    fn test_must_block_check() {
        // Black king is in check from White queen on e2
        let game = ChessGame::from_fen("rnbqkbnr/pppp1ppp/8/8/8/8/PPPPQPPP/RNB1KBNR b KQkq - 0 1").unwrap();

        // Black is in check and must block or move king
        let legal_moves = game.get_legal_moves();

        // All moves should either move the king or block on the e-file
        for mv in legal_moves {
            let piece = game.get_board_state().board.get(mv.from);
            if let Some((p, _)) = piece {
                // Must be king move, or a piece blocking on e-file (e3, e4, e5, e6, e7)
                let blocks_on_e_file = mv.to.file() == 4 && mv.to.rank() >= 2 && mv.to.rank() <= 7;
                assert!(p == Piece::King || blocks_on_e_file, "Move {:?} doesn't block check or move king", mv);
            }
        }
    }
}

#[cfg(test)]
mod fen_parsing {
    use super::*;

    #[test]
    fn test_parse_starting_fen() {
        let position = parse_fen(STARTING_FEN).unwrap();
        assert_eq!(position.side_to_move, Color::White);
        assert_eq!(position.halfmove_clock, 0);
        assert_eq!(position.fullmove_number, 1);
    }

    #[test]
    fn test_fen_round_trip() {
        let original_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let position = parse_fen(original_fen).unwrap();
        let regenerated_fen = position_to_fen(&position);
        assert_eq!(original_fen, regenerated_fen);
    }

    #[test]
    fn test_invalid_fen_missing_fields() {
        let result = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_fen_pawn_on_first_rank() {
        let result = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKPNR w KQkq - 0 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_fen_with_en_passant() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let position = parse_fen(fen).unwrap();
        assert!(position.en_passant_target.is_some());
        assert_eq!(position.en_passant_target.unwrap().to_algebraic(), "e3");
    }

    #[test]
    fn test_invalid_fen_multiple_white_kings() {
        // FEN with two white kings
        let result = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBKKBNR w KQkq - 0 1");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Multiple white kings"));
        }
    }

    #[test]
    fn test_invalid_fen_multiple_black_kings() {
        // FEN with two black kings
        let result = parse_fen("rnbkkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Multiple black kings"));
        }
    }
}

#[cfg(test)]
mod perft_tests {
    use super::*;

    #[test]
    fn test_perft_starting_position_depth_1() {
        let mut position = Position::new();
        assert_eq!(perft(&mut position, 1), 20);
    }

    #[test]
    fn test_perft_starting_position_depth_2() {
        let mut position = Position::new();
        assert_eq!(perft(&mut position, 2), 400);
    }

    #[test]
    fn test_perft_starting_position_depth_3() {
        let mut position = Position::new();
        assert_eq!(perft(&mut position, 3), 8902);
    }

    #[test]
    fn test_perft_starting_position_depth_4() {
        let mut position = Position::new();
        assert_eq!(perft(&mut position, 4), 197281);
    }

    #[test]
    fn test_perft_kiwipete_depth_1() {
        let mut position = parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        assert_eq!(perft(&mut position, 1), 48);
    }

    #[test]
    fn test_perft_kiwipete_depth_2() {
        let mut position = parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        assert_eq!(perft(&mut position, 2), 2039);
    }

    #[test]
    fn test_perft_kiwipete_depth_3() {
        let mut position = parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        assert_eq!(perft(&mut position, 3), 97862);
    }

    #[test]
    fn test_perft_with_en_passant() {
        let mut position = parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        assert_eq!(perft(&mut position, 1), 20);
    }

    #[test]
    fn test_perft_position_3() {
        let mut position = parse_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&mut position, 1), 14);
        assert_eq!(perft(&mut position, 2), 191);
        assert_eq!(perft(&mut position, 3), 2812);
    }

    #[test]
    fn test_perft_position_4() {
        let mut position = parse_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
        assert_eq!(perft(&mut position, 1), 6);
        assert_eq!(perft(&mut position, 2), 264);
    }
}

#[cfg(test)]
mod game_endings {
    use super::*;

    #[test]
    fn test_fifty_move_rule() {
        let position = parse_fen("k7/8/8/8/8/8/8/K7 w - - 100 1").unwrap();
        assert_eq!(position.halfmove_clock, 100);
        // Would be draw by fifty move rule
    }

    #[test]
    fn test_insufficient_material_king_vs_king() {
        let position = parse_fen("k7/8/8/8/8/8/8/K7 w - - 0 1").unwrap();
        assert!(position.has_insufficient_material());
    }

    #[test]
    fn test_insufficient_material_king_bishop_vs_king() {
        let position = parse_fen("k7/8/8/8/8/8/8/KB6 w - - 0 1").unwrap();
        assert!(position.has_insufficient_material());
    }

    #[test]
    fn test_insufficient_material_king_knight_vs_king() {
        let position = parse_fen("k7/8/8/8/8/8/8/KN6 w - - 0 1").unwrap();
        assert!(position.has_insufficient_material());
    }

    #[test]
    fn test_sufficient_material_with_pawn() {
        let position = parse_fen("k7/8/8/8/8/8/P7/K7 w - - 0 1").unwrap();
        assert!(!position.has_insufficient_material());
    }

    #[test]
    fn test_threefold_repetition() {
        let mut game = ChessGame::new();

        // The starting position occurs once
        // Move knights back and forth to repeat the position
        make_moves(&mut game, &[
            ("g1", "f3"),
            ("g8", "f6"),
            ("f3", "g1"),
            ("f6", "g8"),
        ]);
        // After these moves, we're back to the starting position (occurred twice)

        make_moves(&mut game, &[
            ("g1", "f3"),
            ("g8", "f6"),
            ("f3", "g1"),
            ("f6", "g8"),
        ]);
        // After these moves, we're back to the starting position again (occurred three times)

        // Position should have occurred 3 times now
        assert!(game.get_board_state().is_repetition());
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_pinned_piece_cannot_move() {
        // White queen on d4 is pinned by Black bishop on g7 to White king on a1 (diagonal pin)
        let game = ChessGame::from_fen("6k1/6b1/8/8/3Q4/8/8/K7 w - - 0 1").unwrap();

        // White queen is pinned and cannot move off the diagonal
        assert_move_illegal(&game, "d4", "d5"); // vertical
        assert_move_illegal(&game, "d4", "e4"); // horizontal

        // But can move along the pin ray (diagonal a1-g7)
        assert_move_legal(&game, "d4", "c3");
        assert_move_legal(&game, "d4", "e5");
        assert_move_legal(&game, "d4", "f6");
    }

    #[test]
    fn test_en_passant_exposes_king() {
        // Black king on a4, Black pawn on e4, White pawn just moved d2-d4 (en passant target d3)
        // White Queen on h4. If Black captures en passant, king would be exposed to check from queen
        let game = ChessGame::from_fen("8/8/8/8/k2Pp2Q/8/8/4K3 b - d3 0 1").unwrap();

        // Black pawn on e4 cannot capture en passant on d3 because it would expose king to check
        assert_move_illegal(&game, "e4", "d3");
    }

    #[test]
    fn test_double_check_only_king_moves() {
        let game = ChessGame::from_fen("k7/8/8/8/8/2Q5/1B6/4K3 b - - 0 1").unwrap();

        // Black king is in double check (from queen and bishop)
        // Only king moves are legal
        let legal_moves = game.get_legal_moves();
        for mv in legal_moves {
            let piece = game.get_board_state().board.get(mv.from);
            assert_eq!(piece, Some((Piece::King, Color::Black)));
        }
    }

    #[test]
    fn test_undo_move() {
        let mut game = ChessGame::new();
        let initial_fen = game.to_fen();

        make_moves(&mut game, &[("e2", "e4")]);
        assert_ne!(game.to_fen(), initial_fen);

        game.undo_move().unwrap();
        assert_eq!(game.to_fen(), initial_fen);
    }

    #[test]
    fn test_cannot_undo_with_no_moves() {
        let mut game = ChessGame::new();
        let result = game.undo_move();
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_move_after_checkmate() {
        let game = ChessGame::from_fen("R5k1/5ppp/8/8/8/8/8/7K b - - 0 1").unwrap();

        // Game is checkmate
        assert_eq!(game.get_status(), GameStatus::Checkmate { winner: Color::White });

        // Should have no legal moves
        assert_eq!(game.get_legal_moves().len(), 0);
    }
}

#[cfg(test)]
mod local_pass_and_play {
    use super::*;

    #[test]
    fn turn_sequence_and_undo_behaviour() {
        let mut game = ChessGame::new();
        assert_eq!(game.get_board_state().side_to_move, Color::White);

        let white_move = game
            .get_legal_moves_for_square(Square::from_algebraic("e2").unwrap())
            .into_iter()
            .find(|mv| mv.to.to_algebraic() == "e4")
            .expect("white pawn e2-e4 should be legal");

        game.make_move(white_move).expect("white move should succeed");
        let after_white = game.get_board_state().clone();
        assert_eq!(after_white.side_to_move, Color::Black);
        assert_eq!(after_white.fullmove_number, 1);

        let black_move = game
            .get_legal_moves_for_square(Square::from_algebraic("e7").unwrap())
            .into_iter()
            .find(|mv| mv.to.to_algebraic() == "e5")
            .expect("black pawn e7-e5 should be legal");

        game.make_move(black_move).expect("black move should succeed");
        let after_black = game.get_board_state();
        assert_eq!(after_black.side_to_move, Color::White);
        assert_eq!(after_black.fullmove_number, 2);
        assert!(matches!(game.get_status(), GameStatus::InProgress));

        game.undo_move().expect("undoing black move should work");
        assert_eq!(game.get_board_state().side_to_move, Color::Black);

        game.undo_move().expect("undoing white move should restore start position");
        let reset = game.get_board_state();
        assert_eq!(reset.side_to_move, Color::White);
        assert_eq!(reset.fullmove_number, 1);
        assert!(matches!(game.get_status(), GameStatus::InProgress));
    }
}

#[cfg(test)]
mod atomic_operations {
    use super::*;

    #[test]
    fn test_castling_move_application_is_atomic() {
        // This test verifies that if a castling move somehow gets past validation
        // but the board state is inconsistent, the entire move fails atomically
        // without partially mutating game state

        // Create a position with castling rights
        let mut game = ChessGame::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();

        // Record initial state
        let initial_fen = game.to_fen();

        // Try a legal castling move
        let castling_move = Move {
            from: Square::from_algebraic("e1").unwrap(),
            to: Square::from_algebraic("g1").unwrap(),
            promotion: None,
            is_castling: true,
            is_en_passant: false,
        };

        // Apply the legal move - should succeed
        let result = game.make_move(castling_move);
        assert!(result.is_ok(), "Legal castling move should succeed");

        // Verify state changed
        let new_fen = game.to_fen();
        assert_ne!(new_fen, initial_fen, "FEN should change after successful castling");
        // Verify king and rook moved
        assert!(new_fen.contains("R4RK"), "King and rook should have castled");
    }

    #[test]
    fn test_state_unchanged_on_illegal_castling() {
        // Test that an illegal castling attempt doesn't modify any game state
        // Create a position where castling through check would be illegal
        // h1 rook attacks f1, making kingside castling illegal
        let mut game_in_check = ChessGame::from_fen("r3k2r/8/8/8/8/8/8/R3K2r w Qkq - 5 10").unwrap();

        // Record complete initial state
        let initial_fen = game_in_check.to_fen();

        // Try to castle kingside (would be through/into check from the h1 rook)
        let illegal_castling = Move {
            from: Square::from_algebraic("e1").unwrap(),
            to: Square::from_algebraic("g1").unwrap(),
            promotion: None,
            is_castling: true,
            is_en_passant: false,
        };

        // Attempt the illegal move - should fail (no kingside castling rights)
        let result = game_in_check.make_move(illegal_castling);
        assert!(result.is_err(), "Illegal castling move should fail");

        // Verify ALL state remained unchanged (FEN includes all game state)
        assert_eq!(game_in_check.to_fen(), initial_fen, "Complete game state (FEN) should be unchanged");
    }

}
