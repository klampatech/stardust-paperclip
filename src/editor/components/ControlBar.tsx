// FUL-5: Phase 6 - Control Bar Component
import type { OverlayMode } from '../simulation';

interface ControlBarProps {
  isPlaying: boolean;
  speed: number;
  overlayMode: OverlayMode;
  onTogglePlay: () => void;
  onClear: () => void;
  onSpeedChange: (speed: number) => void;
  onStep?: () => void;
  onToggleStructures?: () => void;
  showStructures?: boolean;
  onOverlayChange?: (mode: OverlayMode) => void;
}

const SPEED_OPTIONS = [
  { value: 0.1, label: '0.1x' },
  { value: 0.5, label: '0.5x' },
  { value: 1, label: '1x' },
  { value: 2, label: '2x' },
  { value: 4, label: '4x' },
  { value: 10, label: '10x' },
];

export default function ControlBar({
  isPlaying,
  speed,
  overlayMode,
  onTogglePlay,
  onClear,
  onSpeedChange,
  onStep,
  onToggleStructures,
  showStructures,
  onOverlayChange,
}: ControlBarProps) {
  return (
    <div className="control-bar">
      <button
        className={`primary ${isPlaying ? '' : 'active'}`}
        onClick={onTogglePlay}
        title={isPlaying ? 'Pause (Space)' : 'Play (Space)'}
      >
        {isPlaying ? '⏸ Pause' : '▶ Play'}
      </button>

      {onStep && (
        <button
          onClick={onStep}
          title="Step one frame (S)"
          disabled={isPlaying}
        >
          ⏭ Step
        </button>
      )}

      <div className="divider" />

      <button
        onClick={onClear}
        title="Clear canvas (C)"
      >
        🗑 Clear
      </button>

      <div className="divider" />

      <div className="speed-control">
        <label>Speed:</label>
        <select
          value={speed}
          onChange={(e) => onSpeedChange(parseFloat(e.target.value))}
        >
          {SPEED_OPTIONS.map((opt) => (
            <option key={opt.value} value={opt.value}>
              {opt.label}
            </option>
          ))}
        </select>
      </div>

      <div className="divider" />

      <div className="overlay-control">
        <label>Overlay:</label>
        <button
          className={overlayMode === 'none' ? 'active' : ''}
          onClick={() => onOverlayChange?.('none')}
          title="No overlay (P)"
        >
          None
        </button>
        <button
          className={overlayMode === 'temperature' ? 'active' : ''}
          onClick={() => onOverlayChange?.('temperature')}
          title="Temperature overlay (P)"
        >
          🌡️ Temp
        </button>
        <button
          className={overlayMode === 'velocity' ? 'active' : ''}
          onClick={() => onOverlayChange?.('velocity')}
          title="Velocity overlay (P)"
        >
          💨 Vel
        </button>
      </div>

      {onToggleStructures && (
        <>
          <div className="divider" />
          <button
            className={showStructures ? 'active' : ''}
            onClick={onToggleStructures}
            title="Toggle structures"
          >
            🏗 Structures
          </button>
        </>
      )}

      <div className="keyboard-hints">
        <kbd>Space</kbd> Play/Pause &nbsp;
        <kbd>C</kbd> Clear &nbsp;
        <kbd>S</kbd> Step &nbsp;
        <kbd>P</kbd> Overlay &nbsp;
        <kbd>[</kbd>/<kbd>]</kbd> Brush &nbsp;
        <kbd>RMB</kbd> Pick
      </div>
    </div>
  );
}