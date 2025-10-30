import React, { useState } from 'react';
import type { EvaluatedMove } from '@/hooks/useBestMove';
import { formatEvaluation } from '@/types';
import './BestMoveHint.css';

interface BestMoveHintProps {
  bestMove: EvaluatedMove | null;
  isCalculating: boolean;
  onShowHint: (move: EvaluatedMove) => void;
}

/**
 * Convert square index (0-63) to algebraic notation
 */
function indexToAlgebraic(index: number): string {
  const file = String.fromCharCode(97 + (index % 8)); // a-h
  const rank = Math.floor(index / 8) + 1; // 1-8
  return `${file}${rank}`;
}

const BestMoveHint: React.FC<BestMoveHintProps> = ({
  bestMove,
  isCalculating,
  onShowHint,
}) => {
  const [hintShown, setHintShown] = useState(false);

  const handleShowHint = () => {
    if (bestMove) {
      setHintShown(true);
      onShowHint(bestMove);
      // Auto-hide hint after 3 seconds
      setTimeout(() => setHintShown(false), 3000);
    }
  };

  if (isCalculating) {
    return (
      <div className="best-move-hint">
        <button className="hint-button" disabled>
          Calculating...
        </button>
      </div>
    );
  }

  if (!bestMove) {
    return null;
  }

  const from = indexToAlgebraic(bestMove.move.from.index);
  const to = indexToAlgebraic(bestMove.move.to.index);

  return (
    <div className="best-move-hint">
      <button
        className="hint-button"
        onClick={handleShowHint}
        disabled={hintShown}
      >
        {hintShown ? 'Hint Shown' : 'Show Best Move'}
      </button>
      {hintShown && (
        <div className="hint-details">
          <div className="hint-move">
            {from} → {to}
            {bestMove.move.promotion && ` = ${bestMove.move.promotion}`}
          </div>
          <div className="hint-evaluation">
            Eval: {formatEvaluation(bestMove.evaluation)}
          </div>
        </div>
      )}
    </div>
  );
};

export default BestMoveHint;
