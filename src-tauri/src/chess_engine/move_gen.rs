use crate::chess_engine::board::{Board, is_valid_square};
use crate::chess_engine::position::Position;
use crate::chess_engine::types::{Color, Piece, Square, Move};

pub fn generate_pseudo_legal_moves(position: &Position) -> Vec<Move> {
    let mut moves = Vec::new();
    let color = position.side_to_move;

    for (square, piece) in position.board.pieces_of_color(color) {
        match piece {
            Piece::Pawn => moves.extend(generate_pawn_moves(&position.board, square, color, position.en_passant_target)),
            Piece::Knight => moves.extend(generate_knight_moves(&position.board, square, color)),
            Piece::Bishop => moves.extend(generate_bishop_moves(&position.board, square, color)),
            Piece::Rook => moves.extend(generate_rook_moves(&position.board, square, color)),
            Piece::Queen => moves.extend(generate_queen_moves(&position.board, square, color)),
            Piece::King => moves.extend(generate_king_moves(&position.board, square, color)),
        }
    }

    // Add castling moves
    moves.extend(generate_castling_moves(position));

    moves
}

fn generate_pawn_moves(board: &Board, from: Square, color: Color, en_passant: Option<Square>) -> Vec<Move> {
    let mut moves = Vec::new();
    let direction: i8 = if color == Color::White { 1 } else { -1 };
    let start_rank = if color == Color::White { 1 } else { 6 };
    let promotion_rank = if color == Color::White { 7 } else { 0 };

    let from_rank = from.rank() as i8;
    let from_file = from.file() as i8;

    // Single push
    let one_ahead_rank = from_rank + direction;
    if is_valid_square(one_ahead_rank, from_file) {
        if let Some(one_ahead) = Square::from_rank_file(one_ahead_rank as u8, from_file as u8) {
            if board.is_empty(one_ahead) {
                if one_ahead_rank as u8 == promotion_rank {
                    // Promotions
                    for promotion_piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                        let mut mv = Move::new(from, one_ahead);
                        mv.promotion = Some(promotion_piece);
                        moves.push(mv);
                    }
                } else {
                    moves.push(Move::new(from, one_ahead));
                }

                // Double push from starting position
                if from_rank == start_rank {
                    let two_ahead_rank = from_rank + (2 * direction);
                    if is_valid_square(two_ahead_rank, from_file) {
                        if let Some(two_ahead) = Square::from_rank_file(two_ahead_rank as u8, from_file as u8) {
                            if board.is_empty(two_ahead) {
                                moves.push(Move::new(from, two_ahead));
                            }
                        }
                    }
                }
            }
        }
    }

    // Captures
    for file_offset in [-1, 1] {
        let capture_rank = from_rank + direction;
        let capture_file = from_file + file_offset;

        if is_valid_square(capture_rank, capture_file) {
            if let Some(capture_square) = Square::from_rank_file(capture_rank as u8, capture_file as u8) {
                let can_capture = if let Some((_, piece_color)) = board.get(capture_square) {
                    piece_color != color
                } else {
                    false
                };

                if can_capture {
                    if capture_rank as u8 == promotion_rank {
                        // Promotion captures
                        for promotion_piece in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                            let mut mv = Move::new(from, capture_square);
                            mv.promotion = Some(promotion_piece);
                            moves.push(mv);
                        }
                    } else {
                        moves.push(Move::new(from, capture_square));
                    }
                }

                // En passant
                if let Some(ep_target) = en_passant {
                    if capture_square == ep_target {
                        let mut mv = Move::new(from, capture_square);
                        mv.is_en_passant = true;
                        moves.push(mv);
                    }
                }
            }
        }
    }

    moves
}

fn generate_knight_moves(board: &Board, from: Square, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();
    const KNIGHT_OFFSETS: [(i8, i8); 8] = [
        (-2, -1), (-2, 1), (-1, -2), (-1, 2),
        (1, -2), (1, 2), (2, -1), (2, 1),
    ];

    let from_rank = from.rank() as i8;
    let from_file = from.file() as i8;

    for (rank_offset, file_offset) in KNIGHT_OFFSETS {
        let to_rank = from_rank + rank_offset;
        let to_file = from_file + file_offset;

        if is_valid_square(to_rank, to_file) {
            if let Some(to_square) = Square::from_rank_file(to_rank as u8, to_file as u8) {
                let can_move = if let Some((_, piece_color)) = board.get(to_square) {
                    piece_color != color
                } else {
                    true
                };

                if can_move {
                    moves.push(Move::new(from, to_square));
                }
            }
        }
    }

    moves
}

fn generate_bishop_moves(board: &Board, from: Square, color: Color) -> Vec<Move> {
    const BISHOP_DIRECTIONS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    generate_sliding_moves(board, from, color, &BISHOP_DIRECTIONS)
}

fn generate_rook_moves(board: &Board, from: Square, color: Color) -> Vec<Move> {
    const ROOK_DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    generate_sliding_moves(board, from, color, &ROOK_DIRECTIONS)
}

fn generate_queen_moves(board: &Board, from: Square, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();
    moves.extend(generate_bishop_moves(board, from, color));
    moves.extend(generate_rook_moves(board, from, color));
    moves
}

fn generate_king_moves(board: &Board, from: Square, color: Color) -> Vec<Move> {
    let mut moves = Vec::new();
    const KING_OFFSETS: [(i8, i8); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let from_rank = from.rank() as i8;
    let from_file = from.file() as i8;

    for (rank_offset, file_offset) in KING_OFFSETS {
        let to_rank = from_rank + rank_offset;
        let to_file = from_file + file_offset;

        if is_valid_square(to_rank, to_file) {
            if let Some(to_square) = Square::from_rank_file(to_rank as u8, to_file as u8) {
                let can_move = if let Some((_, piece_color)) = board.get(to_square) {
                    piece_color != color
                } else {
                    true
                };

                if can_move {
                    moves.push(Move::new(from, to_square));
                }
            }
        }
    }

    moves
}

fn generate_castling_moves(position: &Position) -> Vec<Move> {
    let mut moves = Vec::new();
    let color = position.side_to_move;
    let rank = if color == Color::White { 0 } else { 7 };

    // Kingside castling
    if position.castling_rights.can_castle(color, true) {
        let king_square = Square::from_rank_file(rank, 4).unwrap();
        let rook_square = Square::from_rank_file(rank, 7).unwrap();
        let f_square = Square::from_rank_file(rank, 5).unwrap();
        let g_square = Square::from_rank_file(rank, 6).unwrap();

        // Verify king is present on its starting square
        let king_present = matches!(position.board.get(king_square), Some((Piece::King, c)) if c == color);

        // Verify rook is present on the corner square
        let rook_present = matches!(position.board.get(rook_square), Some((Piece::Rook, c)) if c == color);

        if king_present && rook_present && position.board.is_empty(f_square) && position.board.is_empty(g_square) {
            let mut mv = Move::new(king_square, g_square);
            mv.is_castling = true;
            moves.push(mv);
        }
    }

    // Queenside castling
    if position.castling_rights.can_castle(color, false) {
        let king_square = Square::from_rank_file(rank, 4).unwrap();
        let rook_square = Square::from_rank_file(rank, 0).unwrap();
        let b_square = Square::from_rank_file(rank, 1).unwrap();
        let c_square = Square::from_rank_file(rank, 2).unwrap();
        let d_square = Square::from_rank_file(rank, 3).unwrap();

        // Verify king is present on its starting square
        let king_present = matches!(position.board.get(king_square), Some((Piece::King, c)) if c == color);

        // Verify rook is present on the corner square
        let rook_present = matches!(position.board.get(rook_square), Some((Piece::Rook, c)) if c == color);

        if king_present && rook_present &&
           position.board.is_empty(b_square) &&
           position.board.is_empty(c_square) &&
           position.board.is_empty(d_square) {
            let mut mv = Move::new(king_square, c_square);
            mv.is_castling = true;
            moves.push(mv);
        }
    }

    moves
}

fn generate_sliding_moves(
    board: &Board,
    from: Square,
    color: Color,
    directions: &[(i8, i8)],
) -> Vec<Move> {
    let mut moves = Vec::new();
    let from_rank = from.rank() as i8;
    let from_file = from.file() as i8;

    for (rank_dir, file_dir) in directions {
        let mut rank = from_rank;
        let mut file = from_file;

        loop {
            rank += rank_dir;
            file += file_dir;

            if !is_valid_square(rank, file) {
                break;
            }

            if let Some(to_square) = Square::from_rank_file(rank as u8, file as u8) {
                if let Some((_, piece_color)) = board.get(to_square) {
                    if piece_color != color {
                        moves.push(Move::new(from, to_square));
                    }
                    break;
                } else {
                    moves.push(Move::new(from, to_square));
                }
            }
        }
    }

    moves
}
