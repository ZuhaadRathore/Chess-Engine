/**
 * Tauri command helpers for interacting with the local chess engine.
 * These wrappers provide typed return values for the front-end.
 */

import { invoke } from '@tauri-apps/api/core';
import type { GameStatus, Move, Position, PromotionPiece, MoveAnalysis } from './index';

/** Reset the engine to the initial position. */
export async function newGame(): Promise<void> {
  await invoke('new_game');
}

/** Retrieve the full board state for the active game. */
export async function getBoardState(): Promise<Position> {
  return await invoke<Position>('get_board_state');
}

/** Get all legal moves in the current position. */
export async function getLegalMoves(): Promise<Move[]> {
  return await invoke<Move[]>('get_legal_moves');
}

/** Get legal moves available from a specific square (algebraic notation). */
export async function getLegalMovesForSquare(square: string): Promise<Move[]> {
  return await invoke<Move[]>('get_legal_moves_for_square', { square });
}

/** Make a move and return the resulting game status. */
export async function makeMove(
  from: string,
  to: string,
  promotion?: PromotionPiece,
): Promise<GameStatus> {
  return await invoke<GameStatus>('make_move', { from, to, promotion });
}

/** Undo the last move and return the updated game status. */
export async function undoMove(): Promise<GameStatus> {
  return await invoke<GameStatus>('undo_move');
}

/** Fetch the current game status (in progress, check, etc.). */
export async function getGameStatus(): Promise<GameStatus> {
  return await invoke<GameStatus>('get_game_status');
}

/** Load a custom position from FEN notation. */
export async function loadFen(fen: string): Promise<Position> {
  return await invoke<Position>('load_fen', { fen });
}

/** Export the current position as FEN. */
export async function getFen(): Promise<string> {
  return await invoke<string>('get_fen');
}

/** Analyze a specific move and return detailed information. */
export async function analyzeMove(
  from: string,
  to: string,
  promotion?: PromotionPiece,
): Promise<MoveAnalysis> {
  return await invoke<MoveAnalysis>('analyze_move', { from, to, promotion });
}

/** Analyze all legal moves in the current position. */
export async function analyzeAllLegalMoves(): Promise<MoveAnalysis[]> {
  return await invoke<MoveAnalysis[]>('analyze_all_legal_moves');
}

/** Evaluate the current position and return a score in centipawns.
 * Positive score = White advantage, Negative score = Black advantage
 */
export async function evaluatePosition(): Promise<number> {
  return await invoke<number>('evaluate_position');
}
