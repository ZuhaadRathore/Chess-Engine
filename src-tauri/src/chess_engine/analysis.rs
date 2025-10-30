use serde::{Deserialize, Serialize};
use crate::chess_engine::{Move, Piece, Position};

/// Category of chess move based on its characteristics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MoveCategory {
    Quiet,              // Normal move, no special characteristics
    Capture,            // Captures an opponent's piece
    Check,              // Puts opponent's king in check
    CheckCapture,       // Both captures and gives check
    Castle,             // Castling move (kingside or queenside)
    Promotion,          // Pawn promotion
    PromotionCapture,   // Promotion with capture
    EnPassant,          // En passant capture
}

/// Detailed analysis of a chess move
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveAnalysis {
    /// The move being analyzed
    pub move_data: Move,

    /// Whether this move captures a piece
    pub is_capture: bool,

    /// Whether this move gives check
    pub is_check: bool,

    /// The piece captured (if any)
    pub captured_piece: Option<Piece>,

    /// Category classification of the move
    pub category: MoveCategory,

    /// Change in material balance (in centipawns)
    pub material_change: i32,
}

impl MoveAnalysis {
    /// Analyze a move in the context of a position
    pub fn analyze(chess_move: &Move, position: &Position) -> Self {
        use crate::chess_engine::validation::{apply_move_for_validation, is_in_check};

        // Determine if this is a capture
        let captured_piece = if chess_move.is_en_passant {
            Some(Piece::Pawn)
        } else {
            position.board.get(chess_move.to).map(|(piece, _)| piece)
        };

        let is_capture = captured_piece.is_some();

        // Calculate material change
        let material_change = if let Some(piece) = captured_piece {
            piece_value(piece)
        } else {
            0
        };

        // Apply the move to check if it results in check
        let mut test_position = position.clone();
        apply_move_for_validation(&mut test_position, chess_move);

        // Check if opponent king is in check after this move
        let opponent_color = position.side_to_move.opposite();
        let is_check = is_in_check(&test_position, opponent_color);

        // Categorize the move
        let category = categorize_move(chess_move, is_capture, is_check);

        MoveAnalysis {
            move_data: chess_move.clone(),
            is_capture,
            is_check,
            captured_piece,
            category,
            material_change,
        }
    }
}

/// Categorize a move based on its properties
fn categorize_move(chess_move: &Move, is_capture: bool, is_check: bool) -> MoveCategory {
    // Handle special moves first
    if chess_move.is_castling {
        return MoveCategory::Castle;
    }

    if chess_move.is_en_passant {
        return MoveCategory::EnPassant;
    }

    if chess_move.promotion.is_some() {
        return if is_capture {
            MoveCategory::PromotionCapture
        } else {
            MoveCategory::Promotion
        };
    }

    // Handle normal moves
    match (is_capture, is_check) {
        (true, true) => MoveCategory::CheckCapture,
        (true, false) => MoveCategory::Capture,
        (false, true) => MoveCategory::Check,
        (false, false) => MoveCategory::Quiet,
    }
}

/// Get the material value of a piece in centipawns (100 = 1 pawn)
pub fn piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 0, // King has no material value in evaluation
    }
}

/// Analyze all legal moves for a position
pub fn analyze_all_moves(position: &Position) -> Vec<MoveAnalysis> {
    use crate::chess_engine::validation::generate_legal_moves;

    let legal_moves = generate_legal_moves(position);
    legal_moves.iter()
        .map(|m| MoveAnalysis::analyze(m, position))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess_engine::types::Square;

    #[test]
    fn test_piece_values() {
        assert_eq!(piece_value(Piece::Pawn), 100);
        assert_eq!(piece_value(Piece::Knight), 320);
        assert_eq!(piece_value(Piece::Bishop), 330);
        assert_eq!(piece_value(Piece::Rook), 500);
        assert_eq!(piece_value(Piece::Queen), 900);
    }

    #[test]
    fn test_quiet_move_categorization() {
        let chess_move = Move {
            from: Square::new(12).unwrap(), // e2
            to: Square::new(28).unwrap(),   // e4
            promotion: None,
            is_castling: false,
            is_en_passant: false,
        };

        let category = categorize_move(&chess_move, false, false);
        assert_eq!(category, MoveCategory::Quiet);
    }

    #[test]
    fn test_capture_categorization() {
        let chess_move = Move {
            from: Square::new(28).unwrap(),
            to: Square::new(35).unwrap(),
            promotion: None,
            is_castling: false,
            is_en_passant: false,
        };

        let category = categorize_move(&chess_move, true, false);
        assert_eq!(category, MoveCategory::Capture);
    }

    #[test]
    fn test_castling_categorization() {
        let chess_move = Move {
            from: Square::new(4).unwrap(),  // e1
            to: Square::new(6).unwrap(),    // g1
            promotion: None,
            is_castling: true,
            is_en_passant: false,
        };

        let category = categorize_move(&chess_move, false, false);
        assert_eq!(category, MoveCategory::Castle);
    }
}
