// FUL-45: Upgrade Menu Component
// Full upgrade panel for spacecraft progression

import React from 'react';
import { AllUpgrades, UPGRADE_DEFINITIONS, getUpgradeSummary, canPurchaseUpgrade, purchaseUpgrade } from '../upgrades';
import { formatNumber } from '../scoring';

interface UpgradeMenuProps {
  upgrades: AllUpgrades;
  availablePoints: number;
  onPurchase: (upgradeType: string) => void;
  onClose: () => void;
}

export default function UpgradeMenu({
  upgrades,
  availablePoints,
  onPurchase,
  onClose,
}: UpgradeMenuProps) {
  const summary = getUpgradeSummary(upgrades);

  return (
    <div className="upgrade-menu-overlay" onClick={onClose}>
      <div className="upgrade-menu" onClick={e => e.stopPropagation()}>
        <div className="upgrade-menu-header">
          <h2>⚙️ Ship Upgrades</h2>
          <button className="upgrade-menu-close" onClick={onClose}>×</button>
        </div>

        <div className="upgrade-points-display">
          <span className="points-label">Available Points:</span>
          <span className="points-value">{formatNumber(availablePoints)}</span>
        </div>

        <div className="upgrade-grid">
          {summary.map(({ definition, state, nextCost, nextEffect }) => {
            const check = canPurchaseUpgrade(definition.id, upgrades, availablePoints);
            const isMaxed = state.level >= definition.maxLevel;

            return (
              <div key={definition.id} className={`upgrade-card ${isMaxed ? 'maxed' : ''} ${!check.canBuy && !isMaxed ? 'disabled' : ''}`}>
                <div className="upgrade-card-header">
                  <span className="upgrade-icon">{definition.icon}</span>
                  <span className="upgrade-name">{definition.name}</span>
                </div>

                <div className="upgrade-card-body">
                  <p className="upgrade-description">{definition.description}</p>
                  
                  <div className="upgrade-level-display">
                    <span className="level-label">Level</span>
                    <div className="level-pips">
                      {Array.from({ length: definition.maxLevel }).map((_, i) => (
                        <span 
                          key={i} 
                          className={`level-pip ${i < state.level ? 'filled' : ''}`}
                        />
                      ))}
                    </div>
                    <span className="level-value">{state.level}/{definition.maxLevel}</span>
                  </div>

                  {isMaxed ? (
                    <div className="upgrade-maxed">MAX LEVEL</div>
                  ) : (
                    <>
                      <div className="upgrade-effect">
                        <span className="effect-current">
                          Current: +{definition.effectPerLevel * state.level} {definition.effectUnit}
                        </span>
                        <span className="effect-next">
                          Next: +{nextEffect} {definition.effectUnit}
                        </span>
                      </div>

                      <button 
                        className={`upgrade-purchase-btn ${!check.canBuy ? 'cant-afford' : ''}`}
                        onClick={() => onPurchase(definition.id)}
                        disabled={!check.canBuy}
                      >
                        <span className="purchase-cost">{formatNumber(nextCost!)} pts</span>
                        {!check.canBuy && <span className="purchase-reason">{check.reason}</span>}
                      </button>
                    </>
                  )}
                </div>
              </div>
            );
          })}
        </div>

        <div className="upgrade-menu-footer">
          <p className="upgrade-tip">
            💡 Tips: Earn points by feeding particles to the black hole. More particles consumed = more points!
          </p>
        </div>
      </div>
    </div>
  );
}