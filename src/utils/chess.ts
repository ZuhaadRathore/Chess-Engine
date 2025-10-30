import type { Piece, Color, Position } from '@/types/index';

/**
 * Convert 0-63 index to rank (0-7) and file (0-7)
 */
export function indexToRankFile(index: number): { rank: number; file: number } {
  const rank = Math.floor(index / 8);
  const file = index % 8;
  return { rank, file };
}

/**
 * Convert rank (0-7) and file (0-7) to 0-63 index
 */
export function rankFileToIndex(rank: number, file: number): number {
  if (rank < 0 || rank > 7 || file < 0 || file > 7) {
    throw new Error(`Invalid rank or file: rank=${rank}, file=${file}`);
  }
  return rank * 8 + file;
}

/**
 * Convert 0-63 index to algebraic notation (e.g., "e4")
 */
export function indexToAlgebraic(index: number): string {
  const { rank, file } = indexToRankFile(index);
  const fileLetters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
  return `${fileLetters[file]}${rank + 1}`;
}

/**
 * Convert algebraic notation (e.g., "e4") to 0-63 index
 */
export function algebraicToIndex(algebraic: string): number {
  if (algebraic.length !== 2) {
    throw new Error(`Invalid algebraic notation: ${algebraic}`);
  }

  const fileLetter = algebraic[0].toLowerCase();
  const rankNumber = parseInt(algebraic[1], 10);

  const fileLetters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
  const file = fileLetters.indexOf(fileLetter);
  const rank = rankNumber - 1;

  if (file === -1 || rank < 0 || rank > 7) {
    throw new Error(`Invalid algebraic notation: ${algebraic}`);
  }

  return rankFileToIndex(rank, file);
}

/**
 * Determine if a square is light or dark
 */
export function getSquareColor(index: number): 'light' | 'dark' {
  const { rank, file } = indexToRankFile(index);
  return (rank + file) % 2 === 0 ? 'dark' : 'light';
}

/**
 * Check if a square is highlighted
 */
export function isSquareHighlighted(index: number, highlightedSquares: number[]): boolean {
  return highlightedSquares.includes(index);
}

/**
 * Get asset path for a piece SVG
 */
export function getPieceImagePath(piece: Piece, color: Color): string {
  const colorLower = color.toLowerCase();
  const pieceLower = piece.toLowerCase();
  return `/src/assets/pieces/${colorLower}-${pieceLower}.svg`;
}

/**
 * Calculate captured pieces from current position
 */
export function calculateCapturedPieces(position: Position): { white: Piece[]; black: Piece[] } {
  // Initial piece counts per side
  const initialCounts: Record<Piece, number> = {
    Pawn: 8,
    Rook: 2,
    Knight: 2,
    Bishop: 2,
    Queen: 1,
    King: 1,
  };

  // Count current pieces on board
  const whiteCounts: Record<Piece, number> = {
    Pawn: 0,
    Rook: 0,
    Knight: 0,
    Bishop: 0,
    Queen: 0,
    King: 0,
  };

  const blackCounts: Record<Piece, number> = {
    Pawn: 0,
    Rook: 0,
    Knight: 0,
    Bishop: 0,
    Queen: 0,
    King: 0,
  };

  // Count pieces on board
  for (const square of position.board) {
    if (square !== null) {
      const [piece, color] = square;
      if (color === 'White') {
        whiteCounts[piece]++;
      } else {
        blackCounts[piece]++;
      }
    }
  }

  // Calculate extra promoted pieces for each side
  const extraPromotedWhite = Math.max(0, whiteCounts.Queen - 1) +
                              Math.max(0, whiteCounts.Rook - 2) +
                              Math.max(0, whiteCounts.Bishop - 2) +
                              Math.max(0, whiteCounts.Knight - 2);

  const extraPromotedBlack = Math.max(0, blackCounts.Queen - 1) +
                              Math.max(0, blackCounts.Rook - 2) +
                              Math.max(0, blackCounts.Bishop - 2) +
                              Math.max(0, blackCounts.Knight - 2);

  // Adjust missing pawn counts to account for promotions
  const whiteMissingPawns = Math.max(0, initialCounts.Pawn - whiteCounts.Pawn - extraPromotedWhite);
  const blackMissingPawns = Math.max(0, initialCounts.Pawn - blackCounts.Pawn - extraPromotedBlack);

  // Calculate captured pieces (missing from initial counts)
  const whiteCaptured: Piece[] = [];
  const blackCaptured: Piece[] = [];

  const pieces: Piece[] = ['Rook', 'Knight', 'Bishop', 'Queen'];

  // Add adjusted pawn counts
  for (let i = 0; i < whiteMissingPawns; i++) {
    whiteCaptured.push('Pawn');
  }
  for (let i = 0; i < blackMissingPawns; i++) {
    blackCaptured.push('Pawn');
  }

  // Handle other pieces with existing logic, clamping at zero
  for (const piece of pieces) {
    const whiteMissing = Math.max(0, initialCounts[piece] - whiteCounts[piece]);
    const blackMissing = Math.max(0, initialCounts[piece] - blackCounts[piece]);

    // Add missing white pieces to white captured (captured by black)
    for (let i = 0; i < whiteMissing; i++) {
      whiteCaptured.push(piece);
    }

    // Add missing black pieces to black captured (captured by white)
    for (let i = 0; i < blackMissing; i++) {
      blackCaptured.push(piece);
    }
  }

  return { white: whiteCaptured, black: blackCaptured };
}

/**
 * Check if index is valid (0-63)
 */
export function isValidSquareIndex(index: number): boolean {
  return index >= 0 && index <= 63;
}

/**
 * Check if a move requires promotion (pawn to back rank)
 */
export function isPromotionMove(_from: number, to: number, piece: Piece): boolean {
  if (piece !== 'Pawn') {
    return false;
  }

  const { rank: toRank } = indexToRankFile(to);
  return toRank === 0 || toRank === 7;
}
