# FUL-10 FUL-8b: GPU Compute Shader Pipeline - Requirements

## Issue: FUL-10 FUL-8b
**Type:** Feature Enhancement (GPU Acceleration)
**Priority:** High
**Target:** 60fps with 500,000+ particles

---

## Overview

Implement GPU compute shader acceleration for the falling sand particle simulation using WebGPU (wgpu). This enables parallel processing of particle physics across thousands of GPU threads.

---

## Implementation Status

### ✅ Complete - Scaffold Phase

| Component | Status | Lines |
|-----------|--------|-------|
| `src/gpu/mod.rs` | ✅ | 9 |
| `src/gpu/compute.rs` | ✅ | 370 |
| `src/gpu/shaders.wgsl` | ✅ | 220 |
| `examples/gpu_demo.rs` | ✅ | 215 |
| `examples/gpu_bench.rs` | ✅ | 150 |
| `Cargo.toml` | ✅ | Updated |

---

## GPU Pipeline Architecture

```
┌─────────────────────────────────────────────────┐
│              CPU (Rust)                         │
├─────────────────────────────────────────────────┤
│  Grid → encode_grid() → particle_buffer_in      │
│  particle_buffer_out → decode_grid() → Grid     │
└─────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────┐
│              GPU (WebGPU)                       │
├─────────────────────────────────────────────────┤
│  Bind Group 0: [uniforms, particle_in, out]    │
│  Compute Pass: 256 threads/workgroup            │
│  Dispatch: (cells + 255) / 256 workgroups     │
└─────────────────────────────────────────────────┘
```

---

## Components

### 1. GpuPipeline (`src/gpu/compute.rs`)

```rust
pub struct GpuPipeline {
    device: wgpu::Device,
    queue: wgpu::Queue,
    compute_pipeline: wgpu::ComputePipeline,
    particle_buffer_in: Buffer,   // Ping
    particle_buffer_out: Buffer,  // Pong
    uniform_buffer: Buffer,
    bind_group: BindGroup,        // Ping
    bind_group_alt: BindGroup,    // Pong
    grid_size: (u32, u32),
    tick: u32,
    ping_pong: usize,
}
```

### 2. WGSL Shader (`src/gpu/shaders.wgsl`)

- `@workgroup_size(256)` - 256 threads per workgroup
- Material constants: 0-12 for 13 materials
- Helper functions: `has_gravity()`, `rises()`, `can_move()`
- Gravity physics: sand, water, oil, ice, ash fall down
- Rise physics: fire, smoke, steam rise up
- Water special: horizontal flow when blocked

### 3. Material Encoding

| Bits | Field | Values |
|------|-------|--------|
| 0-4 | Material ID | 0-12 (13 materials) |
| 5-31 | Flags | Reserved |

---

## Features

### ✅ Implemented
- [x] WebGPU initialization with fallback
- [x] Async pipeline creation (`pub async fn new()`)
- [x] Ping-pong double buffering
- [x] Bind groups for shader inputs
- [x] WGSL compute shader with material physics
- [x] CPU fallback when GPU feature disabled

### ⏳ Pending
- [ ] GPU↔CPU buffer sync implementation
- [ ] Render pipeline (vertex/fragment shaders)
- [ ] Performance profiling

### 📋 Future
- [ ] LOD system (distance-based particle merging)
- [ ] GPU frustum culling
- [ ] Frame timing instrumentation

---

## Build Commands

```bash
# CPU-only (default)
cargo build

# GPU-enabled
cargo build --features gpu

# GPU demo
cargo run --example gpu_demo --features gpu

# GPU benchmark (async)
cargo run --example gpu_bench --features gpu --release

# CPU demo
cargo run --example demo

# Tests
cargo test
```

---

## Dependencies

```toml
[features]
gpu = ["wgpu", "bytemuck", "tokio"]

[dependencies]
wgpu = { version = "0.17", optional = true }
bytemuck = { version = "1.14", optional = true }
tokio = { version = "1", optional = true, features = ["rt"] }
```

---

## Performance Targets

| Tier | Particles | FPS | Notes |
|------|-----------|-----|-------|
| Minimum | 100,000 | 30 | Baseline |
| Target | 500,000 | 60 | Goal |
| Stretch | 1,000,000 | LOD | Future |

---

## Handoff Notes

**Completed:**
- GPU module scaffold
- WebGPU compute pipeline
- WGSL shader with material physics
- Ping-pong buffer management
- CPU fallback implementation

**For CTO:**
1. Test `cargo run --example gpu_bench --features gpu` on GPU-enabled system
2. If WebGPU unavailable, work on buffer sync
3. Consider render pipeline for visualization

---

*Game Developer - FUL-10 GPU Pipeline Scaffold Complete*