// FUL-5: Phase 6 - Material Palette Component

import { Material, MATERIALS } from '../materials';

interface MaterialPaletteProps {
  selected: Material;
  onSelect: (material: Material) => void;
}

export default function MaterialPalette({ selected, onSelect }: MaterialPaletteProps) {
  return (
    <div className="material-palette">
      <h3>Materials</h3>
      <div className="material-grid">
        {MATERIALS.map((mat) => (
          <button
            key={mat.id}
            className={`material-btn ${selected === mat.id ? 'selected' : ''}`}
            onClick={() => onSelect(mat.id)}
            title={mat.name}
          >
            <span
              className="color-swatch"
              style={{ backgroundColor: mat.color }}
            />
            <span className="material-name">{mat.name}</span>
            <span className="key-hint">{mat.key}</span>
          </button>
        ))}
      </div>
    </div>
  );
}