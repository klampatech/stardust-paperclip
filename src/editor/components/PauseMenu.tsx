// FUL-48: Pause Menu Component

import React from 'react';

interface PauseMenuProps {
  onResume: () => void;
  onRestart: () => void;
  onShowControls: () => void;
  onMainMenu: () => void;
}

export default function PauseMenu({ onResume, onRestart, onShowControls, onMainMenu }: PauseMenuProps) {
  return (
    <div className="pause-menu-overlay">
      <div className="pause-menu">
        <h2 className="pause-title">PAUSED</h2>
        
        <div className="pause-buttons">
          <button className="pause-btn primary" onClick={onResume}>
            ▶ RESUME
          </button>
          <button className="pause-btn" onClick={onRestart}>
            ↻ RESTART
          </button>
          <button className="pause-btn" onClick={onShowControls}>
            ❓ CONTROLS
          </button>
          <button className="pause-btn" onClick={onMainMenu}>
            ⌂ MAIN MENU
          </button>
        </div>
      </div>

      <style>{`
        .pause-menu-overlay {
          position: absolute;
          inset: 0;
          background: rgba(10, 10, 26, 0.85);
          display: flex;
          align-items: center;
          justify-content: center;
          backdrop-filter: blur(8px);
          z-index: 100;
        }

        .pause-menu {
          background: linear-gradient(135deg, rgba(30, 30, 60, 0.95) 0%, rgba(20, 20, 40, 0.95) 100%);
          border: 2px solid #6366f1;
          border-radius: 16px;
          padding: 2.5rem 3rem;
          text-align: center;
          box-shadow: 0 0 40px rgba(99, 102, 241, 0.3);
        }

        .pause-title {
          font-family: 'Orbitron', 'Segoe UI', sans-serif;
          font-size: 2.5rem;
          font-weight: 700;
          color: #fff;
          margin: 0 0 2rem;
          letter-spacing: 0.2em;
          text-shadow: 0 0 20px #6366f1;
        }

        .pause-buttons {
          display: flex;
          flex-direction: column;
          gap: 0.75rem;
        }

        .pause-btn {
          font-family: 'Orbitron', 'Segoe UI', sans-serif;
          font-size: 1rem;
          font-weight: 600;
          padding: 0.875rem 2.5rem;
          min-width: 220px;
          border: 1px solid #6366f1;
          background: rgba(99, 102, 241, 0.1);
          color: #fff;
          border-radius: 8px;
          cursor: pointer;
          transition: all 0.2s ease;
          text-transform: uppercase;
          letter-spacing: 0.1em;
        }

        .pause-btn:hover {
          background: rgba(99, 102, 241, 0.3);
          border-color: #818cf8;
          transform: scale(1.03);
        }

        .pause-btn.primary {
          background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
          border-color: transparent;
        }

        .pause-btn.primary:hover {
          background: linear-gradient(135deg, #818cf8 0%, #a78bfa 100%);
          box-shadow: 0 0 20px rgba(99, 102, 241, 0.5);
        }
      `}</style>
    </div>
  );
}