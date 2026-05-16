// FUL-5: Phase 6 - Main App Component
// FUL-35c: Added spacecraft control mode
// FUL-45: Added Strip Sandbox, Gravity Gun, Scoring, Upgrades

import { useState, useRef, useEffect, useCallback } from 'react';
import { Material, MATERIALS, getMaterialByKey } from './materials';
import { SimulationCanvas } from './simulation-optimized';
import type { OverlayMode } from './simulation';
import { ShipClass, SHIP_CLASS_INFO } from './spacecraft';
import MaterialPalette from './components/MaterialPalette';
import ControlBar from './components/ControlBar';
import BrushSelector from './components/BrushSelector';
import StatusBar from './components/StatusBar';
import HUD from './components/HUD';
import UpgradeMenu from './components/UpgradeMenu';
import { GravityGunState, GravityGunMode, createGravityGunState, updateGravityGunState } from './gravityGun';
import { SessionScore, createSessionScore, tickSurvivalScore, consumeParticleScore, destroyEnemyScore, loadHighScores, saveHighScore } from './scoring';
import { AllUpgrades, createDefaultUpgrades, loadUpgrades, saveUpgrades, purchaseUpgrade, getGravityGunStats, getUpgradeSummary } from './upgrades';
import { formatNumber } from './scoring';

const GRID_WIDTH = 200;
const GRID_HEIGHT = 150;
const SCALE = 4;

// App mode type
type AppMode = 'full' | 'strip-sandbox' | 'space-game';

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
  // App mode (FUL-45)
  const [appMode, setAppMode] = useState<AppMode>('full');
  const [showUpgradeMenu, setShowUpgradeMenu] = useState(false);
  
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

  // FUL-45: Gravity Gun state
  const [gravityGunState, setGravityGunState] = useState<GravityGunState>(createGravityGunState());
  
  // FUL-45: Scoring state
  const [sessionScore, setSessionScore] = useState<SessionScore>(createSessionScore());
  const [availablePoints, setAvailablePoints] = useState(0);
  
  // FUL-45: Upgrades state
  const [upgrades, setUpgrades] = useState<AllUpgrades>(createDefaultUpgrades());

  // Refs
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const simulationRef = useRef<SimulationCanvas | null>(null);
  const animationFrameRef = useRef<number>(0);
  const lastTickRef = useRef<number>(0);
  const isMouseDownRef = useRef(false);
  const lastMousePosRef = useRef<{ x: number; y: number } | null>(null);
  const survivalTimerRef = useRef<number>(0);

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

  // Load saved data (FUL-45)
  useEffect(() => {
    const saved = loadUpgrades();
    setUpgrades(saved.upgrades);
    setAvailablePoints(saved.points);
  }, []);

  // Save data periodically (FUL-45)
  useEffect(() => {
    const saveInterval = setInterval(() => {
      saveUpgrades(upgrades, availablePoints);
    }, 10000); // Save every 10 seconds
    return () => clearInterval(saveInterval);
  }, [upgrades, availablePoints]);

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
      
      const newStats = sim.getStats();
      const consumedDiff = newStats.particlesConsumed - stats.particlesConsumed;
      if (consumedDiff > 0) {
        // Award points for consumed particles
        setSessionScore(prev => {
          let updated = prev;
          for (let i = 0; i < consumedDiff; i++) {
            updated = consumeParticleScore(updated, 'particle');
          }
          return updated;
        });
        setAvailablePoints(prev => prev + consumedDiff);
      }
      setStats(newStats);
      
      // Survival timer (once per second)
      survivalTimerRef.current++;
      if (survivalTimerRef.current >= 60) {
        survivalTimerRef.current = 0;
        setSessionScore(prev => tickSurvivalScore(prev));
        setAvailablePoints(prev => prev + 1);
      }
      
      // FUL-45: Check for game over (hull depleted)
      if (spacecraftMode) {
        const player = sim.getPlayerSpacecraft();
        if (player && player.isDestroyed) {
          setPlayerStats({ hull: 0, fuel: 0, shields: 0 });
        } else if (player) {
          setPlayerStats({
            hull: Math.round(player.props.hull),
            fuel: Math.round(player.props.fuel),
            shields: Math.round(player.props.shields)
          });
        }
      }
    }

    // Render
    sim.render();
    
    // FUL-45: Sync gravity gun state to simulation for rendering
    if (gravityGunState) {
      sim.gravityGunState = gravityGunState;
    }

    animationFrameRef.current = requestAnimationFrame(gameLoop);
  }, [isPlaying, speed, stats.particlesConsumed]);

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
    
    // FUL-45: Handle gravity gun modes
    if (appMode === 'strip-sandbox' || appMode === 'space-game') {
      if (e.button === 0) { // Left click
        setGravityGunState(prev => updateGravityGunState(prev, { 
          active: true, 
          mode: 'attract',
          cursorX: x,
          cursorY: y
        }));
      } else if (e.button === 2) { // Right click
        setGravityGunState(prev => updateGravityGunState(prev, { 
          active: true, 
          mode: 'repel',
          cursorX: x,
          cursorY: y
        }));
      } else if (e.button === 1) { // Middle click
        setGravityGunState(prev => updateGravityGunState(prev, { 
          active: true, 
          mode: 'vortex',
          cursorX: x,
          cursorY: y
        }));
      }
      return;
    }
    
    simulationRef.current.spawnBrush(x, y, selectedMaterial, brushSize);
    setParticleCount(simulationRef.current.countParticles());
  }, [selectedMaterial, brushSize, appMode]);

  const handleMouseMove = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!simulationRef.current || !canvasRef.current) return;

    const rect = canvasRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    // FUL-45: Update gravity gun cursor position
    if ((appMode === 'strip-sandbox' || appMode === 'space-game') && isMouseDownRef.current) {
      setGravityGunState(prev => updateGravityGunState(prev, {
        cursorX: x,
        cursorY: y
      }));
    }

    const lastPos = lastMousePosRef.current;
    if (lastPos && (appMode === 'full' || !isMouseDownRef.current)) {
      simulationRef.current.spawnLine(lastPos.x, lastPos.y, x, y, selectedMaterial, brushSize);
    }

    lastMousePosRef.current = { x, y };
    setParticleCount(simulationRef.current.countParticles());
  }, [selectedMaterial, brushSize, appMode]);

  const handleMouseUp = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    isMouseDownRef.current = false;
    lastMousePosRef.current = null;
    
    // FUL-45: Deactivate gravity gun on mouse up
    if (appMode === 'strip-sandbox' || appMode === 'space-game') {
      setGravityGunState(prev => updateGravityGunState(prev, { active: false }));
    }
  }, [appMode]);

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
    
    if (appMode === 'full') {
      simulationRef.current.spawnBrush(x, y, selectedMaterial, brushSize);
      setParticleCount(simulationRef.current.countParticles());
    }
  }, [selectedMaterial, brushSize, appMode]);

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
        // FUL-45: Mode switching
        case 'm':
          // Cycle mode: full -> strip-sandbox -> space-game -> full
          setAppMode(current => {
            if (current === 'full') return 'strip-sandbox';
            if (current === 'strip-sandbox') return 'space-game';
            return 'full';
          });
          break;
        case 'u':
          // Toggle upgrade menu
          setShowUpgradeMenu(prev => !prev);
          break;
        case 'g':
          // Toggle gravity gun mode
          setGravityGunState(prev => {
            const modes: GravityGunMode[] = ['attract', 'repel', 'vortex'];
            const currentIdx = modes.indexOf(prev.mode);
            return updateGravityGunState(prev, { 
              mode: modes[(currentIdx + 1) % modes.length] 
            });
          });
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
    setSessionScore(createSessionScore());
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

  // FUL-45: Mode toggle handler
  const handleModeToggle = useCallback(() => {
    setAppMode(current => {
      if (current === 'full') return 'strip-sandbox';
      return 'full';
    });
  }, []);

  // FUL-45: Handle upgrade purchase
  const handlePurchaseUpgrade = useCallback((upgradeType: string) => {
    const result = purchaseUpgrade(upgradeType as any, upgrades, availablePoints);
    if (result.success) {
      setUpgrades(result.newUpgrades);
      setAvailablePoints(result.newPoints);
    }
  }, [upgrades, availablePoints]);

  // FUL-45: Quick start for strip sandbox
  const handleQuickStart = useCallback(() => {
    if (!canvasRef.current || !simulationRef.current) return;
    // Spawn a black hole at center
    const rect = canvasRef.current.getBoundingClientRect();
    const cx = rect.width / 2;
    const cy = rect.height / 2;
    simulationRef.current.spawnBrush(cx, cy, Material.BlackHole, 20);
    // Spawn some particles around it
    for (let i = 0; i < 5; i++) {
      const angle = (i / 5) * Math.PI * 2;
      const px = cx + Math.cos(angle) * 150;
      const py = cy + Math.sin(angle) * 150;
      simulationRef.current.spawnBrush(px, py, Material.Sand, 30);
    }
    setParticleCount(simulationRef.current.countParticles());
  }, []);

  // Render based on mode
  const renderContent = () => {
    if (appMode === 'strip-sandbox' || appMode === 'space-game') {
      return (
        <div className={`canvas-container ${appMode === 'strip-sandbox' ? 'strip-sandbox' : ''}`}>
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
          {/* FUL-45: HUD for game modes */}
          <HUD
            score={sessionScore}
            upgrades={upgrades}
            availablePoints={availablePoints}
            gravityGunMode={gravityGunState.mode}
            gravityGunActive={gravityGunState.active}
            showUpgradeMenu={() => setShowUpgradeMenu(true)}
            compact={appMode === 'strip-sandbox'}
          />
        </div>
      );
    }

    return (
      <>
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
      </>
    );
  };

  return (
    <div className="editor-container">
      {/* FUL-45: Strip Sandbox simplified controls */}
      {appMode === 'strip-sandbox' || appMode === 'space-game' ? (
        <div className="control-bar">
          <button onClick={handleTogglePlay} className={isPlaying ? '' : 'active'}>
            {isPlaying ? '⏸ Pause' : '▶ Play'}
          </button>
          <button onClick={handleClear}>🗑 Clear</button>
          <div className="divider" />
          <span className={`gun-mode-indicator ${gravityGunState.mode}`}>
            {gravityGunState.mode === 'attract' ? '⟲ ATTRACT' : 
             gravityGunState.mode === 'repel' ? '⟳ REPEL' : '🌀 VORTEX'}
          </span>
          <button onClick={handleQuickStart} className="primary">🚀 Quick Start</button>
          {appMode === 'space-game' && (
            <>
              {!spacecraftMode ? (
                <button onClick={handleToggleSpacecraftMode} title="Activate Ship [T]">
                  🚀 Activate Ship
                </button>
              ) : (
                <span className="ship-hud-indicator">
                  🛡 {playerStats.shields} ⚡ {playerStats.fuel} ❤️ {playerStats.hull}
                </span>
              )}
            </>
          )}
          <div className="divider" />
          <button onClick={handleModeToggle} className="mode-toggle-btn">
            {appMode === 'strip-sandbox' ? '🎮 Space Game' : '📝 Full Editor'}
          </button>
          <button onClick={() => setShowUpgradeMenu(true)}>
            ⚙️ Upgrades ({formatNumber(availablePoints)} pts)
          </button>
        </div>
      ) : (
        <>
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
            spacecraftMode={spacecraftMode}
            onToggleSpacecraftMode={handleToggleSpacecraftMode}
            playerStats={playerStats}
          />

          {/* FUL-45: Mode toggle button */}
          <div className="control-bar">
            <button onClick={handleModeToggle} className="mode-toggle-btn">
              🎮 Strip Sandbox Mode
            </button>
            <span className="keyboard-hints">
              <kbd>M</kbd> to cycle modes
            </span>
          </div>
        </>
      )}

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

      {renderContent()}

      {appMode === 'full' && (
        <StatusBar
          selectedMaterial={selectedMaterial}
          brushSize={brushSize}
          particleCount={particleCount}
          onBrushSizeChange={handleBrushSizeChange}
          stats={stats}
          onResetStats={handleResetStats}
        />
      )}

      {/* FUL-45: Upgrade Menu */}
      {showUpgradeMenu && (
        <UpgradeMenu
          upgrades={upgrades}
          availablePoints={availablePoints}
          onPurchase={handlePurchaseUpgrade}
          onClose={() => setShowUpgradeMenu(false)}
        />
      )}
    </div>
  );
}