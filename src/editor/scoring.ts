// FUL-45: Scoring System
// Points tracking, high score persistence, and score display

export interface ScoreEvent {
  type: 'particle_consumed' | 'enemy_destroyed' | 'time_survived' | 'debris_collected' | 'efficiency_bonus';
  points: number;
  timestamp: number;
  details?: string;
}

export interface SessionScore {
  total: number;
  particlesConsumed: number;
  enemiesDestroyed: number;
  timeSurvived: number;
  debrisCollected: number;
  efficiencyMultiplier: number;
  events: ScoreEvent[];
}

export interface HighScore {
  score: number;
  date: string;
  particlesConsumed: number;
  enemiesDestroyed: number;
  survivalTime: number;
}

export interface ScoreConfig {
  pointsPerParticle: number;
  pointsPerEnemy: number;
  pointsPerSecond: number;
  pointsPerDebris: number;
  efficiencyThreshold: number;
  efficiencyMultiplier: number;
  maxHighScores: number;
}

export const DEFAULT_SCORE_CONFIG: ScoreConfig = {
  pointsPerParticle: 1,
  pointsPerEnemy: 100,
  pointsPerSecond: 1,
  pointsPerDebris: 10,
  efficiencyThreshold: 10,
  efficiencyMultiplier: 2,
  maxHighScores: 10,
};

export function createSessionScore(): SessionScore {
  return {
    total: 0,
    particlesConsumed: 0,
    enemiesDestroyed: 0,
    timeSurvived: 0,
    debrisCollected: 0,
    efficiencyMultiplier: 1,
    events: [],
  };
}

export function recordScoreEvent(
  score: SessionScore,
  event: Omit<ScoreEvent, 'timestamp'>
): SessionScore {
  const timestamp = Date.now();
  let points = event.points;

  // Apply efficiency multiplier for mass particle events
  if (event.type === 'particle_consumed' && event.points >= 10) {
    points = Math.floor(points * score.efficiencyMultiplier);
  }

  const newEvent: ScoreEvent = { ...event, points, timestamp };

  const updatedScore: SessionScore = {
    ...score,
    total: score.total + points,
    particlesConsumed: score.particlesConsumed + (event.type === 'particle_consumed' ? 1 : 0),
    enemiesDestroyed: score.enemiesDestroyed + (event.type === 'enemy_destroyed' ? 1 : 0),
    timeSurvived: score.timeSurvived + (event.type === 'time_survived' ? 1 : 0),
    debrisCollected: score.debrisCollected + (event.type === 'debris_collected' ? 1 : 0),
    events: [...score.events, newEvent],
  };

  return updatedScore;
}

export function tickSurvivalScore(score: SessionScore): SessionScore {
  return recordScoreEvent(score, {
    type: 'time_survived',
    points: DEFAULT_SCORE_CONFIG.pointsPerSecond,
    details: 'Time survived bonus',
  });
}

export function consumeParticleScore(score: SessionScore, particleType: string): SessionScore {
  return recordScoreEvent(score, {
    type: 'particle_consumed',
    points: DEFAULT_SCORE_CONFIG.pointsPerParticle,
    details: `Consumed ${particleType}`,
  });
}

export function destroyEnemyScore(score: SessionScore, enemyType: string): SessionScore {
  return recordScoreEvent(score, {
    type: 'enemy_destroyed',
    points: DEFAULT_SCORE_CONFIG.pointsPerEnemy,
    details: `Destroyed ${enemyType}`,
  });
}

export function collectDebrisScore(score: SessionScore): SessionScore {
  return recordScoreEvent(score, {
    type: 'debris_collected',
    points: DEFAULT_SCORE_CONFIG.pointsPerDebris,
    details: 'Collected space debris',
  });
}

export function applyEfficiencyBonus(score: SessionScore, particleCount: number): SessionScore {
  if (particleCount >= DEFAULT_SCORE_CONFIG.efficiencyThreshold) {
    return recordScoreEvent(score, {
      type: 'efficiency_bonus',
      points: particleCount * DEFAULT_SCORE_CONFIG.pointsPerParticle,
      details: `Efficiency bonus for ${particleCount} particles`,
    });
  }
  return score;
}

// LocalStorage persistence
const HIGH_SCORES_KEY = 'stardust_high_scores';

export function getHighScores(): HighScore[] {
  try {
    const stored = localStorage.getItem(HIGH_SCORES_KEY);
    if (stored) {
      return JSON.parse(stored);
    }
  } catch (e) {
    console.error('Failed to load high scores:', e);
  }
  return [];
}

export function saveHighScore(score: SessionScore): { isNewHighScore: boolean; rank: number } {
  const highScores = getHighScores();
  const newScore: HighScore = {
    score: score.total,
    date: new Date().toISOString(),
    particlesConsumed: score.particlesConsumed,
    enemiesDestroyed: score.enemiesDestroyed,
    survivalTime: score.timeSurvived,
  };

  // Find position
  let rank = highScores.findIndex(hs => score.total > hs.score);
  if (rank === -1) rank = highScores.length;

  // Check if qualifies
  if (rank >= DEFAULT_SCORE_CONFIG.maxHighScores) {
    return { isNewHighScore: false, rank: -1 };
  }

  // Insert and sort
  highScores.splice(rank, 0, newScore);
  highScores.splice(DEFAULT_SCORE_CONFIG.maxHighScores);

  try {
    localStorage.setItem(HIGH_SCORES_KEY, JSON.stringify(highScores));
    return { isNewHighScore: true, rank: rank + 1 };
  } catch (e) {
    console.error('Failed to save high score:', e);
    return { isNewHighScore: false, rank: -1 };
  }
}

export function clearHighScores(): void {
  localStorage.removeItem(HIGH_SCORES_KEY);
}

export function getSessionStats(score: SessionScore): {
  total: number;
  particlesConsumed: number;
  enemiesDestroyed: number;
  survivalTime: number;
  pointsPerMinute: number;
} {
  const minutes = Math.max(1, score.timeSurvived / 60);
  return {
    total: score.total,
    particlesConsumed: score.particlesConsumed,
    enemiesDestroyed: score.enemiesDestroyed,
    survivalTime: score.timeSurvived,
    pointsPerMinute: Math.floor(score.total / minutes),
  };
}

export function formatSurvivalTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}

export function formatNumber(num: number): string {
  if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
  if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
  return num.toString();
}