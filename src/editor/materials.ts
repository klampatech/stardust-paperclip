// FUL-5: Phase 6 - Material Types
// These match the Rust implementation in src/particle.rs

export enum Material {
  Air = 0,
  Sand = 1,
  Water = 2,
  Stone = 3,
  Fire = 4,
  Smoke = 5,
  BlackHole = 6,
  Steam = 7,
  Ice = 8,
  Oil = 9,
  Wood = 10,
  Lava = 11,
  Ash = 12,
  Eraser = 13, // Special: clears particles
}

export interface MaterialInfo {
  id: Material;
  name: string;
  color: string;
  key: string;
}

export const MATERIALS: MaterialInfo[] = [
  { id: Material.Sand, name: 'Sand', color: '#C2B280', key: '1' },
  { id: Material.Water, name: 'Water', color: '#40A4DF', key: '2' },
  { id: Material.Stone, name: 'Stone', color: '#808080', key: '3' },
  { id: Material.Fire, name: 'Fire', color: '#FF6432', key: '4' },
  { id: Material.Smoke, name: 'Smoke', color: '#64646E', key: '5' },
  { id: Material.BlackHole, name: 'Black Hole', color: '#000000', key: '6' },
  { id: Material.Steam, name: 'Steam', color: '#C8C8FF', key: '7' },
  { id: Material.Ice, name: 'Ice', color: '#ADD8FA', key: '8' },
  { id: Material.Oil, name: 'Oil', color: '#654321', key: '9' },
  { id: Material.Wood, name: 'Wood', color: '#8B5A2B', key: '0' },
  { id: Material.Lava, name: 'Lava', color: '#FF4500', key: 'Q' },
  { id: Material.Ash, name: 'Ash', color: '#323237', key: 'W' },
  { id: Material.Eraser, name: 'Eraser', color: '#14141E', key: 'E' },
];

export function getMaterialByKey(key: string): MaterialInfo | undefined {
  return MATERIALS.find(m => m.key.toLowerCase() === key.toLowerCase());
}

export function getMaterialById(id: Material): MaterialInfo | undefined {
  return MATERIALS.find(m => m.id === id);
}

// WASM material name mapping (TypeScript enum → Rust string)
export const MATERIAL_NAMES: Record<Material, string> = {
  [Material.Air]: 'air',
  [Material.Sand]: 'sand',
  [Material.Water]: 'water',
  [Material.Stone]: 'stone',
  [Material.Fire]: 'fire',
  [Material.Smoke]: 'smoke',
  [Material.BlackHole]: 'blackhole',
  [Material.Steam]: 'steam',
  [Material.Ice]: 'ice',
  [Material.Oil]: 'oil',
  [Material.Wood]: 'wood',
  [Material.Lava]: 'lava',
  [Material.Ash]: 'ash',
  [Material.Eraser]: 'eraser',
};