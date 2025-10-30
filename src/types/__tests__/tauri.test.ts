import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';
import type { Position } from '@/types';
import {
  getBoardState,
  getFen,
  getGameStatus,
  getLegalMoves,
  getLegalMovesForSquare,
  loadFen,
  makeMove,
  newGame,
  undoMove,
} from '@/types/tauri';

const mockedInvoke = vi.mocked(invoke);

function samplePosition(): Position {
  return {
    board: Array(64).fill(null),
    side_to_move: 'White',
    castling_rights: {
      white_kingside: true,
      white_queenside: true,
      black_kingside: true,
      black_queenside: true,
    },
    en_passant_target: null,
    halfmove_clock: 0,
    fullmove_number: 1,
    position_history: [],
  };
}

beforeEach(() => {
  mockedInvoke.mockReset();
});

describe('tauri command wrappers', () => {
  it('creates a new game', async () => {
    mockedInvoke.mockResolvedValueOnce(undefined);
    await newGame();
    expect(mockedInvoke).toHaveBeenCalledWith('new_game');
  });

  it('fetches the current board state', async () => {
    mockedInvoke.mockResolvedValueOnce(samplePosition());
    const position = await getBoardState();
    expect(position.board).toHaveLength(64);
    expect(mockedInvoke).toHaveBeenCalledWith('get_board_state');
  });

  it('fetches all legal moves', async () => {
    mockedInvoke.mockResolvedValueOnce([]);
    const moves = await getLegalMoves();
    expect(moves).toEqual([]);
    expect(mockedInvoke).toHaveBeenCalledWith('get_legal_moves');
  });

  it('fetches legal moves for a square', async () => {
    mockedInvoke.mockResolvedValueOnce([]);
    await getLegalMovesForSquare('e2');
    expect(mockedInvoke).toHaveBeenCalledWith('get_legal_moves_for_square', { square: 'e2' });
  });

  it('makes a move with optional promotion', async () => {
    mockedInvoke.mockResolvedValueOnce({ type: 'InProgress' });
    await makeMove('e2', 'e4');
    expect(mockedInvoke).toHaveBeenLastCalledWith('make_move', {
      from: 'e2',
      to: 'e4',
      promotion: undefined,
    });

    mockedInvoke.mockResolvedValueOnce({ type: 'InProgress' });
    await makeMove('e7', 'e8', 'Queen');
    expect(mockedInvoke).toHaveBeenLastCalledWith('make_move', {
      from: 'e7',
      to: 'e8',
      promotion: 'Queen',
    });
  });

  it('undoes the last move', async () => {
    mockedInvoke.mockResolvedValueOnce({ type: 'InProgress' });
    await undoMove();
    expect(mockedInvoke).toHaveBeenCalledWith('undo_move');
  });

  it('reads the current status and position encoding', async () => {
    mockedInvoke.mockResolvedValueOnce({ type: 'InProgress' });
    await getGameStatus();
    expect(mockedInvoke).toHaveBeenLastCalledWith('get_game_status');

    mockedInvoke.mockResolvedValueOnce('startpos');
    await getFen();
    expect(mockedInvoke).toHaveBeenLastCalledWith('get_fen');
  });

  it('loads a FEN string', async () => {
    const position = samplePosition();
    mockedInvoke.mockResolvedValueOnce(position);
    const result = await loadFen('startpos');
    expect(result).toEqual(position);
    expect(mockedInvoke).toHaveBeenCalledWith('load_fen', { fen: 'startpos' });
  });
});
