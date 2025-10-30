import { useRef, useEffect, useState } from 'react';
import type { UiMove } from '@/types/ui';
import type { MoveQuality } from '@/hooks/useGameAnalysis';
import { getMoveQualitySymbol } from '@/hooks/useGameAnalysis';
import './MoveHistory.css';

interface MoveHistoryProps {
  moves: UiMove[];
  maxHeight?: string;
  moveQualities?: MoveQuality[];
}

interface MoveTurn {
  white: string;
  black?: string;
  whiteQuality?: MoveQuality;
  blackQuality?: MoveQuality;
}

function groupMovesByTurn(moves: UiMove[], qualities?: MoveQuality[]): MoveTurn[] {
  const grouped: MoveTurn[] = [];
  for (let i = 0; i < moves.length; i += 2) {
    grouped.push({
      white: moves[i].notation,
      black: moves[i + 1]?.notation,
      whiteQuality: qualities?.[i],
      blackQuality: qualities?.[i + 1],
    });
  }
  return grouped;
}

function MoveHistory({ moves, maxHeight = '400px', moveQualities }: MoveHistoryProps) {
  const scrollRef = useRef<HTMLDivElement>(null);
  const [autoScroll, setAutoScroll] = useState(true);

  useEffect(() => {
    if (autoScroll && scrollRef.current) {
      scrollRef.current.scrollTo({
        top: scrollRef.current.scrollHeight,
        behavior: 'smooth',
      });
    }
  }, [moves, autoScroll]);

  const groupedMoves = groupMovesByTurn(moves, moveQualities);

  return (
    <div className="move-history">
      <div className="move-history-header">
        <h3>Move History</h3>
        <span className="move-count">({moves.length} moves)</span>
      </div>

      <div className="move-list" ref={scrollRef} style={{ maxHeight }}>
        {moves.length === 0 ? (
          <div className="empty-state">No moves yet</div>
        ) : (
          groupedMoves.map((turn, index) => (
            <div
              key={index}
              className={`move-turn ${index === groupedMoves.length - 1 ? 'last-move' : ''}`}
            >
              <span className="turn-number">{index + 1}.</span>
              <span className={`move white-move move-quality-${turn.whiteQuality || 'normal'}`}>
                {turn.white}
                {turn.whiteQuality && (
                  <span className="move-annotation">{getMoveQualitySymbol(turn.whiteQuality)}</span>
                )}
              </span>
              {turn.black && (
                <span className={`move black-move move-quality-${turn.blackQuality || 'normal'}`}>
                  {turn.black}
                  {turn.blackQuality && (
                    <span className="move-annotation">{getMoveQualitySymbol(turn.blackQuality)}</span>
                  )}
                </span>
              )}
            </div>
          ))
        )}
      </div>

      <div className="move-history-footer">
        <label>
          <input
            type="checkbox"
            checked={autoScroll}
            onChange={(e) => setAutoScroll(e.target.checked)}
          />
          Auto-scroll
        </label>
      </div>
    </div>
  );
}

export default MoveHistory;
