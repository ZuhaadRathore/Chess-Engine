import React, { useEffect, useRef } from 'react';
import type { PromotionPiece, Color } from '@/types/index';
import PieceComponent from './Piece';
import './PromotionDialog.css';

interface PromotionDialogProps {
  color: Color;
  onSelect: (piece: PromotionPiece) => void;
  onCancel: () => void;
}

const PromotionDialog: React.FC<PromotionDialogProps> = ({ color, onSelect, onCancel }) => {
  const promotionOptions: PromotionPiece[] = ['Queen', 'Rook', 'Bishop', 'Knight'];
  const dialogRef = useRef<HTMLDivElement>(null);
  const previousFocusRef = useRef<HTMLElement | null>(null);

  // Focus trapping and Escape key handling
  useEffect(() => {
    // Save the currently focused element
    previousFocusRef.current = document.activeElement as HTMLElement;

    // Focus the first focusable element in the dialog
    if (dialogRef.current) {
      const focusableElements = dialogRef.current.querySelectorAll<HTMLElement>(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      );
      if (focusableElements.length > 0) {
        focusableElements[0].focus();
      }
    }

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onCancel();
        return;
      }

      // Focus trapping with Tab key
      if (e.key === 'Tab' && dialogRef.current) {
        const focusableElements = dialogRef.current.querySelectorAll<HTMLElement>(
          'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        );
        const firstElement = focusableElements[0];
        const lastElement = focusableElements[focusableElements.length - 1];

        if (e.shiftKey) {
          // Shift + Tab
          if (document.activeElement === firstElement) {
            e.preventDefault();
            lastElement?.focus();
          }
        } else {
          // Tab
          if (document.activeElement === lastElement) {
            e.preventDefault();
            firstElement?.focus();
          }
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);

    // Return focus to previous element on unmount
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      if (previousFocusRef.current) {
        previousFocusRef.current.focus();
      }
    };
  }, [onCancel]);

  const handleOverlayClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onCancel();
    }
  };

  return (
    <div
      className="promotion-dialog-overlay"
      onClick={handleOverlayClick}
      role="dialog"
      aria-modal="true"
      aria-labelledby="promotion-dialog-title"
    >
      <div ref={dialogRef} className="promotion-dialog" onClick={(e) => e.stopPropagation()}>
        <h3 id="promotion-dialog-title">Choose promotion piece</h3>

        <div className="promotion-choices">
          {promotionOptions.map((piece) => (
            <button
              key={piece}
              className="promotion-choice"
              onClick={() => onSelect(piece)}
              aria-label={`Promote to ${piece}`}
            >
              <PieceComponent piece={piece} color={color} size={60} />
              <span>{piece}</span>
            </button>
          ))}
        </div>

        <button className="promotion-cancel btn" onClick={onCancel}>
          Cancel
        </button>
      </div>
    </div>
  );
};

export default PromotionDialog;
