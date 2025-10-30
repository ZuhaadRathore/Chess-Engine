import React, { useState, useEffect } from 'react';
import type { Position, GameStatus, Piece, PromotionPiece } from '@/types/index';
import { isGameOver } from '@/types/index';
import { newGame, loadFen, getBoardState, getGameStatus, makeMove, undoMove, getLegalMovesForSquare } from '@/types/tauri';
import { indexToAlgebraic, calculateCapturedPieces, isPromotionMove } from '@/utils/chess';
import { triggerImpact, triggerNotification, triggerSelection } from '@/utils/haptics';
import { useMoveAnalysis } from '@/hooks/useMoveAnalysis';
import Square from './Square';
import GameStatusComponent from './GameStatus';
import CapturedPieces from './CapturedPieces';
import PromotionDialog from './PromotionDialog';
import './ChessBoard.css';

interface ChessBoardProps {
  initialFen?: string;
  onGameEnd?: (status: GameStatus) => void;
  onMove?: (from: string, to: string, promotion?: string) => void;
  onReady?: () => void;
  onPositionChange?: (position: Position) => void;
  onStatusChange?: (status: GameStatus) => void;
  readOnly?: boolean;
  refreshToken?: number;
  hideUndoButton?: boolean;
  enableSwipeUndo?: boolean;
  playerColor?: 'White' | 'Black';
  showMoveAnalysis?: boolean;
}

const ChessBoard: React.FC<ChessBoardProps> = ({ initialFen, onGameEnd, onMove, onReady, onPositionChange, onStatusChange, readOnly = false, refreshToken, hideUndoButton = false, enableSwipeUndo = false, playerColor = 'White', showMoveAnalysis = true }) => {
  const [position, setPosition] = useState<Position | null>(null);
  const [selectedSquare, setSelectedSquare] = useState<number | null>(null);
  const [highlightedSquares, setHighlightedSquares] = useState<number[]>([]);
  const [lastMove, setLastMove] = useState<{ from: number; to: number } | null>(null);
  const [gameStatus, setGameStatus] = useState<GameStatus | null>(null);
  const [capturedPieces, setCapturedPieces] = useState<{ white: Piece[]; black: Piece[] }>({ white: [], black: [] });
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [showPromotionDialog, setShowPromotionDialog] = useState<boolean>(false);
  const [pendingPromotion, setPendingPromotion] = useState<{ from: number; to: number } | null>(null);
  const [swipeStart, setSwipeStart] = useState<{ x: number; y: number; time: number } | null>(null);

  // Get move analysis
  const { getAnalysisForMove } = useMoveAnalysis(position, showMoveAnalysis);

  useEffect(() => {
    if (position && onPositionChange) {
      onPositionChange(position);
    }
  }, [position, onPositionChange]);

  useEffect(() => {
    if (gameStatus && onStatusChange) {
      onStatusChange(gameStatus);
    }
  }, [gameStatus, onStatusChange]);

  // Initialize game
  useEffect(() => {
    const initGame = async () => {
      setIsLoading(true);
      setError(null);
      try {
        let boardState: Position;

        if (initialFen) {
          boardState = await loadFen(initialFen);
        } else {
          await newGame();
          boardState = await getBoardState();
        }

        const status = await getGameStatus();

        setPosition(boardState);
        setGameStatus(status);
        setCapturedPieces(calculateCapturedPieces(boardState));

        // Notify parent that board is ready
        if (onReady) {
          onReady();
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to initialize game');
      } finally {
        setIsLoading(false);
      }
    };

    initGame();
  }, [initialFen, onReady]);

  // Refresh board state when refreshToken changes (e.g., after opponent moves)
  useEffect(() => {
    const refreshBoardState = async () => {
      if (refreshToken !== undefined && refreshToken > 0) {
        try {
          const [boardState, status] = await Promise.all([
            getBoardState(),
            getGameStatus()
          ]);

          setPosition(boardState);
          setGameStatus(status);
          setCapturedPieces(calculateCapturedPieces(boardState));
        } catch (err) {
          console.error('Failed to refresh board state:', err);
        }
      }
    };

    refreshBoardState();
  }, [refreshToken]);

  // Update legal moves when selection changes
  useEffect(() => {
    const updateLegalMoves = async () => {
      if (selectedSquare !== null && position) {
        try {
          const algebraic = indexToAlgebraic(selectedSquare);
          const moves = await getLegalMovesForSquare(algebraic);

          // Extract destination indices from moves
          const destinations = moves.map(move => move.to.index);
          setHighlightedSquares(destinations);
        } catch (err) {
          console.error('Failed to get legal moves:', err);
          setHighlightedSquares([]);
        }
      } else {
        setHighlightedSquares([]);
      }
    };

    updateLegalMoves();
  }, [selectedSquare, position]);

  const handleSquareClick = (index: number) => {
    if (readOnly || isLoading || !position) return;

    const clickedSquare = position.board[index];

    if (selectedSquare === null) {
      // No square selected - try to select this square
      if (clickedSquare !== null) {
        const [, color] = clickedSquare;
        // Check if it's the current player's piece
        if (color === position.side_to_move) {
          setSelectedSquare(index);
          triggerSelection();
        }
      }
    } else {
      // Square already selected
      if (selectedSquare === index) {
        // Clicking same square - deselect
        setSelectedSquare(null);
        setHighlightedSquares([]);
      } else if (highlightedSquares.includes(index)) {
        // Valid move destination
        const selectedPiece = position.board[selectedSquare];
        if (selectedPiece !== null) {
          const [piece] = selectedPiece;

          // Haptic feedback for move confirmation
          triggerSelection();

          // Check if promotion is needed
          if (isPromotionMove(selectedSquare, index, piece)) {
            setPendingPromotion({ from: selectedSquare, to: index });
            setShowPromotionDialog(true);
          } else {
            executeMove(selectedSquare, index);
          }
        }
      } else if (clickedSquare !== null) {
        // Clicking different square with own piece - select new square
        const [, color] = clickedSquare;
        if (color === position.side_to_move) {
          setSelectedSquare(index);
          triggerSelection();
        }
      } else {
        // Clicking empty square that's not a valid move - deselect
        setSelectedSquare(null);
        setHighlightedSquares([]);
      }
    }
  };

  const executeMove = async (from: number, to: number, promotion?: PromotionPiece) => {
    setIsLoading(true);
    setError(null);

    try {
      // Check if this is a capture by seeing if there's a piece at destination
      const isCapture = position?.board[to] !== null;

      const fromAlgebraic = indexToAlgebraic(from);
      const toAlgebraic = indexToAlgebraic(to);

      await makeMove(fromAlgebraic, toAlgebraic, promotion);

      // Haptic feedback for successful move (heavy for captures, medium for normal moves)
      try {
        await triggerImpact(isCapture ? 'heavy' : 'medium');
      } catch (err) {
        // Haptics may not be supported on all devices, silently ignore
      }

      // Notify parent component of the move
      if (onMove) {
        onMove(fromAlgebraic, toAlgebraic, promotion);
      }

      // Update state with new position and status
      setLastMove({ from, to });

      const [boardState, status] = await Promise.all([
        getBoardState(),
        getGameStatus()
      ]);

      setPosition(boardState);
      setGameStatus(status);
      setCapturedPieces(calculateCapturedPieces(boardState));

      // Clear selection
      setSelectedSquare(null);
      setHighlightedSquares([]);

      // Check if game ended
      if (isGameOver(status)) {
        if (status.type === 'Checkmate') {
          await triggerNotification('success');
        } else {
          await triggerNotification('warning');
        }

        if (onGameEnd) {
          onGameEnd(status);
        }
      }
    } catch (err) {
      await triggerNotification('error');
      setError(err instanceof Error ? err.message : 'Failed to make move');
      setSelectedSquare(null);
      setHighlightedSquares([]);
    } finally {
      setIsLoading(false);
    }
  };

  const handlePromotionChoice = (piece: PromotionPiece) => {
    if (pendingPromotion) {
      executeMove(pendingPromotion.from, pendingPromotion.to, piece);
      setShowPromotionDialog(false);
      setPendingPromotion(null);
    }
  };

  const handleUndoMove = async () => {
    if (readOnly || isLoading) return;

    setIsLoading(true);
    setError(null);

    try {
      await undoMove();

      // Haptic feedback for undo
      try {
        await triggerImpact('light');
      } catch (err) {
        // Haptics may not be supported on all devices, silently ignore
      }

      const [boardState, status] = await Promise.all([
        getBoardState(),
        getGameStatus()
      ]);

      setPosition(boardState);
      setGameStatus(status);
      setCapturedPieces(calculateCapturedPieces(boardState));
      setLastMove(null);
      setSelectedSquare(null);
      setHighlightedSquares([]);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to undo move');
    } finally {
      setIsLoading(false);
    }
  };

  const handleBoardPointerDown = (e: React.PointerEvent) => {
    if (readOnly || isLoading || !enableSwipeUndo) return;
    setSwipeStart({ x: e.clientX, y: e.clientY, time: Date.now() });
  };

  const handleBoardPointerUp = (e: React.PointerEvent) => {
    if (readOnly || isLoading || !swipeStart || !enableSwipeUndo) return;

    const deltaX = e.clientX - swipeStart.x;
    const deltaY = e.clientY - swipeStart.y;
    const deltaTime = Date.now() - swipeStart.time;
    const totalDistance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);

    // Right-to-left swipe detection with tighter thresholds
    const isHorizontalSwipe = Math.abs(deltaX) > Math.abs(deltaY) * 1.5; // More horizontal bias
    const isRightToLeft = deltaX < -80; // Increased threshold from 60 to 80px
    const isFastSwipe = deltaTime < 350; // Tightened from 400ms to 350ms
    const hasMinimumTravel = totalDistance > 90; // Minimum total travel distance

    if (isHorizontalSwipe && isRightToLeft && isFastSwipe && hasMinimumTravel) {
      handleUndoMove();
    }

    setSwipeStart(null);
  };

  if (!position) {
    return (
      <div className="chess-board-container">
        <div className="loading-message">Loading chess board...</div>
      </div>
    );
  }

  return (
    <div className="chess-board-container">
      <GameStatusComponent status={gameStatus} position={position} />

      <div className="chess-board-wrapper">
        <CapturedPieces pieces={capturedPieces.black} color="Black" />

        <div className="chess-board-with-notation">
          {/* File labels (top) */}
          <div className="board-notation file-labels-top">
            {Array.from({ length: 8 }, (_, i) => {
              const file = playerColor === 'White' ? i : 7 - i;
              return <div key={`file-top-${i}`} className="notation-label">{String.fromCharCode(97 + file)}</div>;
            })}
          </div>

          <div className="board-with-rank-labels">
            {/* Rank labels (left) */}
            <div className="board-notation rank-labels-left">
              {Array.from({ length: 8 }, (_, i) => {
                const rank = playerColor === 'White' ? 8 - i : i + 1;
                return <div key={`rank-left-${i}`} className="notation-label">{rank}</div>;
              })}
            </div>

            <div
              className="chess-board"
              onPointerDown={handleBoardPointerDown}
              onPointerUp={handleBoardPointerUp}
            >
              {Array.from({ length: 8 }, (_, rankIndex) => {
                // Flip rank order if playing as Black
                const rank = playerColor === 'White' ? 7 - rankIndex : rankIndex;
                return Array.from({ length: 8 }, (_, fileIndex) => {
                  // Flip file order if playing as Black
                  const file = playerColor === 'White' ? fileIndex : 7 - fileIndex;
                  const index = rank * 8 + file;
                  const piece = position.board[index];
                  const isSelected = selectedSquare === index;
                  const isHighlighted = highlightedSquares.includes(index);
                  const isLastMoveSquare = lastMove?.from === index || lastMove?.to === index;

                  // Get move analysis for highlighted squares (moves to this square)
                  const moveAnalysis = isHighlighted && selectedSquare !== null
                    ? getAnalysisForMove(selectedSquare, index)
                    : null;

                  return (
                    <Square
                      key={index}
                      index={index}
                      piece={piece}
                      isSelected={isSelected}
                      isHighlighted={isHighlighted}
                      isLastMove={isLastMoveSquare}
                      onClick={() => handleSquareClick(index)}
                      disabled={readOnly || isLoading}
                      moveAnalysis={moveAnalysis}
                    />
                  );
                });
              }).flat()}
            </div>

            {/* Rank labels (right) */}
            <div className="board-notation rank-labels-right">
              {Array.from({ length: 8 }, (_, i) => {
                const rank = playerColor === 'White' ? 8 - i : i + 1;
                return <div key={`rank-right-${i}`} className="notation-label">{rank}</div>;
              })}
            </div>
          </div>

          {/* File labels (bottom) */}
          <div className="board-notation file-labels-bottom">
            {Array.from({ length: 8 }, (_, i) => {
              const file = playerColor === 'White' ? i : 7 - i;
              return <div key={`file-bottom-${i}`} className="notation-label">{String.fromCharCode(97 + file)}</div>;
            })}
          </div>
        </div>

        <CapturedPieces pieces={capturedPieces.white} color="White" />
      </div>

      {!hideUndoButton && (
        <div className="chess-board-controls">
          <button
            onClick={handleUndoMove}
            disabled={isLoading || readOnly}
            className="btn"
          >
            Undo Move
          </button>
        </div>
      )}

      {showPromotionDialog && (
        <PromotionDialog
          color={position.side_to_move}
          onSelect={handlePromotionChoice}
          onCancel={() => {
            setShowPromotionDialog(false);
            setPendingPromotion(null);
            setSelectedSquare(null);
          }}
        />
      )}

      {error && <div className="error-message">{error}</div>}
    </div>
  );
};

export default ChessBoard;
