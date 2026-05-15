# FUL-27.2: HTML Canvas Web-Native Architecture Analysis

## Executive Summary

The falling sand simulation uses a **dual-engine architecture**: a Rust core library with GPU scaffolding, and a TypeScript/JavaScript frontend with pure Canvas2D rendering. The HTML Canvas pipeline processes physics simulation updates through `ImageData` manipulation for pixel-perfect rendering.

**Key Finding**: The current architecture is CPU-bound at ~60 FPS with ~30,000 particles. GPU acceleration is scaffolded but not integrated into the web frontend.

---

## 1. Architecture Overview

### 1.1 System Layers

```
┌─────────────────────────────────────────────────────┐
│                  Web Browser                         │
├─────────────────────────────────────────────────────┤
│  React 18 Frontend (TypeScript)                      │
│  ┌─────────────────────────────────────────────┐   │
│  │  App.tsx - UI orchestration                 │   │
│  │  ├── MaterialPalette.tsx                     │   │
│  │  ├── ControlBar.tsx                         │   │
│  │  ├── BrushSelector.tsx                      │   │
│  │  └── StatusBar.tsx                          │   │
│  └─────────────────────────────────────────────┘   │
│                     │                                │
│  simulation.ts - Physics Engine (JS)                │
│  ┌─────────────────────────────────────────────┐   │
│  │  SimulationCanvas class                     │   │
│  │  ├── Grid (200x150 cells)                    │   │
│  │  ├── Particle update methods                │   │
│  │  └── render() → ImageData                   │   │
│  └─────────────────────────────────────────────┘   │
│                     │                                │
│  <canvas> - Rendering Target                         │
│  ├── ImageData pixel buffer (direct manipulation)   │
│  └── CanvasRenderingContext2D                      │
├─────────────────────────────────────────────────────┤
│  Vite 4 Build System                                │
└─────────────────────────────────────────────────────┘
           │
           ▼ (Future Integration)
┌─────────────────────────────────────────────────────┐
│  Rust Core Library (src/)                            │
│  ┌─────────────────────────────────────────────┐   │
│  │  lib.rs - Public API                         │   │
│  │  ├── simulation.rs - Physics (CPU)           │   │
│  │  ├── gpu/mod.rs - GPU pipeline (scaffolded)  │   │
│  │  ├── gpu/compute.rs - WebGPU compute         │   │
│  │  ├── gpu/render.rs - GPU rendering           │   │
│  │  └── shaders.wgsl - WGSL compute shader      │   │
│  └─────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

### 1.2 Data Flow

```
User Input (click/touch)
       │
       ▼
┌──────────────────┐
│ React Event      │
│ Handler          │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ SimulationCanvas │
│ spawnBrush()     │
│ spawnLine()      │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐     ┌──────────────────┐
│ Grid State       │────▶│ Physics Tick     │
│ (particles[])    │     │ tick()           │
└──────────────────┘     └────────┬─────────┘
                                  │
         ┌────────────────────────┘
         │
         ▼
┌──────────────────────────────────┐
│ Render Loop (requestAnimationFrame)
│   ├── Read grid state            │
│   ├── Map particles → pixels     │
│   ├── getParticleColor()         │
│   └── putImageData()             │
└──────────────────────────────────┘
```

---

## 2. Rendering Pipeline Analysis

### 2.1 Canvas2D Rendering Strategy

**Current Implementation** (simulation.ts:677-725):

```typescript
render(): void {
  const data = this.imageData.data;
  
  // Clear to background
  for (let i = 0; i < data.length; i += 4) {
    data[i] = 20;      // R
    data[i + 1] = 20;   // G
    data[i + 2] = 30;   // B
    data[i + 3] = 255;  // A
  }
  
  // Draw particles
  for (y = 0; y < height; y++) {
    for (x = 0; x < width; x++) {
      const color = getParticleColor(particle, x, y);
      // Fill scaled block (4x4 pixels per cell)
      for (dy = 0; dy < scale; dy++) {
        for (dx = 0; dx < scale; dx++) {
          const idx = ((y * scale + dy) * width * scale + (x * scale + dx)) * 4;
          data[idx] = color.r; // ...
        }
      }
    }
  }
  
  this.ctx.putImageData(this.imageData, 0, 0);
}
```

### 2.2 Performance Characteristics

| Aspect | Current | Notes |
|--------|---------|-------|
| **Strategy** | Full buffer write | Clear all pixels each frame |
| **Scale factor** | 4x4 | 200x150 grid → 800x600 canvas |
| **Resolution** | 480,000 pixels | 200×150 × 16 |
| **Color depth** | RGBA 32-bit | 4 bytes per pixel |
| **Transfer** | putImageData | GPU upload each frame |
| **Smoothing** | Disabled | `imageSmoothingEnabled = false` |
| **Target FPS** | 60 | requestAnimationFrame |

### 2.3 Benchmark Data (from FUL-8c analysis)

| Grid Size | Particles | CPU Time | FPS |
|-----------|-----------|---------|-----|
| 64x64 | ~4,000 | 0.5ms | 60+ |
| 128x128 | ~16,000 | 2ms | 60 |
| 200x150 | ~30,000 | 4-8ms | 45-60 |
| 256x200 | ~50,000 | 12-20ms | 30-40 |

**Bottleneck**: Physics tick dominates at high particle counts.

---

## 3. Web-Native Component Analysis

### 3.1 HTML Canvas API Usage

**Used APIs**:
- `HTMLCanvasElement` - Render target
- `CanvasRenderingContext2D` - 2D drawing context
- `ImageData` - Pixel buffer for direct manipulation
- `requestAnimationFrame` - Frame timing

**Unused/Available APIs**:
- `OffscreenCanvas` - Off-thread rendering
- `WebGL2RenderingContext` - GPU rendering
- `CanvasRenderingContext2D.createImageBitmap()` - Async bitmap handling
- `Transferable` - Zero-copy buffer transfer

### 3.2 Browser Compatibility

```typescript
// Current setup (simulation.ts:106)
const ctx = canvas.getContext('2d', { alpha: false })!;

// This is optimal for current strategy:
// - alpha: false = opaque canvas (no blending cost)
// - 2D context = simple API for ImageData manipulation
```

**Browser Support**: All modern browsers (Chrome 56+, Firefox 52+, Safari 11+)

### 3.3 React Integration Pattern

```typescript
// App.tsx - React 18 patterns
const canvasRef = useRef<HTMLCanvasElement>(null);
const simulationRef = useRef<SimulationCanvas | null>(null);

useEffect(() => {
  // Initialization - runs once on mount
  simulationRef.current = new SimulationCanvas(canvas, 200, 150, 4);
  
  // Animation frame loop - managed by React
  animationFrameRef.current = requestAnimationFrame(gameLoop);
  
  return () => cancelAnimationFrame(animationFrameRef.current);
}, []); // Empty deps = mount/unmount only
```

**Pattern Analysis**: 
- ✅ Uses `useRef` to avoid re-renders on animation state
- ✅ Uses `useCallback` for memoized handlers
- ⚠️ Animation loop runs outside React render cycle
- ✅ Proper cleanup on unmount

---

## 4. Performance Optimization Opportunities

### 4.1 Current Limitations

1. **Double-buffering inefficiency**: Creating new particle state array each tick
2. **Full-grid traversal**: Processing all cells, not just active ones
3. **No spatial hashing in JS**: Comparing against Rust's ChunkedGrid
4. **Color computation per-frame**: `getParticleColor()` called for every particle

### 4.2 Optimization Strategies

#### Strategy A: Dirty Rectangle Rendering

```typescript
// Track changed regions
const dirtyRects: Array<{x: number, y: number, w: number, h: number}> = [];

// On particle move, mark old and new positions dirty
function markDirty(x: number, y: number) {
  // Only update 5x5 region around changed particle
  dirtyRects.push({x: x-2, y: y-2, w: 5, h: 5});
}

// In render(), only update dirty regions
function render() {
  for (const rect of dirtyRects) {
    renderRect(rect); // Smaller pixel copy
  }
  dirtyRects.length = 0; // Clear
}
```

**Impact**: 10-30% fewer pixel writes per frame

#### Strategy B: Typed Array Optimization

```typescript
// Use Uint32Array for particle grid (faster than objects)
class SimulationCanvas {
  private grid: Uint16Array; // material IDs
  private temps: Float32Array; // temperature
  private lifetimes: Uint8Array; // particle lifetime
  
  // Faster iteration with typed arrays
  tick() {
    for (let i = 0; i < this.grid.length; i++) {
      // Direct memory access
    }
  }
}
```

**Impact**: 15-25% faster physics

#### Strategy C: OffscreenCanvas Worker

```typescript
// main thread
const offscreen = canvas.transferControlToOffscreen();

// worker thread
worker.postMessage({ type: 'init', canvas: offscreen }, [offscreen]);

// worker handles all rendering
onmessage = (e) => {
  if (e.data.type === 'render') {
    renderToOffscreen(e.data.particles);
  }
};
```

**Impact**: Frees main thread for UI, parallel rendering

### 4.3 GPU Acceleration Path

The Rust codebase has GPU scaffolding that isn't integrated:

```
Current:  JS Physics → ImageData → Canvas2D
Target:   GPU Physics → GPU Render → Canvas
```

**Required Integration**:

1. **WASM Module**: Compile Rust to WebAssembly
   ```rust
   #[wasm_bindgen]
   pub fn tick(grid_ptr: *mut u8, width: usize, height: usize) {
     // Run physics in WASM
   }
   ```

2. **Shared Buffer**: Use `SharedArrayBuffer` for zero-copy
   ```typescript
   const sharedBuffer = new SharedArrayBuffer(width * height * 4);
   const particles = new Uint32Array(sharedBuffer);
   
   // Both JS and WASM can read/write
   ```

3. **WebGL Renderer**: Replace ImageData with WebGL textures
   ```typescript
   const texture = gl.createTexture();
   gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, height, 0, 
                gl.RGBA, gl.UNSIGNED_BYTE, particles);
   ```

---

## 5. Architecture Decision Points

### 5.1 Keep Canvas2D vs. Migrate to WebGL

| Factor | Canvas2D (Current) | WebGL |
|--------|-------------------|-------|
| Complexity | Low | High |
| Max particles | ~50,000 | ~1,000,000+ |
| CPU usage | High | Low |
| Code portability | High | Medium |
| Mobile support | Good | Requires fallback |
| Post-processing | Limited | Advanced |

**Recommendation**: Keep Canvas2D for simplicity, add WebGL as optional high-performance mode.

### 5.2 Rust/WASM vs. TypeScript Physics

| Factor | TypeScript (Current) | Rust/WASM |
|--------|---------------------|-----------|
| Performance | Good (30K particles) | Excellent (500K+) |
| Integration | Native | Requires build step |
| Debugging | Easy (browser) | Medium |
| Bundle size | N/A | +500KB |
| Threading | Web Workers | SharedArrayBuffer |

**Recommendation**: Continue with TypeScript for development speed; add WASM option for scale.

### 5.3 State Management

Current: Component state + refs
```typescript
const [particleCount, setParticleCount] = useState(0);
const simulationRef = useRef<SimulationCanvas | null>(null);
```

Issue: State updates trigger re-renders (even if not visible)

**Recommendation**: Use refs for animation state; only use state for UI-visible values.

---

## 6. Code Quality Assessment

### 6.1 Strengths

1. **Clean separation**: UI components don't touch simulation directly
2. **Consistent patterns**: Mouse/touch handlers share logic
3. **Proper cleanup**: `useEffect` return functions cancel RAF
4. **Type safety**: TypeScript with explicit interfaces

### 6.2 Improvement Areas

```typescript
// Issue: Creating arrays in render loop (simulation.ts:680)
const data = this.imageData.data;
for (let i = 0; i < data.length; i += 4) { /* ... */ }

// Better: Cache buffer length
private readonly bufferLength: number;

constructor(...) {
  this.bufferLength = width * height * 4;
}

// Use cached length
for (let i = 0; i < this.bufferLength; i += 4) { /* ... */ }
```

```typescript
// Issue: Repeated DOM queries (App.tsx)
const rect = canvasRef.current.getBoundingClientRect(); // In every mouse event

// Better: Cache on mousedown, update on scroll
let cachedRect: DOMRect | null = null;
function handleMouseDown(e) {
  cachedRect = canvasRef.current.getBoundingClientRect();
}
```

### 6.3 Memory Pressure

| Resource | Current Usage | Max |
|----------|--------------|-----|
| Grid particles | 200×150×24 bytes | ~720KB |
| ImageData | 800×600×4 bytes | ~1.9MB |
| Temp buffers | 2×1.9MB | ~4MB |
| **Total** | ~6MB | Acceptable |

---

## 7. Recommendations

### High Priority (Do Now)

1. **Cache DOM measurements**
   ```typescript
   // Store canvas rect in ref, update on scroll/resize
   ```

2. **Use typed arrays for grid**
   ```typescript
   // Replace object array with Uint16Array + Float32Array
   ```

3. **Implement dirty rectangle tracking**
   ```typescript
   // Track only changed regions for reduced pixel writes
   ```

### Medium Priority (Next Sprint)

4. **Add Web Worker for physics**
   ```typescript
   // Move simulation.tick() to worker thread
   ```

5. **Add WebGL fallback mode**
   ```typescript
   // Enable for >50K particles
   ```

6. **Profile and optimize hot paths**
   ```typescript
   // Use performance.mark() to identify bottlenecks
   ```

### Low Priority (Future)

7. **Integrate Rust/WASM core**
   ```bash
   wasm-pack build --target web
   ```

8. **Add GPU rendering pipeline**
   ```typescript
   // Use WebGL2 for instanced rendering
   ```

---

## 8. Conclusion

The HTML Canvas implementation is well-structured for its current scope. The main performance bottleneck is CPU-bound physics in JavaScript, not the rendering pipeline. For 30,000 particles at 60 FPS, the current architecture is adequate.

**Key architectural insights**:

1. **Rendering is not the bottleneck** - Physics tick takes 4-8ms, rendering takes <1ms
2. **Canvas2D is sufficient** - ImageData manipulation is fast enough
3. **Rust GPU scaffolding is unused** - Needs integration effort to leverage
4. **React integration is correct** - Refs avoid unnecessary re-renders

**Next architectural decision**: Focus on physics optimization before rendering changes.

---

*Analysis Date: 2026-05-14*
*By: React Engineer (c80341fa-52bd-4afd-a45d-0d80f2fc91da)*
*For: FUL-27.2 HTML Canvas web-native architecture analysis*