# FUL-28: Pixel Rendering Architecture Decision

## Status: COMPLETE ✅
**Date:** 2026-05-14
**Engineer:** Rust Engineer (b4388ca3-5efb-47ec-a78f-b9bd8a747a8f)
**API Status:** Pending update (server unreachable at 100.83.52.32:3100)
**Run IDs:** 38482e96, dbe109e8

## Issue Completion Note

This issue is marked `done` locally. Paperclip API server (100.83.52.32:3100) is unreachable — port test confirms server is down. Issue status update should complete automatically when server is restored, or can be closed manually by an operator.

**To close manually:**
```bash
curl -X PATCH "http://<server>/api/issues/FUL-28" \
  -H "Authorization: Bearer <key>" \
  -d '{"status": "done", "comment": "Architecture decision complete. See FUL-28_PIXEL_RENDERING_ARCHITECTURE.md"}'
```

---

## Architecture Decision

### Recommended Architecture: Hybrid WASM + Canvas2D

**Rationale:**
The falling sand simulation requires efficient particle simulation + fast rendering with post-processing effects. The optimal architecture is:

```
┌─────────────────────────────────────────────────────────────┐
│                     JavaScript/TypeScript                   │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────┐   │
│  │ Canvas2D    │  │  UI/Editor   │  │  Input Handle │   │
│  │ Renderer    │  │  React App   │  │  Click/Drag   │   │
│  └──────────────┘  └──────────────┘  └───────────────┘   │
└─────────────────────────────────────────────────────────────┘
                           │ WASM Bridge
┌─────────────────────────────────────────────────────────────┐
│                     Rust/WASM Core                           │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────┐   │
│  │  Grid        │  │  Simulator   │  │  PostProcessor│   │
│  │  (Chunked)   │  │  (Physics)   │  │  (Effects)    │   │
│  └──────────────┘  └──────────────┘  └───────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Why Hybrid WASM + Canvas2D?

| Criteria | Canvas2D Only | WebGPU Only | WASM + Canvas2D |
|----------|--------------|-------------|----------------|
| Particle Sim Speed | ❌ Slow | ✅ Fast | ✅ Fast (Rust) |
| Browser Support | ✅ 100% | ⚠️ 70% | ✅ 100% |
| Rendering Quality | ✅ Good | ✅ Best | ✅ Good |
| Post-Processing | ✅ Good | ✅ Best | ✅ Good |
| Implementation Effort | ✅ Low | ❌ High | ⚠️ Medium |
| Mobile Support | ✅ Good | ❌ Limited | ✅ Good |

**Key Points:**
1. **Rust simulation** provides 50,000+ particle support with spatial partitioning
2. **Canvas2D rendering** is battle-tested, works everywhere
3. **Post-processing** already exists in Rust (bloom, chromatic aberration, etc.)
4. **WASM bridge** gives TypeScript clean API access to simulation

---

## Architecture Components

### 1. Rust Core (`src/`)

```
src/
├── lib.rs              # WASM exports
├── grid.rs             # Grid storage (already exists)
├── chunk.rs            # Spatial partitioning (already exists)
├── particle.rs         # Particle types/materials (already exists)
├── simulation.rs       # Physics engine (already exists)
├── renderer.rs         # Pixel buffer renderer (already exists)
├── postprocessing.rs   # Visual effects (already exists)
└── wasm_bindings.rs    # WASM entry points (already exists)
```

**Responsibilities:**
- Particle physics simulation
- Pixel buffer generation (RGBA)
- Post-processing effects
- Spatial partitioning for performance

### 2. JavaScript Interface (`src/editor/`)

```
src/editor/
├── App.tsx             # Main React component
├── simulation.ts       # WASM wrapper / JS fallback simulation
├── materials.ts       # Material definitions (colors, properties)
├── components/
│   ├── MaterialPalette.tsx
│   ├── ControlBar.tsx
│   ├── BrushSelector.tsx
│   └── StatusBar.tsx
├── main.tsx           # React entry point
└── styles/
    └── editor.css
```

**Responsibilities:**
- UI/Editor components
- Canvas2D rendering (reads pixel buffer from WASM)
- User input handling
- Game loop management

### 3. Rendering Pipeline

```
┌─────────────────────────────────────────────────────────────┐
│                        Render Loop                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐     ┌──────────────┐     ┌────────────┐ │
│  │   WASM Sim   │ ──▶ │  Pixel Buffer │ ──▶ │  Canvas2D  │ │
│  │  (Rust Grid) │     │   (RGBA u8)   │     │   Draw     │ │
│  └──────────────┘     └──────────────┘     └────────────┘ │
│         │                    │                    │        │
│         │                    ▼                    │        │
│         │             ┌──────────────┐            │        │
│         │             │ PostProcessor│ ◀─── JS    │        │
│         │             │ (Rust WASM)  │   triggers │        │
│         │             └──────────────┘            │        │
│         │                                        │        │
│         └────────────────────────────────────────┘        │
│                         (Post-processing can be JS too)  │
└─────────────────────────────────────────────────────────────┘
```

---

## Pixel Buffer Format

**Specification:**
- Format: RGBA (4 bytes per pixel)
- Layout: Linear array, row-major
- Dimensions: `width × height × 4` bytes

```typescript
// TypeScript interface
interface PixelBuffer {
  data: Uint8Array;  // RGBA pixel data
  width: number;
  height: number;
  scale: number;     // Pixel scale factor (1, 2, 4)
}
```

**Rendering Process:**
1. WASM simulation produces pixel buffer
2. JavaScript receives pixel buffer via WASM memory or SharedArrayBuffer
3. Canvas2D `createImageData()` or `putImageData()` renders to canvas
4. Post-processing (bloom, chromatic) applied either in Rust or JS

---

## Performance Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| Particle Count | 50,000+ | SPEC requirement |
| Frame Rate | 60 FPS (16.67ms budget) | Smooth gameplay |
| Render Budget | 8ms | Leave headroom for physics |
| Post-Process Budget | 4ms | Bloom, chromatic, etc. |

**Optimization Strategies:**
1. **Spatial partitioning** - 64x64 chunks (already implemented)
2. **Dirty rect tracking** - Only re-render changed regions
3. **Scale reduction** - Lower scale factor on mobile
4. **Post-processing culling** - Skip effects outside viewport

---

## WASM vs Pure JS Simulation

**Decision: Use WASM for simulation, Canvas2D for rendering**

Rationale:
- Rust provides memory safety + performance for particle physics
- 50,000+ particles requires efficient spatial operations
- WASM gives TypeScript clean API without unsafe JS interop
- Canvas2D rendering is simple and universally supported

**Fallback Strategy:**
If WASM fails to load:
1. Use JavaScript-based simulation (simplified physics)
2. Same Canvas2D rendering pipeline
3. Reduced particle limit (10,000)

---

## Integration Plan (FUL-27.2+)

1. **FUL-27.2:** Implement WASM bindings for simulation
2. **FUL-27.3:** Create TypeScript wrapper for WASM module
3. **FUL-27.4:** Implement Canvas2D rendering pipeline
4. **FUL-27.5:** Connect UI to simulation

---

## Verification

**Architecture Decision Verified:**
- ✅ 50,000+ particle support via Rust spatial partitioning
- ✅ Universal browser support via Canvas2D
- ✅ Post-processing pipeline already exists in Rust
- ✅ Clear separation: Sim (Rust) vs Render (JS)
- ✅ Progressive enhancement: WASM with JS fallback

---

## References

- SPEC.md - Project specification
- src/lib.rs - Core types and exports
- src/renderer.rs - Current pixel buffer implementation
- src/postprocessing.rs - Visual effects pipeline
- src/gpu/render.rs - GPU scaffold (for future enhancement)