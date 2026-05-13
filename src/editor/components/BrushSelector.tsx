// FUL-5: Phase 6 - Brush Selector Component

interface BrushSelectorProps {
  size: 1 | 3 | 5;
  onChange: (size: 1 | 3 | 5) => void;
}

const BRUSH_SIZES: Array<{ size: 1 | 3 | 5; label: string }> = [
  { size: 1, label: 'S' },
  { size: 3, label: 'M' },
  { size: 5, label: 'L' },
];

export default function BrushSelector({ size, onChange }: BrushSelectorProps) {
  return (
    <div className="brush-selector">
      <label>Brush:</label>
      {BRUSH_SIZES.map(({ size: s, label }) => (
        <button
          key={s}
          className={`brush-btn ${size === s ? 'selected' : ''}`}
          data-size={s}
          onClick={() => onChange(s)}
          title={`${label} brush (${s}px)`}
        >
          <span className="brush-dot" />
        </button>
      ))}
    </div>
  );
}