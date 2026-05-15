// FUL-35c: Spacecraft Control System
// Handles player-controlled spacecraft movement, physics, and game mechanics

import { 
  Spacecraft, 
  ShipClass, 
  createSpacecraft, 
  SHIP_CLASS_DEFAULTS 
} from './spacecraft';

// Control state for keyboard input
interface ControlState {
  thrust: boolean;    // W or Up arrow
  reverse: boolean;    // S or Down arrow
  rotateLeft: boolean; // A or Left arrow
  rotateRight: boolean; // D or Right arrow
  fire: boolean;       // Space
}

// Physics constants for spacecraft movement
const THRUST_POWER = 0.15;
const ROTATION_SPEED = 0.08;
const FUEL_CONSUMPTION = 0.05; // Per tick while thrusting
const MAX_VELOCITY = 10;
const DRAG = 0.995; // Space drag (very low)
const SHIP_RADIUS = 12;

export class SpacecraftControl {
  private spacecraft: Spacecraft;
  private controls: ControlState = {
    thrust: false,
    reverse: false,
    rotateLeft: false,
    rotateRight: false,
    fire: false,
  };
  private onStateChange: ((ship: Spacecraft) => void) | null = null;

  constructor(
    spacecraft: Spacecraft,
    onStateChange?: (ship: Spacecraft) => void
  ) {
    this.spacecraft = spacecraft;
    this.onStateChange = onStateChange || null;
  }

  // Set the callback for state changes
  setOnStateChange(callback: (ship: Spacecraft) => void): void {
    this.onStateChange = callback;
  }

  // Get current spacecraft state
  getSpacecraft(): Spacecraft {
    return { ...this.spacecraft };
  }

  // Handle keyboard input
  setControl(control: keyof ControlState, active: boolean): void {
    this.controls[control] = active;
  }

  // Physics tick - call this every frame
  tick(deltaTime: number = 1): void {
    const ship = this.spacecraft;
    
    if (ship.isDestroyed) return;

    // Rotation
    if (this.controls.rotateLeft) {
      ship.angle -= ROTATION_SPEED * deltaTime;
    }
    if (this.controls.rotateRight) {
      ship.angle += ROTATION_SPEED * deltaTime;
    }

    // Normalize angle
    ship.angle = ship.angle % (Math.PI * 2);
    if (ship.angle < 0) ship.angle += Math.PI * 2;

    // Thrust
    if (this.controls.thrust && ship.props.fuel > 0) {
      const thrustX = Math.cos(ship.angle) * THRUST_POWER * ship.props.engine_power;
      const thrustY = Math.sin(ship.angle) * THRUST_POWER * ship.props.engine_power;
      
      ship.velocity.x += thrustX * deltaTime;
      ship.velocity.y += thrustY * deltaTime;
      
      // Consume fuel
      ship.props.fuel = Math.max(0, ship.props.fuel - FUEL_CONSUMPTION * deltaTime);
    }

    // Reverse thrust (slower)
    if (this.controls.reverse && ship.props.fuel > 0) {
      const thrustX = -Math.cos(ship.angle) * THRUST_POWER * 0.5 * ship.props.engine_power;
      const thrustY = -Math.sin(ship.angle) * THRUST_POWER * 0.5 * ship.props.engine_power;
      
      ship.velocity.x += thrustX * deltaTime;
      ship.velocity.y += thrustY * deltaTime;
      
      ship.props.fuel = Math.max(0, ship.props.fuel - FUEL_CONSUMPTION * 0.3 * deltaTime);
    }

    // Apply drag
    ship.velocity.x *= DRAG;
    ship.velocity.y *= DRAG;

    // Clamp velocity
    const speed = Math.sqrt(ship.velocity.x ** 2 + ship.velocity.y ** 2);
    if (speed > MAX_VELOCITY) {
      ship.velocity.x = (ship.velocity.x / speed) * MAX_VELOCITY;
      ship.velocity.y = (ship.velocity.y / speed) * MAX_VELOCITY;
    }

    // Update position
    ship.position.x += ship.velocity.x * deltaTime;
    ship.position.y += ship.velocity.y * deltaTime;

    // Notify state change
    if (this.onStateChange) {
      this.onStateChange({ ...ship });
    }
  }

  // Apply damage to the spacecraft
  applyDamage(amount: number): void {
    const ship = this.spacecraft;
    
    // Shields absorb damage first
    if (ship.props.shields > 0) {
      const shieldDamage = Math.min(ship.props.shields, amount);
      ship.props.shields -= shieldDamage;
      amount -= shieldDamage;
    }
    
    // Rest goes to hull
    if (amount > 0) {
      ship.props.hull = Math.max(0, ship.props.hull - amount);
    }
    
    // Check for destruction
    if (ship.props.hull <= 0) {
      ship.isDestroyed = true;
    }

    if (this.onStateChange) {
      this.onStateChange({ ...ship });
    }
  }

  // Repair the spacecraft
  repair(amount: number): void {
    const ship = this.spacecraft;
    ship.props.hull = Math.min(ship.props.max_hull, ship.props.hull + amount);
    
    if (this.onStateChange) {
      this.onStateChange({ ...ship });
    }
  }

  // Recharge shields
  rechargeShields(amount: number): void {
    const ship = this.spacecraft;
    ship.props.shields = Math.min(ship.props.max_shields, ship.props.shields + amount);
    
    if (this.onStateChange) {
      this.onStateChange({ ...ship });
    }
  }

  // Refuel
  refuel(amount: number): void {
    const ship = this.spacecraft;
    ship.props.fuel = Math.min(ship.props.max_fuel, ship.props.fuel + amount);
    
    if (this.onStateChange) {
      this.onStateChange({ ...ship });
    }
  }

  // Check collision with a point
  checkCollision(x: number, y: number, radius: number = SHIP_RADIUS): boolean {
    const ship = this.spacecraft;
    const dx = x - ship.position.x;
    const dy = y - ship.position.y;
    const dist = Math.sqrt(dx * dx + dy * dy);
    return dist < radius + SHIP_RADIUS;
  }

  // Get distance to a point
  distanceTo(x: number, y: number): number {
    const ship = this.spacecraft;
    const dx = x - ship.position.x;
    const dy = y - ship.position.y;
    return Math.sqrt(dx * dx + dy * dy);
  }

  // Reset controls
  resetControls(): void {
    this.controls = {
      thrust: false,
      reverse: false,
      rotateLeft: false,
      rotateRight: false,
      fire: false,
    };
  }
}

// Keyboard setup helper - returns cleanup function
export function setupKeyboardControls(
  control: SpacecraftControl
): () => void {
  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;
    
    switch (e.key.toLowerCase()) {
      case 'w':
      case 'arrowup':
        control.setControl('thrust', true);
        e.preventDefault();
        break;
      case 's':
      case 'arrowdown':
        control.setControl('reverse', true);
        e.preventDefault();
        break;
      case 'a':
      case 'arrowleft':
        control.setControl('rotateLeft', true);
        e.preventDefault();
        break;
      case 'd':
      case 'arrowright':
        control.setControl('rotateRight', true);
        e.preventDefault();
        break;
      case ' ':
        control.setControl('fire', true);
        e.preventDefault();
        break;
    }
  };

  const handleKeyUp = (e: KeyboardEvent) => {
    switch (e.key.toLowerCase()) {
      case 'w':
      case 'arrowup':
        control.setControl('thrust', false);
        break;
      case 's':
      case 'arrowdown':
        control.setControl('reverse', false);
        break;
      case 'a':
      case 'arrowleft':
        control.setControl('rotateLeft', false);
        break;
      case 'd':
      case 'arrowright':
        control.setControl('rotateRight', false);
        break;
      case ' ':
        control.setControl('fire', false);
        break;
    }
  };

  window.addEventListener('keydown', handleKeyDown);
  window.addEventListener('keyup', handleKeyUp);

  // Return cleanup function
  return () => {
    window.removeEventListener('keydown', handleKeyDown);
    window.removeEventListener('keyup', handleKeyUp);
  };
}

// Generate a unique ID for spacecraft
let spacecraftIdCounter = 0;
export function generateSpacecraftId(): string {
  return `ship_${Date.now()}_${++spacecraftIdCounter}`;
}