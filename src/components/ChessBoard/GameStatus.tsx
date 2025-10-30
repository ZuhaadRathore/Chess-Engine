import React from 'react';
import type { GameStatus as GameStatusType, Position } from '@/types/index';
import { isCheckmate } from '@/types/index';
import './GameStatus.css';

interface GameStatusProps {
  status: GameStatusType | null;
  position: Position | null;
}

const renderStatusMessage = (status: GameStatusType | null): string => {
  if (!status) return '';

  switch (status.type) {
    case 'InProgress':
      return '';
    case 'Check':
      return 'Check!';
    case 'Checkmate':
      if (isCheckmate(status)) {
        const winner = status.winner;
        return `Checkmate! ${winner} wins!`;
      }
      return 'Checkmate!';
    case 'Stalemate':
      return 'Stalemate - Draw';
    case 'DrawByFiftyMoveRule':
      return 'Draw by fifty-move rule';
    case 'DrawByInsufficientMaterial':
      return 'Draw by insufficient material';
    case 'DrawByRepetition':
      return 'Draw by threefold repetition';
    default:
      return '';
  }
};

const GameStatus: React.FC<GameStatusProps> = ({ status, position }) => {
  const statusMessage = renderStatusMessage(status);
  const statusType = status?.type.toLowerCase() || 'inprogress';

  return (
    <div className="game-status">
      <div className="turn-indicator" aria-live="polite" aria-atomic="true">
        {position && (
          <span className={`turn-${position.side_to_move.toLowerCase()}`}>
            {position.side_to_move} to move
          </span>
        )}
      </div>

      {statusMessage && (
        <div className={`status-message status-${statusType}`} aria-live="polite" aria-atomic="true">
          {statusMessage}
        </div>
      )}

      {position && (
        <div className="move-counter">
          Move {position.fullmove_number}
        </div>
      )}
    </div>
  );
};

export default GameStatus;
