import { useState, useEffect } from 'react';
import { analyzeAllLegalMoves } from '@/types/tauri';
import type { Position, MoveAnalysis } from '@/types';

/**
 * Hook to automatically analyze all legal moves in the current position
 * Returns an array of move analyses with categorization and details
 */
export function useMoveAnalysis(position: Position | null, enabled: boolean = true) {
  const [analyses, setAnalyses] = useState<MoveAnalysis[]>([]);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    if (!position || !enabled) {
      setAnalyses([]);
      return;
    }

    let isCancelled = false;

    const analyze = async () => {
      setIsAnalyzing(true);
      setError(null);

      try {
        const moveAnalyses = await analyzeAllLegalMoves();

        if (!isCancelled) {
          setAnalyses(moveAnalyses);
        }
      } catch (err) {
        if (!isCancelled) {
          setError(err instanceof Error ? err : new Error('Failed to analyze moves'));
          setAnalyses([]);
        }
      } finally {
        if (!isCancelled) {
          setIsAnalyzing(false);
        }
      }
    };

    analyze();

    return () => {
      isCancelled = true;
    };
  }, [position, enabled]);

  /**
   * Get analysis for a specific move by square indices
   */
  const getAnalysisForMove = (fromIndex: number, toIndex: number): MoveAnalysis | null => {
    return analyses.find(
      (analysis) => analysis.move_data.from.index === fromIndex && analysis.move_data.to.index === toIndex
    ) || null;
  };

  /**
   * Get all analyses for moves from a specific square
   */
  const getAnalysesFromSquare = (fromIndex: number): MoveAnalysis[] => {
    return analyses.filter((analysis) => analysis.move_data.from.index === fromIndex);
  };

  return {
    analyses,
    isAnalyzing,
    error,
    getAnalysisForMove,
    getAnalysesFromSquare,
  };
}
