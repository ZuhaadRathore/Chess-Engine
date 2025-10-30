import React from 'react';
import { formatEvaluation } from '@/types';
import './EvaluationBar.css';

interface EvaluationBarProps {
  /** Evaluation score in centipawns (positive = White advantage, negative = Black advantage) */
  evaluation: number | null;
  /** Whether to show the numeric value */
  showValue?: boolean;
  /** Orientation of the board (affects display) */
  orientation?: 'White' | 'Black';
}

const EvaluationBar: React.FC<EvaluationBarProps> = ({
  evaluation,
  showValue = true,
  orientation = 'White',
}) => {
  if (evaluation === null) {
    return (
      <div className="evaluation-bar-container">
        <div className="evaluation-bar-loading">Calculating...</div>
      </div>
    );
  }

  // Convert centipawns to a percentage (clamped between -1000 and +1000 centipawns)
  // Using a logarithmic scale for better visualization
  const clampedEval = Math.max(-1000, Math.min(1000, evaluation));
  const normalizedEval = clampedEval / 1000; // -1 to +1

  // Apply logarithmic scaling for better visualization
  // This makes small advantages visible while not making huge advantages take up the whole bar
  const scaledEval = Math.sign(normalizedEval) * Math.sqrt(Math.abs(normalizedEval));

  // Convert to percentage (50% = equal, 0% = Black winning, 100% = White winning)
  const whitePercentage = 50 + (scaledEval * 50);

  // Determine if the advantage is significant
  const absEval = Math.abs(evaluation);
  const isSignificant = absEval > 100; // More than 1 pawn
  const isDecisive = absEval > 300; // More than 3 pawns

  // Flip display if playing as Black
  const displayPercentage = orientation === 'White' ? whitePercentage : 100 - whitePercentage;
  const displayEval = orientation === 'White' ? evaluation : -evaluation;

  // Tick marks at evaluation levels (in centipawns)
  const tickLevels = [
    { value: 900, label: '+9' },
    { value: 600, label: '+6' },
    { value: 300, label: '+3' },
    { value: 100, label: '+1' },
    { value: 0, label: '0' },
    { value: -100, label: '-1' },
    { value: -300, label: '-3' },
    { value: -600, label: '-6' },
    { value: -900, label: '-9' },
  ];

  // Calculate tick positions
  const getTickPosition = (evalValue: number) => {
    const clampedEval = Math.max(-1000, Math.min(1000, evalValue));
    const normalizedEval = clampedEval / 1000;
    const scaledEval = Math.sign(normalizedEval) * Math.sqrt(Math.abs(normalizedEval));
    const percentage = 50 + (scaledEval * 50);
    return orientation === 'White' ? percentage : 100 - percentage;
  };

  // Determine who has the advantage
  const getAdvantageText = () => {
    if (Math.abs(evaluation) < 50) {
      return { text: 'Equal', color: 'white', arrow: '=' };
    }

    const advantage = evaluation > 0 ? 'White' : 'Black';
    const absValue = Math.abs(evaluation);
    const formattedValue = (absValue / 100).toFixed(1);

    return {
      text: `${advantage} +${formattedValue}`,
      color: advantage.toLowerCase(),
      arrow: evaluation > 0 ? '↑' : '↓'
    };
  };

  const advantage = getAdvantageText();

  return (
    <div className="evaluation-bar-container">
      {/* Dynamic Advantage Indicator */}
      {showValue && (
        <div className={`advantage-indicator advantage-${advantage.color}`}>
          <div className="advantage-arrow">{advantage.arrow}</div>
          <div className="advantage-text">{advantage.text}</div>
        </div>
      )}

      <div className="evaluation-bar-wrapper">
        {/* Tick labels - left side */}
        <div className="evaluation-bar-labels">
          {tickLevels.map((tick) => {
            const position = getTickPosition(tick.value);
            const isOriented = orientation === 'White';
            const displayLabel = isOriented ? tick.label : (tick.value === 0 ? '0' : tick.label.startsWith('+') ? tick.label.replace('+', '-') : tick.label.replace('-', '+'));

            return (
              <div
                key={tick.value}
                className="evaluation-bar-label"
                style={{ bottom: `${position}%` }}
              >
                {displayLabel}
              </div>
            );
          })}
        </div>

        {/* The bar itself */}
        <div
          className="evaluation-bar"
          role="progressbar"
          aria-valuenow={displayPercentage}
          aria-valuemin={0}
          aria-valuemax={100}
          aria-label={`Position evaluation: ${formatEvaluation(displayEval)}`}
        >
          <div
            className={`evaluation-bar-fill ${isDecisive ? 'decisive' : isSignificant ? 'significant' : ''}`}
            style={{ height: `${displayPercentage}%` }}
          />

          {/* Tick marks */}
          {tickLevels.map((tick) => (
            <div
              key={tick.value}
              className={`evaluation-bar-tick ${tick.value === 0 ? 'evaluation-bar-tick-center' : ''}`}
              style={{ bottom: `${getTickPosition(tick.value)}%` }}
            />
          ))}
        </div>
      </div>
    </div>
  );
};

export default EvaluationBar;
