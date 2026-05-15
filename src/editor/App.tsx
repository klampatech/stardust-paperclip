// FUL-5: Phase 6 - Main App Component
// FUL-35c: Added spacecraft control mode

import { useState, useRef, useEffect, useCallback } from 'react';
import { Material, MATERIALS, getMaterialByKey } from './materials';
import { SimulationCanvas } from './simulation-optimized';
import type { OverlayMode } from './simulation';
import { ShipClass, SHIP_CLASS_INFO } from './spacecraft';
import MaterialPalette from './components/MaterialPalette';
import ControlBar from './components/ControlBar';
import BrushSelector from './components/BrushSelector';
import StatusBar from './components/StatusBar';

const GRID_WIDTH = 200;
const GRID_HEIGHT = 150;
const SCALE = 4;

// Ship class selector component
function ShipClassSelector({ 
  onSelect, 
  onCancel 
}: { 
  onSelect: (shipClass: ShipClass) => void; 
  onCancel: () => void;
}) {
  return (
    <div className="ship-selector-overlay">
      <div className="ship-selector">
        <h3>Select Ship Class</h3>
        <div className="ship-options">
          {Object.values(ShipClass).map((shipClass) => {
            const info = SHIP_CLASS_INFO[shipClass as ShipClass];
            return (
              <button
                key={shipClass}
                className="ship-option"
                onClick={() => onSelect(shipClass as ShipClass)}
              >
                <span className="ship-icon">{info.icon}</span>
                <span className="ship-name">{info.name}</span>
              </button>
            );
          })}
        </div>
        <button className="cancel-btn" onClick={onCancel}>Cancel</button>
      </div>
    </div>
  );
}

export default function App() {
  // State
  const [selectedMaterial, setSelectedMaterial] = useState<Material>(Material.Sand);
  const [brushSize, setBrushSize] = useState<1 | 3 | 5>(3);
  const [isPlaying, setIsPlaying] = useState(true);
  const [speed, setSpeed] = useState(1);
  const [particleCount, setParticleCount] = useState(0);
  const [stats, setStats] = useState({ particlesConsumed: 0, totalMass: 0, blackHoles: 0 });
  const [showStructures, setShowStructures] = useState(false);
  const [overlayMode, setOverlayMode] = useState<OverlayMode>('none');
  // FUL-35c: Spacecraft mode state
  const [spacecraftMode, setSpacecraftMode] = useState(false);
  const [showShipSelector, setShowShipSelector] = useState(false);
  const [playerStats, setPlayerStats] = useState({ hull: 0, fuel: 0, shields: 0 });

  // Refs
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const simulationRef = useRef<SimulationCanvas | null>(null);
  const animationFrameRef = useRef<number>(0);
  const lastTickRef = useRef<number>(0);
  const isMouseDownRef = useRef(false);
  const lastMousePosRef = useRef<{ x: number; y: number } | null>(null);

  // Initialize simulation
  useEffect(() => {
    if (!canvasRef.current) return;

    const canvas = canvasRef.current;
    simulationRef.current = new SimulationCanvas(canvas, GRID_WIDTH, GRID_HEIGHT, SCALE);

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, []);

  // Main game loop
  const gameLoop = useCallback((timestamp: number) => {
    const sim = simulationRef.current;
    if (!sim) return;

    // Calculate tick interval based on speed
    const tickInterval = 1000 / (60 * speed);
    const elapsed = timestamp - lastTickRef.current;

    // Run physics ticks
    if (isPlaying && elapsed >= tickInterval) {
      sim.tick();
      lastTickRef.current = timestamp;
      setParticleCount(sim.countParticles());
      setStats(sim.getStats());
    }

    // Render
    sim.render();

    animationFrameRef.current = requestAnimationFrame(gameLoop);
  }, [isPlaying, speed]);

  // Start game loop
  useEffect(() => {
    animationFrameRef.current = requestAnimationFrame(gameLoop);
    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [gameLoop]);

  // Mouse handlers
  const handleMouseDown = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!simulationRef.current || !canvasRef.current) return;
    
    isMouseDownRef.current = true;
    const rect = canvasRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    lastMousePosRef.current = { x, y };
    simulationRef.current.spawnBrush(x, y, selectedMaterial, brushSize);
    setParticleCount(simulationRef.current.countParticles());
  }, [selectedMaterial, brushSize]);

  const handleMouseMove = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!simulationRef.current || !canvasRef.current || !isMouseDownRef.current) return;

    const rect = canvasRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    const lastPos = lastMousePosRef.current;
    if (lastPos) {
      simulationRef.current.spawnLine(lastPos.x, lastPos.y, x, y, selectedMaterial, brushSize);
    }

    lastMousePosRef.current = { x, y };
    setParticleCount(simulationRef.current.countParticles());
  }, [selectedMaterial, brushSize]);

  const handleMouseUp = useCallback(() => {
    isMouseDownRef.current = false;
    lastMousePosRef.current = null;
  }, []);

  // Touch handlers
  const handleTouchStart = useCallback((e: React.TouchEvent<HTMLCanvasElement>) => {
    if (!simulationRef.current || !canvasRef.current) return;
    e.preventDefault();

    const touch = e.touches[0];
    const rect = canvasRef.current.getBoundingClientRect();
    const x = touch.clientX - rect.left;
    const y = touch.clientY - rect.top;

    isMouseDownRef.current = true;
    lastMousePosRef.current = { x, y };
    simulationRef.current.spawnBrush(x, y, selectedMaterial, brushSize);
    setParticleCount(simulationRef.current.countParticles());
  }, [selectedMaterial, brushSize]);

  const handleTouchMove = useCallback((e: React.TouchEvent<HTMLCanvasElement>) => {
    if (!simulationRef.current || !canvasRef.current || !isMouseDownRef.current) return;
    e.preventDefault();

    const touch = e.touches[0];
    const rect = canvasRef.current.getBoundingClientRect();
    const x = touch.clientX - rect.left;
    const y = touch.clientY - rect.top;

    const lastPos = lastMousePosRef.current;
    if (lastPos) {
      simulationRef.current.spawnLine(lastPos.x, lastPos.y, x, y, selectedMaterial, brushSize);
    }

    lastMousePosRef.current = { x, y };
    setParticleCount(simulationRef.current.countParticles());
  }, [selectedMaterial, brushSize]);

  const handleTouchEnd = useCallback(() => {
    isMouseDownRef.current = false;
    lastMousePosRef.current = null;
  }, []);

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

      switch (e.key.toLowerCase()) {
        case ' ':
          e.preventDefault();
          setIsPlaying(p => !p);
          break;
        case 'p':
          // Cycle overlay: none -> temperature -> velocity -> none
          setOverlayMode(current => {
            if (current === 'none') return 'temperature';
            if (current === 'temperature') return 'velocity';
            return 'none';
          });
          break;
        case 'c':
          simulationRef.current?.clear();
          setParticleCount(0);
          break;
        case '[':
          setBrushSize(s => (s > 1 ? (s - 2) as 1 | 3 | 5 : s));
          break;
        case ']':
          setBrushSize(s => (s < 5 ? (s + 2) as 1 | 3 | 5 : s));
          break;
        case '1': case '2': case '3': case '4': case '5':
        case '6': case '7': case '8': case '9': case '0':
        case 'q': case 'w': case 'e':
          const key = e.key.toLowerCase();
          const mat = MATERIALS.find(m => m.key.toLowerCase() === key);
          if (mat) setSelectedMaterial(mat.id);
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  // Actions
  const handleClear = useCallback(() => {
    simulationRef.current?.clear();
    setParticleCount(0);
  }, []);

  const handleTogglePlay = useCallback(() => {
    setIsPlaying(p => !p);
  }, []);

  const handleSpeedChange = useCallback((newSpeed: number) => {
    setSpeed(newSpeed);
  }, []);

  const handleMaterialSelect = useCallback((material: Material) => {
    setSelectedMaterial(material);
  }, []);

  const handleBrushSizeChange = useCallback((size: 1 | 3 | 5) => {
    setBrushSize(size);
  }, []);

  // Step (single tick)
  const handleStep = useCallback(() => {
    if (simulationRef.current) {
      simulationRef.current.tick();
      setParticleCount(simulationRef.current.countParticles());
      setStats(simulationRef.current.getStats());
    }
  }, []);

  // Spawn structure
  const handleSpawnStructure = useCallback((type: 'ship' | 'asteroid' | 'station') => {
    if (!canvasRef.current || !simulationRef.current) return;
    const rect = canvasRef.current.getBoundingClientRect();
    const x = rect.width / 2;
    const y = rect.height / 2;
    simulationRef.current.spawnStructure(type, x, y);
    setParticleCount(simulationRef.current.countParticles());
  }, []);
  
  // Handle overlay toggle
  const handleOverlayChange = useCallback((mode: OverlayMode) => {
    setOverlayMode(mode);
    simulationRef.current?.setOverlayMode(mode);
  }, []);

  // Reset stats
  const handleResetStats = useCallback(() => {
    if (simulationRef.current) {
      simulationRef.current.resetStats();
      setStats({ particlesConsumed: 0, totalMass: 0, blackHoles: 0 });
    }
  }, []);

  // FUL-35c: Spacecraft mode handlers
  const handleToggleSpacecraftMode = useCallback(() => {
    if (spacecraftMode) {
      // Exit spacecraft mode
      simulationRef.current?.deactivateSpacecraftMode();
      setSpacecraftMode(false);
      setPlayerStats({ hull: 0, fuel: 0, shields: 0 });
    } else {
      // Show ship selector
      setShowShipSelector(true);
    }
  }, [spacecraftMode]);

  const handleSelectShipClass = useCallback((shipClass: ShipClass) => {
    setShowShipSelector(false);
    if (simulationRef.current) {
      simulationRef.current.activateSpacecraftMode(shipClass);
      setSpacecraftMode(true);
      // Spawn some enemy ships for gameplay
      simulationRef.current.spawnEnemyShip(ShipClass.Fighter, 600, 200);
      simulationRef.current.spawnEnemyShip(ShipClass.Fighter, 100, 400);
      simulationRef.current.spawnEnemyShip(ShipClass.Freighter, 700, 500);
    }
  }, []);

  const handleCancelShipSelect = useCallback(() => {
    setShowShipSelector(false);
  }, []);

  return (
    <div className="editor-container">
      <ControlBar
        isPlaying={isPlaying}
        speed={speed}
        overlayMode={overlayMode}
        onTogglePlay={handleTogglePlay}
        onClear={handleClear}
        onSpeedChange={handleSpeedChange}
        onStep={handleStep}
        onToggleStructures={() => setShowStructures(s => !s)}
        showStructures={showStructures}
        onOverlayChange={handleOverlayChange}
        // FUL-35c: Spacecraft mode
        spacecraftMode={spacecraftMode}
        onToggleSpacecraftMode={handleToggleSpacecraftMode}
        playerStats={playerStats}
      />

      {/* FUL-35c: Ship class selector modal */}
      {showShipSelector && (
        <ShipClassSelector 
          onSelect={handleSelectShipClass}
          onCancel={handleCancelShipSelect}
        />
      )}

      {showStructures && (
        <div className="structure-bar">
          <span>Structures:</span>
          <button onClick={() => handleSpawnStructure('ship')} title="Spawn Ship">🚀 Ship</button>
          <button onClick={() => handleSpawnStructure('asteroid')} title="Spawn Asteroid">🪨 Asteroid</button>
          <button onClick={() => handleSpawnStructure('station')} title="Spawn Station">🛸 Station</button>
        </div>
      )}

      <div className="canvas-container">
        <canvas
          ref={canvasRef}
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
          onTouchStart={handleTouchStart}
          onTouchMove={handleTouchMove}
          onTouchEnd={handleTouchEnd}
          onContextMenu={(e) => e.preventDefault()}
          style={{ cursor: 'crosshair' }}
        />
      </div>

      <MaterialPalette
        selected={selectedMaterial}
        onSelect={handleMaterialSelect}
      />

      <StatusBar
        selectedMaterial={selectedMaterial}
        brushSize={brushSize}
        particleCount={particleCount}
        onBrushSizeChange={handleBrushSizeChange}
        stats={stats}
        onResetStats={handleResetStats}
      />
    </div>
  );
}