import { describe, expect, it } from 'vitest';
import { algebraicToIndex, indexToAlgebraic, rankFileToIndex } from '@/utils/chess';

describe('chess coordinate helpers', () => {
  it('converts algebraic notation to board indices', () => {
    expect(algebraicToIndex('a1')).toBe(0);
    expect(algebraicToIndex('h1')).toBe(7);
    expect(algebraicToIndex('a8')).toBe(56);
    expect(algebraicToIndex('h8')).toBe(63);
    expect(algebraicToIndex('e4')).toBe(28);
  });

  it('converts board indices back to algebraic notation', () => {
    expect(indexToAlgebraic(0)).toBe('a1');
    expect(indexToAlgebraic(7)).toBe('h1');
    expect(indexToAlgebraic(56)).toBe('a8');
    expect(indexToAlgebraic(63)).toBe('h8');
    expect(indexToAlgebraic(28)).toBe('e4');
  });

  it('round-trips conversions without loss', () => {
    for (let rank = 0; rank < 8; rank++) {
      for (let file = 0; file < 8; file++) {
        const index = rankFileToIndex(rank, file);
        const algebraic = indexToAlgebraic(index);
        expect(algebraicToIndex(algebraic)).toBe(index);
      }
    }
  });
});
