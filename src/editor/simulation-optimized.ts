// FUL-35b: Phase 1 - Optimized Canvas2D Simulation with Typed Arrays & Dirty Rect Tracking
// Performance optimizations: typed arrays, DOM caching, dirty rects
// Target: 30%+ improvement over current implementation
// FUL-35c: Added spacecraft control support
// FUL-45: Added gravity gun integration

import { Material, MATERIALS } from './materials';
import { Spacecraft, ShipClass, createSpacecraft, SHIP_CLASS_INFO } from './spacecraft';
import { SpacecraftControl, setupKeyboardControls, generateSpacecraftId } from './spacecraftControl';
import { SpacecraftRenderer } from './spacecraftRenderer';
import { GravityGunState } from './gravityGun';
import { renderGravityGunEffect } from './gravityGun';

// Material colors from Rust implementation
const MATERIAL_COLORS: [number, number, number][] = [
  [20, 20, 30],      // Air
  [194, 178, 128],   // Sand
  [64, 164, 223],    // Water
  [128, 128, 128],   // Stone
  [255, 100, 50],    // Fire
  [100, 100, 110],   // Smoke
  [0, 0, 0],         // BlackHole
  [200, 200, 255],   // Steam
  [173, 216, 250],   // Ice
  [101, 67, 33],     // Oil
  [139, 90, 43],     // Wood
  [255, 69, 0],      // Lava
  [50, 50, 55],      // Ash
  [20, 20, 30],      // Eraser
];

// Particle data structure using typed arrays for performance
// Layout: [material, temperature, lifetime, vx, vy, flags, stretch]
const PARTICLE_STRIDE = 7;

// Performance optimization: Use bitfields for flags
const FLAG_BURNING = 1;
const FLAG_ACTIVE = 2;

// Material property lookups (no runtime type checking)
const MATERIAL_PROPS: {
  hasGravity: boolean;
  rises: boolean;
  isFluid: boolean;
  isStatic: boolean;
  lifetime: number;
}[] = [
  { hasGravity: false, rises: false, isFluid: false, isStatic: true, lifetime: 0 },       // Air
  { hasGravity: true, rises: false, isFluid: false, isStatic: false, lifetime: 0 },     // Sand
  { hasGravity: true, rises: false, isFluid: true, isStatic: false, lifetime: 0 },     // Water
  { hasGravity: false, rises: false, isFluid: false, isStatic: true, lifetime: 0 },      // Stone
  { hasGravity: false, rises: true, isFluid: false, isStatic: false, lifetime: 50 },    // Fire
  { hasGravity: false, rises: true, isFluid: false, isStatic: false, lifetime: 100 },   // Smoke
  { hasGravity: false, rises: false, isFluid: false, isStatic: true, lifetime: 0 },     // BlackHole
  { hasGravity: false, rises: true, isFluid: false, isStatic: false, lifetime: 100 },   // Steam
  { hasGravity: true, rises: false, isFluid: false, isStatic: false, lifetime: 0 },      // Ice
  { hasGravity: true, rises: false, isFluid: true, isStatic: false, lifetime: 0 },       // Oil
  { hasGravity: false, rises: false, isFluid: false, isStatic: true, lifetime: 0 },     // Wood
  { hasGravity: true, rises: false, isFluid: true, isStatic: false, lifetime: 0 },      // Lava
  { hasGravity: true, rises: false, isFluid: false, isStatic: false, lifetime: 0 },       // Ash
  { hasGravity: false, rises: false, isFluid: false, isStatic: true, lifetime: 0 },      // Eraser
];

export type OverlayMode = 'none' | 'temperature' | 'velocity';

// FUL-35c: Spacecraft state interface
export interface SpacecraftState {
  playerShip: Spacecraft | null;
  enemyShips: Spacecraft[];
  control: SpacecraftControl | null;
  cleanupControls: (() => void) | null;
  isActive: boolean;
}

interface DirtyRect {
  x: number;
  y: number;
  w: number;
  h: number;
}

// Pre-allocated arrays for updates (avoid GC pressure)
const UPDATE_BUFFER_SIZE = 1024;
const updateX: Int32Array = new Int32Array(UPDATE_BUFFER_SIZE);
const updateY: Int32Array = new Int32Array(UPDATE_BUFFER_SIZE);
const updateFromX: Int32Array = new Int32Array(UPDATE_BUFFER_SIZE);
const updateFromY: Int32Array = new Int32Array(UPDATE_BUFFER_SIZE);

export class SimulationCanvas {
  // Cached dimensions (avoid DOM lookups)
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private width: number;           // Grid width
  private height: number;          // Grid height
  private scale: number;           // Pixel scale
  private canvasWidth: number;     // Cached: width * scale
  private canvasHeight: number;    // Cached: height * scale
  
  // Typed array grid (performance: no objects, no GC)
  // Each cell: [material, temp, lifetime, vx, vy, flags, stretch]
  private grid: Float32Array;
  
  // Image data
  private imageData: ImageData;
  private pixels: Uint8ClampedArray;
  
  // Dirty rect tracking
  private dirtyRects: DirtyRect[] = [];
  private currentDirty: DirtyRect = { x: this.width || 0, y: this.height || 0, w: 0, h: 0 };
  
  // Black hole tracking
  private blackHoles: Array<{ x: number; y: number; mass: number; consumed: number }> = [];
  private stats = { particlesConsumed: 0, totalMass: 0 };
  
  // Overlay mode
  private overlayMode: OverlayMode = 'none';
  
  // FUL-35c: Spacecraft mode
  private spacecraftState: SpacecraftState = {
    playerShip: null,
    enemyShips: [],
    control: null,
    cleanupControls: null,
    isActive: false,
  };
  private spacecraftRenderer: SpacecraftRenderer | null = null;
  
  // FUL-45: Gravity gun state for interactive particle manipulation
  public gravityGunState: GravityGunState | null = null;
  
  // FUL-45: Parallax starfield for space game
  private starLayers: Array<{ x: number; y: number; size: number; brightness: number; parallax: number }> = [];
  private starLayerCount = 3;
  private starsPerLayer = 80;

  // Black hole constants (matching Rust)
  private readonly BLACK_HOLE_MASS = 200;
  private readonly EVENT_HORIZON_RADIUS = 4;
  private readonly INFLUENCE_RADIUS = 60;
  private readonly GRAVITY_CONSTANT = 2500;

  constructor(canvas: HTMLCanvasElement, width: number, height: number, scale: number = 4) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d', { alpha: false })!;
    this.width = width;
    this.height = height;
    this.scale = scale;
    
    // Cache canvas dimensions
    this.canvasWidth = width * scale;
    this.canvasHeight = height * scale;

    // Set canvas size once
    canvas.width = this.canvasWidth;
    canvas.height = this.canvasHeight;

    // Disable image smoothing for pixel-perfect rendering
    this.ctx.imageSmoothingEnabled = false;

    // FUL-35c: Initialize spacecraft renderer
    this.spacecraftRenderer = new SpacecraftRenderer(this.ctx);

    // Initialize grid with typed array
    this.grid = new Float32Array(width * height * PARTICLE_STRIDE);
    this.initGrid();

    // Initialize image data
    this.imageData = this.ctx.createImageData(this.canvasWidth, this.canvasHeight);
    this.pixels = this.imageData.data;
    
    // FUL-45: Initialize parallax starfield
    this.initStarfield();
  }
  
  private initStarfield(): void {
    this.starLayers = [];
    for (let layer = 0; layer < this.starLayerCount; layer++) {
      const parallax = 0.1 + layer * 0.15; // 0.1, 0.25, 0.4
      const count = this.starsPerLayer - layer * 15; // Fewer stars in foreground
      const sizeRange = 1 + layer * 0.5; // Larger stars in front
      
      for (let i = 0; i < count; i++) {
        this.starLayers.push({
          x: Math.random() * this.canvasWidth * 2 - this.canvasWidth * 0.5,
          y: Math.random() * this.canvasHeight * 2 - this.canvasHeight * 0.5,
          size: 0.5 + Math.random() * sizeRange,
          brightness: 0.3 + Math.random() * 0.7,
          parallax,
        });
      }
    }
  }
  
  private renderStarfield(): void {
    // Use player ship or black hole position as parallax anchor
    let anchorX = this.canvasWidth / 2;
    let anchorY = this.canvasHeight / 2;
    
    if (this.spacecraftState.playerShip) {
      anchorX = this.spacecraftState.playerShip.position.x;
      anchorY = this.spacecraftState.playerShip.position.y;
    } else if (this.blackHoles.length > 0) {
      anchorX = this.blackHoles[0].x;
      anchorY = this.blackHoles[0].y;
    }
    
    const offsetX = anchorX - this.canvasWidth / 2;
    const offsetY = anchorY - this.canvasHeight / 2;
    
    for (const star of this.starLayers) {
      // Apply parallax offset
      const sx = star.x - offsetX * star.parallax;
      const sy = star.y - offsetY * star.parallax;
      
      // Wrap stars around screen
      const wx = ((sx % this.canvasWidth) + this.canvasWidth) % this.canvasWidth;
      const wy = ((sy % this.canvasHeight) + this.canvasHeight) % this.canvasHeight;
      
      // Draw star with size-based brightness
      const baseAlpha = Math.floor(star.brightness * 200);
      const size = Math.max(1, Math.round(star.size));
      
      // Simple circle star
      this.ctx.fillStyle = `rgba(255, 255, 255, ${baseAlpha / 255})`;
      this.ctx.beginPath();
      this.ctx.arc(wx, wy, size * 0.5, 0, Math.PI * 2);
      this.ctx.fill();
      
      // Add glow for larger stars
      if (size > 1.5) {
        this.ctx.fillStyle = `rgba(200, 220, 255, ${(baseAlpha / 255) * 0.3})`;
        this.ctx.beginPath();
        this.ctx.arc(wx, wy, size, 0, Math.PI * 2);
        this.ctx.fill();
      }
    }
  }

  private initGrid(): void {
    const grid = this.grid;
    for (let i = 0; i < this.width * this.height; i++) {
      const idx = i * PARTICLE_STRIDE;
      grid[idx] = Material.Air;         // material
      grid[idx + 1] = 293;             // temperature
      grid[idx + 2] = 0;               // lifetime
      grid[idx + 3] = 0;               // velocityX
      grid[idx + 4] = 0;               // velocityY
      grid[idx + 5] = 0;               // flags
      grid[idx + 6] = 1.0;             // stretch
    }
  }

  private getIndex(x: number, y: number): number {
    return (y * this.width + x) * PARTICLE_STRIDE;
  }

  private inBounds(x: number, y: number): boolean {
    return x >= 0 && x < this.width && y >= 0 && y < this.height;
  }

  // Fast particle access using typed arrays
  private getMaterial(x: number, y: number): number {
    if (!this.inBounds(x, y)) return -1;
    return this.grid[(y * this.width + x) * PARTICLE_STRIDE];
  }

  private setMaterial(x: number, y: number, material: number): void {
    if (!this.inBounds(x, y)) return;
    const idx = (y * this.width + x) * PARTICLE_STRIDE;
    this.grid[idx] = material;
    
    // Initialize lifetime based on material
    if (material === Material.Fire) {
      this.grid[idx + 2] = 30 + Math.random() * 20;
    } else if (material === Material.Smoke || material === Material.Steam) {
      this.grid[idx + 2] = 60 + Math.random() * 40;
    }
  }

  get(x: number, y: number): number {
    return this.getMaterial(x, y);
  }

  set(x: number, y: number, material: Material): boolean {
    const current = this.getMaterial(x, y);
    if (current !== Material.Air && material !== Material.Air) return false;
    this.setMaterial(x, y, material);
    this.markDirty(x, y);
    return true;
  }

  spawn(x: number, y: number, material: Material): boolean {
    return this.set(x, y, material);
  }

  remove(x: number, y: number): void {
    this.setMaterial(x, y, Material.Air);
    this.markDirty(x, y);
  }

  clear(): void {
    this.grid.fill(0);
    this.initGrid();
    this.dirtyRects.push({ x: 0, y: 0, w: this.width, h: this.height });
  }

  countParticles(): number {
    let count = 0;
    const grid = this.grid;
    for (let i = 0; i < this.width * this.height; i++) {
      if (grid[i * PARTICLE_STRIDE] !== Material.Air) count++;
    }
    return count;
  }

  // Mark a cell as dirty for efficient rendering
  private markDirty(x: number, y: number): void {
    const scale = this.scale;
    const px = x * scale;
    const py = y * scale;
    
    // Extend current dirty rect if adjacent, otherwise start new one
    if (this.currentDirty.w === 0) {
      this.currentDirty = { x: px, y: py, w: scale, h: scale };
    } else if (px >= this.currentDirty.x && px <= this.currentDirty.x + this.currentDirty.w &&
               py >= this.currentDirty.y && py <= this.currentDirty.y + this.currentDirty.h) {
      // Expand existing rect
      const nx = Math.min(this.currentDirty.x, px);
      const ny = Math.min(this.currentDirty.y, py);
      this.currentDirty.w = Math.max(this.currentDirty.x + this.currentDirty.w, px + scale) - nx;
      this.currentDirty.h = Math.max(this.currentDirty.y + this.currentDirty.h, py + scale) - ny;
      this.currentDirty.x = nx;
      this.currentDirty.y = ny;
    } else {
      // Start new dirty rect
      this.dirtyRects.push(this.currentDirty);
      this.currentDirty = { x: px, y: py, w: scale, h: scale };
    }
  }

  private flushDirty(): void {
    if (this.currentDirty.w > 0) {
      this.dirtyRects.push(this.currentDirty);
      this.currentDirty = { x: 0, y: 0, w: 0, h: 0 };
    }
  }

  // Physics simulation tick - optimized with typed arrays
  tick(): void {
    const { width, height, grid } = this;
    const air = Material.Air;
    const fire = Material.Fire;
    
    let updateCount = 0;

    // Process from bottom to top
    for (let y = height - 2; y >= 0; y--) {
      // Randomize left-right order to prevent bias
      const leftFirst = Math.random() < 0.5;
      
      for (let xi = 0; xi < width; xi++) {
        const x = leftFirst ? xi : width - 1 - xi;
        const idx = (y * width + x) * PARTICLE_STRIDE;
        const material = grid[idx];
        
        if (material === air) continue;
        
        // Inline material behavior (no function call overhead)
        const props = MATERIAL_PROPS[material];
        
        if (props.isStatic) continue;
        
        // Sand-like materials
        if (material === Material.Sand || material === Material.Ash || material === Material.Ice) {
          if (this.moveDownFast(grid, width, height, x, y, air)) continue;
          
          // Diagonal fall
          const r = Math.random();
          if (r < 0.5) {
            if (x > 0 && grid[((y + 1) * width + (x - 1)) * PARTICLE_STRIDE] === air) {
              this.swapCells(x, y, x - 1, y + 1);
            } else if (x + 1 < width && grid[((y + 1) * width + (x + 1)) * PARTICLE_STRIDE] === air) {
              this.swapCells(x, y, x + 1, y + 1);
            }
          } else {
            if (x + 1 < width && grid[((y + 1) * width + (x + 1)) * PARTICLE_STRIDE] === air) {
              this.swapCells(x, y, x + 1, y + 1);
            } else if (x > 0 && grid[((y + 1) * width + (x - 1)) * PARTICLE_STRIDE] === air) {
              this.swapCells(x, y, x - 1, y + 1);
            }
          }
        }
        // Water-like materials
        else if (material === Material.Water || material === Material.Oil || material === Material.Lava) {
          if (this.moveDownFast(grid, width, height, x, y, air)) continue;
          
          // Diagonal
          const dl = x > 0 && grid[((y + 1) * width + (x - 1)) * PARTICLE_STRIDE] === air;
          const dr = x + 1 < width && grid[((y + 1) * width + (x + 1)) * PARTICLE_STRIDE] === air;
          if (dl || dr) {
            if (dl && dr) {
              this.swapCells(x, y, x + (Math.random() < 0.5 ? -1 : 1), y + 1);
            } else {
              this.swapCells(x, y, dl ? x - 1 : x + 1, y + 1);
            }
            continue;
          }
          
          // Horizontal flow
          if (Math.random() < 0.7) {
            const fl = x > 0 && grid[(y * width + (x - 1)) * PARTICLE_STRIDE] === air;
            const fr = x + 1 < width && grid[(y * width + (x + 1)) * PARTICLE_STRIDE] === air;
            if (fl || fr) {
              this.swapCells(x, y, fl && fr ? x + (Math.random() < 0.5 ? -1 : 1) : (fl ? x - 1 : x + 1), y);
            }
          }
        }
        // Fire
        else if (material === fire) {
          grid[idx + 2]--; // Decrease lifetime
          
          // Extinguish in water
          for (let dy = -1; dy <= 1; dy++) {
            for (let dx = -1; dx <= 1; dx++) {
              if (dx === 0 && dy === 0) continue;
              const nx = x + dx, ny = y + dy;
              if (!this.inBounds(nx, ny)) continue;
              if (grid[(ny * width + nx) * PARTICLE_STRIDE] === Material.Water) {
                this.setMaterial(x, y, Material.Smoke);
                grid[(ny * width + nx) * PARTICLE_STRIDE + 2] = 80 + Math.random() * 20;
                this.markDirty(x, y);
                this.markDirty(nx, ny);
                break;
              }
            }
          }
          
          // Rise
          if (y > 0 && grid[((y - 1) * width + x) * PARTICLE_STRIDE] === air && Math.random() < 0.7) {
            this.swapCells(x, y, x, y - 1);
            this.markDirty(x, y);
            this.markDirty(x, y - 1);
          }
          
          // Die
          if (grid[idx + 2] <= 0) {
            this.setMaterial(x, y, Material.Smoke);
            grid[idx + 2] = 60 + Math.random() * 40;
            this.markDirty(x, y);
          }
        }
        // Gas (smoke/steam)
        else if (material === Material.Smoke || material === Material.Steam) {
          grid[idx + 2]--; // Decrease lifetime
          
          if (grid[idx + 2] <= 0) {
            this.setMaterial(x, y, air);
            this.markDirty(x, y);
            continue;
          }
          
          // Rise faster for steam
          const riseSpeed = material === Material.Steam ? 0.9 : 0.7;
          if (y > 0 && grid[((y - 1) * width + x) * PARTICLE_STRIDE] === air && Math.random() < riseSpeed) {
            this.swapCells(x, y, x, y - 1);
            this.markDirty(x, y);
            this.markDirty(x, y - 1);
          }
        }
        // Black hole
        else if (material === Material.BlackHole) {
          this.updateBlackHoleFast(grid, width, height, x, y);
        }
      }
    }

    // FUL-35c: Update spacecraft physics
    this.updateSpacecraft();
  }

  private moveDownFast(grid: Float32Array, width: number, height: number, x: number, y: number, air: number): boolean {
    if (y + 1 >= height) return false;
    const belowIdx = ((y + 1) * width + x) * PARTICLE_STRIDE;
    if (grid[belowIdx] === air) {
      this.swapCells(x, y, x, y + 1);
      this.markDirty(x, y);
      this.markDirty(x, y + 1);
      return true;
    }
    return false;
  }

  private swapCells(x1: number, y1: number, x2: number, y2: number): void {
    const i1 = (y1 * this.width + x1) * PARTICLE_STRIDE;
    const i2 = (y2 * this.width + x2) * PARTICLE_STRIDE;
    const grid = this.grid;
    
    // Swap 7 values
    for (let i = 0; i < PARTICLE_STRIDE; i++) {
      const tmp = grid[i1 + i];
      grid[i1 + i] = grid[i2 + i];
      grid[i2 + i] = tmp;
    }
  }

  private updateBlackHoleFast(grid: Float32Array, width: number, height: number, x: number, y: number): void {
    const idx = (y * width + x) * PARTICLE_STRIDE;
    
    // Find or create black hole entry
    let bh = this.blackHoles.find(b => b.x === x && b.y === y);
    if (!bh) {
      bh = { x, y, mass: this.BLACK_HOLE_MASS, consumed: 0 };
      this.blackHoles.push(bh);
    }

    // Gravitational pull (optimized: check every 4th pixel)
    const step = 4;
    for (let dy = -this.INFLUENCE_RADIUS; dy <= this.INFLUENCE_RADIUS; dy += step) {
      for (let dx = -this.INFLUENCE_RADIUS; dx <= this.INFLUENCE_RADIUS; dx += step) {
        const nx = x + dx;
        const ny = y + dy;
        if (!this.inBounds(nx, ny)) continue;
        
        const nidx = (ny * width + nx) * PARTICLE_STRIDE;
        const mat = grid[nidx];
        
        // Skip air, static materials, other black holes
        if (mat === Material.Air || mat === Material.Stone || mat === Material.Wood || mat === Material.BlackHole) continue;
        
        const distSq = dx * dx + dy * dy;
        const dist = Math.sqrt(distSq);
        
        if (dist < this.EVENT_HORIZON_RADIUS) {
          // Consume
          this.stats.particlesConsumed++;
          this.stats.totalMass += this.getParticleMass(mat);
          bh.consumed++;
          grid[nidx] = Material.Air;
          this.markDirty(nx, ny);
        } else if (dist < this.INFLUENCE_RADIUS) {
          // Gravitational acceleration
          const force = this.GRAVITY_CONSTANT * bh.mass / (distSq + 50);
          grid[nidx + 3] = grid[nidx + 3] * 0.9 + (dx / dist) * force * 0.1;
          grid[nidx + 4] = grid[nidx + 4] * 0.9 + (dy / dist) * force * 0.1;
          
          // Clamp velocity
          const vel = Math.sqrt(grid[nidx + 3] ** 2 + grid[nidx + 4] ** 2);
          if (vel > 15) {
            grid[nidx + 3] = (grid[nidx + 3] / vel) * 15;
            grid[nidx + 4] = (grid[nidx + 4] / vel) * 15;
          }
          
          // Spaghettification near event horizon
          const tidalThreshold = this.EVENT_HORIZON_RADIUS * 2;
          if (dist < tidalThreshold && dist > this.EVENT_HORIZON_RADIUS) {
            const distFromHorizon = dist - this.EVENT_HORIZON_RADIUS;
            const tidalFactor = this.getStretchFactor(distFromHorizon, this.EVENT_HORIZON_RADIUS);
            if (tidalFactor > 0.05 && this.canStretch(mat)) {
              grid[nidx + 6] = tidalFactor;
            }
          }
          
          // Apply velocity movement
          if (vel > 0.3) {
            const moveX = Math.round(grid[nidx + 3]);
            const moveY = Math.round(grid[nidx + 4]);
            
            if (moveX !== 0 || moveY !== 0) {
              const targetX = Math.max(0, Math.min(width - 1, nx + moveX));
              const targetY = Math.max(0, Math.min(height - 1, ny + moveY));
              const tidx = (targetY * width + targetX) * PARTICLE_STRIDE;
              
              if (grid[tidx] === Material.Air) {
                this.swapCells(nx, ny, targetX, targetY);
                grid[tidx + 3] *= 0.85;
                grid[tidx + 4] *= 0.85;
                this.markDirty(nx, ny);
                this.markDirty(targetX, targetY);
              }
            }
          }
        }
      }
    }
    
    // Hawking radiation
    if (Math.random() < 0.02) {
      const angle = Math.random() * Math.PI * 2;
      const ex = Math.round(x + Math.cos(angle) * (this.EVENT_HORIZON_RADIUS + 1));
      const ey = Math.round(y + Math.sin(angle) * (this.EVENT_HORIZON_RADIUS + 1));
      
      if (this.inBounds(ex, ey)) {
        const eidx = (ey * width + ex) * PARTICLE_STRIDE;
        if (grid[eidx] === Material.Air) {
          grid[eidx] = Math.random() < 0.5 ? Material.Fire : Material.Smoke;
          grid[eidx + 1] = 800 + Math.random() * 400; // temperature
          grid[eidx + 2] = 30 + Math.random() * 20;   // lifetime
          this.markDirty(ex, ey);
        }
      }
    }
  }

  private getParticleMass(material: number): number {
    switch (material) {
      case Material.Fire: return 0.5;
      case Material.Smoke: return 0.3;
      case Material.Steam: return 0.4;
      case Material.Sand: return 2.0;
      case Material.Water: return 1.5;
      case Material.Oil: return 1.2;
      case Material.Ice: return 1.8;
      case Material.Lava: return 3.0;
      case Material.Stone: return 4.0;
      case Material.Ash: return 1.0;
      default: return 1.0;
    }
  }

  private getStretchFactor(distFromHorizon: number, horizonRadius: number): number {
    if (distFromHorizon <= 0 || horizonRadius <= 0) return 1.0;
    const proximity = horizonRadius / (horizonRadius + distFromHorizon);
    return proximity * proximity * 2.0;
  }

  private canStretch(material: number): boolean {
    return material === Material.Sand || material === Material.Water || material === Material.Oil ||
           material === Material.Stone || material === Material.Wood || material === Material.Ice ||
           material === Material.Lava;
  }

  // Optimized rendering with dirty rect tracking
  render(): void {
    const { width, height, scale, pixels, grid } = this;
    
    // FUL-45: Render parallax starfield background for space modes
    this.renderStarfield();
    
    const bg = MATERIAL_COLORS[Material.Air];
    
    // Clear entire canvas (background)
    const canvasW = this.canvasWidth;
    const canvasH = this.canvasHeight;
    for (let i = 0; i < pixels.length; i += 4) {
      const py = Math.floor((i / 4) / canvasW);
      const px = (i / 4) % canvasW;
      
      // Only clear if pixel is in a dirty rect
      let inDirty = false;
      for (const rect of this.dirtyRects) {
        if (px >= rect.x && px < rect.x + rect.w && py >= rect.y && py < rect.y + rect.h) {
          inDirty = true;
          break;
        }
      }
      if (!inDirty && py < this.currentDirty.y + this.currentDirty.h && 
          py >= this.currentDirty.y && px >= this.currentDirty.x && px < this.currentDirty.x + this.currentDirty.w) {
        inDirty = true;
      }
      
      if (inDirty) {
        pixels[i] = bg[0];
        pixels[i + 1] = bg[1];
        pixels[i + 2] = bg[2];
        pixels[i + 3] = 255;
      }
    }

    // Draw particles (only dirty regions)
    this.flushDirty();
    
    for (const rect of this.dirtyRects) {
      const startX = Math.floor(rect.x / scale);
      const startY = Math.floor(rect.y / scale);
      const endX = Math.min(width, Math.ceil((rect.x + rect.w) / scale));
      const endY = Math.min(height, Math.ceil((rect.y + rect.h) / scale));

      for (let y = startY; y < endY; y++) {
        for (let x = startX; x < endX; x++) {
          const idx = (y * width + x) * PARTICLE_STRIDE;
          const material = grid[idx];
          
          if (material === Material.Air) continue;

          const color = MATERIAL_COLORS[material];
          let r = color[0];
          let g = color[1];
          let b = color[2];
          let a = 255;

          // Fire flicker
          if (material === Material.Fire) {
            const flicker = Math.floor(Math.sin(x * 0.1 + y * 0.1 + grid[idx + 2] * 0.5) * 30 + 30);
            r = Math.min(255, r + flicker);
            g = Math.min(255, g + flicker * 2);
          }

          // Spaghettification tint
          if (grid[idx + 6] > 1.01) {
            const stretchFactor = Math.min(1, grid[idx + 6] - 1.0);
            r = Math.min(255, Math.floor(r * (1 - stretchFactor * 0.5) + 200 * stretchFactor * 0.5));
            g = Math.min(255, Math.floor(g * (1 - stretchFactor * 0.5) + 80 * stretchFactor * 0.5));
          }

          // Overlay modes
          if (this.overlayMode === 'temperature') {
            const temp = grid[idx + 1];
            const normalized = Math.max(0, Math.min(1, (temp - 200) / 1500));
            r = Math.floor(r * (1 - normalized * 0.5) + 255 * normalized * 0.5);
            b = Math.floor(b * (1 - normalized * 0.5));
          } else if (this.overlayMode === 'velocity') {
            const vx = grid[idx + 3];
            const vy = grid[idx + 4];
            const vel = Math.sqrt(vx * vx + vy * vy);
            const normalized = Math.min(1, vel / 5);
            r = Math.floor(r * (1 - normalized * 0.5) + 255 * normalized * 0.5);
            g = Math.floor(g * (1 - normalized * 0.7));
            b = Math.floor(b * (1 - normalized * 0.5));
          }

          // Fill scaled block
          const px0 = x * scale;
          const py0 = y * scale;
          
          for (let dy = 0; dy < scale; dy++) {
            for (let dx = 0; dx < scale; dx++) {
              const pi = ((py0 + dy) * canvasW + (px0 + dx)) * 4;
              pixels[pi] = r;
              pixels[pi + 1] = g;
              pixels[pi + 2] = b;
              pixels[pi + 3] = a;
            }
          }
        }
      }
    }

    // Clear dirty rects after render
    this.dirtyRects = [];
    this.currentDirty = { x: 0, y: 0, w: 0, h: 0 };

    this.ctx.putImageData(this.imageData, 0, 0);

    // FUL-35c: Render spacecraft on top of particles
    this.renderSpacecraft();
    
    // FUL-45: Render gravity gun effect
    if (this.gravityGunState) {
      renderGravityGunEffect(this.ctx, this.gravityGunState, this.scale);
    }
  }

  // FUL-35c: Render all spacecraft
  private renderSpacecraft(): void {
    if (!this.spacecraftRenderer || !this.spacecraftState.isActive) return;

    // Render enemy ships
    for (const ship of this.spacecraftState.enemyShips) {
      this.spacecraftRenderer.render(ship, this.scale);
    }

    // Render player ship on top
    if (this.spacecraftState.playerShip) {
      this.spacecraftRenderer.render(this.spacecraftState.playerShip, this.scale);
    }
  }

  // Spawn particles in a circular brush pattern
  spawnBrush(centerX: number, centerY: number, material: Material, radius: number): number {
    let count = 0;
    const gridX = Math.floor(centerX / this.scale);
    const gridY = Math.floor(centerY / this.scale);
    const r = Math.floor(radius / this.scale);

    for (let dy = -r; dy <= r; dy++) {
      for (let dx = -r; dx <= r; dx++) {
        if (Math.sqrt(dx * dx + dy * dy) <= r) {
          if (this.spawn(gridX + dx, gridY + dy, material)) count++;
        }
      }
    }

    return count;
  }

  // Interpolate line for smooth brush strokes
  spawnLine(x0: number, y0: number, x1: number, y1: number, material: Material, radius: number): number {
    const dx = Math.abs(x1 - x0);
    const dy = Math.abs(y1 - y0);
    const steps = Math.max(dx, dy);

    if (steps === 0) return this.spawnBrush(x0, y0, material, radius);

    let count = 0;
    for (let i = 0; i <= steps; i++) {
      const t = i / steps;
      const x = Math.round(x0 + (x1 - x0) * t);
      const y = Math.round(y0 + (y1 - y0) * t);
      count += this.spawnBrush(x, y, material, radius);
    }

    return count;
  }
  
  getStats(): { particlesConsumed: number; totalMass: number; blackHoles: number } {
    return {
      particlesConsumed: this.stats.particlesConsumed,
      totalMass: this.stats.totalMass,
      blackHoles: this.blackHoles.length
    };
  }
  
  setOverlayMode(mode: OverlayMode): void {
    this.overlayMode = mode;
  }
  
  getOverlayMode(): OverlayMode {
    return this.overlayMode;
  }
  
  getMaterialAt(x: number, y: number): Material | null {
    const gridX = Math.floor(x / this.scale);
    const gridY = Math.floor(y / this.scale);
    const mat = this.getMaterial(gridX, gridY);
    return mat >= 0 ? mat : null;
  }
  
  getVelocityAt(x: number, y: number): { vx: number; vy: number } {
    const gridX = Math.floor(x / this.scale);
    const gridY = Math.floor(y / this.scale);
    const idx = (gridY * this.width + gridX) * PARTICLE_STRIDE;
    return { vx: this.grid[idx + 3], vy: this.grid[idx + 4] };
  }
  
  resetStats(): void {
    this.stats = { particlesConsumed: 0, totalMass: 0 };
    this.blackHoles = [];
  }
  
  spawnStructure(type: 'ship' | 'asteroid' | 'station', centerX: number, centerY: number): void {
    const gridCX = Math.floor(centerX / this.scale);
    const gridCY = Math.floor(centerY / this.scale);
    
    switch (type) {
      case 'ship':
        for (let dy = 0; dy < 8; dy++) {
          for (let dx = -dy; dx <= dy; dx++) {
            this.spawn(gridCX + dx, gridCY + dy, Material.Stone);
          }
        }
        for (let i = 0; i < 3; i++) {
          this.spawn(gridCX, gridCY + 8 + i, Material.Fire);
        }
        break;
        
      case 'asteroid':
        for (let angle = 0; angle < Math.PI * 2; angle += 0.3) {
          const radius = 5 + Math.random() * 3;
          const px = Math.round(gridCX + Math.cos(angle) * radius);
          const py = Math.round(gridCY + Math.sin(angle) * radius);
          this.spawn(px, py, Material.Stone);
          for (let dx = -2; dx <= 2; dx++) {
            for (let dy = -2; dy <= 2; dy++) {
              if (Math.random() < 0.7) {
                this.spawn(px + dx, py + dy, Material.Stone);
              }
            }
          }
        }
        break;
        
      case 'station':
        for (let r = 0; r < 5; r++) {
          for (let angle = 0; angle < Math.PI * 2; angle += Math.PI / 3) {
            const px = Math.round(gridCX + Math.cos(angle) * (r + 3));
            const py = Math.round(gridCY + Math.sin(angle) * (r + 3));
            this.spawn(px, py, Material.Stone);
          }
        }
        for (let dx = -2; dx <= 2; dx++) {
          for (let dy = -2; dy <= 2; dy++) {
            if (Math.abs(dx) + Math.abs(dy) <= 2) {
              this.spawn(gridCX + dx, gridCY + dy, Material.Wood);
            }
          }
        }
        break;
    }
  }

  // ============================================
  // FUL-35c: Spacecraft Control & Game Mechanics
  // ============================================

  // Activate spacecraft game mode
  activateSpacecraftMode(shipClass: ShipClass = ShipClass.Fighter): void {
    if (this.spacecraftState.isActive) return;

    // Create player ship at center
    const centerX = (this.width * this.scale) / 2;
    const centerY = (this.height * this.scale) / 2;
    const playerShip = createSpacecraft(
      generateSpacecraftId(),
      { x: centerX, y: centerY },
      shipClass,
      true // isPlayer
    );

    this.spacecraftState.playerShip = playerShip;

    // Create spacecraft control
    this.spacecraftState.control = new SpacecraftControl(
      playerShip,
      (updatedShip) => {
        this.spacecraftState.playerShip = updatedShip;
      }
    );

    // Setup keyboard controls
    if (this.spacecraftState.control) {
      this.spacecraftState.cleanupControls = setupKeyboardControls(this.spacecraftState.control);
    }

    this.spacecraftState.isActive = true;
  }

  // Deactivate spacecraft game mode
  deactivateSpacecraftMode(): void {
    if (!this.spacecraftState.isActive) return;

    // Cleanup keyboard controls
    if (this.spacecraftState.cleanupControls) {
      this.spacecraftState.cleanupControls();
      this.spacecraftState.cleanupControls = null;
    }

    this.spacecraftState.control = null;
    this.spacecraftState.playerShip = null;
    this.spacecraftState.enemyShips = [];
    this.spacecraftState.isActive = false;
  }

  // Check if spacecraft mode is active
  isSpacecraftModeActive(): boolean {
    return this.spacecraftState.isActive;
  }

  // Get player spacecraft
  getPlayerSpacecraft(): Spacecraft | null {
    return this.spacecraftState.playerShip;
  }

  // Get spacecraft control
  getSpacecraftControl(): SpacecraftControl | null {
    return this.spacecraftState.control;
  }

  // Spawn an enemy ship
  spawnEnemyShip(shipClass: ShipClass, x: number, y: number): Spacecraft {
    const enemy = createSpacecraft(
      generateSpacecraftId(),
      { x, y },
      shipClass,
      false // not player
    );
    this.spacecraftState.enemyShips.push(enemy);
    return enemy;
  }

  // Update spacecraft physics (called each tick)
  updateSpacecraft(deltaTime: number = 1): void {
    if (!this.spacecraftState.isActive || !this.spacecraftState.control) return;

    // Update player ship physics
    this.spacecraftState.control.tick(deltaTime);

    // Update enemy ships (simple AI)
    for (const enemy of this.spacecraftState.enemyShips) {
      if (enemy.isDestroyed) continue;

      if (this.spacecraftState.playerShip && !this.spacecraftState.playerShip.isDestroyed) {
        const dx = this.spacecraftState.playerShip.position.x - enemy.position.x;
        const dy = this.spacecraftState.playerShip.position.y - enemy.position.y;
        const dist = Math.sqrt(dx * dx + dy * dy);

        if (dist > 100) {
          enemy.velocity.x += (dx / dist) * 0.02;
          enemy.velocity.y += (dy / dist) * 0.02;
        } else if (dist < 50) {
          enemy.velocity.x -= (dx / dist) * 0.02;
          enemy.velocity.y -= (dy / dist) * 0.02;
        } else {
          // Orbit
          enemy.velocity.x += (-dy / dist) * 0.01;
          enemy.velocity.y += (dx / dist) * 0.01;
        }

        enemy.angle = Math.atan2(dy, dx) + Math.PI / 2;
      }

      enemy.velocity.x *= 0.99;
      enemy.velocity.y *= 0.99;

      const maxVel = 5;
      const speed = Math.sqrt(enemy.velocity.x ** 2 + enemy.velocity.y ** 2);
      if (speed > maxVel) {
        enemy.velocity.x = (enemy.velocity.x / speed) * maxVel;
        enemy.velocity.y = (enemy.velocity.y / speed) * maxVel;
      }

      enemy.position.x += enemy.velocity.x * deltaTime;
      enemy.position.y += enemy.velocity.y * deltaTime;

      // Keep in bounds
      const margin = 30;
      const canvasWidth = this.width * this.scale;
      const canvasHeight = this.height * this.scale;

      if (enemy.position.x < margin) enemy.velocity.x += 0.1;
      if (enemy.position.x > canvasWidth - margin) enemy.velocity.x -= 0.1;
      if (enemy.position.y < margin) enemy.velocity.y += 0.1;
      if (enemy.position.y > canvasHeight - margin) enemy.velocity.y -= 0.1;
    }

    // Check player-enemy collisions
    if (this.spacecraftState.playerShip && !this.spacecraftState.playerShip.isDestroyed) {
      for (const enemy of this.spacecraftState.enemyShips) {
        if (enemy.isDestroyed) continue;

        const dx = this.spacecraftState.playerShip.position.x - enemy.position.x;
        const dy = this.spacecraftState.playerShip.position.y - enemy.position.y;
        const dist = Math.sqrt(dx * dx + dy * dy);

        if (dist < 24) {
          this.spacecraftState.control?.applyDamage(10);
          enemy.props.hull -= 5;
          if (enemy.props.hull <= 0) {
            enemy.isDestroyed = true;
          }
        }
      }
    }

    // Keep player in bounds
    if (this.spacecraftState.playerShip && !this.spacecraftState.playerShip.isDestroyed) {
      const margin = 20;
      const canvasWidth = this.width * this.scale;
      const canvasHeight = this.height * this.scale;

      if (this.spacecraftState.playerShip.position.x < margin) {
        this.spacecraftState.playerShip.velocity.x += 0.2;
      }
      if (this.spacecraftState.playerShip.position.x > canvasWidth - margin) {
        this.spacecraftState.playerShip.velocity.x -= 0.2;
      }
      if (this.spacecraftState.playerShip.position.y < margin) {
        this.spacecraftState.playerShip.velocity.y += 0.2;
      }
      if (this.spacecraftState.playerShip.position.y > canvasHeight - margin) {
        this.spacecraftState.playerShip.velocity.y -= 0.2;
      }
    }
  }

  // Get all alive enemy ships
  getEnemySpacecraft(): Spacecraft[] {
    return this.spacecraftState.enemyShips.filter(s => !s.isDestroyed);
  }

  // Remove destroyed ships
  cleanupDestroyedShips(): void {
    this.spacecraftState.enemyShips = this.spacecraftState.enemyShips.filter(s => !s.isDestroyed);
  }
}