# FUL-5: Phase 6 - Editor & Gameplay Requirements

## Status: IN PROGRESS

## Issue Information
- **Issue ID**: FUL-5
- **Blocked by**: FUL-6 ✅ (Phase 3 - Full Material System)
- **Priority**: High
- **Sprint**: 2
- **Estimated Duration**: 2-3 days (16-24 hours)

---

## Executive Summary

Phase 6 transforms the falling sand simulation from a demo into an interactive editor, allowing users to paint materials and control the simulation in real-time.

---

## User Stories

### US-6.1: Material Selection
**As a** player
**I want** to select different materials from a palette
**So that** I can create varied simulations

**Acceptance Criteria:**
- [ ] 13 material buttons displayed in palette
- [ ] Each button shows material color
- [ ] Selected material is highlighted
- [ ] Keyboard shortcuts for materials (1-9, 0, q, w, e, r)

### US-6.2: Click-to-Spawn
**As a** player
**I want** to click on the canvas to place particles
**So that** I can create patterns and structures

**Acceptance Criteria:**
- [ ] Left-click spawns selected material
- [ ] Particles appear at cursor position
- [ ] Multiple particles spawn in brush radius
- [ ] Satisfying visual feedback on spawn

### US-6.3: Drag Painting
**As a** player
**I want** to drag while holding mouse button
**So that** I can draw continuous lines of material

**Acceptance Criteria:**
- [ ] Holding mouse spawns continuously
- [ ] Smooth line without gaps
- [ ] Performance remains stable during drag
- [ ] Works with all brush sizes

### US-6.4: Brush Size Control
**As a** player
**I want** to choose brush size
**So that** I can paint fine details or broad strokes

**Acceptance Criteria:**
- [ ] Small brush: 1px radius (1 particle)
- [ ] Medium brush: 3px radius (7 particles)
- [ ] Large brush: 5px radius (19 particles)
- [ ] Visual indicator shows brush size on cursor

### US-6.5: Play/Pause Control
**As a** player
**I want** to pause and resume the simulation
**So that** I can place materials without them moving

**Acceptance Criteria:**
- [ ] Pause stops all physics ticks
- [ ] Play resumes physics ticks
- [ ] Button clearly shows current state
- [ ] Keyboard shortcut: Space

### US-6.6: Clear Canvas
**As a** player
**I want** to clear all particles
**So that** I can start fresh

**Acceptance Criteria:**
- [ ] Single button clears entire grid
- [ ] Confirmation not required (quick action)
- [ ] Grid resets to empty state
- [ ] Keyboard shortcut: C

### US-6.7: Speed Control
**As a** player
**I want** to adjust simulation speed
**So that** I can watch slow burns or fast destruction

**Acceptance Criteria:**
- [ ] Speed slider or buttons
- [ ] Options: 0.5x, 1x, 2x, 4x
- [ ] Visual indicator of current speed
- [ ] Smooth transition between speeds

---

## UI Layout Specification

```
┌──────────────────────────────────────────────────────────────────┐
│  [▶ Play] [⏸ Pause]  |  [Clear]  |  Speed: [1x ▼]              │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│                                                                  │
│                                                                  │
│                       Canvas Area                                │
│                    (800 x 600 px)                                │
│                                                                  │
│                                                                  │
│                                                                  │
├──────────────────────────────────────────────────────────────────┤
│  Materials:                                                      │
│  [Sand] [Water] [Stone] [Fire] [Smoke] [BlackHole] [Steam]     │
│  [Ice] [Oil] [Wood] [Lava] [Ash] [Eraser]                        │
├──────────────────────────────────────────────────────────────────┤
│  Brush: [●] [○] [◎]  |  Selected: Sand  |  Particles: 12,345   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Technical Implementation

### State Management

```typescript
interface EditorState {
  // Grid state
  grid: Grid;
  width: number;
  height: number;
  
  // Editor state
  selectedMaterial: Material;
  brushSize: 1 | 3 | 5;
  isPlaying: boolean;
  speed: number;
  
  // Interaction state
  isMouseDown: boolean;
  lastMousePos: [number, number] | null;
}
```

### Event Handlers

```typescript
// Mouse events
canvas.onmousedown = (e) => {
  isMouseDown = true;
  spawnAtPosition(e.clientX, e.clientY);
};

canvas.onmousemove = (e) => {
  if (isMouseDown) {
    // Interpolate line between last and current position
    spawnLine(lastPos, currentPos);
  }
  lastMousePos = currentPos;
};

canvas.onmouseup = () => {
  isMouseDown = false;
};

// Touch support
canvas.ontouchstart = (e) => { /* same logic */ };
canvas.ontouchmove = (e) => { /* same logic */ };
canvas.ontouchend = () => { /* same logic */ };
```

### Spawn Logic

```typescript
function spawnAtPosition(x: number, y: number) {
  const gridX = Math.floor((x - canvasOffsetX) / scale);
  const gridY = Math.floor((y - canvasOffsetY) / scale);
  
  // Spawn in radius based on brush size
  const radius = (brushSize - 1) / 2;
  for (let dy = -radius; dy <= radius; dy++) {
    for (let dx = -radius; dx <= radius; dx++) {
      const dist = Math.sqrt(dx*dx + dy*dy);
      if (dist <= radius) {
        grid.set(gridX + dx, gridY + dy, selectedMaterial);
      }
    }
  }
}

function spawnLine(from: Point, to: Point) {
  // Bresenham's line algorithm
  const dx = to.x - from.x;
  const dy = to.y - from.y;
  const steps = Math.max(Math.abs(dx), Math.abs(dy));
  
  for (let i = 0; i <= steps; i++) {
    const t = i / steps;
    const x = Math.round(from.x + dx * t);
    const y = Math.round(from.y + dy * t);
    spawnAtPosition(x, y);
  }
}
```

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Space | Play/Pause toggle |
| C | Clear canvas |
| 1-9 | Select materials 1-9 |
| 0 | Select material 10 |
| Q | Select material 11 |
| W | Select material 12 |
| E | Select material 13 |
| [ | Decrease brush size |
| ] | Increase brush size |
| 1-4 | Speed: 0.5x, 1x, 2x, 4x |

---

## Performance Targets

| Metric | Target |
|--------|--------|
| Frame rate | 60 fps |
| Particle limit | 50,000+ |
| Spawn rate | 1000+ particles/frame during drag |
| Memory | < 100MB |
| Initial load | < 2s |

---

## Material Mapping

| Index | Material | Color | Key |
|-------|----------|-------|-----|
| 0 | Sand | #C2B280 | 1 |
| 1 | Water | #40A4DF | 2 |
| 2 | Stone | #808080 | 3 |
| 3 | Fire | #FF6432 | 4 |
| 4 | Smoke | #64646E | 5 |
| 5 | BlackHole | #000000 | 6 |
| 6 | Steam | #C8C8FF | 7 |
| 7 | Ice | #ADD8FA | 8 |
| 8 | Oil | #654321 | 9 |
| 9 | Wood | #8B5A2B | 0 |
| 10 | Lava | #FF4500 | Q |
| 11 | Ash | #323237 | W |
| 12 | Eraser | #14141E | E |

---

## Implementation Plan

### Day 1: Core Editor
- Create Vite + React project structure
- Implement basic canvas rendering
- Add material palette with selection
- Implement click-to-spawn

### Day 2: Enhanced Interaction
- Add drag painting with line interpolation
- Implement brush size control
- Add play/pause and clear buttons
- Add speed control

### Day 3: Polish
- Add keyboard shortcuts
- Add particle counter
- Performance optimization
- Mobile touch support
- Responsive layout

---

## File Structure

```
src/
├── editor/
│   ├── App.tsx              # Main editor component
│   ├── main.tsx             # React entry point
│   ├── components/
│   │   ├── Canvas.tsx       # Simulation canvas
│   │   ├── MaterialPalette.tsx
│   │   ├── ControlBar.tsx
│   │   ├── BrushSelector.tsx
│   │   └── StatusBar.tsx
│   ├── hooks/
│   │   ├── useSimulation.ts # Physics loop
│   │   └── useEditor.ts     # Editor state
│   ├── wasm/
│   │   └── simulation.ts    # WASM bindings
│   └── styles/
│       └── editor.css
├── index.html
├── vite.config.ts
└── package.json
```

---

## Success Criteria

- [ ] Material palette displays all 13 materials
- [ ] Click spawns particles at cursor
- [ ] Drag paints continuous lines
- [ ] Brush size affects spawn radius
- [ ] Play/Pause toggles simulation
- [ ] Clear resets grid
- [ ] Speed control works
- [ ] Keyboard shortcuts functional
- [ ] 60fps with 50k particles
- [ ] Responsive on mobile

---

*Generated: 2026-05-13*
*CTO Requirements Complete*
*Ready for Frontend Lead assignment*