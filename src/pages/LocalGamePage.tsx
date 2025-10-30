import { useCallback, useEffect, useState } from 'react';
import ChessBoard from '@/components/ChessBoard';
import MoveHistory from '@/components/MoveHistory';
import EvaluationBar from '@/components/EvaluationBar';
import GameSummary from '@/components/GameSummary';
import BestMoveHint from '@/components/BestMoveHint';
import { usePositionEvaluation } from '@/hooks/usePositionEvaluation';
import { useGameAnalysis } from '@/hooks/useGameAnalysis';
import { useBestMove } from '@/hooks/useBestMove';
import type { Color, GameStatus, Position } from '@/types';
import type { UiMove } from '@/types/ui';
import { formatMoveSimple } from '@/utils/moveNotation';
import { algebraicToIndex } from '@/utils/chess';
import './LocalGamePage.css';

function LocalGamePage() {
  const [moveHistory, setMoveHistory] = useState<UiMove[]>([]);
  const [position, setPosition] = useState<Position | null>(null);
  const [_status, setStatus] = useState<GameStatus | null>(null);
  const [orientation, setOrientation] = useState<Color>('White');
  const [autoFlip, setAutoFlip] = useState(false);
  const [boardKey, setBoardKey] = useState(0);
  const [_resultMessage, setResultMessage] = useState<string | null>(null);
  const [showAnalysis, setShowAnalysis] = useState(true);
  const [showBestMove, setShowBestMove] = useState(false);

  // Get position evaluation
  const { evaluation } = usePositionEvaluation(position, showAnalysis);

  // Track game analysis (evaluation history and move quality)
  const gameAnalysis = useGameAnalysis();

  // Get best move suggestion
  const { bestMove, isCalculating } = useBestMove(position, showAnalysis && showBestMove);

  useEffect(() => {
    if (!autoFlip || !position) {
      return;
    }

    if (orientation !== position.side_to_move) {
      setOrientation(position.side_to_move);
    }
  }, [autoFlip, orientation, position]);

  // Track evaluation after each move
  useEffect(() => {
    if (evaluation !== null && position && moveHistory.length > 0) {
      // Determine which side just moved based on current side to move
      // If it's currently White's turn, Black just moved
      const wasWhiteMove = position.side_to_move === 'Black';
      const lastMove = moveHistory[moveHistory.length - 1];
      gameAnalysis.recordMove(evaluation, position.fullmove_number, wasWhiteMove, lastMove?.notation);
    }
  }, [evaluation, position, gameAnalysis, moveHistory.length]);

  const handleMove = useCallback(
    (from: string, to: string) => {
      if (!position) {
        return;
      }

      const fromIndex = algebraicToIndex(from);
      const square = position.board[fromIndex];

      if (!square) {
        return;
      }

      const [piece, color] = square;

      setMoveHistory((prev) => [
        ...prev,
        {
          from,
          to,
          piece,
          color,
          notation: formatMoveSimple(from, to),
        },
      ]);

      setResultMessage(null);
    },
    [position],
  );

  const handleStatusChange = useCallback((nextStatus: GameStatus) => {
    setStatus(nextStatus);
  }, []);

  const handleGameEnd = useCallback((finalStatus: GameStatus) => {
    setStatus(finalStatus);

    switch (finalStatus.type) {
      case 'Checkmate':
        setResultMessage(`${finalStatus.winner} wins by checkmate`);
        break;
      case 'Stalemate':
        setResultMessage('Draw by stalemate');
        break;
      case 'DrawByFiftyMoveRule':
        setResultMessage('Draw by fifty-move rule');
        break;
      case 'DrawByInsufficientMaterial':
        setResultMessage('Draw by insufficient material');
        break;
      case 'DrawByRepetition':
        setResultMessage('Draw by repetition');
        break;
      default:
        setResultMessage(null);
    }
  }, []);

  const handleNewGame = useCallback(() => {
    setBoardKey((value) => value + 1);
    setMoveHistory([]);
    setResultMessage(null);
    setStatus(null);
    setOrientation('White');
    gameAnalysis.reset();
  }, [gameAnalysis]);

  const toggleAutoFlip = useCallback(() => {
    setAutoFlip((value) => !value);
  }, []);

  const handleManualFlip = useCallback(() => {
    setOrientation((value) => (value === 'White' ? 'Black' : 'White'));
  }, []);

  const toggleAnalysis = useCallback(() => {
    setShowAnalysis((value) => !value);
  }, []);

  const toggleBestMove = useCallback(() => {
    setShowBestMove((value) => !value);
  }, []);

  const handleShowHint = useCallback(() => {
    // Hint is displayed by the BestMoveHint component itself
    // This callback can be used for additional actions in the future
  }, []);

  return (
    <main className="local-game-page">
      <div className="local-game-layout">
        <section className="local-game-board-wrapper">
          <div className="local-game-board-container">
            {showAnalysis && (
              <div className="local-game-evaluation">
                <EvaluationBar evaluation={evaluation} orientation={orientation} />
              </div>
            )}

            <div className="local-game-board">
              <ChessBoard
                key={boardKey}
                onMove={handleMove}
                onGameEnd={handleGameEnd}
                onPositionChange={setPosition}
                onStatusChange={handleStatusChange}
                playerColor={orientation}
                enableSwipeUndo
                showMoveAnalysis={showAnalysis}
              />
            </div>
          </div>
        </section>

        <aside className="local-game-sidebar">
          {showAnalysis && showBestMove && (
            <BestMoveHint
              bestMove={bestMove}
              isCalculating={isCalculating}
              onShowHint={handleShowHint}
            />
          )}

          {showAnalysis && (
            <GameSummary evaluationHistory={gameAnalysis.evaluationHistory} />
          )}

          <MoveHistory
            moves={moveHistory}
            maxHeight="360px"
            moveQualities={showAnalysis ? gameAnalysis.evaluationHistory.map((point) => point.quality) : undefined}
          />

          <div className="local-game-controls">
            <button className="btn" onClick={handleNewGame}>
              New Game
            </button>
            <button
              className={`btn ${autoFlip ? 'btn-secondary' : ''}`}
              onClick={toggleAutoFlip}
            >
              Auto Flip: {autoFlip ? 'On' : 'Off'}
            </button>
            <button className="btn btn-secondary" onClick={handleManualFlip}>
              Flip Board
            </button>
            <button
              className={`btn ${showAnalysis ? 'btn-secondary' : ''}`}
              onClick={toggleAnalysis}
            >
              Analysis: {showAnalysis ? 'On' : 'Off'}
            </button>
            {showAnalysis && (
              <button
                className={`btn ${showBestMove ? 'btn-secondary' : ''}`}
                onClick={toggleBestMove}
              >
                Best Move: {showBestMove ? 'On' : 'Off'}
              </button>
            )}
          </div>
        </aside>
      </div>
    </main>
  );
}

export default LocalGamePage;
