use crate::chess_engine::board::Board;
use crate::chess_engine::position::{Position, CastlingRights};
use crate::chess_engine::types::{Color, Piece, Square};
use crate::chess_engine::error::{ChessError, Result};

#[allow(dead_code)]
pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse_fen(fen: &str) -> Result<Position> {
    let parts: Vec<&str> = fen.split_whitespace().collect();

    if parts.len() != 6 {
        return Err(ChessError::InvalidFen {
            reason: format!("Expected 6 fields, got {}", parts.len()),
        });
    }

    let mut position = Position::empty();

    // Parse piece placement (field 1)
    parse_piece_placement(&mut position.board, parts[0])?;

    // Parse active color (field 2)
    position.side_to_move = parse_active_color(parts[1])?;

    // Parse castling rights (field 3)
    position.castling_rights = parse_castling_rights(parts[2])?;

    // Parse en passant target (field 4)
    position.en_passant_target = parse_en_passant(parts[3])?;

    // Parse halfmove clock (field 5)
    position.halfmove_clock = parts[4].parse().map_err(|_| ChessError::InvalidFen {
        reason: format!("Invalid halfmove clock: {}", parts[4]),
    })?;

    // Parse fullmove number (field 6)
    position.fullmove_number = parts[5].parse().map_err(|_| ChessError::InvalidFen {
        reason: format!("Invalid fullmove number: {}", parts[5]),
    })?;

    // Validate the position
    validate_position(&position)?;

    // Initialize position history
    let hash = position.compute_zobrist_hash();
    position.position_history.push(hash);

    Ok(position)
}

fn parse_piece_placement(board: &mut Board, placement: &str) -> Result<()> {
    let ranks: Vec<&str> = placement.split('/').collect();

    if ranks.len() != 8 {
        return Err(ChessError::InvalidFen {
            reason: format!("Expected 8 ranks, got {}", ranks.len()),
        });
    }

    for (rank_index, rank_str) in ranks.iter().enumerate() {
        let rank = 7 - rank_index; // FEN starts from rank 8
        let mut file = 0;

        for c in rank_str.chars() {
            if file >= 8 {
                return Err(ChessError::InvalidFen {
                    reason: format!("Too many squares in rank {}", rank + 1),
                });
            }

            if c.is_ascii_digit() {
                // Only digits 1-8 are valid in FEN notation
                let empty_count = match c {
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    _ => {
                        return Err(ChessError::InvalidFen {
                            reason: format!("Invalid digit '{}' in FEN (must be 1-8)", c),
                        });
                    }
                };

                // Validate that adding empty squares doesn't overflow the rank
                if file + empty_count > 8 {
                    return Err(ChessError::InvalidFen {
                        reason: format!("Rank {} has too many squares (file {} + {} empty squares > 8)", rank + 1, file, empty_count),
                    });
                }

                file += empty_count;
            } else {
                let (piece, color) = fen_char_to_piece(c).ok_or_else(|| ChessError::InvalidFen {
                    reason: format!("Invalid piece character: {}", c),
                })?;

                let square = Square::from_rank_file(rank as u8, file).ok_or_else(|| {
                    ChessError::InvalidFen {
                        reason: format!("Invalid square: rank {}, file {}", rank, file),
                    }
                })?;

                board.set(square, Some((piece, color)));
                file += 1;
            }
        }

        if file != 8 {
            return Err(ChessError::InvalidFen {
                reason: format!("Rank {} has {} squares, expected 8", rank + 1, file),
            });
        }
    }

    Ok(())
}

fn parse_active_color(s: &str) -> Result<Color> {
    match s {
        "w" => Ok(Color::White),
        "b" => Ok(Color::Black),
        _ => Err(ChessError::InvalidFen {
            reason: format!("Invalid active color: {}", s),
        }),
    }
}

fn parse_castling_rights(s: &str) -> Result<CastlingRights> {
    if s == "-" {
        return Ok(CastlingRights::none());
    }

    let mut rights = CastlingRights::none();

    for c in s.chars() {
        match c {
            'K' => rights.white_kingside = true,
            'Q' => rights.white_queenside = true,
            'k' => rights.black_kingside = true,
            'q' => rights.black_queenside = true,
            _ => {
                return Err(ChessError::InvalidFen {
                    reason: format!("Invalid castling character: {}", c),
                })
            }
        }
    }

    Ok(rights)
}

fn parse_en_passant(s: &str) -> Result<Option<Square>> {
    if s == "-" {
        Ok(None)
    } else {
        Square::from_algebraic(s).map(Some)
    }
}

fn validate_position(position: &Position) -> Result<()> {
    // Count kings to ensure exactly one per side
    let mut white_king_count = 0;
    let mut black_king_count = 0;

    for i in 0..64 {
        if let Some(square) = Square::new(i) {
            if let Some((Piece::King, color)) = position.board.get(square) {
                match color {
                    Color::White => white_king_count += 1,
                    Color::Black => black_king_count += 1,
                }
            }
        }
    }

    if white_king_count == 0 {
        return Err(ChessError::InvalidFen {
            reason: "White king not found".to_string(),
        });
    }

    if white_king_count > 1 {
        return Err(ChessError::InvalidFen {
            reason: format!("Multiple white kings found: {}", white_king_count),
        });
    }

    if black_king_count == 0 {
        return Err(ChessError::InvalidFen {
            reason: "Black king not found".to_string(),
        });
    }

    if black_king_count > 1 {
        return Err(ChessError::InvalidFen {
            reason: format!("Multiple black kings found: {}", black_king_count),
        });
    }

    // Check no pawns on ranks 1 or 8
    for file in 0..8 {
        if let Some(square) = Square::from_rank_file(0, file) {
            if let Some((Piece::Pawn, _)) = position.board.get(square) {
                return Err(ChessError::InvalidFen {
                    reason: "Pawn on rank 1".to_string(),
                });
            }
        }

        if let Some(square) = Square::from_rank_file(7, file) {
            if let Some((Piece::Pawn, _)) = position.board.get(square) {
                return Err(ChessError::InvalidFen {
                    reason: "Pawn on rank 8".to_string(),
                });
            }
        }
    }

    // Validate en passant square
    if let Some(ep_square) = position.en_passant_target {
        let expected_rank = if position.side_to_move == Color::White { 5 } else { 2 };
        if ep_square.rank() != expected_rank {
            return Err(ChessError::InvalidFen {
                reason: format!("Invalid en passant square: {}", ep_square.to_algebraic()),
            });
        }
    }

    // Validate castling rights against board pieces
    if position.castling_rights.white_kingside {
        let king_square = Square::from_rank_file(0, 4).unwrap();
        let rook_square = Square::from_rank_file(0, 7).unwrap();

        if !matches!(position.board.get(king_square), Some((Piece::King, Color::White))) {
            return Err(ChessError::InvalidFen {
                reason: "White kingside castling right requires white king on e1".to_string(),
            });
        }
        if !matches!(position.board.get(rook_square), Some((Piece::Rook, Color::White))) {
            return Err(ChessError::InvalidFen {
                reason: "White kingside castling right requires white rook on h1".to_string(),
            });
        }
    }

    if position.castling_rights.white_queenside {
        let king_square = Square::from_rank_file(0, 4).unwrap();
        let rook_square = Square::from_rank_file(0, 0).unwrap();

        if !matches!(position.board.get(king_square), Some((Piece::King, Color::White))) {
            return Err(ChessError::InvalidFen {
                reason: "White queenside castling right requires white king on e1".to_string(),
            });
        }
        if !matches!(position.board.get(rook_square), Some((Piece::Rook, Color::White))) {
            return Err(ChessError::InvalidFen {
                reason: "White queenside castling right requires white rook on a1".to_string(),
            });
        }
    }

    if position.castling_rights.black_kingside {
        let king_square = Square::from_rank_file(7, 4).unwrap();
        let rook_square = Square::from_rank_file(7, 7).unwrap();

        if !matches!(position.board.get(king_square), Some((Piece::King, Color::Black))) {
            return Err(ChessError::InvalidFen {
                reason: "Black kingside castling right requires black king on e8".to_string(),
            });
        }
        if !matches!(position.board.get(rook_square), Some((Piece::Rook, Color::Black))) {
            return Err(ChessError::InvalidFen {
                reason: "Black kingside castling right requires black rook on h8".to_string(),
            });
        }
    }

    if position.castling_rights.black_queenside {
        let king_square = Square::from_rank_file(7, 4).unwrap();
        let rook_square = Square::from_rank_file(7, 0).unwrap();

        if !matches!(position.board.get(king_square), Some((Piece::King, Color::Black))) {
            return Err(ChessError::InvalidFen {
                reason: "Black queenside castling right requires black king on e8".to_string(),
            });
        }
        if !matches!(position.board.get(rook_square), Some((Piece::Rook, Color::Black))) {
            return Err(ChessError::InvalidFen {
                reason: "Black queenside castling right requires black rook on a8".to_string(),
            });
        }
    }

    Ok(())
}

pub fn position_to_fen(position: &Position) -> String {
    let mut fen = String::new();

    // Piece placement
    for rank in (0..8).rev() {
        let mut empty_count = 0;

        for file in 0..8 {
            let square = Square::from_rank_file(rank, file).unwrap();

            if let Some((piece, color)) = position.board.get(square) {
                if empty_count > 0 {
                    fen.push_str(&empty_count.to_string());
                    empty_count = 0;
                }
                fen.push(piece_to_fen_char(piece, color));
            } else {
                empty_count += 1;
            }
        }

        if empty_count > 0 {
            fen.push_str(&empty_count.to_string());
        }

        if rank > 0 {
            fen.push('/');
        }
    }

    // Active color
    fen.push(' ');
    fen.push(match position.side_to_move {
        Color::White => 'w',
        Color::Black => 'b',
    });

    // Castling rights
    fen.push(' ');
    let mut castling = String::new();
    if position.castling_rights.white_kingside {
        castling.push('K');
    }
    if position.castling_rights.white_queenside {
        castling.push('Q');
    }
    if position.castling_rights.black_kingside {
        castling.push('k');
    }
    if position.castling_rights.black_queenside {
        castling.push('q');
    }
    if castling.is_empty() {
        fen.push('-');
    } else {
        fen.push_str(&castling);
    }

    // En passant target
    fen.push(' ');
    if let Some(ep_square) = position.en_passant_target {
        fen.push_str(&ep_square.to_algebraic());
    } else {
        fen.push('-');
    }

    // Halfmove clock
    fen.push(' ');
    fen.push_str(&position.halfmove_clock.to_string());

    // Fullmove number
    fen.push(' ');
    fen.push_str(&position.fullmove_number.to_string());

    fen
}

fn piece_to_fen_char(piece: Piece, color: Color) -> char {
    let c = match piece {
        Piece::Pawn => 'p',
        Piece::Knight => 'n',
        Piece::Bishop => 'b',
        Piece::Rook => 'r',
        Piece::Queen => 'q',
        Piece::King => 'k',
    };

    if color == Color::White {
        c.to_ascii_uppercase()
    } else {
        c
    }
}

fn fen_char_to_piece(c: char) -> Option<(Piece, Color)> {
    let color = if c.is_ascii_uppercase() {
        Color::White
    } else {
        Color::Black
    };

    let piece = match c.to_ascii_lowercase() {
        'p' => Piece::Pawn,
        'n' => Piece::Knight,
        'b' => Piece::Bishop,
        'r' => Piece::Rook,
        'q' => Piece::Queen,
        'k' => Piece::King,
        _ => return None,
    };

    Some((piece, color))
}
