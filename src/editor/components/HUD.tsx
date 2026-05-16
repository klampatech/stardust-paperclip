// FUL-45: HUD Component
// Heads-up display for game stats, score, and player info

import React from 'react';
import { SessionScore, getSessionStats, formatSurvivalTime, formatNumber } from '../scoring';
import { AllUpgrades, getUpgradeSummary, UPGRADE_DEFINITIONS } from '../upgrades';
import type { GravityGunMode } from '../gravityGun';

interface HUDProps {
  score: SessionScore;
  upgrades: AllUpgrades;
  availablePoints: number;
  gravityGunMode: GravityGunMode;
  gravityGunActive: boolean;
  showUpgradeMenu: () => void;
  compact?: boolean;
}

export default function HUD({
  score,
  upgrades,
  availablePoints,
  gravityGunMode,
  gravityGunActive,
  showUpgradeMenu,
  compact = false,
}: HUDProps) {
  const stats = getSessionStats(score);
  const upgradeSummary = getUpgradeSummary(upgrades);

  if (compact) {
    return (
      <div className="hud-compact">
        <div className="hud-score-compact">
          <span className="score-label">SCORE</span>
          <span className="score-value">{formatNumber(stats.total)}</span>
        </div>
        <div className="hud-points-compact">
          <span className="points-label">PTS</span>
          <span className="points-value">{formatNumber(availablePoints)}</span>
        </div>
        <button className="hud-upgrade-btn" onClick={showUpgradeMenu} title="Upgrades [U]">
          ⬆️
        </button>
      </div>
    );
  }

  return (
    <div className="hud-overlay">
      <div className="hud-panel hud-top-left">
        <div className="hud-section">
          <div className="hud-label">SCORE</div>
          <div className="hud-value-large">{formatNumber(stats.total)}</div>
        </div>
        
        <div className="hud-section">
          <div className="hud-label">AVAILABLE POINTS</div>
          <div className="hud-value">{formatNumber(availablePoints)}</div>
        </div>

        <div className="hud-divider" />

        <div className="hud-section hud-stats">
          <div className="hud-stat">
            <span className="stat-label">Particles</span>
            <span className="stat-value">{formatNumber(stats.particlesConsumed)}</span>
          </div>
          <div className="hud-stat">
            <span className="stat-label">Enemies</span>
            <span className="stat-value">{stats.enemiesDestroyed}</span>
          </div>
          <div className="hud-stat">
            <span className="stat-label">Survived</span>
            <span className="stat-value">{formatSurvivalTime(stats.survivalTime)}</span>
          </div>
          <div className="hud-stat">
            <span className="stat-label">PPM</span>
            <span className="stat-value">{stats.pointsPerMinute}</span>
          </div>
        </div>
      </div>

      <div className="hud-panel hud-top-right">
        <div className="hud-section">
          <div className="hud-label">GRAVITY GUN</div>
          <div className="hud-gun-status">
            <span className={`gun-mode ${gravityGunMode}`}>
              {gravityGunMode === 'attract' ? '⟲ ATTRACT' : 
               gravityGunMode === 'repel' ? '⟳ REPEL' : '🌀 VORTEX'}
            </span>
            <span className={`gun-active ${gravityGunActive ? 'active' : ''}`}>
              {gravityGunActive ? '● ACTIVE' : '○ READY'}
            </span>
          </div>
        </div>

        <div className="hud-section hud-upgrades-preview">
          <div className="hud-label">UPGRADES</div>
          <div className="upgrade-bars">
            {upgradeSummary.slice(0, 4).map(({ definition, state }) => (
              <div key={definition.id} className="upgrade-bar">
                <span className="upgrade-icon">{definition.icon}</span>
                <div className="upgrade-level-bar">
                  <div 
                    className="upgrade-level-fill" 
                    style={{ width: `${(state.level / definition.maxLevel) * 100}%` }}
                  />
                </div>
                <span className="upgrade-level-text">{state.level}/{definition.maxLevel}</span>
              </div>
            ))}
          </div>
          <button className="hud-view-upgrades" onClick={showUpgradeMenu}>
            View All [U]
          </button>
        </div>
      </div>

      <div className="hud-panel hud-bottom">
        <div className="hud-controls-hint">
          <span className="hint-key">LMB</span> Attract
          <span className="hint-key">RMB</span> Repel
          <span className="hint-key">MMB</span> Vortex
          <span className="hint-key">U</span> Upgrades
          <span className="hint-key">ESC</span> Menu
        </div>
      </div>
    </div>
  );
}

// Simple mini HUD for strip sandbox mode
export function MiniHUD({ score, time }: { score: number; time: string }) {
  return (
    <div className="mini-hud">
      <div className="mini-score">
        <span className="mini-label">SCORE</span>
        <span className="mini-value">{formatNumber(score)}</span>
      </div>
      <div className="mini-time">
        <span className="mini-label">TIME</span>
        <span className="mini-value">{time}</span>
      </div>
    </div>
  );
}