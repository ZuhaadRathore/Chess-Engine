import type { EvaluationPoint, MoveQuality } from '@/hooks/useGameAnalysis';
import './GameSummary.css';

interface GameSummaryProps {
  evaluationHistory: EvaluationPoint[];
}

interface TeamStats {
  brilliant: EvaluationPoint[];
  good: EvaluationPoint[];
  inaccuracies: EvaluationPoint[];
  mistakes: EvaluationPoint[];
  blunders: EvaluationPoint[];
}

function getTeamStats(history: EvaluationPoint[], isWhite: boolean): TeamStats {
  const teamMoves = history.filter(p => p.isWhiteMove === isWhite);
  return {
    brilliant: teamMoves.filter(p => p.quality === 'brilliant'),
    good: teamMoves.filter(p => p.quality === 'good'),
    inaccuracies: teamMoves.filter(p => p.quality === 'inaccuracy'),
    mistakes: teamMoves.filter(p => p.quality === 'mistake'),
    blunders: teamMoves.filter(p => p.quality === 'blunder'),
  };
}

function formatMoveList(moves: EvaluationPoint[]): string {
  if (moves.length === 0) return '';
  return moves.map(m => {
    const moveNum = m.isWhiteMove ? `${m.moveNumber}.` : `${m.moveNumber}...`;
    return m.moveNotation ? `${moveNum}${m.moveNotation}` : `${moveNum}`;
  }).join(', ');
}

interface StatItemProps {
  quality: MoveQuality;
  icon: string;
  label: string;
  moves: EvaluationPoint[];
}

function StatItem({ quality, icon, label, moves }: StatItemProps) {
  if (moves.length === 0) return null;

  const moveList = formatMoveList(moves);

  return (
    <div className={`stat-item stat-${quality}`}>
      <span className="stat-icon">{icon}</span>
      <span className="stat-label">{label}</span>
      <span className="stat-value">{moves.length}</span>
      {moveList && <span className="stat-moves">({moveList})</span>}
    </div>
  );
}

function GameSummary({ evaluationHistory }: GameSummaryProps) {
  if (evaluationHistory.length === 0) {
    return null;
  }

  const whiteStats = getTeamStats(evaluationHistory, true);
  const blackStats = getTeamStats(evaluationHistory, false);

  // Only show if there are notable moves
  const hasNotableMoves = (stats: TeamStats) => {
    return stats.brilliant.length + stats.good.length +
           stats.inaccuracies.length + stats.mistakes.length +
           stats.blunders.length > 0;
  };

  if (!hasNotableMoves(whiteStats) && !hasNotableMoves(blackStats)) {
    return null;
  }

  return (
    <div className="game-summary">
      <div className="game-summary-header">
        <h3>Analysis</h3>
      </div>
      <div className="game-summary-content">
        {hasNotableMoves(whiteStats) && (
          <div className="team-stats">
            <h4 className="team-header white-team">White</h4>
            <div className="game-summary-stats">
              <StatItem quality="brilliant" icon="!!" label="Brilliant" moves={whiteStats.brilliant} />
              <StatItem quality="good" icon="!" label="Good" moves={whiteStats.good} />
              <StatItem quality="inaccuracy" icon="?!" label="Inaccuracy" moves={whiteStats.inaccuracies} />
              <StatItem quality="mistake" icon="?" label="Mistake" moves={whiteStats.mistakes} />
              <StatItem quality="blunder" icon="??" label="Blunder" moves={whiteStats.blunders} />
            </div>
          </div>
        )}
        {hasNotableMoves(blackStats) && (
          <div className="team-stats">
            <h4 className="team-header black-team">Black</h4>
            <div className="game-summary-stats">
              <StatItem quality="brilliant" icon="!!" label="Brilliant" moves={blackStats.brilliant} />
              <StatItem quality="good" icon="!" label="Good" moves={blackStats.good} />
              <StatItem quality="inaccuracy" icon="?!" label="Inaccuracy" moves={blackStats.inaccuracies} />
              <StatItem quality="mistake" icon="?" label="Mistake" moves={blackStats.mistakes} />
              <StatItem quality="blunder" icon="??" label="Blunder" moves={blackStats.blunders} />
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default GameSummary;
