# FUL-5: Phase 6 - Editor & Gameplay

## Status: SCOPING COMPLETE ✅

## Issue Information
- **Issue ID**: FUL-5
- **Blocked by**: FUL-6 ✅ (Phase 3 - Full Material System)
- **Priority**: High
- **Estimated Duration**: 2-3 days

## Executive Summary

Phase 6 adds an interactive editor for placing particles and basic gameplay controls. This transforms the simulation from a demo into a user-facing application.

## Current State (Post-Phase 3)

| Component | Status |
|-----------|--------|
| 13 Materials | ✅ |
| Temperature/Phase System | ✅ |
| Canvas2D Renderer | ✅ |
| WASM Bindings | ✅ |
| Interactive Editor | ❌ |
| User Controls | ❌ |

## Phase 6 Scope

### Core Editor Features

| Feature | Priority | Description |
|---------|----------|-------------|
| Material Palette | P0 | Visual selection of 13 materials |
| Click-to-Spawn | P0 | Left-click places selected material |
| Brush Sizes | P0 | Small (1px), Medium (3px), Large (5px) |
| Clear Canvas | P0 | Reset entire grid |
| Pause/Play | P0 | Toggle simulation |

### Enhanced Editor Features

| Feature | Priority | Description |
|---------|----------|-------------|
| Drag-to-Paint | P0 | Hold mouse to continuously spawn |
| Undo/Redo | P1 | Step back/forward through states |
| Speed Control | P1 | Adjust simulation speed |
| Save/Load | P2 | Persist patterns to localStorage |
| Screenshot | P2 | Export current state as PNG |

### UI Layout

```
┌─────────────────────────────────────────────────────────┐
│  Controls: [▶ Play] [⏸ Pause] [Clear] Speed: [====]   │
├─────────────────────────────────────────────────────────┤
│                                                         │
│                                                         │
│                   Canvas (Simulation)                    │
│                                                         │
│                                                         │
├────────────┬────────────────────────────────────────────┤
│ Materials  │  [Sand][Water][Stone][Fire][Smoke]...      │
│ Palette    │  [BlackHole][Steam][Ice][Oil][Wood]...     │
│            │  [Lava][Ash]                               │
├────────────┴────────────────────────────────────────────┤
│  Brush: [●][○][◎]  Selected: Sand  Particles: 0        │
└─────────────────────────────────────────────────────────┘
```

## Implementation Approach

### Option A: Simple HTML Canvas (Fastest)
- Single HTML file with inline JavaScript
- Canvas2D for rendering
- Simple DOM buttons for UI
- **Pros**: Quick to implement, no build step
- **Cons**: Limited performance, basic look

### Option B: React + Canvas (Balanced)
- React for UI components
- Canvas2D for simulation rendering
- State management for editor
- **Pros**: Good UX, componentized
- **Cons**: Requires build step

### Option C: WASM + React (Production)
- Rust simulation in WASM (already exists)
- React frontend
- Web Workers for physics (non-blocking)
- **Pros**: Best performance, scalable
- **Cons**: More setup

## Recommendation

**Option B (React + Canvas)** - Balances development speed with good user experience. Leverages existing Canvas2D renderer code pattern.

## Files to Create

```
src/editor/
├── Editor.tsx           # Main editor component
├── MaterialPalette.tsx  # Material selection UI
├── ControlBar.tsx       # Play/Pause/Clear buttons
├── BrushSelector.tsx    # Brush size buttons
└── styles.css           # Editor styles

src/editor/
├── App.tsx              # Main app entry
├── main.tsx             # React mount
├── index.html            # HTML shell
├── simulation-worker.ts  # Web Worker for physics (optional)
└── vite.config.ts       # Build config
```

## Technical Decisions

### Rendering Strategy
- Use `requestAnimationFrame` for smooth rendering
- Separate physics tick rate (60fps) from render rate
- Batch particle spawns during drag

### State Management
```typescript
interface EditorState {
  grid: Grid;
  selectedMaterial: Material;
  brushSize: 1 | 3 | 5;
  isPlaying: boolean;
  speed: number; // 1x, 2x, 4x
}
```

### Event Handling
- `mousedown` + `mousemove` for drag-painting
- `mouseup` to stop painting
- Touch events for mobile support

## Acceptance Criteria

- [ ] Material palette displays all 13 materials with colors
- [ ] Clicking canvas spawns selected material
- [ ] Drag painting works smoothly
- [ ] Brush size affects spawn radius
- [ ] Play/Pause toggles simulation
- [ ] Clear resets grid to empty
- [ ] Speed control changes tick rate
- [ ] 60fps rendering with 10,000+ particles
- [ ] Responsive layout (mobile-friendly)

## Team Assignment

**Recommended: Frontend Lead** with Rust support for WASM integration

**Estimated Hours:** 16-24 hours

## Dependencies

- [x] FUL-6: Phase 3 Material System (done)
- [ ] None remaining

---

*Generated: 2026-05-13*
*CTO Scoping Complete*