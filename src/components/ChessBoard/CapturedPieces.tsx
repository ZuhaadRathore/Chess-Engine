import React from 'react';
import type { Piece as PieceType, Color } from '@/types/index';
import Piece from './Piece';
import './CapturedPieces.css';

interface CapturedPiecesProps {
  pieces: PieceType[];
  color: Color;
}

interface GroupedPiece {
  piece: PieceType;
  count: number;
}

const groupPiecesByType = (pieces: PieceType[]): GroupedPiece[] => {
  // Count occurrences of each piece type
  const counts: Partial<Record<PieceType, number>> = {};

  for (const piece of pieces) {
    counts[piece] = (counts[piece] || 0) + 1;
  }

  // Sort by piece value (Queen > Rook > Bishop > Knight > Pawn)
  const pieceOrder: PieceType[] = ['Queen', 'Rook', 'Bishop', 'Knight', 'Pawn'];
  const grouped: GroupedPiece[] = [];

  for (const piece of pieceOrder) {
    if (counts[piece]) {
      grouped.push({ piece, count: counts[piece]! });
    }
  }

  return grouped;
};

const CapturedPieces: React.FC<CapturedPiecesProps> = ({ pieces, color }) => {
  const groupedPieces = groupPiecesByType(pieces);

  return (
    <div className="captured-pieces">
      <div className="captured-pieces-label">
        Captured from {color}
      </div>

      <div className="captured-pieces-list">
        {pieces.length === 0 ? (
          <span className="no-captures">—</span>
        ) : (
          groupedPieces.map(({ piece, count }) => (
            <div key={piece} className="captured-piece-group">
              <Piece piece={piece} color={color} size={24} />
              {count > 1 && <span className="piece-count">×{count}</span>}
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default CapturedPieces;
