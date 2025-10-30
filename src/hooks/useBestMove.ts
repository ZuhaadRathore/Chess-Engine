import { useState, useEffect } from 'react';
import { getLegalMoves, makeMove, undoMove, evaluatePosition } from '@/types/tauri';
import type { Position, Move } from '@/types';

/**
 * Move with its evaluation score
 */
export interface EvaluatedMove {
  move: Move;
  evaluation: number;
}

/**
 * Hook to find the best move in the current position
 * This evaluates all legal moves and returns the best one
 */
export function useBestMove(position: Position | null, enabled: boolean = true) {
  const [bestMove, setBestMove] = useState<EvaluatedMove | null>(null);
  const [isCalculating, setIsCalculating] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    if (!position || !enabled) {
      setBestMove(null);
      setIsCalculating(false);
      return;
    }

    let isCancelled = false;
    let timeoutId: NodeJS.Timeout;

    const findBestMove = async () => {
      setIsCalculating(true);
      setError(null);

      try {
        // Get all legal moves
        const legalMoves = await getLegalMoves();

        if (isCancelled) return;

        if (legalMoves.length === 0) {
          if (!isCancelled) {
            setBestMove(null);
          }
          return;
        }

        // Evaluate each move
        const evaluatedMoves: EvaluatedMove[] = [];

        for (const move of legalMoves) {
          // Check if cancelled before each move evaluation
          if (isCancelled) return;

          // Convert square indices to algebraic notation
          const from = indexToAlgebraic(move.from.index);
          const to = indexToAlgebraic(move.to.index);

          // Make the move - only pass promotion if it's a valid promotion piece
          const promotion = move.promotion && move.promotion !== 'Pawn' && move.promotion !== 'King'
            ? move.promotion
            : undefined;
          await makeMove(from, to, promotion);

          if (isCancelled) {
            // If cancelled mid-calculation, try to undo the last move
            try {
              await undoMove();
            } catch {
              // Ignore undo errors during cancellation
            }
            return;
          }

          // Evaluate the resulting position
          // Note: evaluation is from White's perspective, so we need to negate
          // it if it's Black's turn to get the evaluation from the current player's view
          const evaluation = await evaluatePosition();
          const adjustedEval = position.side_to_move === 'White' ? evaluation : -evaluation;

          evaluatedMoves.push({
            move,
            evaluation: adjustedEval,
          });

          // Undo the move to restore the position
          await undoMove();
        }

        if (!isCancelled && evaluatedMoves.length > 0) {
          // Find the move with the best evaluation (highest score = best for current player)
          const best = evaluatedMoves.reduce((prev, current) =>
            current.evaluation > prev.evaluation ? current : prev
          );

          setBestMove(best);
        }
      } catch (err) {
        if (!isCancelled) {
          setError(err instanceof Error ? err : new Error('Failed to calculate best move'));
          setBestMove(null);
        }
      } finally {
        if (!isCancelled) {
          setIsCalculating(false);
        }
      }
    };

    // Add a delay before starting calculation to allow user moves to take priority
    // This prevents the calculation from blocking immediate user actions
    timeoutId = setTimeout(() => {
      if (!isCancelled) {
        findBestMove();
      }
    }, 500); // 500ms delay gives user time to make moves

    return () => {
      isCancelled = true;
      clearTimeout(timeoutId);
      setIsCalculating(false);
    };
  }, [position, enabled]);

  return {
    bestMove,
    isCalculating,
    error,
  };
}

/**
 * Convert square index (0-63) to algebraic notation
 * 0=a1, 1=b1, ..., 63=h8
 */
function indexToAlgebraic(index: number): string {
  const file = String.fromCharCode(97 + (index % 8)); // a-h
  const rank = Math.floor(index / 8) + 1; // 1-8
  return `${file}${rank}`;
}
