import { useEffect, useState } from 'react';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import type { Color, GameStatus, Position } from '@/types';
import { algebraicToIndex } from '@/utils/chess';
import { ThemeProvider } from '@/contexts/ThemeContext';

vi.mock('@/components/ChessBoard', () => {
  const React = require('react') as typeof import('react');

  type Props = {
    onMove?: (from: string, to: string, promotion?: string) => void;
    onPositionChange?: (position: Position) => void;
    onStatusChange?: (status: GameStatus) => void;
    onGameEnd?: (status: GameStatus) => void;
    playerColor?: Color;
  };

  const e2 = algebraicToIndex('e2');
  const e4 = algebraicToIndex('e4');
  const e7 = algebraicToIndex('e7');

  const makePosition = (turn: Color, whiteOnE4: boolean): Position => {
    const board: Position['board'] = Array(64).fill(null);
    if (whiteOnE4) {
      board[e4] = ['Pawn', 'White'];
    } else {
      board[e2] = ['Pawn', 'White'];
    }
    board[e7] = ['Pawn', 'Black'];

    return {
      board,
      side_to_move: turn,
      castling_rights: {
        white_kingside: true,
        white_queenside: true,
        black_kingside: true,
        black_queenside: true,
      },
      en_passant_target: null,
      halfmove_clock: 0,
      fullmove_number: turn === 'White' ? 1 : 2,
      position_history: [],
    };
  };

  const MockChessBoard: React.FC<Props> = ({
    onMove,
    onPositionChange,
    onStatusChange,
    onGameEnd,
  }) => {
    const [turn, setTurn] = useState<Color>('White');
    const [whiteAdvanced, setWhiteAdvanced] = useState(false);

    useEffect(() => {
      onPositionChange?.(makePosition(turn, whiteAdvanced));
      onStatusChange?.({ type: 'InProgress' });
    }, [turn, whiteAdvanced, onPositionChange, onStatusChange]);

    const triggerMove = () => {
      if (turn === 'White') {
        onMove?.('e2', 'e4');
        setWhiteAdvanced(true);
        setTurn('Black');
      } else {
        onMove?.('e7', 'e5');
        setTurn('White');
      }
    };

    const triggerCheckmate = () => {
      const result: GameStatus = { type: 'Checkmate', winner: turn === 'White' ? 'Black' : 'White' };
      onStatusChange?.(result);
      onGameEnd?.(result);
    };

    return (
      <div data-testid="mock-chess-board">
        <span data-testid="board-orientation">{turn}</span>
        <button type="button" onClick={triggerMove}>
          make-move
        </button>
        <button type="button" onClick={triggerCheckmate}>
          finish-game
        </button>
      </div>
    );
  };

  return { default: MockChessBoard };
});

import LocalGamePage from '../LocalGamePage';

function renderLocalGame() {
  return render(
    <ThemeProvider>
      <LocalGamePage />
    </ThemeProvider>,
  );
}

describe('LocalGamePage end-to-end behaviour', () => {
  it('tracks move history and auto-flips turns', async () => {
    renderLocalGame();
    const user = userEvent.setup();

    expect(screen.getByText(/Status/i)).toBeInTheDocument();
    expect(screen.getByText('White to move')).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: /make-move/i }));

    expect(screen.getByText('Black to move')).toBeInTheDocument();
    expect(screen.getByText('e2-e4')).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: /Move History/i })).toBeInTheDocument();
  });

  it('allows manual control of board orientation', async () => {
    renderLocalGame();
    const user = userEvent.setup();

    const autoFlipButton = screen.getByRole('button', { name: /Auto Flip: On/i });
    await user.click(autoFlipButton);
    expect(screen.getByRole('button', { name: /Auto Flip: Off/i })).toBeInTheDocument();

    const flipButton = screen.getByRole('button', { name: /Flip Board/i });
    expect(flipButton).not.toBeDisabled();
    await user.click(flipButton);
  });

  it('shows game results and resets state on new game', async () => {
    renderLocalGame();
    const user = userEvent.setup();

    await user.click(screen.getByRole('button', { name: /make-move/i }));
    expect(screen.getByText('e2-e4')).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: /finish-game/i }));
    const [resultBanner] = screen.getAllByText(/wins by checkmate/i);
    expect(resultBanner).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: /New Game/i }));
    expect(screen.queryByText('e2-e4')).not.toBeInTheDocument();
    expect(screen.queryByText(/wins by checkmate/i)).not.toBeInTheDocument();
  });
});
