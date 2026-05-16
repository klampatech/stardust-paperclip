// FUL-47.2: Object Spawning System
// Debris collection, periodic spawning, and collectible objects
// Integrates with scoring.ts for point tracking

import { Material } from './materials';

export interface DebrisObject {
  id: string;
  x: number;
  y: number;
  size: number; // 1=small, 2=medium, 3=large
  material: 'sand' | 'stone' | 'ice';
  rotation: number;
  rotationSpeed: number;
  velocityX: number;
  velocityY: number;
  collected: boolean;
  spawnTime: number;
}

export interface SpawnConfig {
  enabled: boolean;
  intervalMs: number;
  maxActiveDebris: number;
  spawnAtEdge: boolean; // Spawn at screen edges vs random
  types: DebrisType[];
}

export interface DebrisType {
  name: string;
  material: 'sand' | 'stone' | 'ice';
  sizeRange: [number, number];
  weight: number; // Spawn probability weight
  points: number;
  collectRadius: number;
}

export const DEFAULT_DEBRIS_TYPES: DebrisType[] = [
  { name: 'Sand Chunk', material: 'sand', sizeRange: [1, 2], weight: 40, points: 10, collectRadius: 20 },
  { name: 'Rock Fragment', material: 'stone', sizeRange: [2, 3], weight: 30, points: 15, collectRadius: 25 },
  { name: 'Ice Crystal', material: 'ice', sizeRange: [1, 2], weight: 30, points: 12, collectRadius: 18 },
];

export const DEFAULT_SPAWN_CONFIG: SpawnConfig = {
  enabled: true,
  intervalMs: 3000,
  maxActiveDebris: 15,
  spawnAtEdge: true,
  types: DEFAULT_DEBRIS_TYPES,
};

let debrisIdCounter = 0;

function generateDebrisId(): string {
  return `debris_${++debrisIdCounter}_${Date.now()}`;
}

export class DebrisManager {
  private debris: DebrisObject[] = [];
  private config: SpawnConfig;
  private lastSpawnTime: number = 0;
  private canvasWidth: number;
  private canvasHeight: number;
  
  // Callbacks for scoring
  public onDebrisCollected?: (debris: DebrisObject, points: number) => void;
  
  constructor(canvasWidth: number, canvasHeight: number, config?: Partial<SpawnConfig>) {
    this.canvasWidth = canvasWidth;
    this.canvasHeight = canvasHeight;
    this.config = { ...DEFAULT_SPAWN_CONFIG, ...config };
  }
  
  updateConfig(config: Partial<SpawnConfig>): void {
    this.config = { ...this.config, ...config };
  }
  
  updateCanvasSize(width: number, height: number): void {
    this.canvasWidth = width;
    this.canvasHeight = height;
  }
  
  tick(deltaTime: number = 1): void {
    const now = Date.now();
    
    // Periodic spawning
    if (this.config.enabled && now - this.lastSpawnTime >= this.config.intervalMs) {
      if (this.debris.filter(d => !d.collected).length < this.config.maxActiveDebris) {
        this.spawnRandomDebris();
        this.lastSpawnTime = now;
      }
    }
    
    // Update debris physics
    for (const d of this.debris) {
      if (d.collected) continue;
      
      // Slow drift in space
      d.x += d.velocityX * deltaTime;
      d.y += d.velocityY * deltaTime;
      d.rotation += d.rotationSpeed * deltaTime;
      
      // Wrap around screen edges
      if (d.x < -20) d.x = this.canvasWidth + 20;
      if (d.x > this.canvasWidth + 20) d.x = -20;
      if (d.y < -20) d.y = this.canvasHeight + 20;
      if (d.y > this.canvasHeight + 20) d.y = -20;
    }
    
    // Remove collected debris
    this.debris = this.debris.filter(d => !d.collected);
  }
  
  spawnRandomDebris(): void {
    const types = this.config.types;
    const totalWeight = types.reduce((sum, t) => sum + t.weight, 0);
    let random = Math.random() * totalWeight;
    
    let selectedType = types[0];
    for (const type of types) {
      random -= type.weight;
      if (random <= 0) {
        selectedType = type;
        break;
      }
    }
    
    const size = selectedType.sizeRange[0] + 
      Math.random() * (selectedType.sizeRange[1] - selectedType.sizeRange[0]);
    
    // Spawn position
    let x: number, y: number;
    if (this.config.spawnAtEdge) {
      // Spawn at screen edge
      const edge = Math.floor(Math.random() * 4);
      const margin = 30;
      switch (edge) {
        case 0: x = margin; y = Math.random() * this.canvasHeight; break;
        case 1: x = this.canvasWidth - margin; y = Math.random() * this.canvasHeight; break;
        case 2: x = Math.random() * this.canvasWidth; y = margin; break;
        default: x = Math.random() * this.canvasWidth; y = this.canvasHeight - margin; break;
      }
    } else {
      x = Math.random() * this.canvasWidth;
      y = Math.random() * this.canvasHeight;
    }
    
    const debris: DebrisObject = {
      id: generateDebrisId(),
      x,
      y,
      size,
      material: selectedType.material,
      rotation: Math.random() * Math.PI * 2,
      rotationSpeed: (Math.random() - 0.5) * 0.1,
      velocityX: (Math.random() - 0.5) * 0.3,
      velocityY: (Math.random() - 0.5) * 0.3,
      collected: false,
      spawnTime: Date.now(),
    };
    
    this.debris.push(debris);
  }
  
  // Spawn debris at specific position (for initial game setup)
  spawnDebrisAt(x: number, y: number, type?: DebrisType): void {
    const debrisType = type || this.config.types[Math.floor(Math.random() * this.config.types.length)];
    const size = debrisType.sizeRange[0] + 
      Math.random() * (debrisType.sizeRange[1] - debrisType.sizeRange[0]);
    
    const debris: DebrisObject = {
      id: generateDebrisId(),
      x,
      y,
      size,
      material: debrisType.material,
      rotation: Math.random() * Math.PI * 2,
      rotationSpeed: (Math.random() - 0.5) * 0.05,
      velocityX: (Math.random() - 0.5) * 0.2,
      velocityY: (Math.random() - 0.5) * 0.2,
      collected: false,
      spawnTime: Date.now(),
    };
    
    this.debris.push(debris);
  }
  
  // Check if player ship collects any debris
  checkCollection(playerX: number, playerY: number, playerRadius: number = 15): void {
    for (const d of this.debris) {
      if (d.collected) continue;
      
      const dx = playerX - d.x;
      const dy = playerY - d.y;
      const dist = Math.sqrt(dx * dx + dy * dy);
      
      // Find debris type for points
      const debrisType = this.config.types.find(t => t.material === d.material) || this.config.types[0];
      const collectRadius = debrisType.collectRadius * (d.size / 2);
      
      if (dist < playerRadius + collectRadius) {
        d.collected = true;
        if (this.onDebrisCollected) {
          this.onDebrisCollected(d, debrisType.points);
        }
      }
    }
  }
  
  getActiveDebris(): DebrisObject[] {
    return this.debris.filter(d => !d.collected);
  }
  
  getDebrisCount(): { active: number; total: number } {
    const active = this.debris.filter(d => !d.collected).length;
    return { active, total: this.debris.length };
  }
  
  clear(): void {
    this.debris = [];
  }
  
  // Spawn initial debris field (call during game start)
  spawnInitialField(count: number = 5): void {
    for (let i = 0; i < count; i++) {
      const x = Math.random() * this.canvasWidth;
      const y = Math.random() * this.canvasHeight;
      this.spawnDebrisAt(x, y);
    }
  }
}

// Canvas rendering for debris
export function renderDebris(
  ctx: CanvasRenderingContext2D,
  debris: DebrisObject[],
  scale: number = 4
): void {
  for (const d of debris) {
    if (d.collected) continue;
    
    ctx.save();
    ctx.translate(d.x, d.y);
    ctx.rotate(d.rotation);
    
    // Color based on material
    let color: string;
    switch (d.material) {
      case 'sand': color = '#C2B280'; break;
      case 'stone': color = '#808080'; break;
      case 'ice': color = '#ADD8FA'; break;
      default: color = '#808080';
    }
    
    const baseSize = d.size * 6 * scale;
    
    // Draw irregular shape
    ctx.fillStyle = color;
    ctx.beginPath();
    const points = 6;
    for (let i = 0; i < points; i++) {
      const angle = (i / points) * Math.PI * 2;
      const variance = 0.7 + Math.random() * 0.3;
      const px = Math.cos(angle) * baseSize * variance;
      const py = Math.sin(angle) * baseSize * variance;
      if (i === 0) ctx.moveTo(px, py);
      else ctx.lineTo(px, py);
    }
    ctx.closePath();
    ctx.fill();
    
    // Highlight
    ctx.fillStyle = 'rgba(255,255,255,0.2)';
    ctx.beginPath();
    ctx.arc(-baseSize * 0.2, -baseSize * 0.2, baseSize * 0.3, 0, Math.PI * 2);
    ctx.fill();
    
    ctx.restore();
  }
}