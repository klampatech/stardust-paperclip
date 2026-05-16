// FUL-45: Gravity Gun Tool
// Interactive particle manipulation tool for the space black hole game

export type GravityGunMode = 'attract' | 'repel' | 'vortex';

export interface GravityGunConfig {
  mode: GravityGunMode;
  strength: number;      // 1-10, upgradeable
  radius: number;        // pixels affected
  maxParticles: number;  // particles affected at once
}

export interface GravityGunState {
  active: boolean;
  mode: GravityGunMode;
  cursorX: number;
  cursorY: number;
  strength: number;
  radius: number;
  shipX?: number;
  shipY?: number;
}

export const DEFAULT_GUN_CONFIG: GravityGunConfig = {
  mode: 'attract',
  strength: 5,
  radius: 80,
  maxParticles: 200,
};

export const GUN_MODE_COSTS = {
  attract: 0,
  repel: 0,
  vortex: 3, // Costs more energy
};

export function createGravityGunState(): GravityGunState {
  return {
    active: false,
    mode: 'attract',
    cursorX: 0,
    cursorY: 0,
    strength: 5,
    radius: 80,
  };
}

export function updateGravityGunState(
  state: GravityGunState,
  updates: Partial<GravityGunState>
): GravityGunState {
  return { ...state, ...updates };
}

// Apply gravity gun physics to particles
export function applyGravityGun(
  state: GravityGunState,
  particles: Array<{ x: number; y: number; velocityX: number; velocityY: number; material: string }>,
  getIndex: (x: number, y: number) => number,
  inBounds: (x: number, y: number) => boolean,
  gridWidth: number,
  gridHeight: number
): { particlesToMove: Map<number, { dx: number; dy: number }>; energyCost: number } {
  if (!state.active) {
    return { particlesToMove: new Map(), energyCost: 0 };
  }

  const particlesToMove = new Map<number, { dx: number; dy: number }>();
  const { cursorX, cursorY, strength, radius, mode } = state;
  const strengthFactor = strength * 0.1;
  const radiusSq = radius * radius;

  let energyCost = 0;
  let affected = 0;
  const maxParticles = 200;

  for (let i = 0; i < particles.length && affected < maxParticles; i++) {
    const p = particles[i];
    if (p.material === 'Air' || p.material === 'BlackHole') continue;

    const dx = cursorX - p.x;
    const dy = cursorY - p.y;
    const distSq = dx * dx + dy * dy;

    if (distSq > radiusSq || distSq < 1) continue;

    const dist = Math.sqrt(distSq);
    let force = (strengthFactor * 50) / (distSq + 10);

    // Vortex adds rotational component
    if (mode === 'vortex') {
      force *= 0.7;
      const perpX = -dy / dist;
      const perpY = dx / dist;
      const rotDx = perpX * force * 20;
      const rotDy = perpY * force * 20;
      particlesToMove.set(i, { dx: rotDx, dy: rotDy });
      energyCost += GUN_MODE_COSTS.vortex;
    } else {
      const direction = mode === 'attract' ? 1 : -1;
      const normX = dx / dist;
      const normY = dy / dist;
      particlesToMove.set(i, {
        dx: normX * force * 20 * direction,
        dy: normY * force * 20 * direction,
      });
      energyCost += GUN_MODE_COSTS[mode];
    }

    affected++;
  }

  return { particlesToMove, energyCost };
}

// Render gravity gun visual effect
// FUL-47.4: Added beam from ship to cursor
// FUL-47.4: Added particle effects in beam path
export function renderGravityGunEffect(
  ctx: CanvasRenderingContext2D,
  state: GravityGunState,
  scale: number
): void {
  if (!state.active) return;

  const { cursorX, cursorY, mode, radius, shipX, shipY } = state;
  
  // Draw beam from ship to cursor if ship position is available
  if (shipX !== undefined && shipY !== undefined) {
    const beamLength = Math.sqrt((cursorX - shipX) ** 2 + (cursorY - shipY) ** 2);
    const beamAngle = Math.atan2(cursorY - shipY, cursorX - shipX);
    
    // Beam colors by mode
    const beamColor = mode === 'attract' ? { r: 100, g: 200, b: 255 } :
                      mode === 'repel' ? { r: 255, g: 100, b: 100 } :
                      { r: 150, g: 100, b: 255 };
    
    // Draw main beam (cone from ship)
    const beamWidth = 15; // Base width
    const gradient = ctx.createLinearGradient(shipX, shipY, cursorX, cursorY);
    gradient.addColorStop(0, `rgba(${beamColor.r}, ${beamColor.g}, ${beamColor.b}, 0.8)`);
    gradient.addColorStop(0.3, `rgba(${beamColor.r}, ${beamColor.g}, ${beamColor.b}, 0.5)`);
    gradient.addColorStop(1, `rgba(${beamColor.r}, ${beamColor.g}, ${beamColor.b}, 0.1)`);
    
    // Draw tapered beam shape
    ctx.save();
    ctx.beginPath();
    ctx.moveTo(shipX, shipY);
    
    // Calculate perpendicular for beam width
    const perpX = Math.sin(beamAngle) * beamWidth;
    const perpY = -Math.cos(beamAngle) * beamWidth;
    
    ctx.lineTo(cursorX + perpX, cursorY + perpY);
    ctx.lineTo(cursorX - perpX, cursorY - perpY);
    ctx.closePath();
    ctx.fillStyle = gradient;
    ctx.fill();
    ctx.restore();
    
    // Draw particle effects along beam
    const particleCount = 8;
    for (let i = 0; i < particleCount; i++) {
      const t = Math.random();
      const px = shipX + (cursorX - shipX) * t + (Math.random() - 0.5) * 10;
      const py = shipY + (cursorY - shipY) * t + (Math.random() - 0.5) * 10;
      const size = 2 + Math.random() * 3;
      const alpha = (1 - t) * 0.6;
      
      ctx.fillStyle = `rgba(${beamColor.r}, ${beamColor.g}, ${beamColor.b}, ${alpha})`;
      ctx.beginPath();
      ctx.arc(px, py, size, 0, Math.PI * 2);
      ctx.fill();
    }
  }

  // Draw affected radius
  ctx.beginPath();
  ctx.arc(cursorX, cursorY, radius, 0, Math.PI * 2);
  ctx.strokeStyle = mode === 'attract' ? 'rgba(100, 200, 255, 0.3)' :
                    mode === 'repel' ? 'rgba(255, 100, 100, 0.3)' :
                    'rgba(150, 100, 255, 0.3)';
  ctx.lineWidth = 2;
  ctx.stroke();

  // Draw mode-specific effect
  if (mode === 'attract') {
    // Inward arrows
    ctx.strokeStyle = 'rgba(100, 200, 255, 0.6)';
    for (let angle = 0; angle < Math.PI * 2; angle += Math.PI / 6) {
      const innerR = radius * 0.6;
      const outerR = radius * 0.9;
      ctx.beginPath();
      ctx.moveTo(cursorX + Math.cos(angle) * outerR, cursorY + Math.sin(angle) * outerR);
      ctx.lineTo(cursorX + Math.cos(angle) * innerR, cursorY + Math.sin(angle) * innerR);
      ctx.stroke();
    }
  } else if (mode === 'repel') {
    // Outward arrows
    ctx.strokeStyle = 'rgba(255, 100, 100, 0.6)';
    for (let angle = 0; angle < Math.PI * 2; angle += Math.PI / 6) {
      const innerR = radius * 0.1;
      const outerR = radius * 0.4;
      ctx.beginPath();
      ctx.moveTo(cursorX + Math.cos(angle) * innerR, cursorY + Math.sin(angle) * innerR);
      ctx.lineTo(cursorX + Math.cos(angle) * outerR, cursorY + Math.sin(angle) * outerR);
      ctx.stroke();
    }
  } else {
    // Vortex spiral
    ctx.strokeStyle = 'rgba(150, 100, 255, 0.6)';
    ctx.beginPath();
    for (let t = 0; t < Math.PI * 4; t += 0.1) {
      const r = 10 + t * (radius / (Math.PI * 4));
      const x = cursorX + Math.cos(t) * r;
      const y = cursorY + Math.sin(t) * r;
      if (t === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.stroke();
  }

  // Draw center crosshair
  ctx.beginPath();
  ctx.strokeStyle = 'rgba(255, 255, 255, 0.8)';
  ctx.lineWidth = 1;
  ctx.moveTo(cursorX - 10, cursorY);
  ctx.lineTo(cursorX + 10, cursorY);
  ctx.moveTo(cursorX, cursorY - 10);
  ctx.lineTo(cursorX, cursorY + 10);
  ctx.stroke();
}