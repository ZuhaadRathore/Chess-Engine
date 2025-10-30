/**
 * UI-specific type definitions for the chess application.
 * These types are used for UI state and display, separate from the core chess engine types.
 */

import type { Piece, Color } from './index';

/**
 * Represents a move in the UI for display in move history
 */
export interface UiMove {
  /** Starting square in algebraic notation (e.g., "e2") */
  from: string;
  /** Destination square in algebraic notation (e.g., "e4") */
  to: string;
  /** The piece that was moved */
  piece: Piece;
  /** The color of the player who made the move */
  color: Color;
  /** Display notation for the move (e.g., "e4", "Nf3", "exd5") */
  notation: string;
}
