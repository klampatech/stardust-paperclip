// FUL-48: Title Screen Component

import React, { useState, useEffect } from 'react';

interface TitleScreenProps {
  onPlay: () => void;
  onShowControls: () => void;
}

// Animated starfield for title background
function Starfield() {
  const stars = React.useMemo(() => {
    return Array.from({ length: 100 }, (_, i) => ({
      id: i,
      x: Math.random() * 100,
      y: Math.random() * 100,
      size: Math.random() * 2 + 1,
      opacity: Math.random() * 0.5 + 0.3,
      twinkleSpeed: Math.random() * 2000 + 1000,
    }));
  }, []);

  return (
    <div className="title-starfield">
      {stars.map(star => (
        <div
          key={star.id}
          className="title-star"
          style={{
            left: `${star.x}%`,
            top: `${star.y}%`,
            width: `${star.size}px`,
            height: `${star.size}px`,
            opacity: star.opacity,
            animation: `twinkle ${star.twinkleSpeed}ms ease-in-out infinite`,
          }}
        />
      ))}
    </div>
  );
}

export default function TitleScreen({ onPlay, onShowControls }: TitleScreenProps) {
  const [showTitle, setShowTitle] = useState(false);

  useEffect(() => {
    // Stagger title entrance
    const timer = setTimeout(() => setShowTitle(true), 100);
    return () => clearTimeout(timer);
  }, []);

  return (
    <div className="title-screen">
      <Starfield />
      
      <div className={`title-content ${showTitle ? 'visible' : ''}`}>
        <h1 className="title-logo">STARDUST</h1>
        <p className="title-tagline">A Space Black Hole Game</p>
        
        <div className="title-buttons">
          <button className="title-btn primary" onClick={onPlay}>
            ▶ PLAY
          </button>
          <button className="title-btn" onClick={onShowControls}>
            ❓ CONTROLS
          </button>
        </div>
      </div>

      <style>{`
        .title-screen {
          position: absolute;
          inset: 0;
          background: linear-gradient(135deg, #0a0a1a 0%, #1a0a2a 50%, #0a1a2a 100%);
          display: flex;
          align-items: center;
          justify-content: center;
          overflow: hidden;
        }

        .title-starfield {
          position: absolute;
          inset: 0;
          pointer-events: none;
        }

        .title-star {
          position: absolute;
          background: white;
          border-radius: 50%;
        }

        @keyframes twinkle {
          0%, 100% { opacity: 0.3; transform: scale(1); }
          50% { opacity: 1; transform: scale(1.2); }
        }

        .title-content {
          position: relative;
          z-index: 1;
          text-align: center;
          opacity: 0;
          transform: translateY(20px);
          transition: opacity 0.8s ease-out, transform 0.8s ease-out;
        }

        .title-content.visible {
          opacity: 1;
          transform: translateY(0);
        }

        .title-logo {
          font-family: 'Orbitron', 'Segoe UI', sans-serif;
          font-size: 5rem;
          font-weight: 900;
          color: #fff;
          text-shadow: 
            0 0 10px #ec4899,
            0 0 20px #ec4899,
            0 0 40px #ec4899,
            0 0 80px #ec4899;
          margin: 0;
          letter-spacing: 0.2em;
        }

        .title-tagline {
          font-family: 'Segoe UI', sans-serif;
          font-size: 1.2rem;
          color: #a78bfa;
          margin: 1rem 0 3rem;
          letter-spacing: 0.3em;
          text-transform: uppercase;
        }

        .title-buttons {
          display: flex;
          flex-direction: column;
          gap: 1rem;
          align-items: center;
        }

        .title-btn {
          font-family: 'Orbitron', 'Segoe UI', sans-serif;
          font-size: 1.2rem;
          font-weight: 600;
          padding: 1rem 3rem;
          min-width: 200px;
          border: 2px solid #6366f1;
          background: rgba(99, 102, 241, 0.1);
          color: #fff;
          border-radius: 8px;
          cursor: pointer;
          transition: all 0.3s ease;
          text-transform: uppercase;
          letter-spacing: 0.1em;
        }

        .title-btn:hover {
          background: rgba(99, 102, 241, 0.3);
          border-color: #818cf8;
          transform: scale(1.05);
          box-shadow: 0 0 20px rgba(99, 102, 241, 0.5);
        }

        .title-btn.primary {
          background: linear-gradient(135deg, #ec4899 0%, #6366f1 100%);
          border-color: transparent;
        }

        .title-btn.primary:hover {
          background: linear-gradient(135deg, #f472b6 0%, #818cf8 100%);
          box-shadow: 0 0 30px rgba(236, 72, 153, 0.6);
        }
      `}</style>
    </div>
  );
}