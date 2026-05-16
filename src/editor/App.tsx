// FUL-48: Main App Component - Pure Game State Machine
// Removes all sandbox/editor mode, becomes a pure game with title screen

import { useState, useRef, useEffect, useCallback } from 'react';
import { Material } from './materials';
import { ShipClass } from './spacecraft';
import { SimulationCanvas } from './simulation-optimized';
import { SessionScore, createSessionScore, tickSurvivalScore, consumeParticleScore, destroyEnemyScore, saveHighScore, collectDebrisScore } from './scoring';
import { AllUpgrades, createDefaultUpgrades, loadUpgrades, saveUpgrades, purchaseUpgrade } from './upgrades';
import HUD from './components/HUD';
import TitleScreen from './components/TitleScreen';
import PauseMenu from './components/PauseMenu';
import GameOverScreen from './components/GameOverScreen';
import ControlsOverlay from './components/ControlsOverlay';

const GRID_WIDTH = 200;
const GRID_HEIGHT = 150;
const SCALE = 4;

// Game state machine
type GameState = 'title' | 'playing' | 'paused' | 'gameOver';

export default function App() {
  // Game state (FUL-48: replaces appMode)
  const [gameState, setGameState] = useState<GameState>('title');
  const [showControls, setShowControls] = useState(false);
  const [showUpgradeMenu, setShowUpgradeMenu] = useState(false);

  // Game stats
  const [particleCount, setParticleCount] = useState(0);
  const [playerStats, setPlayerStats] = useState({ hull: 100, fuel: 100, shields: 100 });

  // FUL-45: Gravity Gun state
  const [gravityGunState, setGravityGunState] = useState({
    active: false,
    mode: 'attract' as 'attract' | 'repel' | 'vortex',
    cursorX: 0,
    cursorY: 0,
    strength: 5,
    radius: 80,
  });

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

    // FUL-47.2: Setup object collection callback
    if (simulationRef.current) {
      simulationRef.current.onObjectCollected = (obj, points) => {
        setSessionScore(prev => collectDebrisScore(prev));
        setAvailablePoints(prev => prev + points);
      };
    }

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
      // Save high score
      saveHighScore(sessionScore);
    }, 10000);
    return () => clearInterval(saveInterval);
  }, [upgrades, availablePoints, sessionScore]);

  // Update gravity gun state helper
  const updateGunState = useCallback((updates: Partial<typeof gravityGunState>) => {
    setGravityGunState(prev => ({ ...prev, ...updates }));
  }, []);

  // Main game loop (only runs when playing)
  const gameLoop = useCallback((timestamp: number) => {
    const sim = simulationRef.current;
    if (!sim) return;

    const tickInterval = 1000 / 60;
    const elapsed = timestamp - lastTickRef.current;

    if (elapsed >= tickInterval) {
      sim.tick();
      lastTickRef.current = timestamp;
      setParticleCount(sim.countParticles());

      // Check for consumed particles (score)
      const newStats = sim.getStats();
      const consumedDiff = newStats.particlesConsumed;
      if (consumedDiff > 0) {
        setSessionScore(prev => {
          let updated = prev;
          for (let i = 0; i < consumedDiff; i++) {
            updated = consumeParticleScore(updated, 'particle');
          }
          return updated;
        });
        setAvailablePoints(prev => prev + consumedDiff);
      }

      // Survival timer (once per second)
      survivalTimerRef.current++;
      if (survivalTimerRef.current >= 60) {
        survivalTimerRef.current = 0;
        setSessionScore(prev => tickSurvivalScore(prev));
        setAvailablePoints(prev => prev + 1);
      }

      // Check player status
      const player = sim.getPlayerSpacecraft();
      if (player) {
        setPlayerStats({
          hull: Math.round(player.props.hull),
          fuel: Math.round(player.props.fuel),
          shields: Math.round(player.props.shields)
        });

        // Check for game over
        if (player.isDestroyed || player.props.hull <= 0) {
          saveHighScore(sessionScore);
          setGameState('gameOver');
          return;
        }

        // Check for debris collection
        const debrisMgr = (sim as any).debrisManager;
        if (debrisMgr) {
          debrisMgr.checkCollection(player.position.x, player.position.y, 15);
        }
      }
    }

    // Render
    sim.render();
     
    // Sync gravity gun state with ship position
    if (gravityGunState) {
      const player = sim.getPlayerSpacecraft();
      if (player) {
        sim.gravityGunState = {
          ...gravityGunState,
          shipX: player.position.x,
          shipY: player.position.y
        };
      } else {
        sim.gravityGunState = gravityGunState;
      }
    }

    animationFrameRef.current = requestAnimationFrame(gameLoop);
  }, [gravityGunState, sessionScore]);

  // Start/stop game loop based on game state
  useEffect(() => {
    if (gameState === 'playing') {
      lastTickRef.current = performance.now();
      survivalTimerRef.current = 0;
      animationFrameRef.current = requestAnimationFrame(gameLoop);
    } else {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    }

    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [gameState, gameLoop]);

  // Start game (from title screen)
  const handlePlay = useCallback(() => {
    if (!simulationRef.current) return;

    // Reset score
    setSessionScore(createSessionScore());
    setAvailablePoints(0);
    setPlayerStats({ hull: 100, fuel: 100, shields: 100 });

    // Initialize game world with ship, black hole, and spawning objects
    if (canvasRef.current) {
      const rect = canvasRef.current.getBoundingClientRect();
      const cx = rect.width / 2;
      const cy = rect.height / 2;

      // Spawn black hole at center
      simulationRef.current.spawnBrush(cx, cy, Material.BlackHole, 20);

      // Ship spawns automatically in space game mode
      simulationRef.current.activateSpacecraftMode(ShipClass.Fighter);

      // Spawn initial debris field using DebrisManager
      const sim = simulationRef.current;
      const debrisMgr = (sim as any).debrisManager;
      if (debrisMgr) {
        debrisMgr.spawnInitialField(8);
        // FUL-47.2: Setup collection callback to update score
        debrisMgr.onObjectCollected = (obj: { material: string; size: number }, points: number) => {
          setSessionScore(prev => consumeParticleScore(prev, obj.material as 'particle'));
          setAvailablePoints(prev => prev + points);
        };
      }

      // Also spawn some visual asteroids (stone particles)
      for (let i = 0; i < 5; i++) {
        const angle = (i / 5) * Math.PI * 2;
        const px = cx + Math.cos(angle) * 150;
        const py = cy + Math.sin(angle) * 150;
        simulationRef.current.spawnBrush(px, py, Material.Stone, 30);
      }
    }

    setGameState('playing');
  }, []);

  // Restart game
  const handleRestart = useCallback(() => {
    if (!simulationRef.current) return;
    simulationRef.current.clear();
    setPlayerStats({ hull: 100, fuel: 100, shields: 100 });
    handlePlay();
  }, [handlePlay]);

  // Return to title screen
  const handleMainMenu = useCallback(() => {
    if (simulationRef.current) {
      simulationRef.current.clear();
    }
    setGameState('title');
  }, []);

  // Mouse handlers
  const handleMouseDown = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!simulationRef.current || !canvasRef.current || gameState !== 'playing') return;

    isMouseDownRef.current = true;
    const rect = canvasRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    lastMousePosRef.current = { x, y };

    // Gravity gun modes
    if (e.button === 0) { // Left click - attract
      updateGunState({ active: true, mode: 'attract', cursorX: x, cursorY: y });
    } else if (e.button === 2) { // Right click - repel
      updateGunState({ active: true, mode: 'repel', cursorX: x, cursorY: y });
    } else if (e.button === 1) { // Middle click - vortex
      updateGunState({ active: true, mode: 'vortex', cursorX: x, cursorY: y });
    }
  }, [gameState, updateGunState]);

  const handleMouseMove = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!simulationRef.current || !canvasRef.current || gameState !== 'playing') return;

    const rect = canvasRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    if (isMouseDownRef.current) {
      updateGunState({ cursorX: x, cursorY: y });
    }

    lastMousePosRef.current = { x, y };
  }, [gameState, updateGunState]);

  const handleMouseUp = useCallback(() => {
    isMouseDownRef.current = false;
    lastMousePosRef.current = null;
    updateGunState({ active: false });
  }, [updateGunState]);

  // Touch handlers
  const handleTouchStart = useCallback((e: React.TouchEvent<HTMLCanvasElement>) => {
    if (!simulationRef.current || !canvasRef.current || gameState !== 'playing') return;
    e.preventDefault();

    const touch = e.touches[0];
    const rect = canvasRef.current.getBoundingClientRect();
    const x = touch.clientX - rect.left;
    const y = touch.clientY - rect.top;

    isMouseDownRef.current = true;
    lastMousePosRef.current = { x, y };
    updateGunState({ active: true, mode: 'attract', cursorX: x, cursorY: y });
  }, [gameState, updateGunState]);

  const handleTouchMove = useCallback((e: React.TouchEvent<HTMLCanvasElement>) => {
    if (!simulationRef.current || !canvasRef.current || !isMouseDownRef.current || gameState !== 'playing') return;
    e.preventDefault();

    const touch = e.touches[0];
    const rect = canvasRef.current.getBoundingClientRect();
    const x = touch.clientX - rect.left;
    const y = touch.clientY - rect.top;

    updateGunState({ cursorX: x, cursorY: y });
    lastMousePosRef.current = { x, y };
  }, [gameState, updateGunState]);

  const handleTouchEnd = useCallback(() => {
    isMouseDownRef.current = false;
    lastMousePosRef.current = null;
    updateGunState({ active: false });
  }, [updateGunState]);

  // Keyboard shortcuts
  useEffect(() => {
    let control = simulationRef.current?.getSpacecraftControl();

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

      // Get fresh control reference
      const sim = simulationRef.current;
      if (sim?.isSpacecraftModeActive()) {
        const ctrl = sim.getSpacecraftControl();
        if (ctrl) {
          switch (e.key.toLowerCase()) {
            case 'w':
            case 'arrowup':
              ctrl.setControl('thrust', true);
              e.preventDefault();
              break;
            case 's':
            case 'arrowdown':
              ctrl.setControl('reverse', true);
              e.preventDefault();
              break;
            case 'a':
            case 'arrowleft':
              ctrl.setControl('rotateLeft', true);
              e.preventDefault();
              break;
            case 'd':
            case 'arrowright':
              ctrl.setControl('rotateRight', true);
              e.preventDefault();
              break;
          }
        }
      }

      switch (e.key) {
        case 'Escape':
          e.preventDefault();
          if (showControls) {
            setShowControls(false);
          } else if (showUpgradeMenu) {
            setShowUpgradeMenu(false);
          } else if (gameState === 'playing') {
            setGameState('paused');
          } else if (gameState === 'paused') {
            setGameState('playing');
          }
          break;
        case 'u':
        case 'U':
          if (gameState === 'playing' || gameState === 'paused') {
            setShowUpgradeMenu(prev => !prev);
          }
          break;
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      const sim = simulationRef.current;
      if (sim?.isSpacecraftModeActive()) {
        const ctrl = sim.getSpacecraftControl();
        if (ctrl) {
          switch (e.key.toLowerCase()) {
            case 'w':
            case 'arrowup':
              ctrl.setControl('thrust', false);
              break;
            case 's':
            case 'arrowdown':
              ctrl.setControl('reverse', false);
              break;
            case 'a':
            case 'arrowleft':
              ctrl.setControl('rotateLeft', false);
              break;
            case 'd':
            case 'arrowright':
              ctrl.setControl('rotateRight', false);
              break;
          }
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
    };
  }, [gameState, showControls, showUpgradeMenu]);

  // Handle upgrade purchase
  const handlePurchaseUpgrade = useCallback((upgradeType: string) => {
    const result = purchaseUpgrade(upgradeType as any, upgrades, availablePoints);
    if (result.success) {
      setUpgrades(result.newUpgrades);
      setAvailablePoints(result.newPoints);
    }
  }, [upgrades, availablePoints]);

  // Render based on game state
  return (
    <div className="game-container">
      {/* Title Screen */}
      {gameState === 'title' && (
        <TitleScreen
          onPlay={handlePlay}
          onShowControls={() => setShowControls(true)}
        />
      )}

      {/* Game Canvas (always rendered, hidden behind overlays) */}
      {(gameState === 'playing' || gameState === 'paused' || gameState === 'gameOver') && (
        <div className="canvas-wrapper">
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

          {/* HUD */}
          <HUD
            score={sessionScore}
            upgrades={upgrades}
            availablePoints={availablePoints}
            gravityGunMode={gravityGunState.mode}
            gravityGunActive={gravityGunState.active}
            showUpgradeMenu={() => setShowUpgradeMenu(true)}
            compact={false}
          />
        </div>
      )}

      {/* Pause Menu */}
      {gameState === 'paused' && (
        <PauseMenu
          onResume={() => setGameState('playing')}
          onRestart={handleRestart}
          onShowControls={() => setShowControls(true)}
          onMainMenu={handleMainMenu}
        />
      )}

      {/* Game Over Screen */}
      {gameState === 'gameOver' && (
        <GameOverScreen
          score={sessionScore}
          onPlayAgain={handleRestart}
          onMainMenu={handleMainMenu}
        />
      )}

      {/* Controls Overlay */}
      {showControls && (
        <ControlsOverlay onClose={() => setShowControls(false)} />
      )}

      <style>{`
        .game-container {
          width: 100vw;
          height: 100vh;
          position: relative;
          overflow: hidden;
          background: #0a0a1a;
        }

        .canvas-wrapper {
          position: absolute;
          inset: 0;
        }

        .canvas-wrapper canvas {
          width: 100%;
          height: 100%;
          display: block;
        }

        /* HUD styles are in components/HUD.tsx */
      `}</style>
    </div>
  );
}