import React from 'react';
import type { Piece as PieceType, Color } from '@/types/index';
import './Piece.css';

// Import all piece SVGs as React components
import WhiteKing from '@/assets/pieces/white-king.svg?react';
import WhiteQueen from '@/assets/pieces/white-queen.svg?react';
import WhiteRook from '@/assets/pieces/white-rook.svg?react';
import WhiteBishop from '@/assets/pieces/white-bishop.svg?react';
import WhiteKnight from '@/assets/pieces/white-knight.svg?react';
import WhitePawn from '@/assets/pieces/white-pawn.svg?react';
import BlackKing from '@/assets/pieces/black-king.svg?react';
import BlackQueen from '@/assets/pieces/black-queen.svg?react';
import BlackRook from '@/assets/pieces/black-rook.svg?react';
import BlackBishop from '@/assets/pieces/black-bishop.svg?react';
import BlackKnight from '@/assets/pieces/black-knight.svg?react';
import BlackPawn from '@/assets/pieces/black-pawn.svg?react';

interface PieceProps {
  piece: PieceType;
  color: Color;
  size?: number;
}

// Map of piece components
const pieceComponents = {
  White: {
    King: WhiteKing,
    Queen: WhiteQueen,
    Rook: WhiteRook,
    Bishop: WhiteBishop,
    Knight: WhiteKnight,
    Pawn: WhitePawn
  },
  Black: {
    King: BlackKing,
    Queen: BlackQueen,
    Rook: BlackRook,
    Bishop: BlackBishop,
    Knight: BlackKnight,
    Pawn: BlackPawn
  }
};

const Piece: React.FC<PieceProps> = ({ piece, color, size }) => {
  const PieceComponent = pieceComponents[color][piece];

  if (!PieceComponent) {
    console.error(`No component found for ${color} ${piece}`);
    return null;
  }

  const style = size ? { width: size, height: size } : undefined;

  return (
    <div className="chess-piece" style={style}>
      <PieceComponent aria-hidden="true" />
    </div>
  );
};

export default React.memo(Piece);
