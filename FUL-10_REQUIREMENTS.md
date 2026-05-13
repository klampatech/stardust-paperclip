# FUL-10 FUL-8b: GPU Compute Shader Pipeline - Requirements

## Issue: FUL-10/8b
**Type:** Feature Enhancement
**Priority:** High
**Target:** 60fps with 50,000+ particles

---

## Overview

Implement GPU compute shader acceleration for the falling sand particle simulation using WebGPU (wgpu). This enables parallel processing of particle physics across thousands of GPU threads, dramatically improving performance for large simulations.

---

## Implementation Scope

### Core Components

| Component | Status | Notes |
|-----------|--------|-------|
| `src/gpu/mod.rs` | ✅ Done | Module declaration |
| `src/gpu/compute.rs` | ✅ Done | GPU pipeline with WebGPU |
| `src/gpu/shaders.wgsl` | ✅ Done | WGSL compute shader |
| `examples/gpu_demo.rs` | ✅ Done | Demo with benchmark |

### GPU Pipeline Features

- [ ] WebGPU initialization with fallback
- [ ] Compute shader for particle physics (sand, water, fire, etc.)
- [ ] Buffer management for particle data (u32 encoded per cell)
- [ ] Uniform buffer for simulation parameters
- [ ] Async initialization pattern

### Compute Shader Requirements

- [ ] Material encoding (13 materials = 5 bits)
- [ ] Gravity physics (sand, water, oil, ice, ash fall)
- [ ] Rise physics (fire, smoke, steam rise)
- [ ] Diagonal movement for natural piling
- [ ] Collision detection (4-neighbor checks)
- [ ] Atomic operations for thread safety

---

## Architecture

```
┌─────────────────────────────────────────┐
│            CPU Side (Rust)              │
├─────────────────────────────────────────┤
│  Grid → Encode → GPU Buffer            │
│  GPU Buffer → Decode → Grid            │
├─────────────────────────────────────────┤
│            GPU Side (WGSL)              │
├─────────────────────────────────────────┤
│  @compute shader (per-cell threads)     │
│  - Read particle data                   │
│  - Calculate new position               │
│  - Write result with atomic swap        │
└─────────────────────────────────────────┘
```

### Particle Data Encoding

```
Bits 0-4: Material ID (0-12 for 13 materials)
Bits 5-31: Reserved for future flags
```

### Material IDs

| ID | Material |
|----|----------|
| 0 | Air |
| 1 | Sand |
| 2 | Water |
| 3 | Stone |
| 4 | Fire |
| 5 | Smoke |
| 6 | BlackHole |
| 7 | Steam |
| 8 | Ice |
| 9 | Oil |
| 10 | Wood |
| 11 | Lava |
| 12 | Ash |

---

## Success Criteria

- [x] GPU module created with WebGPU support
- [ ] Compute shader compiles (WGSL syntax validated)
- [ ] Pipeline initializes without panics
- [ ] Falls back to CPU when GPU unavailable
- [ ] Performance benchmark shows improvement
- [ ] 60fps target achievable with 50k particles

---

## Build Commands

```bash
# Without GPU (default)
cargo build

# With GPU support
cargo build --features gpu

# Run GPU demo
cargo run --example gpu_demo --features gpu

# Run benchmarks
cargo run --example gpu_demo --features gpu --release
```

---

## Limitations & Notes

1. **Thread Safety**: GPU particle swaps require atomic operations to prevent race conditions
2. **Synchronization**: Bottom-to-top processing in CPU needs equivalent in GPU (work groups/barriers)
3. **Memory Transfer**: GPU↔CPU buffer sync adds latency; minimize transfers
4. **WASM Support**: WebGPU in browsers requires HTTPS and user consent

---

## Future Enhancements

- [ ] Render pipeline (vertex/fragment shaders for visualization)
- [ ] Double-buffering for smooth updates
- [ ] Compute-based fluid dynamics
- [ ] Black hole gravitational pull on GPU
- [ ] Temperature/phase change on GPU

---

*Game Developer - GPU Compute Shader Pipeline Implementation*