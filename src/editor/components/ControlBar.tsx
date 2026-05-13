// FUL-5: Phase 6 - Control Bar Component

interface ControlBarProps {
  isPlaying: boolean;
  speed: number;
  onTogglePlay: () => void;
  onClear: () => void;
  onSpeedChange: (speed: number) => void;
}

const SPEED_OPTIONS = [
  { value: 0.5, label: '0.5x' },
  { value: 1, label: '1x' },
  { value: 2, label: '2x' },
  { value: 4, label: '4x' },
];

export default function ControlBar({
  isPlaying,
  speed,
  onTogglePlay,
  onClear,
  onSpeedChange,
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

      <div className="keyboard-hints">
        <kbd>Space</kbd> Play/Pause &nbsp;
        <kbd>C</kbd> Clear &nbsp;
        <kbd>[</kbd>/<kbd>]</kbd> Brush
      </div>
    </div>
  );
}