import React, { useRef, useEffect } from 'react';
import type { EvaluationPoint } from '@/hooks/useGameAnalysis';
import './EvaluationGraph.css';

interface EvaluationGraphProps {
  evaluationHistory: EvaluationPoint[];
  height?: number;
  maxHeight?: string;
}

const EvaluationGraph: React.FC<EvaluationGraphProps> = ({
  evaluationHistory,
  height = 200,
  maxHeight = '200px',
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || evaluationHistory.length === 0) {
      return;
    }

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      return;
    }

    // Set canvas size
    const dpr = window.devicePixelRatio || 1;
    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width * dpr;
    canvas.height = height * dpr;
    ctx.scale(dpr, dpr);

    const width = rect.width;
    const padding = { top: 20, right: 20, bottom: 30, left: 40 };
    const chartWidth = width - padding.left - padding.right;
    const chartHeight = height - padding.top - padding.bottom;

    // Clear canvas
    ctx.clearRect(0, 0, width, height);

    // Draw background
    ctx.fillStyle = '#1a1a1a';
    ctx.fillRect(0, 0, width, height);

    // Find max/min evaluation for scaling (clamp to reasonable values)
    const maxEval = Math.min(
      1000,
      Math.max(...evaluationHistory.map((p) => p.evaluation), 500)
    );
    const minEval = Math.max(
      -1000,
      Math.min(...evaluationHistory.map((p) => p.evaluation), -500)
    );

    // Helper to convert evaluation to y-coordinate
    const evalToY = (evaluation: number): number => {
      const normalized = (evaluation - minEval) / (maxEval - minEval);
      return padding.top + chartHeight * (1 - normalized);
    };

    // Helper to convert move index to x-coordinate
    const moveToX = (moveIndex: number): number => {
      return padding.left + (moveIndex / Math.max(1, evaluationHistory.length - 1)) * chartWidth;
    };

    // Draw grid lines
    ctx.strokeStyle = '#333';
    ctx.lineWidth = 1;

    // Horizontal grid lines
    const gridLines = [0, 300, -300, 600, -600];
    gridLines.forEach((evalValue) => {
      if (evalValue >= minEval && evalValue <= maxEval) {
        const y = evalToY(evalValue);
        ctx.beginPath();
        ctx.moveTo(padding.left, y);
        ctx.lineTo(padding.left + chartWidth, y);
        ctx.stroke();

        // Draw label
        ctx.fillStyle = '#888';
        ctx.font = '11px monospace';
        ctx.textAlign = 'right';
        ctx.textBaseline = 'middle';
        ctx.fillText((evalValue / 100).toFixed(1), padding.left - 8, y);
      }
    });

    // Draw zero line (thicker)
    const zeroY = evalToY(0);
    ctx.strokeStyle = '#555';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(padding.left, zeroY);
    ctx.lineTo(padding.left + chartWidth, zeroY);
    ctx.stroke();

    // Draw evaluation line
    ctx.strokeStyle = '#4a9eff';
    ctx.lineWidth = 2;
    ctx.beginPath();

    evaluationHistory.forEach((point, index) => {
      const x = moveToX(index);
      const y = evalToY(point.evaluation);

      if (index === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    });

    ctx.stroke();

    // Draw points for each move
    evaluationHistory.forEach((point, index) => {
      const x = moveToX(index);
      const y = evalToY(point.evaluation);

      // Color based on move quality
      let color = '#4a9eff';
      if (point.quality === 'blunder') color = '#ff4444';
      else if (point.quality === 'mistake') color = '#ff8844';
      else if (point.quality === 'inaccuracy') color = '#ffaa44';
      else if (point.quality === 'brilliant') color = '#44ff44';
      else if (point.quality === 'good') color = '#88ff44';

      ctx.fillStyle = color;
      ctx.beginPath();
      ctx.arc(x, y, 4, 0, 2 * Math.PI);
      ctx.fill();

      // Highlight blunders and mistakes with larger circles
      if (point.quality === 'blunder' || point.quality === 'mistake') {
        ctx.strokeStyle = color;
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.arc(x, y, 7, 0, 2 * Math.PI);
        ctx.stroke();
      }
    });

    // Draw axis labels
    ctx.fillStyle = '#aaa';
    ctx.font = '12px sans-serif';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';
    ctx.fillText('Move Number', padding.left + chartWidth / 2, height - 15);

    ctx.save();
    ctx.translate(15, padding.top + chartHeight / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.textAlign = 'center';
    ctx.textBaseline = 'top';
    ctx.fillText('Evaluation (pawns)', 0, 0);
    ctx.restore();
  }, [evaluationHistory, height]);

  if (evaluationHistory.length === 0) {
    return (
      <div className="evaluation-graph-container" style={{ maxHeight }}>
        <div className="evaluation-graph-header">
          <h3>Evaluation Graph</h3>
        </div>
        <div className="evaluation-graph-empty">
          No moves played yet
        </div>
      </div>
    );
  }

  return (
    <div className="evaluation-graph-container" style={{ maxHeight }}>
      <div className="evaluation-graph-header">
        <h3>Evaluation Graph</h3>
        <span className="move-count">({evaluationHistory.length} moves)</span>
      </div>
      <div className="evaluation-graph-canvas-wrapper">
        <canvas
          ref={canvasRef}
          style={{ width: '100%', height: `${height}px` }}
        />
      </div>
      <div className="evaluation-graph-legend">
        <div className="legend-item">
          <span className="legend-dot legend-brilliant"></span>
          <span>Brilliant (!!)</span>
        </div>
        <div className="legend-item">
          <span className="legend-dot legend-good"></span>
          <span>Good (!)</span>
        </div>
        <div className="legend-item">
          <span className="legend-dot legend-inaccuracy"></span>
          <span>Inaccuracy (?!)</span>
        </div>
        <div className="legend-item">
          <span className="legend-dot legend-mistake"></span>
          <span>Mistake (?)</span>
        </div>
        <div className="legend-item">
          <span className="legend-dot legend-blunder"></span>
          <span>Blunder (??)</span>
        </div>
      </div>
    </div>
  );
};

export default EvaluationGraph;
