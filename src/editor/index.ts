// FUL-48: Editor Module Exports
// Game-only exports - sandbox mode removed

export { default as App } from './App';
export { createSessionScore, consumeParticleScore, tickSurvivalScore, destroyEnemyScore, 
         loadHighScores, saveHighScore, formatNumber, formatSurvivalTime } from './scoring';
export { createDefaultUpgrades, loadUpgrades, saveUpgrades, purchaseUpgrade, 
         getGravityGunStats, getUpgradeSummary, UPGRADE_DEFINITIONS, AllUpgrades } from './upgrades';