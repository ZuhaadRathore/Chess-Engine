use crate::chess_engine::position::Position;
use crate::chess_engine::validation::{generate_legal_moves, is_legal_move, is_in_check, is_checkmate, is_stalemate};
use crate::chess_engine::fen::{parse_fen, position_to_fen};
use crate::chess_engine::types::{Color, Piece, Square, Move, GameStatus};
use crate::chess_engine::error::{ChessError, Result};

#[derive(Debug, Clone)]
pub struct ChessGame {
    position: Position,
    move_history: Vec<Move>,
    position_snapshots: Vec<Position>,
    status: GameStatus,
}

impl ChessGame {
    pub fn new() -> Self {
        let position = Position::new();
        let status = Self::compute_game_status_static(&position);

        ChessGame {
            position,
            move_history: Vec::new(),
            position_snapshots: Vec::new(),
            status,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self> {
        let position = parse_fen(fen)?;
        let status = Self::compute_game_status_static(&position);

        Ok(ChessGame {
            position,
            move_history: Vec::new(),
            position_snapshots: Vec::new(),
            status,
        })
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        if !matches!(self.status, GameStatus::InProgress | GameStatus::Check) {
            return Vec::new();
        }
        generate_legal_moves(&self.position)
    }

    pub fn get_legal_moves_for_square(&self, square: Square) -> Vec<Move> {
        self.get_legal_moves()
            .into_iter()
            .filter(|mv| mv.from == square)
            .collect()
    }

    pub fn make_move(&mut self, mv: Move) -> Result<()> {
        // Check if game is already over
        if !matches!(self.status, GameStatus::InProgress | GameStatus::Check) {
            return Err(ChessError::GameOver {
                status: format!("{:?}", self.status),
            });
        }

        // Verify move is legal
        if !is_legal_move(&self.position, &mv) {
            return Err(ChessError::InvalidMove {
                reason: format!("Move {} is not legal", mv.to_uci()),
            });
        }

        // Save current position for undo
        self.position_snapshots.push(self.position.clone());

        // Apply the move (atomic operation for castling)
        // If this fails (e.g., due to invalid castling state), restore the snapshot
        if let Err(e) = self.apply_move_to_position(&mv) {
            // Restore state by removing the snapshot we just added
            self.position_snapshots.pop();
            return Err(e);
        }

        // Add move to history
        self.move_history.push(mv);

        // Update game status
        self.status = self.compute_game_status();

        Ok(())
    }

    pub fn undo_move(&mut self) -> Result<()> {
        if self.position_snapshots.is_empty() {
            return Err(ChessError::InvalidMove {
                reason: "No moves to undo".to_string(),
            });
        }

        // Restore previous position
        self.position = self.position_snapshots.pop().unwrap();

        // Remove last move from history
        self.move_history.pop();

        // Update game status
        self.status = self.compute_game_status();

        Ok(())
    }

    pub fn get_status(&self) -> GameStatus {
        self.status.clone()
    }

    pub fn to_fen(&self) -> String {
        position_to_fen(&self.position)
    }

    pub fn get_board_state(&self) -> &Position {
        &self.position
    }

    fn compute_game_status(&self) -> GameStatus {
        Self::compute_game_status_static(&self.position)
    }

    fn compute_game_status_static(position: &Position) -> GameStatus {
        // Check for checkmate
        if is_checkmate(position) {
            return GameStatus::Checkmate {
                winner: position.side_to_move.opposite(),
            };
        }

        // Check for stalemate
        if is_stalemate(position) {
            return GameStatus::Stalemate;
        }

        // Check for fifty-move rule
        if position.halfmove_clock >= 100 {
            return GameStatus::DrawByFiftyMoveRule;
        }

        // Check for insufficient material
        if position.has_insufficient_material() {
            return GameStatus::DrawByInsufficientMaterial;
        }

        // Check for threefold repetition
        if position.is_repetition() {
            return GameStatus::DrawByRepetition;
        }

        // Check for check
        if is_in_check(position, position.side_to_move) {
            return GameStatus::Check;
        }

        // Game is still in progress
        GameStatus::InProgress
    }

    fn apply_move_to_position(&mut self, mv: &Move) -> Result<()> {
        // Handle special moves (castling must be checked first for atomicity)
        if mv.is_castling {
            // For castling, check preconditions and move pieces atomically
            // If this fails, no state mutation occurs
            self.apply_castling(mv)?;
        } else if mv.is_en_passant {
            self.apply_en_passant(mv);
        } else {
            self.apply_normal_move(mv);
        }

        // Only update castling rights after successful move application
        self.position.update_castling_rights_after_move(mv);

        // Set en passant target for next move
        self.update_en_passant_target(mv);

        // Update halfmove clock
        self.update_halfmove_clock(mv);

        // Update fullmove number (increment after Black's move)
        if self.position.side_to_move == Color::Black {
            self.position.fullmove_number += 1;
        }

        // Switch side to move
        self.position.side_to_move = self.position.side_to_move.opposite();

        // Update position history for repetition detection
        let hash = self.position.compute_zobrist_hash();
        self.position.position_history.push(hash);

        Ok(())
    }

    fn apply_normal_move(&mut self, mv: &Move) {
        let piece = self.position.board.get(mv.from);

        // Move the piece
        self.position.board.set(mv.from, None);

        // Handle promotion
        if let Some(promotion_piece) = mv.promotion {
            if let Some((_, color)) = piece {
                self.position.board.set(mv.to, Some((promotion_piece, color)));
            }
        } else {
            self.position.board.set(mv.to, piece);
        }
    }

    fn apply_castling(&mut self, mv: &Move) -> Result<()> {
        let rank = mv.from.rank();

        // Precondition checks: verify king and rook presence before any state mutation
        let king = self.position.board.get(mv.from);
        let king_color = match king {
            Some((Piece::King, color)) => color,
            _ => {
                return Err(ChessError::InvalidMove {
                    reason: format!("King not found at castling origin square {}", mv.from.to_algebraic()),
                });
            }
        };

        // Determine rook squares based on castling type
        let (rook_from, rook_to) = if mv.to.file() > mv.from.file() {
            // Kingside castling
            (Square::from_rank_file(rank, 7).unwrap(), Square::from_rank_file(rank, 5).unwrap())
        } else {
            // Queenside castling
            (Square::from_rank_file(rank, 0).unwrap(), Square::from_rank_file(rank, 3).unwrap())
        };

        // Verify rook is present and correct color before proceeding
        let rook = self.position.board.get(rook_from);
        if !matches!(rook, Some((Piece::Rook, c)) if c == king_color) {
            return Err(ChessError::InvalidMove {
                reason: format!("Rook not found at expected position {} for castling", rook_from.to_algebraic()),
            });
        }

        // All preconditions satisfied, now apply the castling move
        // Move king
        self.position.board.set(mv.from, None);
        self.position.board.set(mv.to, king);

        // Move rook
        self.position.board.set(rook_from, None);
        self.position.board.set(rook_to, rook);

        Ok(())
    }

    fn apply_en_passant(&mut self, mv: &Move) {
        let pawn = self.position.board.get(mv.from);

        // Move pawn
        self.position.board.set(mv.from, None);
        self.position.board.set(mv.to, pawn);

        // Remove captured pawn
        let captured_pawn_rank = if self.position.side_to_move == Color::White {
            mv.to.rank() - 1
        } else {
            mv.to.rank() + 1
        };

        if let Some(captured_square) = Square::from_rank_file(captured_pawn_rank, mv.to.file()) {
            self.position.board.set(captured_square, None);
        }
    }

    fn update_en_passant_target(&mut self, mv: &Move) {
        // Check if a pawn moved two squares
        if let Some((Piece::Pawn, _)) = self.position.board.get(mv.to) {
            let from_rank = mv.from.rank();
            let to_rank = mv.to.rank();

            if from_rank.abs_diff(to_rank) == 2 {
                // Set en passant target to the square the pawn passed over
                let ep_rank = (from_rank + to_rank) / 2;
                self.position.en_passant_target = Square::from_rank_file(ep_rank, mv.from.file());
                return;
            }
        }

        // Clear en passant target
        self.position.en_passant_target = None;
    }

    fn update_halfmove_clock(&mut self, mv: &Move) {
        // Get the piece that's moving (it's already at the destination)
        let is_pawn_move = if let Some((piece, _)) = self.position.board.get(mv.to) {
            piece == Piece::Pawn
        } else {
            false
        };

        // Check if there was a capture (position snapshot has the piece at destination)
        let is_capture = if let Some(last_pos) = self.position_snapshots.last() {
            last_pos.board.get(mv.to).is_some()
        } else {
            false
        } || mv.is_en_passant;

        if is_pawn_move || is_capture {
            self.position.halfmove_clock = 0;
        } else {
            self.position.halfmove_clock += 1;
        }
    }
}

impl Default for ChessGame {
    fn default() -> Self {
        Self::new()
    }
}
