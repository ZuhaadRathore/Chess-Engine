/**
 * TypeScript type definitions that mirror the Rust chess engine types.
 * These types ensure type safety when communicating with Tauri commands.
 *
 * Coordinate system: Board squares are indexed 0-63, where 0=a1 and 63=h8.
 * All types match the serialization format of the Rust backend structures.
 */

// Player color
export type Color = 'White' | 'Black';

// Chess piece types
export type Piece = 'Pawn' | 'Knight' | 'Bishop' | 'Rook' | 'Queen' | 'King';

// Valid promotion pieces (excludes Pawn and King)
export type PromotionPiece = Exclude<Piece, 'Pawn' | 'King'>;

/**
 * Represents a board square (0-63)
 * 0 = a1, 7 = h1, 56 = a8, 63 = h8
 */
export interface Square {
  index: number;
}

/**
 * Represents a single square on the board
 * Either contains a piece with its color as a tuple [Piece, Color] or is empty (null)
 * Note: Rust serializes Option<(Piece, Color)> as a tuple, not an object
 */
export type BoardSquare = [Piece, Color] | null;

/**
 * Castling rights for both players
 */
export interface CastlingRights {
  white_kingside: boolean;
  white_queenside: boolean;
  black_kingside: boolean;
  black_queenside: boolean;
}

/**
 * Complete game position state
 * This is returned by get_board_state and contains all information
 * needed to fully represent the current position
 */
export interface Position {
  /** Array of 64 squares representing the board state */
  board: BoardSquare[];
  /** Current player's turn */
  side_to_move: Color;
  /** Available castling options */
  castling_rights: CastlingRights;
  /** En passant capture square if available */
  en_passant_target: Square | null;
  /** Moves since last pawn move or capture (for fifty-move rule) */
  halfmove_clock: number;
  /** Current move number (increments after Black's move) */
  fullmove_number: number;
  /**
   * Zobrist hashes for position repetition detection.
   * WARNING: These are u64 values from Rust that may lose precision when serialized to JSON.
   * This field is informational only and should NOT be used for numeric operations in JavaScript.
   * Repetition detection is handled by the Rust backend.
   */
  position_history: number[];
}

/**
 * Represents a chess move
 */
export interface Move {
  /** Origin square */
  from: Square;
  /** Destination square */
  to: Square;
  /** Promotion piece if this is a pawn promotion */
  promotion: Piece | null;
  /** Whether this move is a castling move */
  is_castling: boolean;
  /** Whether this move is an en passant capture */
  is_en_passant: boolean;
}

/**
 * Current game status
 * Uses discriminated union for type safety
 */
export type GameStatus =
  | { type: 'InProgress' }
  | { type: 'Check' }
  | { type: 'Checkmate'; winner: Color }
  | { type: 'Stalemate' }
  | { type: 'DrawByFiftyMoveRule' }
  | { type: 'DrawByInsufficientMaterial' }
  | { type: 'DrawByRepetition' };

/**
 * Type guard to check if the game status is checkmate
 */
export function isCheckmate(status: GameStatus): status is { type: 'Checkmate'; winner: Color } {
  return status.type === 'Checkmate';
}

/**
 * Helper function to check if the game has ended
 */
export function isGameOver(status: GameStatus): boolean {
  return status.type !== 'InProgress' && status.type !== 'Check';
}

/**
 * Category of chess move based on its characteristics
 */
export type MoveCategory =
  | { type: 'Quiet' }           // Normal move, no special characteristics
  | { type: 'Capture' }         // Captures an opponent's piece
  | { type: 'Check' }           // Puts opponent's king in check
  | { type: 'CheckCapture' }    // Both captures and gives check
  | { type: 'Castle' }          // Castling move
  | { type: 'Promotion' }       // Pawn promotion
  | { type: 'PromotionCapture' } // Promotion with capture
  | { type: 'EnPassant' };      // En passant capture

/**
 * Detailed analysis of a chess move
 */
export interface MoveAnalysis {
  /** The move being analyzed */
  move_data: Move;
  /** Whether this move captures a piece */
  is_capture: boolean;
  /** Whether this move gives check */
  is_check: boolean;
  /** The piece captured (if any) */
  captured_piece: Piece | null;
  /** Category classification of the move */
  category: MoveCategory;
  /** Change in material balance (in centipawns) */
  material_change: number;
}

/**
 * Helper function to get a human-readable description of a move category
 */
export function getMoveCategoryDescription(category: MoveCategory): string {
  switch (category.type) {
    case 'Quiet':
      return 'Normal move';
    case 'Capture':
      return 'Capture';
    case 'Check':
      return 'Check';
    case 'CheckCapture':
      return 'Check & Capture';
    case 'Castle':
      return 'Castling';
    case 'Promotion':
      return 'Promotion';
    case 'PromotionCapture':
      return 'Promotion & Capture';
    case 'EnPassant':
      return 'En Passant';
  }
}

/**
 * Helper function to format an evaluation score for display
 * @param score Evaluation in centipawns (100 = 1 pawn)
 * @returns Formatted string (e.g., "+1.5" for white advantage, "-0.3" for black advantage)
 */
export function formatEvaluation(score: number): string {
  const pawns = score / 100;
  const sign = pawns > 0 ? '+' : '';
  return `${sign}${pawns.toFixed(2)}`;
}
