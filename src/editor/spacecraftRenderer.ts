// FUL-35c: Spacecraft Renderer
// Renders spacecraft on the canvas with ship class visuals

import { Spacecraft, ShipClass, SHIP_CLASS_INFO } from './spacecraft';

// Canvas rendering utilities for spacecraft
export class SpacecraftRenderer {
  private ctx: CanvasRenderingContext2D;

  constructor(ctx: CanvasRenderingContext2D) {
    this.ctx = ctx;
  }

  // Render a spacecraft on the canvas
  render(ship: Spacecraft, scale: number = 4): void {
    if (ship.isDestroyed) {
      this.renderDestroyed(ship, scale);
      return;
    }

    const { ctx } = this;
    const x = ship.position.x;
    const y = ship.position.y;
    const angle = ship.angle;
    const info = SHIP_CLASS_INFO[ship.props.ship_class];

    ctx.save();
    ctx.translate(x, y);
    ctx.rotate(angle + Math.PI / 2); // Adjust so 0 angle = pointing up

    // Draw ship based on class
    switch (ship.props.ship_class) {
      case ShipClass.Scout:
        this.drawScout(info.color, ship.props.hull / ship.props.max_hull);
        break;
      case ShipClass.Fighter:
        this.drawFighter(info.color, ship.props.hull / ship.props.max_hull);
        break;
      case ShipClass.Freighter:
        this.drawFreighter(info.color, ship.props.hull / ship.props.max_hull);
        break;
      case ShipClass.Cruiser:
        this.drawCruiser(info.color, ship.props.hull / ship.props.max_hull);
        break;
      case ShipClass.ColonyShip:
        this.drawColonyShip(info.color, ship.props.hull / ship.props.max_hull);
        break;
      case ShipClass.Station:
        this.drawStation(info.color, ship.props.hull / ship.props.max_hull);
        break;
    }

    ctx.restore();

    // Draw HUD (health bars, etc.) - only for player ship
    if (ship.isPlayer) {
      this.drawHUD(ship, scale);
    }
  }

  private drawScout(color: string, healthRatio: number): void {
    const { ctx } = this;
    ctx.fillStyle = color;
    ctx.strokeStyle = '#FFFFFF';
    ctx.lineWidth = 1;

    // Small fast ship - triangle
    ctx.beginPath();
    ctx.moveTo(0, -12);
    ctx.lineTo(-6, 8);
    ctx.lineTo(0, 4);
    ctx.lineTo(6, 8);
    ctx.closePath();
    ctx.fill();
    ctx.stroke();

    // Engine glow
    ctx.fillStyle = '#FF6432';
    ctx.beginPath();
    ctx.arc(0, 6, 2, 0, Math.PI * 2);
    ctx.fill();
  }

  private drawFighter(color: string, healthRatio: number): void {
    const { ctx } = this;
    ctx.fillStyle = color;
    ctx.strokeStyle = '#FFFFFF';
    ctx.lineWidth = 1;

    // Fighter - aggressive angular shape
    ctx.beginPath();
    ctx.moveTo(0, -14);
    ctx.lineTo(-8, 2);
    ctx.lineTo(-4, 6);
    ctx.lineTo(0, 2);
    ctx.lineTo(4, 6);
    ctx.lineTo(8, 2);
    ctx.closePath();
    ctx.fill();
    ctx.stroke();

    // Wings
    ctx.fillStyle = '#CC0000';
    ctx.fillRect(-12, 0, 6, 2);
    ctx.fillRect(6, 0, 6, 2);

    // Engine glow
    ctx.fillStyle = '#FF6432';
    ctx.beginPath();
    ctx.arc(-6, 4, 2, 0, Math.PI * 2);
    ctx.arc(6, 4, 2, 0, Math.PI * 2);
    ctx.fill();
  }

  private drawFreighter(color: string, healthRatio: number): void {
    const { ctx } = this;
    ctx.fillStyle = color;
    ctx.strokeStyle = '#FFFFFF';
    ctx.lineWidth = 1;

    // Freighter - wide cargo ship
    ctx.beginPath();
    ctx.moveTo(0, -10);
    ctx.lineTo(-10, 6);
    ctx.lineTo(-10, 12);
    ctx.lineTo(10, 12);
    ctx.lineTo(10, 6);
    ctx.closePath();
    ctx.fill();
    ctx.stroke();

    // Cargo bays
    ctx.fillStyle = '#654321';
    ctx.fillRect(-8, 0, 6, 8);
    ctx.fillRect(2, 0, 6, 8);

    // Engine
    ctx.fillStyle = '#FF6432';
    ctx.fillRect(-4, 10, 8, 3);
  }

  private drawCruiser(color: string, healthRatio: number): void {
    const { ctx } = this;
    ctx.fillStyle = color;
    ctx.strokeStyle = '#FFFFFF';
    ctx.lineWidth = 1;

    // Cruiser - large capital ship
    ctx.beginPath();
    ctx.moveTo(0, -16);
    ctx.lineTo(-6, -8);
    ctx.lineTo(-10, 8);
    ctx.lineTo(-6, 14);
    ctx.lineTo(6, 14);
    ctx.lineTo(10, 8);
    ctx.lineTo(6, -8);
    ctx.closePath();
    ctx.fill();
    ctx.stroke();

    // Bridge
    ctx.fillStyle = '#7B1FA2';
    ctx.beginPath();
    ctx.arc(0, -4, 3, 0, Math.PI * 2);
    ctx.fill();

    // Engines
    ctx.fillStyle = '#FF6432';
    ctx.beginPath();
    ctx.arc(-4, 12, 3, 0, Math.PI * 2);
    ctx.arc(4, 12, 3, 0, Math.PI * 2);
    ctx.fill();
  }

  private drawColonyShip(color: string, healthRatio: number): void {
    const { ctx } = this;
    ctx.fillStyle = color;
    ctx.strokeStyle = '#FFFFFF';
    ctx.lineWidth = 1;

    // Colony ship - large with dome
    ctx.beginPath();
    ctx.moveTo(0, -12);
    ctx.lineTo(-12, 4);
    ctx.lineTo(-12, 14);
    ctx.lineTo(12, 14);
    ctx.lineTo(12, 4);
    ctx.closePath();
    ctx.fill();
    ctx.stroke();

    // Dome
    ctx.fillStyle = '#64B5F6';
    ctx.beginPath();
    ctx.arc(0, 2, 6, Math.PI, 0);
    ctx.fill();

    // Engines
    ctx.fillStyle = '#FF6432';
    ctx.fillRect(-8, 12, 4, 4);
    ctx.fillRect(4, 12, 4, 4);
  }

  private drawStation(color: string, healthRatio: number): void {
    const { ctx } = this;
    ctx.fillStyle = color;
    ctx.strokeStyle = '#FFFFFF';
    ctx.lineWidth = 1;

    // Station - circular with arms
    ctx.beginPath();
    ctx.arc(0, 0, 10, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();

    // Arms
    for (let i = 0; i < 6; i++) {
      ctx.save();
      ctx.rotate((i * Math.PI) / 3);
      ctx.fillRect(-2, 10, 4, 8);
      ctx.restore();
    }

    // Center lights
    ctx.fillStyle = '#4CAF50';
    ctx.beginPath();
    ctx.arc(0, 0, 4, 0, Math.PI * 2);
    ctx.fill();
  }

  private renderDestroyed(ship: Spacecraft, scale: number): void {
    const { ctx } = this;
    const x = ship.position.x;
    const y = ship.position.y;

    // Draw debris/explosion
    ctx.fillStyle = '#FF4500';
    for (let i = 0; i < 8; i++) {
      const angle = (i / 8) * Math.PI * 2;
      const dist = 5 + Math.random() * 10;
      ctx.beginPath();
      ctx.arc(
        x + Math.cos(angle) * dist,
        y + Math.sin(angle) * dist,
        2 + Math.random() * 2,
        0,
        Math.PI * 2
      );
      ctx.fill();
    }

    // Smoke
    ctx.fillStyle = '#64646E';
    for (let i = 0; i < 5; i++) {
      ctx.beginPath();
      ctx.arc(
        x + (Math.random() - 0.5) * 20,
        y + (Math.random() - 0.5) * 20,
        3 + Math.random() * 4,
        0,
        Math.PI * 2
      );
      ctx.fill();
    }
  }

  private drawHUD(ship: Spacecraft, scale: number): void {
    const { ctx } = this;
    const x = ship.position.x;
    const y = ship.position.y;
    const barWidth = 30;
    const barHeight = 3;
    const spacing = 2;

    // HUD background
    ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
    ctx.fillRect(x - barWidth / 2 - 2, y + 20, barWidth + 4, 18);

    // Hull bar
    this.drawBar(x - barWidth / 2, y + 22, barWidth, barHeight,
      ship.props.hull / ship.props.max_hull, '#4CAF50', '#2E7D32');

    // Fuel bar
    this.drawBar(x - barWidth / 2, y + 27, barWidth, barHeight,
      ship.props.fuel / ship.props.max_fuel, '#FFC107', '#FF8F00');

    // Shields bar
    this.drawBar(x - barWidth / 2, y + 32, barWidth, barHeight,
      ship.props.shields / ship.props.max_shields, '#2196F3', '#1565C0');

    // Class label
    ctx.fillStyle = '#FFFFFF';
    ctx.font = '8px monospace';
    ctx.textAlign = 'center';
    ctx.fillText(ship.props.ship_class, x, y + 16);
  }

  private drawBar(x: number, y: number, width: number, height: number,
    ratio: number, color: string, bgColor: string): void {
    const { ctx } = this;

    // Background
    ctx.fillStyle = bgColor;
    ctx.fillRect(x, y, width, height);

    // Fill
    ctx.fillStyle = color;
    ctx.fillRect(x, y, width * ratio, height);

    // Border
    ctx.strokeStyle = '#FFFFFF';
    ctx.lineWidth = 0.5;
    ctx.strokeRect(x, y, width, height);
  }
}