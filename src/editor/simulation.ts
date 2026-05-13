// FUL-5: Phase 6 - Canvas2D Simulation Renderer
// This implements the physics visualization matching Rust's renderer.rs

import { Material, MATERIALS } from './materials';

export interface Particle {
  material: Material;
  temperature: number;
  lifetime: number;
  velocityX: number;
  velocityY: number;
  burning: boolean;
}

export interface Grid {
  width: number;
  height: number;
  particles: Particle[];
}

// Material colors from Rust implementation
const MATERIAL_COLORS: Record<Material, [number, number, number]> = {
  [Material.Air]: [20, 20, 30],
  [Material.Sand]: [194, 178, 128],
  [Material.Water]: [64, 164, 223],
  [Material.Stone]: [128, 128, 128],
  [Material.Fire]: [255, 100, 50],
  [Material.Smoke]: [100, 100, 110],
  [Material.BlackHole]: [0, 0, 0],
  [Material.Steam]: [200, 200, 255],
  [Material.Ice]: [173, 216, 250],
  [Material.Oil]: [101, 67, 33],
  [Material.Wood]: [139, 90, 43],
  [Material.Lava]: [255, 69, 0],
  [Material.Ash]: [50, 50, 55],
  [Material.Eraser]: [20, 20, 30],
};

function getParticleColor(particle: Particle, x: number, y: number): [number, number, number, number] {
  let [r, g, b] = MATERIAL_COLORS[particle.material];
  let a = 255;

  switch (particle.material) {
    case Material.Fire: {
      // Fire flickers
      const flicker = Math.floor(Math.sin(x * 0.1 + y * 0.1 + particle.lifetime * 0.5) * 30 + 30);
      r = Math.min(255, r + flicker);
      g = Math.min(255, g + flicker * 2);
      break;
    }
    case Material.Smoke: {
      // Smoke varies
      const variation = Math.floor(Math.sin(x * 0.2 + y * 0.2) * 20 + 20);
      r = r + variation;
      g = g + variation;
      b = b + variation + 10;
      break;
    }
    case Material.Lava: {
      // Lava glows
      const temp = Math.min(1, particle.temperature / 1500);
      g = Math.floor(69 + temp * 100);
      r = 255;
      g = Math.min(255, g);
      break;
    }
    case Material.Steam: {
      // Steam fades
      const alpha = Math.max(50, Math.floor((1 - particle.lifetime / 100) * 200));
      a = alpha;
      break;
    }
    case Material.Ice: {
      // Ice sparkle
      if (Math.random() < 0.05) {
        r = Math.min(255, r + 30);
        g = Math.min(255, g + 30);
        b = Math.min(255, b + 30);
      }
      break;
    }
  }

  return [r, g, b, a];
}

export class SimulationCanvas {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private width: number;
  private height: number;
  private scale: number;
  private grid: Grid;
  private imageData: ImageData;
  
  // Black hole tracking for gameplay metrics
  private blackHoles: Array<{ x: number; y: number; mass: number; consumed: number }> = [];
  private stats = { particlesConsumed: 0, totalMass: 0 };

  constructor(canvas: HTMLCanvasElement, width: number, height: number, scale: number = 4) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d', { alpha: false })!;
    this.width = width;
    this.height = height;
    this.scale = scale;

    // Set canvas size
    this.canvas.width = width * scale;
    this.canvas.height = height * scale;

    // Disable image smoothing for pixel-perfect rendering
    this.ctx.imageSmoothingEnabled = false;

    // Initialize grid
    this.grid = {
      width,
      height,
      particles: new Array(width * height).fill(null).map(() => ({
        material: Material.Air,
        temperature: 293,
        lifetime: 0,
        velocityX: 0,
        velocityY: 0,
        burning: false,
      })),
    };

    // Image data for pixel manipulation
    this.imageData = this.ctx.createImageData(width * scale, height * scale);
  }

  private getIndex(x: number, y: number): number {
    return y * this.grid.width + x;
  }

  private inBounds(x: number, y: number): boolean {
    return x >= 0 && x < this.grid.width && y >= 0 && y < this.grid.height;
  }

  get(x: number, y: number): Particle | null {
    if (!this.inBounds(x, y)) return null;
    return this.grid.particles[this.getIndex(x, y)];
  }

  set(x: number, y: number, material: Material): boolean {
    if (!this.inBounds(x, y)) return false;
    if (this.grid.particles[this.getIndex(x, y)].material !== Material.Air && material !== Material.Air) {
      return false;
    }
    const particle = this.grid.particles[this.getIndex(x, y)];
    particle.material = material;
    particle.lifetime = 0;
    if (material === Material.Fire) {
      particle.lifetime = 30 + Math.floor(Math.random() * 20);
    } else if (material === Material.Smoke || material === Material.Steam) {
      particle.lifetime = 60 + Math.floor(Math.random() * 40);
    }
    return true;
  }

  spawn(x: number, y: number, material: Material): boolean {
    return this.set(x, y, material);
  }

  remove(x: number, y: number): void {
    if (!this.inBounds(x, y)) return;
    this.grid.particles[this.getIndex(x, y)].material = Material.Air;
  }

  clear(): void {
    for (let i = 0; i < this.grid.particles.length; i++) {
      this.grid.particles[i].material = Material.Air;
    }
  }

  countParticles(): number {
    let count = 0;
    for (const p of this.grid.particles) {
      if (p.material !== Material.Air) count++;
    }
    return count;
  }

  // Physics simulation tick
  tick(): void {
    const { width, height, particles } = this.grid;

    // Create copy for simultaneous updates
    const newState = particles.map(p => ({ ...p }));

    // Process from bottom to top
    for (let y = height - 2; y >= 0; y--) {
      // Randomize left-right order to prevent bias
      const xOrder = Math.random() < 0.5
        ? Array.from({ length: width }, (_, i) => i)
        : Array.from({ length: width }, (_, i) => width - 1 - i);

      for (const x of xOrder) {
        const idx = y * width + x;
        const particle = particles[idx];

        if (particle.material === Material.Air) continue;

        // Process based on material
        switch (particle.material) {
          case Material.Sand:
          case Material.Ash:
            this.updateSand(newState, width, height, x, y, particle.material === Material.Ash);
            break;
          case Material.Water:
            this.updateWater(newState, width, height, x, y);
            break;
          case Material.Oil:
            this.updateOil(newState, width, height, x, y);
            break;
          case Material.Ice:
            this.updateIce(newState, width, height, x, y);
            break;
          case Material.Fire:
            this.updateFire(newState, width, height, x, y);
            break;
          case Material.Smoke:
          case Material.Steam:
            this.updateGas(newState, width, height, x, y, particle.material === Material.Steam);
            break;
          case Material.Lava:
            this.updateLava(newState, width, height, x, y);
            break;
          case Material.BlackHole:
            this.updateBlackHole(newState, width, height, x, y);
            break;
          case Material.Stone:
          case Material.Wood:
            // Static materials - no movement
            break;
        }
      }
    }

    // Update grid
    this.grid.particles = newState;
  }

  private updateSand(state: Particle[], width: number, height: number, x: number, y: number, isAsh: boolean): void {
    const idx = y * width + x;
    const fallChance = isAsh ? 0.3 : 1; // Ash falls slower

    // Try to fall down
    if (this.moveDown(state, width, height, x, y, idx)) return;

    // Try diagonal
    if (Math.random() < fallChance) {
      const left = x > 0 && y + 1 < height && state[(y + 1) * width + (x - 1)].material === Material.Air;
      const right = x + 1 < width && y + 1 < height && state[(y + 1) * width + (x + 1)].material === Material.Air;

      if (left && right) {
        state[(y + 1) * width + (x + (Math.random() < 0.5 ? -1 : 1))] = { ...state[idx] };
        state[idx].material = Material.Air;
      } else if (left) {
        state[(y + 1) * width + (x - 1)] = { ...state[idx] };
        state[idx].material = Material.Air;
      } else if (right) {
        state[(y + 1) * width + (x + 1)] = { ...state[idx] };
        state[idx].material = Material.Air;
      }
    }
  }

  private updateWater(state: Particle[], width: number, height: number, x: number, y: number): void {
    const idx = y * width + x;

    // Try to fall down
    if (this.moveDown(state, width, height, x, y, idx)) return;

    // Try diagonal
    const left = x > 0 && y + 1 < height && state[(y + 1) * width + (x - 1)].material === Material.Air;
    const right = x + 1 < width && y + 1 < height && state[(y + 1) * width + (x + 1)].material === Material.Air;

    if (left && right) {
      const targetX = x + (Math.random() < 0.5 ? -1 : 1);
      state[(y + 1) * width + targetX] = { ...state[idx] };
      state[idx].material = Material.Air;
      return;
    } else if (left) {
      state[(y + 1) * width + (x - 1)] = { ...state[idx] };
      state[idx].material = Material.Air;
      return;
    } else if (right) {
      state[(y + 1) * width + (x + 1)] = { ...state[idx] };
      state[idx].material = Material.Air;
      return;
    }

    // Flow horizontally
    if (Math.random() < 0.7) {
      const flowLeft = x > 0 && state[y * width + (x - 1)].material === Material.Air;
      const flowRight = x + 1 < width && state[y * width + (x + 1)].material === Material.Air;

      if (flowLeft && flowRight) {
        const targetX = x + (Math.random() < 0.5 ? -1 : 1);
        state[y * width + targetX] = { ...state[idx] };
        state[idx].material = Material.Air;
      } else if (flowLeft) {
        state[y * width + (x - 1)] = { ...state[idx] };
        state[idx].material = Material.Air;
      } else if (flowRight) {
        state[y * width + (x + 1)] = { ...state[idx] };
        state[idx].material = Material.Air;
      }
    }
  }

  private updateOil(state: Particle[], width: number, height: number, x: number, y: number): void {
    const idx = y * width + x;
    const p = state[idx];

    // Check if should ignite (near fire or lava)
    for (let dy = -1; dy <= 1; dy++) {
      for (let dx = -1; dx <= 1; dx++) {
        if (dx === 0 && dy === 0) continue;
        const nx = x + dx, ny = y + dy;
        if (!this.inBounds(nx, ny)) continue;
        const neighbor = state[ny * width + nx];
        if (neighbor.material === Material.Fire || neighbor.material === Material.Lava) {
          p.material = Material.Fire;
          p.lifetime = 60 + Math.floor(Math.random() * 20);
          return;
        }
      }
    }

    // Oil flows slower than water
    if (Math.random() < 0.3) {
      this.updateWater(state, width, height, x, y);
    }
  }

  private updateIce(state: Particle[], width: number, height: number, x: number, y: number): void {
    const idx = y * width + x;
    const p = state[idx];

    // Check if should melt (near heat)
    let nearHeat = false;
    for (let dy = -1; dy <= 1; dy++) {
      for (let dx = -1; dx <= 1; dx++) {
        if (dx === 0 && dy === 0) continue;
        const nx = x + dx, ny = y + dy;
        if (!this.inBounds(nx, ny)) continue;
        const neighbor = state[ny * width + nx];
        if (neighbor.material === Material.Fire || neighbor.material === Material.Lava) {
          nearHeat = true;
          break;
        }
      }
      if (nearHeat) break;
    }

    if (nearHeat) {
      p.material = Material.Water;
      return;
    }

    // Ice is slippery - can slide
    if (Math.random() < 0.2) {
      const slideLeft = x > 0 && state[y * width + (x - 1)].material === Material.Air;
      const slideRight = x + 1 < width && state[y * width + (x + 1)].material === Material.Air;

      if (slideLeft && slideRight) {
        const targetX = x + (Math.random() < 0.5 ? -1 : 1);
        state[y * width + targetX] = { ...p };
        state[idx].material = Material.Air;
      } else if (slideLeft) {
        state[y * width + (x - 1)] = { ...p };
        state[idx].material = Material.Air;
      } else if (slideRight) {
        state[y * width + (x + 1)] = { ...p };
        state[idx].material = Material.Air;
      }
    }
  }

  private updateFire(state: Particle[], width: number, height: number, x: number, y: number): void {
    const idx = y * width + x;
    const p = state[idx];

    // Decrease lifetime
    p.lifetime--;

    // Extinguish if in water
    for (let dy = -1; dy <= 1; dy++) {
      for (let dx = -1; dx <= 1; dx++) {
        if (dx === 0 && dy === 0) continue;
        const nx = x + dx, ny = y + dy;
        if (!this.inBounds(nx, ny)) continue;
        const neighbor = state[ny * width + nx];
        if (neighbor.material === Material.Water) {
          p.material = Material.Smoke;
          p.lifetime = 80 + Math.floor(Math.random() * 20);
          return;
        }
      }
    }

    // Spread to flammable materials
    const flammable = [Material.Oil, Material.Wood];
    if (Math.random() < 0.05) {
      for (let dy = -1; dy <= 1; dy++) {
        for (let dx = -1; dx <= 1; dx++) {
          if (dx === 0 && dy === 0) continue;
          const nx = x + dx, ny = y + dy;
          if (!this.inBounds(nx, ny)) continue;
          const neighborIdx = ny * width + nx;
          if (flammable.includes(state[neighborIdx].material) && Math.random() < 0.1) {
            state[neighborIdx].material = Material.Fire;
            state[neighborIdx].lifetime = 80;
          }
        }
      }
    }

    // Rise upward
    const rise = y > 0 && state[(y - 1) * width + x].material === Material.Air;
    if (rise && Math.random() < 0.7) {
      state[(y - 1) * width + x] = { ...p };
      state[idx].material = Material.Air;
      return;
    }

    // Random horizontal movement while rising
    if (Math.random() < 0.3) {
      const moveLeft = x > 0 && state[(y - 1) * width + (x - 1)].material === Material.Air;
      const moveRight = x + 1 < width && state[(y - 1) * width + (x + 1)].material === Material.Air;

      if (moveLeft && moveRight) {
        const targetX = x + (Math.random() < 0.5 ? -1 : 1);
        state[(y - 1) * width + targetX] = { ...p };
        state[idx].material = Material.Air;
      } else if (moveLeft) {
        state[(y - 1) * width + (x - 1)] = { ...p };
        state[idx].material = Material.Air;
      } else if (moveRight) {
        state[(y - 1) * width + (x + 1)] = { ...p };
        state[idx].material = Material.Air;
      }
    }

    // Die if lifetime exhausted
    if (p.lifetime <= 0) {
      p.material = Material.Smoke;
      p.lifetime = 60 + Math.floor(Math.random() * 40);
    }
  }

  private updateGas(state: Particle[], width: number, height: number, x: number, y: number, isSteam: boolean): void {
    const idx = y * width + x;
    const p = state[idx];

    // Decrease lifetime
    p.lifetime--;

    // Die if lifetime exhausted
    if (p.lifetime <= 0) {
      p.material = Material.Air;
      return;
    }

    // Steam condenses when hitting water
    if (isSteam) {
      for (let dy = -1; dy <= 1; dy++) {
        for (let dx = -1; dx <= 1; dx++) {
          if (dx === 0 && dy === 0) continue;
          const nx = x + dx, ny = y + dy;
          if (!this.inBounds(nx, ny)) continue;
          if (state[ny * width + nx].material === Material.Water) {
            p.material = Material.Air; // Steam disappears when touching water
            return;
          }
        }
      }
    }

    // Rise upward (faster for steam)
    const riseSpeed = isSteam ? 0.9 : 0.7;
    const rise = y > 0 && state[(y - 1) * width + x].material === Material.Air;
    if (rise && Math.random() < riseSpeed) {
      state[(y - 1) * width + x] = { ...p };
      state[idx].material = Material.Air;
      return;
    }

    // Random horizontal movement
    if (Math.random() < 0.4) {
      const moveLeft = x > 0 && state[y * width + (x - 1)].material === Material.Air;
      const moveRight = x + 1 < width && state[y * width + (x + 1)].material === Material.Air;

      if (moveLeft && moveRight) {
        const targetX = x + (Math.random() < 0.5 ? -1 : 1);
        state[y * width + targetX] = { ...p };
        state[idx].material = Material.Air;
      } else if (moveLeft) {
        state[y * width + (x - 1)] = { ...p };
        state[idx].material = Material.Air;
      } else if (moveRight) {
        state[y * width + (x + 1)] = { ...p };
        state[idx].material = Material.Air;
      }
    }
  }

  private updateLava(state: Particle[], width: number, height: number, x: number, y: number): void {
    const idx = y * width + x;
    const p = state[idx];

    // Heat nearby materials
    for (let dy = -1; dy <= 1; dy++) {
      for (let dx = -1; dx <= 1; dx++) {
        if (dx === 0 && dy === 0) continue;
        const nx = x + dx, ny = y + dy;
        if (!this.inBounds(nx, ny)) continue;
        const neighborIdx = ny * width + nx;
        const neighbor = state[neighborIdx];

        // Lava ignites flammable materials
        if (neighbor.material === Material.Oil || neighbor.material === Material.Wood) {
          if (Math.random() < 0.05) {
            neighbor.material = Material.Fire;
            neighbor.lifetime = 80;
          }
        }

        // Lava cools water into stone
        if (neighbor.material === Material.Water && Math.random() < 0.1) {
          neighbor.material = Material.Stone;
          p.material = Material.Stone; // Lava also solidifies
        }
      }
    }

    // Lava flows slowly
    if (Math.random() < 0.25) {
      this.updateWater(state, width, height, x, y);
    }
  }

  // Black hole mass constants
  private readonly BLACK_HOLE_MASS = 100;
  private readonly EVENT_HORIZON_RADIUS = 3;
  private readonly INFLUENCE_RADIUS = 40;
  private readonly GRAVITY_CONSTANT = 500;

  private updateBlackHole(state: Particle[], width: number, height: number, x: number, y: number): void {
    const idx = y * width + x;
    const p = state[idx];
    
    // Find or create black hole entry
    let bh = this.blackHoles.find(b => b.x === x && b.y === y);
    if (!bh) {
      bh = { x, y, mass: this.BLACK_HOLE_MASS, consumed: 0 };
      this.blackHoles.push(bh);
    }

    // Apply gravitational pull to nearby particles
    for (let dy = -this.INFLUENCE_RADIUS; dy <= this.INFLUENCE_RADIUS; dy += 4) {
      for (let dx = -this.INFLUENCE_RADIUS; dx <= this.INFLUENCE_RADIUS; dx += 4) {
        const nx = x + dx;
        const ny = y + dy;
        if (!this.inBounds(nx, ny)) continue;
        
        const neighborIdx = ny * width + nx;
        const neighbor = state[neighborIdx];
        
        // Skip empty, black hole, and static materials
        if (neighbor.material === Material.Air || 
            neighbor.material === Material.Stone || 
            neighbor.material === Material.Wood ||
            neighbor.material === Material.BlackHole) continue;
        
        // Calculate distance to black hole
        const distSq = dx * dx + dy * dy;
        const dist = Math.sqrt(distSq);
        
        if (dist < this.EVENT_HORIZON_RADIUS) {
          // Consume particle (event horizon)
          this.stats.particlesConsumed++;
          this.stats.totalMass += this.getParticleMass(neighbor.material);
          bh.consumed++;
          state[neighborIdx].material = Material.Air;
        } else if (dist < this.INFLUENCE_RADIUS) {
          // Apply gravitational acceleration
          const force = this.GRAVITY_CONSTANT * bh.mass / (distSq + 100);
          const ax = (dx / dist) * force;
          const ay = (dy / dist) * force;
          
          // Update velocity with damping
          neighbor.velocityX = neighbor.velocityX * 0.95 + ax * 0.05;
          neighbor.velocityY = neighbor.velocityY * 0.95 + ay * 0.05;
          
          // Clamp velocity
          const maxVel = 5;
          const vel = Math.sqrt(neighbor.velocityX ** 2 + neighbor.velocityY ** 2);
          if (vel > maxVel) {
            neighbor.velocityX = (neighbor.velocityX / vel) * maxVel;
            neighbor.velocityY = (neighbor.velocityY / vel) * maxVel;
          }
          
          // Apply velocity-based movement if significant
          if (vel > 0.5) {
            const moveX = Math.round(neighbor.velocityX);
            const moveY = Math.round(neighbor.velocityY);
            
            if (moveX !== 0 || moveY !== 0) {
              const targetX = Math.max(0, Math.min(width - 1, nx + moveX));
              const targetY = Math.max(0, Math.min(height - 1, ny + moveY));
              
              if (state[targetY * width + targetX].material === Material.Air) {
                state[targetY * width + targetX] = { ...neighbor };
                state[neighborIdx].material = Material.Air;
                state[targetY * width + targetX].velocityX *= 0.9;
                state[targetY * width + targetX].velocityY *= 0.9;
              }
            }
          }
        }
      }
    }
    
    // Hawking radiation - emit small particles
    if (Math.random() < 0.02) {
      const angle = Math.random() * Math.PI * 2;
      const radius = this.EVENT_HORIZON_RADIUS + 1;
      const ex = Math.round(x + Math.cos(angle) * radius);
      const ey = Math.round(y + Math.sin(angle) * radius);
      
      if (this.inBounds(ex, ey) && state[ey * width + ex].material === Material.Air) {
        state[ey * width + ex] = {
          material: Math.random() < 0.5 ? Material.Fire : Material.Smoke,
          temperature: 800 + Math.random() * 400,
          lifetime: 30 + Math.floor(Math.random() * 20),
          velocityX: Math.cos(angle) * 2,
          velocityY: Math.sin(angle) * 2,
          burning: false
        };
      }
    }
  }
  
  private getParticleMass(material: Material): number {
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

  private moveDown(state: Particle[], width: number, height: number, x: number, y: number, idx: number): boolean {
    if (y + 1 >= height) return false;
    const below = (y + 1) * width + x;
    if (state[below].material === Material.Air) {
      state[below] = { ...state[idx] };
      state[idx].material = Material.Air;
      return true;
    }
    return false;
  }

  // Render the grid to canvas
  render(): void {
    const { width, height, particles } = this.grid;
    const data = this.imageData.data;
    const scale = this.scale;

    // Clear to background
    const bg = MATERIAL_COLORS[Material.Air];
    for (let i = 0; i < data.length; i += 4) {
      data[i] = bg[0];
      data[i + 1] = bg[1];
      data[i + 2] = bg[2];
      data[i + 3] = 255;
    }

    // Draw particles
    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const particle = particles[y * width + x];
        if (particle.material === Material.Air) continue;

        const [r, g, b, a] = getParticleColor(particle, x, y);

        // Fill scaled block
        for (let dy = 0; dy < scale; dy++) {
          for (let dx = 0; dx < scale; dx++) {
            const px = x * scale + dx;
            const py = y * scale + dy;
            const idx = (py * width * scale + px) * 4;
            data[idx] = r;
            data[idx + 1] = g;
            data[idx + 2] = b;
            data[idx + 3] = a;
          }
        }
      }
    }

    this.ctx.putImageData(this.imageData, 0, 0);
  }

  // Spawn particles in a circular brush pattern
  spawnBrush(centerX: number, centerY: number, material: Material, radius: number): number {
    let count = 0;
    const gridX = Math.floor(centerX / this.scale);
    const gridY = Math.floor(centerY / this.scale);
    const r = Math.floor(radius / this.scale);

    for (let dy = -r; dy <= r; dy++) {
      for (let dx = -r; dx <= r; dx++) {
        const dist = Math.sqrt(dx * dx + dy * dy);
        if (dist <= r) {
          if (this.spawn(gridX + dx, gridY + dy, material)) {
            count++;
          }
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
      const t = steps === 0 ? 0 : i / steps;
      const x = Math.round(x0 + (x1 - x0) * t);
      const y = Math.round(y0 + (y1 - y0) * t);
      count += this.spawnBrush(x, y, material, radius);
    }

    return count;
  }
  
  // Get gameplay statistics
  getStats(): { particlesConsumed: number; totalMass: number; blackHoles: number } {
    return {
      particlesConsumed: this.stats.particlesConsumed,
      totalMass: this.stats.totalMass,
      blackHoles: this.blackHoles.length
    };
  }
  
  // Reset gameplay stats
  resetStats(): void {
    this.stats = { particlesConsumed: 0, totalMass: 0 };
    this.blackHoles = [];
  }
  
  // Spawn a pre-built structure
  spawnStructure(type: 'ship' | 'asteroid' | 'station', centerX: number, centerY: number): void {
    const gridCX = Math.floor(centerX / this.scale);
    const gridCY = Math.floor(centerY / this.scale);
    
    switch (type) {
      case 'ship':
        // Small triangular ship
        for (let y = 0; y < 8; y++) {
          for (let x = -y; x <= y; x++) {
            if (Math.abs(x) <= y || y === 0) {
              this.spawn(gridCX + x, gridCY + y, Material.Stone);
            }
          }
        }
        // Engine at back
        for (let i = 0; i < 3; i++) {
          this.spawn(gridCX, gridCY + 8 + i, Material.Fire);
        }
        break;
        
      case 'asteroid':
        // Irregular rock cluster
        for (let angle = 0; angle < Math.PI * 2; angle += 0.3) {
          const radius = 5 + Math.random() * 3;
          const px = Math.round(gridCX + Math.cos(angle) * radius);
          const py = Math.round(gridCY + Math.sin(angle) * radius);
          this.spawn(px, py, Material.Stone);
          // Fill interior
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
        // Central hub with arms
        for (let r = 0; r < 5; r++) {
          for (let angle = 0; angle < Math.PI * 2; angle += Math.PI / 3) {
            const px = Math.round(gridCX + Math.cos(angle) * (r + 3));
            const py = Math.round(gridCY + Math.sin(angle) * (r + 3));
            this.spawn(px, py, Material.Stone);
          }
        }
        // Central core
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
}