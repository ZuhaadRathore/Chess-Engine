use crate::chess_engine::{Color, Piece, Position};
use crate::chess_engine::analysis::piece_value;

/// Chess position evaluator using static evaluation techniques
pub struct Evaluator;

impl Evaluator {
    /// Evaluate a chess position from White's perspective
    /// Returns score in centipawns (100 = 1 pawn advantage for White)
    /// Positive = White is better, Negative = Black is better
    pub fn evaluate(position: &Position) -> i32 {
        let mut score = 0;

        // Material balance (most important factor)
        score += Self::material_balance(position);

        // Piece-square tables (positional value)
        score += Self::piece_square_value(position);

        // Mobility (number of legal moves available)
        score += Self::mobility_bonus(position);

        score
    }

    /// Calculate material balance in centipawns
    fn material_balance(position: &Position) -> i32 {
        use crate::chess_engine::types::Square;

        let mut white_material = 0;
        let mut black_material = 0;

        for square_idx in 0..64 {
            if let Some(square) = Square::new(square_idx) {
                if let Some((piece, color)) = position.board.get(square) {
                    let value = piece_value(piece);
                    match color {
                        Color::White => white_material += value,
                        Color::Black => black_material += value,
                    }
                }
            }
        }

        white_material - black_material
    }

    /// Evaluate piece positioning using piece-square tables
    fn piece_square_value(position: &Position) -> i32 {
        use crate::chess_engine::types::Square;

        let mut score = 0;

        for square_idx in 0..64 {
            if let Some(square) = Square::new(square_idx) {
                if let Some((piece, color)) = position.board.get(square) {
                    let value = Self::get_piece_square_value(piece, color, square_idx);
                    score += value;
                }
            }
        }

        score
    }

    /// Get positional value for a piece on a specific square
    fn get_piece_square_value(piece: Piece, color: Color, square_idx: u8) -> i32 {
        let rank = (square_idx / 8) as usize;
        let file = (square_idx % 8) as usize;

        // Flip rank for black pieces (they play from top)
        let table_rank = match color {
            Color::White => rank,
            Color::Black => 7 - rank,
        };

        let bonus = match piece {
            Piece::Pawn => PAWN_TABLE[table_rank][file],
            Piece::Knight => KNIGHT_TABLE[table_rank][file],
            Piece::Bishop => BISHOP_TABLE[table_rank][file],
            Piece::Rook => ROOK_TABLE[table_rank][file],
            Piece::Queen => QUEEN_TABLE[table_rank][file],
            Piece::King => {
                // Use different tables for middlegame vs endgame
                // For now, use middlegame table (endgame logic can be added later)
                KING_MIDDLEGAME_TABLE[table_rank][file]
            }
        };

        match color {
            Color::White => bonus,
            Color::Black => -bonus,
        }
    }

    /// Calculate mobility bonus (simplified - just counts legal moves)
    fn mobility_bonus(position: &Position) -> i32 {
        use crate::chess_engine::validation::generate_legal_moves;

        let moves = generate_legal_moves(position);
        let mobility = moves.len() as i32;

        // Small bonus for having more moves available (capped to avoid overvaluing)
        let bonus = (mobility - 20).clamp(-20, 20);

        match position.side_to_move {
            Color::White => bonus,
            Color::Black => -bonus,
        }
    }
}

// Piece-Square Tables
// Values are in centipawns, represent positional bonuses for each square
// Tables are from White's perspective (rank 0 = White's back rank)

/// Pawn piece-square table - encourages central pawns and advancing
const PAWN_TABLE: [[i32; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0],   // Rank 1 (back rank - no pawns here normally)
    [50, 50, 50, 50, 50, 50, 50, 50],  // Rank 2
    [10, 10, 20, 30, 30, 20, 10, 10],  // Rank 3
    [5,  5, 10, 25, 25, 10,  5,  5],   // Rank 4
    [0,  0,  0, 20, 20,  0,  0,  0],   // Rank 5
    [5, -5,-10,  0,  0,-10, -5,  5],   // Rank 6
    [5, 10, 10,-20,-20, 10, 10,  5],   // Rank 7
    [0,  0,  0,  0,  0,  0,  0,  0],   // Rank 8 (promotion rank)
];

/// Knight piece-square table - encourages central knights, discourages edge knights
const KNIGHT_TABLE: [[i32; 8]; 8] = [
    [-50,-40,-30,-30,-30,-30,-40,-50], // Rank 1
    [-40,-20,  0,  0,  0,  0,-20,-40], // Rank 2
    [-30,  0, 10, 15, 15, 10,  0,-30], // Rank 3
    [-30,  5, 15, 20, 20, 15,  5,-30], // Rank 4
    [-30,  0, 15, 20, 20, 15,  0,-30], // Rank 5
    [-30,  5, 10, 15, 15, 10,  5,-30], // Rank 6
    [-40,-20,  0,  5,  5,  0,-20,-40], // Rank 7
    [-50,-40,-30,-30,-30,-30,-40,-50], // Rank 8
];

/// Bishop piece-square table - encourages central bishops, long diagonals
const BISHOP_TABLE: [[i32; 8]; 8] = [
    [-20,-10,-10,-10,-10,-10,-10,-20], // Rank 1
    [-10,  0,  0,  0,  0,  0,  0,-10], // Rank 2
    [-10,  0,  5, 10, 10,  5,  0,-10], // Rank 3
    [-10,  5,  5, 10, 10,  5,  5,-10], // Rank 4
    [-10,  0, 10, 10, 10, 10,  0,-10], // Rank 5
    [-10, 10, 10, 10, 10, 10, 10,-10], // Rank 6
    [-10,  5,  0,  0,  0,  0,  5,-10], // Rank 7
    [-20,-10,-10,-10,-10,-10,-10,-20], // Rank 8
];

/// Rook piece-square table - encourages rooks on 7th rank and open files
const ROOK_TABLE: [[i32; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0],   // Rank 1
    [5, 10, 10, 10, 10, 10, 10,  5],   // Rank 2
    [-5,  0,  0,  0,  0,  0,  0, -5],  // Rank 3
    [-5,  0,  0,  0,  0,  0,  0, -5],  // Rank 4
    [-5,  0,  0,  0,  0,  0,  0, -5],  // Rank 5
    [-5,  0,  0,  0,  0,  0,  0, -5],  // Rank 6
    [-5,  0,  0,  0,  0,  0,  0, -5],  // Rank 7
    [0,  0,  0,  5,  5,  0,  0,  0],   // Rank 8
];

/// Queen piece-square table - encourages central queen activity
const QUEEN_TABLE: [[i32; 8]; 8] = [
    [-20,-10,-10, -5, -5,-10,-10,-20], // Rank 1
    [-10,  0,  0,  0,  0,  0,  0,-10], // Rank 2
    [-10,  0,  5,  5,  5,  5,  0,-10], // Rank 3
    [ -5,  0,  5,  5,  5,  5,  0, -5], // Rank 4
    [  0,  0,  5,  5,  5,  5,  0, -5], // Rank 5
    [-10,  5,  5,  5,  5,  5,  0,-10], // Rank 6
    [-10,  0,  5,  0,  0,  0,  0,-10], // Rank 7
    [-20,-10,-10, -5, -5,-10,-10,-20], // Rank 8
];

/// King piece-square table for middlegame - encourages castling and staying safe
const KING_MIDDLEGAME_TABLE: [[i32; 8]; 8] = [
    [-30,-40,-40,-50,-50,-40,-40,-30], // Rank 1
    [-30,-40,-40,-50,-50,-40,-40,-30], // Rank 2
    [-30,-40,-40,-50,-50,-40,-40,-30], // Rank 3
    [-30,-40,-40,-50,-50,-40,-40,-30], // Rank 4
    [-20,-30,-30,-40,-40,-30,-30,-20], // Rank 5
    [-10,-20,-20,-20,-20,-20,-20,-10], // Rank 6
    [ 20, 20,  0,  0,  0,  0, 20, 20], // Rank 7
    [ 20, 30, 10,  0,  0, 10, 30, 20], // Rank 8 - encourages castling
];

/// King piece-square table for endgame - encourages active king
#[allow(dead_code)]
const KING_ENDGAME_TABLE: [[i32; 8]; 8] = [
    [-50,-40,-30,-20,-20,-30,-40,-50], // Rank 1
    [-30,-20,-10,  0,  0,-10,-20,-30], // Rank 2
    [-30,-10, 20, 30, 30, 20,-10,-30], // Rank 3
    [-30,-10, 30, 40, 40, 30,-10,-30], // Rank 4
    [-30,-10, 30, 40, 40, 30,-10,-30], // Rank 5
    [-30,-10, 20, 30, 30, 20,-10,-30], // Rank 6
    [-30,-30,  0,  0,  0,  0,-30,-30], // Rank 7
    [-50,-30,-30,-30,-30,-30,-30,-50], // Rank 8
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess_engine::{Position, ChessGame};

    #[test]
    fn test_starting_position_is_balanced() {
        let position = Position::new();
        let score = Evaluator::evaluate(&position);

        // Starting position should be approximately equal (within small margin for mobility)
        assert!(score.abs() < 50, "Starting position score: {}", score);
    }

    #[test]
    fn test_material_advantage() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBN1 w Qkq - 0 1"; // White missing h1 rook
        let game = ChessGame::from_fen(fen).unwrap();
        let position = game.get_board_state();
        let score = Evaluator::evaluate(position);

        // Black should be significantly ahead (missing rook = -500)
        assert!(score < -400, "Score with material imbalance: {}", score);
    }

    #[test]
    fn test_piece_square_values() {
        // Knight on edge vs center
        let edge_value = Evaluator::get_piece_square_value(Piece::Knight, Color::White, 0); // a1
        let center_value = Evaluator::get_piece_square_value(Piece::Knight, Color::White, 27); // d4

        assert!(center_value > edge_value, "Center knight should be better than edge knight");
    }
}
