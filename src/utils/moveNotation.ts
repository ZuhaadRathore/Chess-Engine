/**
 * Utility functions for chess move notation conversion
 */

/**
 * Parse UCI move string to algebraic notation
 * @param uci - UCI format move (e.g., "e2e4", "e7e8q")
 * @returns Object with from, to, and optional promotion
 */
export function uciToAlgebraic(uci: string): { from: string; to: string; promotion?: string } {
  const from = uci.substring(0, 2);
  const to = uci.substring(2, 4);
  const promotionLetter = uci.length === 5 ? uci[4] : undefined;

  return {
    from,
    to,
    promotion: promotionLetter ? uciToPromotion(promotionLetter) : undefined,
  };
}

/**
 * Convert algebraic notation to UCI format
 * @param from - Starting square (e.g., "e2")
 * @param to - Ending square (e.g., "e4")
 * @param promotion - Optional promotion piece name
 * @returns UCI format string
 */
export function algebraicToUci(from: string, to: string, promotion?: string): string {
  let uci = from + to;
  if (promotion) {
    uci += promotionToUci(promotion);
  }
  return uci;
}

/**
 * Convert promotion piece name to UCI letter
 * @param piece - Piece name (e.g., "Queen")
 * @returns UCI promotion letter
 */
export function promotionToUci(piece: string): string {
  const pieceMap: { [key: string]: string } = {
    'Queen': 'q',
    'Rook': 'r',
    'Bishop': 'b',
    'Knight': 'n',
  };
  return pieceMap[piece] || 'q';
}

/**
 * Convert UCI promotion letter to piece name
 * @param letter - UCI letter (e.g., "q")
 * @returns Piece name
 */
export function uciToPromotion(letter: string): string {
  const letterMap: { [key: string]: string } = {
    'q': 'Queen',
    'r': 'Rook',
    'b': 'Bishop',
    'n': 'Knight',
  };
  return letterMap[letter.toLowerCase()] || 'Queen';
}

/**
 * Format move in simple notation (from-to)
 * @param from - Starting square
 * @param to - Ending square
 * @returns Formatted move string
 */
export function formatMoveSimple(from: string, to: string): string {
  return `${from}-${to}`;
}

/**
 * Validate UCI move format
 * @param uci - UCI move string
 * @returns True if valid UCI format
 */
export function isValidUci(uci: string): boolean {
  if (uci.length !== 4 && uci.length !== 5) {
    return false;
  }

  const from = uci.substring(0, 2);
  const to = uci.substring(2, 4);

  if (!isValidAlgebraic(from) || !isValidAlgebraic(to)) {
    return false;
  }

  if (uci.length === 5) {
    const promotion = uci[4].toLowerCase();
    if (!['q', 'r', 'b', 'n'].includes(promotion)) {
      return false;
    }
  }

  return true;
}

/**
 * Validate algebraic square notation
 * @param square - Square in algebraic notation (e.g., "e4")
 * @returns True if valid
 */
export function isValidAlgebraic(square: string): boolean {
  if (square.length !== 2) {
    return false;
  }

  const file = square[0];
  const rank = square[1];

  return file >= 'a' && file <= 'h' && rank >= '1' && rank <= '8';
}
