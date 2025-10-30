import { useState, useEffect } from 'react';
import { evaluatePosition } from '@/types/tauri';
import type { Position } from '@/types';

/**
 * Hook to automatically evaluate the current chess position
 * Returns the evaluation score in centipawns (positive = White advantage)
 */
export function usePositionEvaluation(position: Position | null, enabled: boolean = true) {
  const [evaluation, setEvaluation] = useState<number | null>(null);
  const [isEvaluating, setIsEvaluating] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    if (!position || !enabled) {
      setEvaluation(null);
      return;
    }

    let isCancelled = false;

    const evaluate = async () => {
      setIsEvaluating(true);
      setError(null);

      try {
        const score = await evaluatePosition();

        if (!isCancelled) {
          setEvaluation(score);
        }
      } catch (err) {
        if (!isCancelled) {
          setError(err instanceof Error ? err : new Error('Failed to evaluate position'));
          setEvaluation(null);
        }
      } finally {
        if (!isCancelled) {
          setIsEvaluating(false);
        }
      }
    };

    evaluate();

    return () => {
      isCancelled = true;
    };
  }, [position, enabled]);

  return {
    evaluation,
    isEvaluating,
    error,
  };
}
