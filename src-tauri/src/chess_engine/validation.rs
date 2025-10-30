use crate::chess_engine::move_gen::generate_pseudo_legal_moves;
use crate::chess_engine::position::Position;
use crate::chess_engine::types::{Color, Piece, Square, Move};

pub fn generate_legal_moves(position: &Position) -> Vec<Move> {
    let pseudo_legal_moves = generate_pseudo_legal_moves(position);
    pseudo_legal_moves
        .into_iter()
        .filter(|mv| is_legal_move(position, mv))
        .collect()
}

pub fn is_legal_move(position: &Position, mv: &Move) -> bool {
    // Special validation for castling
    if mv.is_castling {
        let color = position.side_to_move;
        let kingside = mv.to.file() > mv.from.file();

        if kingside {
            return can_castle_kingside(position, color);
        } else {
            return can_castle_queenside(position, color);
        }
    }

    let mut test_position = position.clone();

    // Apply the move to test position
    apply_move_for_validation(&mut test_position, mv);

    // Check if our king is in check after the move
    let our_color = position.side_to_move;
    !is_in_check(&test_position, our_color)
}

pub(crate) fn apply_move_for_validation(position: &mut Position, mv: &Move) {
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
        let king_color = position.side_to_move;

        if mv.to.file() > mv.from.file() {
            // Kingside castling
            let rook_from = Square::from_rank_file(rank, 7).unwrap();
            let rook_to = Square::from_rank_file(rank, 5).unwrap();
            let rook = position.board.get(rook_from);

            // Verify rook is present and correct color
            debug_assert!(
                matches!(rook, Some((Piece::Rook, c)) if c == king_color),
                "Rook not found or wrong color at kingside castling position"
            );

            position.board.set(rook_from, None);
            position.board.set(rook_to, rook);
        } else {
            // Queenside castling
            let rook_from = Square::from_rank_file(rank, 0).unwrap();
            let rook_to = Square::from_rank_file(rank, 3).unwrap();
            let rook = position.board.get(rook_from);

            // Verify rook is present and correct color
            debug_assert!(
                matches!(rook, Some((Piece::Rook, c)) if c == king_color),
                "Rook not found or wrong color at queenside castling position"
            );

            position.board.set(rook_from, None);
            position.board.set(rook_to, rook);
        }
    }

    // Move the piece
    let piece = position.board.get(mv.from);
    position.board.set(mv.from, None);

    // Handle promotion
    if let Some(promotion_piece) = mv.promotion {
        if let Some((_, color)) = piece {
            position.board.set(mv.to, Some((promotion_piece, color)));
        }
    } else {
        position.board.set(mv.to, piece);
    }
}

pub fn is_in_check(position: &Position, color: Color) -> bool {
    if let Some(king_square) = position.board.find_king(color) {
        position.board.is_attacked_by(king_square, color.opposite())
    } else {
        false
    }
}

pub fn is_checkmate(position: &Position) -> bool {
    is_in_check(position, position.side_to_move) && generate_legal_moves(position).is_empty()
}

pub fn is_stalemate(position: &Position) -> bool {
    !is_in_check(position, position.side_to_move) && generate_legal_moves(position).is_empty()
}

pub fn can_castle_kingside(position: &Position, color: Color) -> bool {
    if !position.castling_rights.can_castle(color, true) {
        return false;
    }

    let rank = if color == Color::White { 0 } else { 7 };
    let king_square = Square::from_rank_file(rank, 4).unwrap();

    // Verify king is present on its starting square
    if !matches!(position.board.get(king_square), Some((Piece::King, c)) if c == color) {
        return false;
    }

    let rook_square = Square::from_rank_file(rank, 7).unwrap();
    let f_square = Square::from_rank_file(rank, 5).unwrap();
    let g_square = Square::from_rank_file(rank, 6).unwrap();

    // Check rook is present on home square
    if !matches!(position.board.get(rook_square), Some((Piece::Rook, c)) if c == color) {
        return false;
    }

    // Check squares are empty
    if !position.board.is_empty(f_square) || !position.board.is_empty(g_square) {
        return false;
    }

    // Check king is not in check
    if is_in_check(position, color) {
        return false;
    }

    // Check king doesn't move through check
    let opponent = color.opposite();
    if position.board.is_attacked_by(f_square, opponent) {
        return false;
    }

    // Check king doesn't end in check
    if position.board.is_attacked_by(g_square, opponent) {
        return false;
    }

    true
}

pub fn can_castle_queenside(position: &Position, color: Color) -> bool {
    if !position.castling_rights.can_castle(color, false) {
        return false;
    }

    let rank = if color == Color::White { 0 } else { 7 };
    let king_square = Square::from_rank_file(rank, 4).unwrap();

    // Verify king is present on its starting square
    if !matches!(position.board.get(king_square), Some((Piece::King, c)) if c == color) {
        return false;
    }

    let rook_square = Square::from_rank_file(rank, 0).unwrap();
    let b_square = Square::from_rank_file(rank, 1).unwrap();
    let c_square = Square::from_rank_file(rank, 2).unwrap();
    let d_square = Square::from_rank_file(rank, 3).unwrap();

    // Check rook is present on home square
    if !matches!(position.board.get(rook_square), Some((Piece::Rook, c)) if c == color) {
        return false;
    }

    // Check squares are empty
    if !position.board.is_empty(b_square) ||
       !position.board.is_empty(c_square) ||
       !position.board.is_empty(d_square) {
        return false;
    }

    // Check king is not in check
    if is_in_check(position, color) {
        return false;
    }

    // Check king doesn't move through check
    let opponent = color.opposite();
    if position.board.is_attacked_by(d_square, opponent) {
        return false;
    }

    // Check king doesn't end in check
    if position.board.is_attacked_by(c_square, opponent) {
        return false;
    }

    true
}

#[allow(dead_code)]
pub fn get_pinned_pieces(position: &Position, color: Color) -> Vec<Square> {
    let mut pinned = Vec::new();

    if let Some(king_square) = position.board.find_king(color) {
        let _opponent = color.opposite();

        // Check all sliding directions from the king
        const DIRECTIONS: [(i8, i8); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for (rank_dir, file_dir) in DIRECTIONS {
            let mut our_piece: Option<Square> = None;
            let mut rank = king_square.rank() as i8;
            let mut file = king_square.file() as i8;

            loop {
                rank += rank_dir;
                file += file_dir;

                if rank < 0 || rank >= 8 || file < 0 || file >= 8 {
                    break;
                }

                if let Some(square) = Square::from_rank_file(rank as u8, file as u8) {
                    if let Some((piece, piece_color)) = position.board.get(square) {
                        if piece_color == color {
                            if our_piece.is_some() {
                                // Second piece of our color, no pin possible
                                break;
                            }
                            our_piece = Some(square);
                        } else {
                            // Opponent piece
                            if let Some(pinned_square) = our_piece {
                                // Check if this opponent piece can pin along this direction
                                let is_diagonal = rank_dir != 0 && file_dir != 0;
                                let can_pin = match piece {
                                    Piece::Queen => true,
                                    Piece::Bishop => is_diagonal,
                                    Piece::Rook => !is_diagonal,
                                    _ => false,
                                };

                                if can_pin {
                                    pinned.push(pinned_square);
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    pinned
}
