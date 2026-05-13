// FUL-5: Phase 6 - Status Bar Component

import { Material } from '../materials';
import { getMaterialById } from '../materials';
import BrushSelector from './BrushSelector';

interface StatusBarProps {
  selectedMaterial: Material;
  brushSize: 1 | 3 | 5;
  particleCount: number;
  onBrushSizeChange: (size: 1 | 3 | 5) => void;
}

export default function StatusBar({
  selectedMaterial,
  brushSize,
  particleCount,
  onBrushSizeChange,
}: StatusBarProps) {
  const materialInfo = getMaterialById(selectedMaterial);

  return (
    <div className="status-bar">
      <BrushSelector
        size={brushSize}
        onChange={onBrushSizeChange}
      />

      <div className="status-info">
        <div className="status-item">
          <span>Selected:</span>
          <strong>{materialInfo?.name || 'Unknown'}</strong>
        </div>

        <div className="status-item">
          <span>Particles:</span>
          <strong>{particleCount.toLocaleString()}</strong>
        </div>
      </div>

      <div className="keyboard-hints">
        <kbd>1</kbd>-<kbd>0</kbd>, <kbd>Q</kbd><kbd>W</kbd><kbd>E</kbd> Select material
      </div>
    </div>
  );
}