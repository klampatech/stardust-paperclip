// FUL-47.2: Object Spawning System
// Continuous spawning of space objects
// Objects naturally attracted toward black hole center

export type ObjectType = 'asteroid' | 'comet' | 'debris' | 'enemy_ship' | 'freighter' | 'planet' | 'neutron_star';

export interface SpaceObject {
  id: string;
  x: number;
  y: number;
  size: number;
  material: 'sand' | 'stone' | 'ice' | 'fire' | 'water';
  objectType: ObjectType;
  rotation: number;
  rotationSpeed: number;
  velocityX: number;
  velocityY: number;
  collected: boolean;
  spawnTime: number;
  points: number;
  mass: number;
  color: string;
  glowColor?: string;
  isSpecial: boolean;
}

export interface SpawnConfig {
  enabled: boolean;
  intervalMs: number;
  maxActiveDebris: number;
  spawnAtEdge: boolean;
  types: SpawnableObjectType[];
  blackHoleAttraction: boolean;
  blackHolePosition?: { x: number; y: number };
  attractionStrength: number;
  attractionRadius: number;
}

export interface SpawnableObjectType {
  id: string;
  name: string;
  objectType: ObjectType;
  material: 'sand' | 'stone' | 'ice' | 'fire' | 'water';
  sizeRange: [number, number];
  weight: number;
  points: number;
  collectRadius: number;
  mass: number;
  color: string;
  glowColor?: string;
  isSpecial?: boolean;
}

export const DEFAULT_SPAWN_TYPES: SpawnableObjectType[] = [
  { id: 'sand_chunk', name: 'Sand Chunk', objectType: 'asteroid', material: 'sand', sizeRange: [1, 2], weight: 35, points: 10, collectRadius: 20, mass: 1.0, color: '#C2B280' },
  { id: 'rock_fragment', name: 'Rock Fragment', objectType: 'asteroid', material: 'stone', sizeRange: [2, 3], weight: 30, points: 15, collectRadius: 25, mass: 1.5, color: '#808080' },
  { id: 'ice_crystal', name: 'Ice Crystal', objectType: 'asteroid', material: 'ice', sizeRange: [1, 2], weight: 25, points: 12, collectRadius: 18, mass: 0.8, color: '#ADD8FA' },
  { id: 'small_comet', name: 'Small Comet', objectType: 'comet', material: 'ice', sizeRange: [2, 4], weight: 10, points: 25, collectRadius: 30, mass: 2.0, color: '#87CEEB', glowColor: 'rgba(135, 206, 235, 0.5)' },
  { id: 'scout_ship', name: 'Scout Ship', objectType: 'enemy_ship', material: 'stone', sizeRange: [3, 4], weight: 8, points: 50, collectRadius: 35, mass: 3.0, color: '#FF4444' },
  { id: 'fighter_ship', name: 'Fighter', objectType: 'enemy_ship', material: 'stone', sizeRange: [4, 5], weight: 5, points: 75, collectRadius: 40, mass: 4.0, color: '#FF6600' },
  { id: 'freighter', name: 'Freighter', objectType: 'freighter', material: 'stone', sizeRange: [5, 7], weight: 3, points: 100, collectRadius: 50, mass: 8.0, color: '#8888FF' },
  { id: 'small_planet', name: 'Small Planet', objectType: 'planet', material: 'stone', sizeRange: [6, 8], weight: 2, points: 200, collectRadius: 60, mass: 15.0, color: '#44AA44', isSpecial: true },
  { id: 'gas_giant', name: 'Gas Giant', objectType: 'planet', material: 'water', sizeRange: [10, 15], weight: 1, points: 500, collectRadius: 100, mass: 30.0, color: '#FFAA44', isSpecial: true },
  { id: 'neutron_star', name: 'Neutron Star', objectType: 'neutron_star', material: 'fire', sizeRange: [3, 5], weight: 0.5, points: 1000, collectRadius: 45, mass: 50.0, color: '#FFFFFF', glowColor: 'rgba(255, 255, 255, 0.8)', isSpecial: true },
];

export const DEFAULT_SPAWN_CONFIG: SpawnConfig = {
  enabled: true,
  intervalMs: 3000,
  maxActiveDebris: 20,
  spawnAtEdge: true,
  types: DEFAULT_SPAWN_TYPES,
  blackHoleAttraction: true,
  blackHolePosition: undefined,
  attractionStrength: 50.0,
  attractionRadius: 1000.0,
};

let debrisIdCounter = 0;

function generateDebrisId(): string {
  return 'debris_' + (++debrisIdCounter) + '_' + Date.now();
}

export class ObjectSpawner {
  private objects: SpaceObject[] = [];
  private config: SpawnConfig;
  private lastSpawnTime: number = 0;
  private canvasWidth: number;
  private canvasHeight: number;
  
  public onObjectCollected?: (obj: SpaceObject, points: number) => void;
  
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
  
  setBlackHolePosition(x: number, y: number): void {
    this.config.blackHolePosition = { x, y };
  }
  
  tick(deltaTime: number = 1): void {
    const now = Date.now();
    
    if (this.config.enabled && now - this.lastSpawnTime >= this.config.intervalMs) {
      if (this.objects.filter(o => !o.collected).length < this.config.maxActiveDebris) {
        this.spawnRandomObject();
        this.lastSpawnTime = now;
      }
    }
    
    for (const obj of this.objects) {
      if (obj.collected) continue;
      
      if (this.config.blackHoleAttraction && this.config.blackHolePosition) {
        this.applyBlackHoleAttraction(obj);
      }
      
      obj.x += obj.velocityX * deltaTime;
      obj.y += obj.velocityY * deltaTime;
      obj.rotation += obj.rotationSpeed * deltaTime;
      
      if (!this.isNearBlackHole(obj.x, obj.y)) {
        if (obj.x < -30) obj.x = this.canvasWidth + 30;
        if (obj.x > this.canvasWidth + 30) obj.x = -30;
        if (obj.y < -30) obj.y = this.canvasHeight + 30;
        if (obj.y > this.canvasHeight + 30) obj.y = -30;
      }
    }
    
    this.objects = this.objects.filter(o => !o.collected);
  }
  
  private applyBlackHoleAttraction(obj: SpaceObject): void {
    const bh = this.config.blackHolePosition!;
    const dx = bh.x - obj.x;
    const dy = bh.y - obj.y;
    const distSq = dx * dx + dy * dy;
    const dist = Math.sqrt(distSq);
    
    if (dist > this.config.attractionRadius || dist < 10) return;
    
    const G = this.config.attractionStrength;
    const force = G * obj.mass / (distSq + 100);
    
    const dirX = dx / dist;
    const dirY = dy / dist;
    
    const maxAccel = 5.0;
    const accelX = Math.min(Math.max(dirX * force, -maxAccel), maxAccel);
    const accelY = Math.min(Math.max(dirY * force, -maxAccel), maxAccel);
    
    obj.velocityX += accelX * 0.1;
    obj.velocityY += accelY * 0.1;
    
    const maxVel = 3.0;
    const velMag = Math.sqrt(obj.velocityX * obj.velocityX + obj.velocityY * obj.velocityY);
    if (velMag > maxVel) {
      obj.velocityX = (obj.velocityX / velMag) * maxVel;
      obj.velocityY = (obj.velocityY / velMag) * maxVel;
    }
  }
  
  private isNearBlackHole(x: number, y: number): boolean {
    if (!this.config.blackHolePosition) return false;
    const bh = this.config.blackHolePosition;
    const dx = bh.x - x;
    const dy = bh.y - y;
    return Math.sqrt(dx * dx + dy * dy) < 200;
  }
  
  spawnRandomObject(): void {
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
    
    const size = selectedType.sizeRange[0] + Math.random() * (selectedType.sizeRange[1] - selectedType.sizeRange[0]);
    
    let x: number, y: number;
    if (this.config.spawnAtEdge) {
      const edge = Math.floor(Math.random() * 4);
      const margin = 50;
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
    
    let vx = 0, vy = 0;
    if (this.config.blackHolePosition) {
      const bh = this.config.blackHolePosition;
      const dx = bh.x - x;
      const dy = bh.y - y;
      const dist = Math.sqrt(dx * dx + dy * dy);
      vx = (dy / dist) * (0.3 + Math.random() * 0.2);
      vy = (-dx / dist) * (0.3 + Math.random() * 0.2);
    }
    
    const obj: SpaceObject = {
      id: generateDebrisId(),
      x,
      y,
      size,
      material: selectedType.material,
      objectType: selectedType.objectType,
      rotation: Math.random() * Math.PI * 2,
      rotationSpeed: (Math.random() - 0.5) * 0.1,
      velocityX: vx + (Math.random() - 0.5) * 0.3,
      velocityY: vy + (Math.random() - 0.5) * 0.3,
      collected: false,
      spawnTime: Date.now(),
      points: selectedType.points,
      mass: selectedType.mass,
      color: selectedType.color,
      glowColor: selectedType.glowColor,
      isSpecial: selectedType.isSpecial || false,
    };
    
    this.objects.push(obj);
  }
  
  spawnObjectAt(x: number, y: number, objectTypeId?: string): void {
    let objType: SpawnableObjectType;
    if (objectTypeId) {
      const found = this.config.types.find(t => t.id === objectTypeId);
      objType = found || this.config.types[0];
    } else {
      objType = this.config.types[Math.floor(Math.random() * this.config.types.length)];
    }
    const size = objType.sizeRange[0] + Math.random() * (objType.sizeRange[1] - objType.sizeRange[0]);
    
    const obj: SpaceObject = {
      id: generateDebrisId(),
      x,
      y,
      size,
      material: objType.material,
      objectType: objType.objectType,
      rotation: Math.random() * Math.PI * 2,
      rotationSpeed: (Math.random() - 0.5) * 0.05,
      velocityX: (Math.random() - 0.5) * 0.2,
      velocityY: (Math.random() - 0.5) * 0.2,
      collected: false,
      spawnTime: Date.now(),
      points: objType.points,
      mass: objType.mass,
      color: objType.color,
      glowColor: objType.glowColor,
      isSpecial: objType.isSpecial || false,
    };
    
    this.objects.push(obj);
  }
  
  checkCollection(playerX: number, playerY: number, playerRadius: number = 15): void {
    for (const obj of this.objects) {
      if (obj.collected) continue;
      
      const dx = playerX - obj.x;
      const dy = playerY - obj.y;
      const dist = Math.sqrt(dx * dx + dy * dy);
      
      const collectRadius = (obj.size * 8) * (obj.isSpecial ? 1.5 : 1.0);
      
      if (dist < playerRadius + collectRadius) {
        obj.collected = true;
        if (this.onObjectCollected) {
          this.onObjectCollected(obj, obj.points);
        }
      }
    }
  }
  
  getActiveObjects(): SpaceObject[] {
    return this.objects.filter(o => !o.collected);
  }
  
  getObjectCount(): { active: number; total: number } {
    const active = this.objects.filter(o => !o.collected).length;
    return { active, total: this.objects.length };
  }
  
  getObjectsByType(objectType: ObjectType): SpaceObject[] {
    return this.objects.filter(o => o.objectType === objectType && !o.collected);
  }
  
  getSpecialObjects(): SpaceObject[] {
    return this.objects.filter(o => o.isSpecial && !o.collected);
  }
  
  clear(): void {
    this.objects = [];
  }
  
  spawnInitialField(count: number = 5): void {
    for (let i = 0; i < count; i++) {
      const x = Math.random() * this.canvasWidth;
      const y = Math.random() * this.canvasHeight;
      this.spawnObjectAt(x, y);
    }
  }
  
  spawnObjectType(typeId: string): SpaceObject | null {
    const found = this.config.types.find(t => t.id === typeId);
    if (!found) return null;
    
    const edge = Math.floor(Math.random() * 4);
    const margin = 50;
    let x: number, y: number;
    switch (edge) {
      case 0: x = margin; y = Math.random() * this.canvasHeight; break;
      case 1: x = this.canvasWidth - margin; y = Math.random() * this.canvasHeight; break;
      case 2: x = Math.random() * this.canvasWidth; y = margin; break;
      default: x = Math.random() * this.canvasWidth; y = this.canvasHeight - margin; break;
    }
    
    this.spawnObjectAt(x, y, typeId);
    return this.objects[this.objects.length - 1];
  }
}

function drawIrregularShape(ctx: CanvasRenderingContext2D, size: number, points: number): void {
  ctx.beginPath();
  const angleStep = (Math.PI * 2) / points;
  for (let i = 0; i < points; i++) {
    const angle = i * angleStep;
    const variance = 0.7 + Math.random() * 0.3;
    const px = Math.cos(angle) * size * variance;
    const py = Math.sin(angle) * size * variance;
    if (i === 0) ctx.moveTo(px, py);
    else ctx.lineTo(px, py);
  }
  ctx.closePath();
  ctx.fill();
  ctx.stroke();
}

export function renderObjects(ctx: CanvasRenderingContext2D, objects: SpaceObject[], scale: number = 4): void {
  for (const obj of objects) {
    if (obj.collected) continue;
    
    ctx.save();
    ctx.translate(obj.x, obj.y);
    ctx.rotate(obj.rotation);
    
    const baseSize = obj.size * 6 * scale;
    
    if (obj.glowColor) {
      const glowGradient = ctx.createRadialGradient(0, 0, 0, 0, 0, baseSize * 1.5);
      glowGradient.addColorStop(0, obj.glowColor);
      glowGradient.addColorStop(1, 'transparent');
      ctx.fillStyle = glowGradient;
      ctx.beginPath();
      ctx.arc(0, 0, baseSize * 1.5, 0, Math.PI * 2);
      ctx.fill();
    }
    
    ctx.fillStyle = obj.color;
    ctx.strokeStyle = obj.isSpecial ? '#FFFFFF' : 'rgba(255,255,255,0.3)';
    ctx.lineWidth = obj.isSpecial ? 2 : 1;
    
    switch (obj.objectType) {
      case 'asteroid':
      case 'debris':
        drawIrregularShape(ctx, baseSize, 6);
        break;
      case 'comet':
        ctx.beginPath();
        ctx.arc(0, 0, baseSize * 0.7, 0, Math.PI * 2);
        ctx.fill();
        ctx.stroke();
        ctx.fillStyle = 'rgba(200, 220, 255, 0.3)';
        ctx.beginPath();
        ctx.moveTo(-baseSize * 0.3, 0);
        ctx.lineTo(-baseSize * 2, -baseSize * 0.3);
        ctx.lineTo(-baseSize * 2, baseSize * 0.3);
        ctx.closePath();
        ctx.fill();
        break;
      case 'enemy_ship':
        ctx.beginPath();
        ctx.moveTo(baseSize, 0);
        ctx.lineTo(-baseSize * 0.7, -baseSize * 0.6);
        ctx.lineTo(-baseSize * 0.5, 0);
        ctx.lineTo(-baseSize * 0.7, baseSize * 0.6);
        ctx.closePath();
        ctx.fill();
        ctx.stroke();
        break;
      case 'freighter':
        ctx.fillRect(-baseSize * 0.8, -baseSize * 0.3, baseSize * 1.6, baseSize * 0.6);
        ctx.strokeRect(-baseSize * 0.8, -baseSize * 0.3, baseSize * 1.6, baseSize * 0.6);
        ctx.fillStyle = 'rgba(255, 100, 0, 0.5)';
        ctx.beginPath();
        ctx.arc(-baseSize * 0.8, 0, baseSize * 0.15, 0, Math.PI * 2);
        ctx.fill();
        break;
      case 'planet':
        ctx.beginPath();
        ctx.arc(0, 0, baseSize, 0, Math.PI * 2);
        ctx.fill();
        ctx.stroke();
        ctx.strokeStyle = 'rgba(100, 200, 255, 0.2)';
        ctx.beginPath();
        ctx.arc(0, 0, baseSize * 1.2, 0, Math.PI * 2);
        ctx.stroke();
        break;
      case 'neutron_star':
        ctx.beginPath();
        ctx.arc(0, 0, baseSize * 0.5, 0, Math.PI * 2);
        ctx.fill();
        ctx.stroke();
        for (let i = 0; i < 4; i++) {
          const angle = (i / 4) * Math.PI * 2 + obj.rotation * 2;
          ctx.beginPath();
          ctx.moveTo(0, 0);
          ctx.lineTo(Math.cos(angle) * baseSize * 2, Math.sin(angle) * baseSize * 2);
          ctx.strokeStyle = 'rgba(255, 255, 255, 0.3)';
          ctx.stroke();
        }
        break;
      default:
        drawIrregularShape(ctx, baseSize, 6);
    }
    
    ctx.restore();
  }
}

export function renderDebris(ctx: CanvasRenderingContext2D, debris: SpaceObject[], scale: number = 4): void {
  renderObjects(ctx, debris, scale);
}

export const DebrisManager = ObjectSpawner;
