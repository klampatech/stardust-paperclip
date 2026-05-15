// FUL-35c: Spacecraft Control & Game Mechanics
// Type definitions for spacecraft game objects

export interface SpacecraftProps {
  ship_class: ShipClass;
  hull: number;         // 0-100, ship destroyed at 0
  fuel: number;         // 0-100, thrust requires fuel
  shields: number;      // 0-100, absorbs damage first
  engine_power: number; // Thrust multiplier (0.5-2.0)
  cargo_capacity: number; // Max cargo units
  max_hull: number;
  max_fuel: number;
  max_shields: number;
}

export enum ShipClass {
  Scout = 'Scout',
  Fighter = 'Fighter',
  Freighter = 'Freighter',
  Cruiser = 'Cruiser',
  ColonyShip = 'ColonyShip',
  Station = 'Station',
}

export interface Spacecraft {
  id: string;
  position: { x: number; y: number };
  velocity: { x: number; y: number };
  angle: number;        // Rotation in radians
  props: SpacecraftProps;
  isPlayer: boolean;    // True if player-controlled
  isDestroyed: boolean;
}

// Ship class defaults for factory functions
export const SHIP_CLASS_DEFAULTS: Record<ShipClass, Omit<SpacecraftProps, 'ship_class'>> = {
  [ShipClass.Scout]: {
    hull: 50, fuel: 100, shields: 25,
    engine_power: 1.5, cargo_capacity: 5,
    max_hull: 50, max_fuel: 100, max_shields: 25,
  },
  [ShipClass.Fighter]: {
    hull: 75, fuel: 80, shields: 50,
    engine_power: 1.2, cargo_capacity: 2,
    max_hull: 75, max_fuel: 80, max_shields: 50,
  },
  [ShipClass.Freighter]: {
    hull: 150, fuel: 60, shields: 30,
    engine_power: 0.7, cargo_capacity: 50,
    max_hull: 150, max_fuel: 60, max_shields: 30,
  },
  [ShipClass.Cruiser]: {
    hull: 200, fuel: 50, shields: 100,
    engine_power: 0.5, cargo_capacity: 20,
    max_hull: 200, max_fuel: 50, max_shields: 100,
  },
  [ShipClass.ColonyShip]: {
    hull: 300, fuel: 40, shields: 20,
    engine_power: 0.3, cargo_capacity: 100,
    max_hull: 300, max_fuel: 40, max_shields: 20,
  },
  [ShipClass.Station]: {
    hull: 500, fuel: 0, shields: 200,
    engine_power: 0, cargo_capacity: 0,
    max_hull: 500, max_fuel: 0, max_shields: 200,
  },
};

// Create a new spacecraft with defaults for a ship class
export function createSpacecraft(
  id: string,
  position: { x: number; y: number },
  shipClass: ShipClass,
  isPlayer: boolean = false
): Spacecraft {
  const defaults = SHIP_CLASS_DEFAULTS[shipClass];
  return {
    id,
    position: { ...position },
    velocity: { x: 0, y: 0 },
    angle: -Math.PI / 2, // Pointing up initially
    props: {
      ship_class: shipClass,
      hull: defaults.hull,
      fuel: defaults.fuel,
      shields: defaults.shields,
      engine_power: defaults.engine_power,
      cargo_capacity: defaults.cargo_capacity,
      max_hull: defaults.max_hull,
      max_fuel: defaults.max_fuel,
      max_shields: defaults.max_shields,
    },
    isPlayer,
    isDestroyed: false,
  };
}

// Ship class display names and colors
export const SHIP_CLASS_INFO: Record<ShipClass, { name: string; color: string; icon: string }> = {
  [ShipClass.Scout]: { name: 'Scout', color: '#4CAF50', icon: '🚀' },
  [ShipClass.Fighter]: { name: 'Fighter', color: '#F44336', icon: '✈️' },
  [ShipClass.Freighter]: { name: 'Freighter', color: '#FF9800', icon: '🚛' },
  [ShipClass.Cruiser]: { name: 'Cruiser', color: '#9C27B0', icon: '🚀' },
  [ShipClass.ColonyShip]: { name: 'Colony Ship', color: '#2196F3', icon: '🏠' },
  [ShipClass.Station]: { name: 'Station', color: '#607D8B', icon: '🛸' },
};