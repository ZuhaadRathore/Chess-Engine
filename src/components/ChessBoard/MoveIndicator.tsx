import React from 'react';
import type { MoveCategory } from '@/types';
import './MoveIndicator.css';

interface MoveIndicatorProps {
  category: MoveCategory;
  isCapture: boolean;
  isCheck: boolean;
}

const MoveIndicator: React.FC<MoveIndicatorProps> = ({ category, isCapture, isCheck }) => {
  // Determine indicator type based on move characteristics
  const getIndicatorClass = (): string => {
    if (category.type === 'EnPassant' || category.type === 'Castle') {
      return 'move-indicator-special';
    }
    if (isCheck && isCapture) {
      return 'move-indicator-check-capture';
    }
    if (isCheck) {
      return 'move-indicator-check';
    }
    if (isCapture) {
      return 'move-indicator-capture';
    }
    return 'move-indicator-quiet';
  };

  const getIndicatorSymbol = (): string => {
    if (category.type === 'Castle') return '♜';
    if (category.type === 'EnPassant') return 'EP';
    if (isCheck && isCapture) return '✕+';
    if (isCheck) return '+';
    if (isCapture) return '✕';
    return '';
  };

  return (
    <div className={`move-indicator ${getIndicatorClass()}`}>
      <span className="move-indicator-symbol">{getIndicatorSymbol()}</span>
    </div>
  );
};

export default MoveIndicator;
