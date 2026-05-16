// FUL-48: Game Over Screen Component

import React from 'react';
import { SessionScore } from '../scoring';

interface GameOverScreenProps {
  score: SessionScore;
  onPlayAgain: () => void;
  onMainMenu: () => void;
}

export default function GameOverScreen({ score, onPlayAgain, onMainMenu }: GameOverScreenProps) {
  const formatTime = (ms: number) => {
    const seconds = Math.floor(ms / 1000);
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;
    return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
  };

  return (
    <div className="gameover-screen">
      <div className="gameover-content">
        <h1 className="gameover-title">GAME OVER</h1>
        
        <div className="gameover-stats">
          <div className="stat-row">
            <span className="stat-label">FINAL SCORE</span>
            <span className="stat-value score">{score.totalScore.toLocaleString()}</span>
          </div>
          <div className="stat-divider" />
          <div className="stat-row">
            <span className="stat-label">SURVIVAL TIME</span>
            <span className="stat-value">{formatTime(score.survivalTime)}</span>
          </div>
          <div className="stat-row">
            <span className="stat-label">OBJECTS CONSUMED</span>
            <span className="stat-value">{score.particlesConsumed.toLocaleString()}</span>
          </div>
          <div className="stat-row">
            <span className="stat-label">ENEMIES DESTROYED</span>
            <span className="stat-value">{score.enemiesDestroyed}</span>
          </div>
          <div className="stat-row">
            <span className="stat-label">HIGH SCORE</span>
            <span className="stat-value highscore">
              {score.highScore.toLocaleString()}
              {score.totalScore > score.highScore && <span className="new-record"> NEW!</span>}
            </span>
          </div>
        </div>
        
        <div className="gameover-buttons">
          <button className="gameover-btn primary" onClick={onPlayAgain}>
            ▶ PLAY AGAIN
          </button>
          <button className="gameover-btn" onClick={onMainMenu}>
            ⌂ MAIN MENU
          </button>
        </div>
      </div>

      <style>{`
        .gameover-screen {
          position: absolute;
          inset: 0;
          background: linear-gradient(135deg, rgba(20, 10, 30, 0.95) 0%, rgba(10, 10, 26, 0.98) 100%);
          display: flex;
          align-items: center;
          justify-content: center;
          overflow: hidden;
        }

        .gameover-content {
          text-align: center;
          z-index: 1;
        }

        .gameover-title {
          font-family: 'Orbitron', 'Segoe UI', sans-serif;
          font-size: 4rem;
          font-weight: 900;
          color: #ef4444;
          margin: 0 0 2rem;
          letter-spacing: 0.15em;
          text-shadow: 
            0 0 10px #ef4444,
            0 0 30px #ef4444,
            0 0 60px #ef4444;
          animation: pulse-red 2s ease-in-out infinite;
        }

        @keyframes pulse-red {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.7; }
        }

        .gameover-stats {
          background: rgba(30, 30, 60, 0.8);
          border: 1px solid #6366f1;
          border-radius: 12px;
          padding: 1.5rem 2.5rem;
          margin-bottom: 2rem;
          min-width: 320px;
        }

        .stat-row {
          display: flex;
          justify-content: space-between;
          align-items: center;
          padding: 0.5rem 0;
        }

        .stat-label {
          font-family: 'Orbitron', 'Segoe UI', sans-serif;
          font-size: 0.85rem;
          color: #a78bfa;
          letter-spacing: 0.1em;
        }

        .stat-value {
          font-family: 'Orbitron', 'Segoe UI', sans-serif;
          font-size: 1.1rem;
          font-weight: 700;
          color: #fff;
        }

        .stat-value.score {
          font-size: 1.8rem;
          color: #fbbf24;
          text-shadow: 0 0 10px rgba(251, 191, 36, 0.5);
        }

        .stat-value.highscore {
          color: #a78bfa;
        }

        .new-record {
          color: #22c55e;
          animation: bounce 0.5s ease infinite alternate;
        }

        @keyframes bounce {
          from { transform: scale(1); }
          to { transform: scale(1.1); }
        }

        .stat-divider {
          height: 1px;
          background: linear-gradient(90deg, transparent, #6366f1, transparent);
          margin: 0.5rem 0;
        }

        .gameover-buttons {
          display: flex;
          flex-direction: column;
          gap: 0.75rem;
          align-items: center;
        }

        .gameover-btn {
          font-family: 'Orbitron', 'Segoe UI', sans-serif;
          font-size: 1.1rem;
          font-weight: 600;
          padding: 1rem 3rem;
          min-width: 240px;
          border: 1px solid #6366f1;
          background: rgba(99, 102, 241, 0.1);
          color: #fff;
          border-radius: 8px;
          cursor: pointer;
          transition: all 0.2s ease;
          text-transform: uppercase;
          letter-spacing: 0.1em;
        }

        .gameover-btn:hover {
          background: rgba(99, 102, 241, 0.3);
          border-color: #818cf8;
          transform: scale(1.03);
        }

        .gameover-btn.primary {
          background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
          border-color: transparent;
        }

        .gameover-btn.primary:hover {
          background: linear-gradient(135deg, #818cf8 0%, #a78bfa 100%);
          box-shadow: 0 0 30px rgba(99, 102, 241, 0.6);
        }
      `}</style>
    </div>
  );
}