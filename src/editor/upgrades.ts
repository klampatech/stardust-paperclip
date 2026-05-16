// FUL-45: Upgrade System
// Spacecraft upgrades with persistence

import type { ShipClass, Spacecraft } from './spacecraft';

export type UpgradeType = 
  | 'hull_plating'
  | 'thruster_power'
  | 'gun_range'
  | 'gun_power'
  | 'shield_capacitor'
  | 'cargo_bay';

export interface UpgradeDefinition {
  id: UpgradeType;
  name: string;
  description: string;
  icon: string;
  baseCost: number;
  costMultiplier: number;
  maxLevel: number;
  effectPerLevel: number;
  effectUnit: string;
}

export interface UpgradeState {
  level: number;
  purchased: number; // Timestamp of last purchase
}

export interface AllUpgrades {
  hull_plating: UpgradeState;
  thruster_power: UpgradeState;
  gun_range: UpgradeState;
  gun_power: UpgradeState;
  shield_capacitor: UpgradeState;
  cargo_bay: UpgradeState;
}

export const UPGRADE_DEFINITIONS: Record<UpgradeType, UpgradeDefinition> = {
  hull_plating: {
    id: 'hull_plating',
    name: 'Hull Plating',
    description: 'Reinforced armor plating. Increases maximum HP.',
    icon: '🛡️',
    baseCost: 200,
    costMultiplier: 1.5,
    maxLevel: 5,
    effectPerLevel: 20,
    effectUnit: 'HP',
  },
  thruster_power: {
    id: 'thruster_power',
    name: 'Thruster Power',
    description: 'More powerful engines. Increases ship speed.',
    icon: '🚀',
    baseCost: 300,
    costMultiplier: 1.5,
    maxLevel: 5,
    effectPerLevel: 15,
    effectUnit: '% speed',
  },
  gun_range: {
    id: 'gun_range',
    name: 'Gravity Gun Range',
    description: 'Extended sensor array. Increases gravity gun radius.',
    icon: '📡',
    baseCost: 250,
    costMultiplier: 1.4,
    maxLevel: 5,
    effectPerLevel: 25,
    effectUnit: '% radius',
  },
  gun_power: {
    id: 'gun_power',
    name: 'Gravity Gun Power',
    description: 'Enhanced force emitters. Increases pull/push strength.',
    icon: '⚡',
    baseCost: 250,
    costMultiplier: 1.4,
    maxLevel: 5,
    effectPerLevel: 25,
    effectUnit: '% strength',
  },
  shield_capacitor: {
    id: 'shield_capacitor',
    name: 'Shield Capacitor',
    description: 'Larger shield generators. Increases maximum shields.',
    icon: '🔮',
    baseCost: 400,
    costMultiplier: 1.6,
    maxLevel: 3,
    effectPerLevel: 30,
    effectUnit: 'shields',
  },
  cargo_bay: {
    id: 'cargo_bay',
    name: 'Cargo Bay',
    description: 'Expanded cargo storage. Increases carry capacity.',
    icon: '📦',
    baseCost: 350,
    costMultiplier: 1.5,
    maxLevel: 3,
    effectPerLevel: 50,
    effectUnit: 'capacity',
  },
};

export function createDefaultUpgrades(): AllUpgrades {
  const defaults: AllUpgrades = {} as AllUpgrades;
  for (const key of Object.keys(UPGRADE_DEFINITIONS) as UpgradeType[]) {
    defaults[key] = { level: 0, purchased: 0 };
  }
  return defaults;
}

export function getUpgradeCost(upgradeType: UpgradeType, currentLevel: number): number {
  const def = UPGRADE_DEFINITIONS[upgradeType];
  return Math.floor(def.baseCost * Math.pow(def.costMultiplier, currentLevel));
}

export function getUpgradeEffect(upgradeType: UpgradeType, level: number): number {
  const def = UPGRADE_DEFINITIONS[upgradeType];
  return def.effectPerLevel * level;
}

export function canPurchaseUpgrade(
  upgradeType: UpgradeType,
  upgrades: AllUpgrades,
  availablePoints: number
): { canBuy: boolean; reason?: string } {
  const currentLevel = upgrades[upgradeType].level;
  const def = UPGRADE_DEFINITIONS[upgradeType];

  if (currentLevel >= def.maxLevel) {
    return { canBuy: false, reason: 'Maximum level reached' };
  }

  const cost = getUpgradeCost(upgradeType, currentLevel);
  if (availablePoints < cost) {
    return { canBuy: false, reason: `Need ${cost} points (have ${availablePoints})` };
  }

  return { canBuy: true };
}

export function purchaseUpgrade(
  upgradeType: UpgradeType,
  upgrades: AllUpgrades,
  points: number
): { success: boolean; newUpgrades: AllUpgrades; newPoints: number; message: string } {
  const check = canPurchaseUpgrade(upgradeType, upgrades, points);
  if (!check.canBuy) {
    return { success: false, newUpgrades: upgrades, newPoints: points, message: check.reason! };
  }

  const cost = getUpgradeCost(upgradeType, upgrades[upgradeType].level);
  const newUpgrades = { ...upgrades };
  newUpgrades[upgradeType] = {
    ...newUpgrades[upgradeType],
    level: newUpgrades[upgradeType].level + 1,
    purchased: Date.now(),
  };

  return {
    success: true,
    newUpgrades,
    newPoints: points - cost,
    message: `Purchased ${UPGRADE_DEFINITIONS[upgradeType].name} Level ${newUpgrades[upgradeType].level}`,
  };
}

// Apply upgrade effects to ship stats
export function applyUpgradesToShip(ship: Spacecraft, upgrades: AllUpgrades): Spacecraft {
  const updatedShip = { ...ship };

  // Hull plating: +20 HP per level
  updatedShip.props.maxHull += getUpgradeEffect('hull_plating', upgrades.hull_plating.level);
  updatedShip.props.hull = Math.min(ship.props.hull, updatedShip.props.maxHull);

  // Thruster power: +15% speed per level
  const speedMultiplier = 1 + (getUpgradeEffect('thruster_power', upgrades.thruster_power.level) / 100);
  updatedShip.props.maxSpeed *= speedMultiplier;

  // Shield capacitor: +30 shields per level
  updatedShip.props.maxShields += getUpgradeEffect('shield_capacitor', upgrades.shield_capacitor.level);
  updatedShip.props.shields = Math.min(ship.props.shields, updatedShip.props.maxShields);

  // Cargo bay: +50 capacity per level
  updatedShip.props.cargoCapacity += getUpgradeEffect('cargo_bay', upgrades.cargo_bay.level);

  return updatedShip;
}

// Get gravity gun stats from upgrades
export function getGravityGunStats(upgrades: AllUpgrades): { radius: number; strength: number } {
  return {
    radius: 80 * (1 + getUpgradeEffect('gun_range', upgrades.gun_range.level) / 100),
    strength: 5 * (1 + getUpgradeEffect('gun_power', upgrades.gun_power.level) / 100),
  };
}

// LocalStorage persistence
const UPGRADES_KEY = 'stardust_upgrades';
const POINTS_KEY = 'stardust_points';

export function saveUpgrades(upgrades: AllUpgrades, points: number): void {
  try {
    localStorage.setItem(UPGRADES_KEY, JSON.stringify(upgrades));
    localStorage.setItem(POINTS_KEY, points.toString());
  } catch (e) {
    console.error('Failed to save upgrades:', e);
  }
}

export function loadUpgrades(): { upgrades: AllUpgrades; points: number } {
  try {
    const upgradesStr = localStorage.getItem(UPGRADES_KEY);
    const pointsStr = localStorage.getItem(POINTS_KEY);
    
    if (upgradesStr) {
      return {
        upgrades: JSON.parse(upgradesStr),
        points: pointsStr ? parseInt(pointsStr, 10) : 0,
      };
    }
  } catch (e) {
    console.error('Failed to load upgrades:', e);
  }
  return { upgrades: createDefaultUpgrades(), points: 0 };
}

export function resetUpgrades(): void {
  localStorage.removeItem(UPGRADES_KEY);
  localStorage.removeItem(POINTS_KEY);
}

// Get total upgrade investment for display
export function getTotalInvestment(upgrades: AllUpgrades): number {
  let total = 0;
  for (const key of Object.keys(upgrades) as UpgradeType[]) {
    for (let lvl = 0; lvl < upgrades[key].level; lvl++) {
      total += getUpgradeCost(key, lvl);
    }
  }
  return total;
}

// Get upgrade summary for UI
export function getUpgradeSummary(upgrades: AllUpgrades): Array<{
  definition: UpgradeDefinition;
  state: UpgradeState;
  nextCost: number | null;
  nextEffect: number | null;
}> {
  return Object.keys(UPGRADE_DEFINITIONS).map(key => {
    const def = UPGRADE_DEFINITIONS[key as UpgradeType];
    const state = upgrades[key as UpgradeType];
    return {
      definition: def,
      state,
      nextCost: state.level < def.maxLevel ? getUpgradeCost(key as UpgradeType, state.level) : null,
      nextEffect: state.level < def.maxLevel ? getUpgradeEffect(key as UpgradeType, state.level + 1) : null,
    };
  });
}