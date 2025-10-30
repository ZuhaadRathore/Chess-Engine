import { useState, useCallback } from 'react';

/**
 * Move quality annotation based on evaluation change
 */
export type MoveQuality =
  | 'brilliant'    // !! - Exceptional move (gain >300cp or best in complex position)
  | 'good'         // ! - Good move (gain >100cp)
  | 'normal'       // No annotation
  | 'inaccuracy'   // ?! - Inaccuracy (loss 50-100cp)
  | 'mistake'      // ? - Mistake (loss 100-200cp)
  | 'blunder';     // ?? - Blunder (loss >200cp)

/**
 * Evaluation data point for a move
 */
export interface EvaluationPoint {
  moveNumber: number;
  evaluation: number;
  quality: MoveQuality;
  isWhiteMove: boolean;
  moveNotation?: string; // The actual move notation (e.g., "e4", "Nf3")
}

/**
 * Determines move quality based on evaluation change
 * @param evalBefore - Evaluation before the move (from current player's perspective)
 * @param evalAfter - Evaluation after the move (from current player's perspective)
 * @returns Move quality classification
 */
function getMoveQuality(evalBefore: number, evalAfter: number): MoveQuality {
  // Calculate change from current player's perspective
  // Positive change = improvement, negative = worsening
  const change = evalAfter - evalBefore;

  if (change <= -200) return 'blunder';
  if (change <= -100) return 'mistake';
  if (change <= -50) return 'inaccuracy';
  if (change >= 300) return 'brilliant';
  if (change >= 100) return 'good';
  return 'normal';
}

/**
 * Get the symbol for a move quality
 */
export function getMoveQualitySymbol(quality: MoveQuality): string {
  switch (quality) {
    case 'brilliant': return '!!';
    case 'good': return '!';
    case 'inaccuracy': return '?!';
    case 'mistake': return '?';
    case 'blunder': return '??';
    case 'normal': return '';
  }
}

/**
 * Get the CSS class for a move quality
 */
export function getMoveQualityClass(quality: MoveQuality): string {
  return `move-quality-${quality}`;
}

/**
 * Hook to track game analysis including evaluation history and move quality
 */
export function useGameAnalysis() {
  const [evaluationHistory, setEvaluationHistory] = useState<EvaluationPoint[]>([]);
  const [previousEvaluation, setPreviousEvaluation] = useState<number | null>(null);

  /**
   * Record a new evaluation point after a move
   * @param evaluationAfterMove - Evaluation after the move (from White's perspective)
   * @param moveNumber - Full move number
   * @param wasWhiteMove - Whether this was White's move
   * @param moveNotation - Optional move notation (e.g., "e4", "Nf3")
   */
  const recordMove = useCallback((
    evaluationAfterMove: number,
    moveNumber: number,
    wasWhiteMove: boolean,
    moveNotation?: string
  ) => {
    setEvaluationHistory((prev) => {
      let quality: MoveQuality = 'normal';

      if (previousEvaluation !== null) {
        // Both evaluations are from White's perspective
        // For White: positive change = good, negative = bad
        // For Black: negative change = good (for Black), positive = bad (for Black)
        if (wasWhiteMove) {
          // White just moved - positive change is good for White
          quality = getMoveQuality(previousEvaluation, evaluationAfterMove);
        } else {
          // Black just moved - negative change is good for Black
          // Flip the evaluations to get Black's perspective
          quality = getMoveQuality(-evaluationAfterMove, -previousEvaluation);
        }
      }

      setPreviousEvaluation(evaluationAfterMove);
      return [...prev, {
        moveNumber,
        evaluation: evaluationAfterMove,
        quality,
        isWhiteMove: wasWhiteMove,
        moveNotation
      }];
    });
  }, [previousEvaluation]);

  /**
   * Reset analysis for a new game
   */
  const reset = useCallback(() => {
    setEvaluationHistory([]);
    setPreviousEvaluation(null);
  }, []);

  /**
   * Get the quality of a specific move by index
   */
  const getMoveQualityByIndex = useCallback((index: number): MoveQuality => {
    if (index < 0 || index >= evaluationHistory.length) {
      return 'normal';
    }
    return evaluationHistory[index].quality;
  }, [evaluationHistory]);

  /**
   * Get all blunders in the game
   */
  const getBlunders = useCallback((): EvaluationPoint[] => {
    return evaluationHistory.filter(point => point.quality === 'blunder');
  }, [evaluationHistory]);

  /**
   * Get all mistakes in the game (mistakes and blunders)
   */
  const getMistakes = useCallback((): EvaluationPoint[] => {
    return evaluationHistory.filter(
      point => point.quality === 'mistake' || point.quality === 'blunder'
    );
  }, [evaluationHistory]);

  return {
    evaluationHistory,
    recordMove,
    reset,
    getMoveQualityByIndex,
    getBlunders,
    getMistakes,
  };
}
