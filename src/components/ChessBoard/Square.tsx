import React from 'react';
import type { BoardSquare, MoveAnalysis } from '@/types/index';
import { getSquareColor, indexToAlgebraic } from '@/utils/chess';
import Piece from './Piece';
import MoveIndicator from './MoveIndicator';
import './Square.css';

interface SquareProps {
  index: number;
  piece: BoardSquare;
  isSelected: boolean;
  isHighlighted: boolean;
  isLastMove: boolean;
  onClick: () => void;
  disabled?: boolean;
  moveAnalysis?: MoveAnalysis | null;
}

const Square: React.FC<SquareProps> = ({
  index,
  piece,
  isSelected,
  isHighlighted,
  isLastMove,
  onClick,
  disabled = false,
  moveAnalysis = null
}) => {
  const squareColor = getSquareColor(index);
  const algebraic = indexToAlgebraic(index);

  // Build class names
  const classNames = [
    'square',
    `square-${squareColor}`,
    isSelected && 'square-selected',
    isHighlighted && 'square-highlighted',
    isLastMove && 'square-last-move',
    disabled && 'square-disabled'
  ].filter(Boolean).join(' ');

  // Build aria label
  const pieceLabel = piece ? `${piece[1]} ${piece[0]}` : 'empty';
  const ariaLabel = `${algebraic}, ${pieceLabel}`;

  const handleClick = () => {
    if (!disabled) {
      onClick();
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (!disabled && (e.key === 'Enter' || e.key === ' ')) {
      e.preventDefault();
      onClick();
    }
  };

  return (
    <div
      className={classNames}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      role="button"
      tabIndex={disabled ? -1 : 0}
      aria-label={ariaLabel}
      data-square={algebraic}
    >
      {piece && <Piece piece={piece[0]} color={piece[1]} />}
      {isHighlighted && moveAnalysis && (
        <MoveIndicator
          category={moveAnalysis.category}
          isCapture={moveAnalysis.is_capture}
          isCheck={moveAnalysis.is_check}
        />
      )}
      {isHighlighted && !moveAnalysis && !piece && <div className="move-indicator" />}
      {isHighlighted && !moveAnalysis && piece && <div className="capture-indicator" />}
    </div>
  );
};

export default React.memo(Square);
