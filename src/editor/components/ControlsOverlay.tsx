// FUL-48: Controls Overlay Component

import React from 'react';

interface ControlsOverlayProps {
  onClose: () => void;
}

const CONTROLS = [
  { key: 'WASD', action: 'Move Ship' },
  { key: 'LMB', action: 'Attract (Gravity Gun)' },
  { key: 'RMB', action: 'Repel (Gravity Gun)' },
  { key: 'MMB', action: 'Vortex (Gravity Gun)' },
  { key: 'U', action: 'Upgrades Menu' },
  { key: 'ESC', action: 'Pause / Menu' },
];

export default function ControlsOverlay({ onClose }: ControlsOverlayProps) {
  return (
    <div className="controls-overlay" onClick={onClose}>
      <div className="controls-panel" onClick={(e) => e.stopPropagation()}>
        <button className="controls-close" onClick={onClose}>✕</button>
        
        <h2 className="controls-title">CONTROLS</h2>
        
        <div className="controls-list">
          {CONTROLS.map(({ key, action }) => (
            <div key={key} className="control-row">
              <span className="control-key">{key}</span>
              <span className="control-action">{action}</span>
            </div>
          ))}
        </div>

        <div className="controls-footer">
          <p className="controls-tip">Use the gravity gun to pull objects toward the black hole and earn points!</p>
        </div>
      </div>

      <style>{`
        .controls-overlay {
          position: absolute;
          inset: 0;
          background: rgba(10, 10, 26, 0.9);
          display: flex;
          align-items: center;
          justify-content: center;
          z-index: 200;
          backdrop-filter: blur(4px);
        }

        .controls-panel {
          position: relative;
          background: linear-gradient(135deg, rgba(30, 30, 60, 0.98) 0%, rgba(20, 20, 45, 0.98) 100%);
          border: 2px solid #6366f1;
          border-radius: 16px;
          padding: 2rem 3rem;
          min-width: 380px;
          box-shadow: 0 0 50px rgba(99, 102, 241, 0.4);
        }

        .controls-close {
          position: absolute;
          top: 1rem;
          right: 1rem;
          width: 32px;
          height: 32px;
          border: none;
          background: rgba(239, 68, 68, 0.2);
          border-radius: 50%;
          color: #ef4444;
          font-size: 1.2rem;
          cursor: pointer;
          transition: all 0.2s ease;
          display: flex;
          align-items: center;
          justify-content: center;
        }

        .controls-close:hover {
          background: rgba(239, 68, 68, 0.4);
          transform: scale(1.1);
        }

        .controls-title {
          font-family: 'Orbitron', 'Segoe UI', sans-serif;
          font-size: 2rem;
          font-weight: 700;
          color: #fff;
          margin: 0 0 2rem;
          text-align: center;
          letter-spacing: 0.15em;
          text-shadow: 0 0 20px #6366f1;
        }

        .controls-list {
          display: flex;
          flex-direction: column;
          gap: 0.75rem;
        }

        .control-row {
          display: flex;
          align-items: center;
          gap: 1.5rem;
          padding: 0.5rem 0;
        }

        .control-key {
          font-family: 'Orbitron', 'Segoe UI', sans-serif;
          font-size: 0.9rem;
          font-weight: 700;
          color: #fbbf24;
          background: rgba(251, 191, 36, 0.15);
          border: 1px solid rgba(251, 191, 36, 0.4);
          border-radius: 6px;
          padding: 0.4rem 0.8rem;
          min-width: 80px;
          text-align: center;
          letter-spacing: 0.05em;
        }

        .control-action {
          font-family: 'Segoe UI', sans-serif;
          font-size: 1rem;
          color: #e2e8f0;
          flex: 1;
        }

        .controls-footer {
          margin-top: 2rem;
          padding-top: 1.5rem;
          border-top: 1px solid rgba(99, 102, 241, 0.3);
        }

        .controls-tip {
          font-family: 'Segoe UI', sans-serif;
          font-size: 0.9rem;
          color: #a78bfa;
          text-align: center;
          margin: 0;
          font-style: italic;
        }
      `}</style>
    </div>
  );
}