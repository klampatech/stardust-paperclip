# Phase 1: Core Falling Sand Prototype - SPEC

## Overview
This is the foundational implementation of a falling sand particle simulation engine with fire and smoke physics, optimized for 50,000+ particles using spatial partitioning.

## Implemented Components

### `lib.rs` - Core Types (310 lines)
- **Material enum**: `Air`, `Sand`, `Water`, `Stone`, `Fire`, `Smoke`
  - `is_fluid()`, `has_gravity()`, `is_flammable()`, `rises()`, `default_temp()`
- **ParticleFlags struct**: `burning`, `remove` flags
- **Particle struct**: 
  - `material`: Material type
  - `velocity`: (f32, f32) for future velocity-based physics
  - `temperature`: f32 for heat simulation
  - `flags`: ParticleFlags
  - `lifetime`: u32 for transient particles (fire, smoke)
- **GridSize struct**: Grid dimensions with bounds checking
- **Grid struct**: Main particle storage with get/set/spawn/swap operations

### `chunk.rs` - Spatial Partitioning (340 lines)
- **ChunkPos**: 2D chunk coordinates
- **ChunkLocalPos**: Position within a chunk
- **Chunk**: 64x64 cell chunk with particle storage
  - Local coordinate access (get_local, set_local)
  - Particle count tracking for empty/full checks
- **ChunkedGrid**: Spatial partitioning system
  - HashMap-based chunk storage
  - Efficient get/set/spawn/swap operations
  - `total_particles()` and `active_chunks()` for statistics
  - `to_flat_grid()` conversion for testing

### `simulation.rs` - Physics Engine (480 lines)
- **Simulator struct**: Processes one physics tick
- Material-specific behavior:
  - **Sand**: Falls down, piles up diagonally when blocked
  - **Water**: Falls, then flows horizontally to fill containers
  - **Stone**: Immovable solid
  - **Fire**: 
    - Rises upward
    - Spreads to adjacent flammable materials (random chance)
    - Has limited lifetime (30-50 ticks)
    - Extinguished by water
  - **Smoke**:
    - Rises upward faster than fire
    - Dissipates over time (60-100 ticks lifetime)
- Dual tick methods:
  - `tick()`: For flat Grid (backwards compatible)
  - `tick_chunked()`: For ChunkedGrid (optimized for large simulations)
- Bottom-to-top processing for falling particles

### `main.rs` - Demo (94 lines)
- Creates a test grid with container walls
- Spawns sand, water, and fire particles
- Runs simulation ticks with ASCII visualization
- Legend: °=Sand, ~=Water, #=Stone, *=Fire, @=Smoke

### `demo.rs` - Large Scale Demo (130 lines)
- Demonstrates ChunkedGrid with 256x200 grid
- Spawns thousands of particles
- Shows spatial partitioning in action

## Physics Rules

### Sand
1. Falls straight down if empty below
2. Falls diagonally if blocked below (random choice if both sides available)
3. Piles naturally into heaps

### Water
1. Falls straight down if empty below
2. Falls diagonally if blocked below
3. Flows horizontally to fill containers
4. Finds lowest point (basic hydraulic behavior)

### Fire
1. Rises upward (negative gravity)
2. Spreads to adjacent flammable cells (8-neighbor)
3. Has limited lifetime
4. Extinguished by contact with water
5. Creates smoke when extinguished

### Smoke
1. Rises upward faster than fire
2. Dissipates over time (lifetime decay)
3. Non-interactive

## Architecture Notes

```
┌─────────────────────────────────────┐
│          ChunkedGrid                │
│  ┌───────────────────────────────┐  │
│  │ HashMap<ChunkPos, Chunk>      │  │
│  │                               │  │
│  │  Chunk[0,0]  Chunk[1,0] ...  │  │
│  │  ┌─────┐   ┌─────┐           │  │
│  │  │64x64│   │64x64│           │  │
│  │  └─────┘   └─────┘           │  │
│  │                               │  │
│  │  Chunk[0,1]  Chunk[1,1] ...  │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
```

## Success Criteria (Phase 1)
- ✅ Particles fall and pile correctly
- ✅ Fire spreads to flammable materials
- ✅ Water extinguishes fire
- ✅ Smoke rises and dissipates
- ✅ Grid system with spatial partitioning (64x64 chunks)
- ✅ Particle data model (type, position, velocity, temperature, flags)
- ✅ Falling sand physics with gravity
- ✅ Basic collision detection (4-neighbor + 8-neighbor for fire)
- Grid size: Supports 256x200+ grids
- Physics tests: 16 test cases (see TEST_STRATEGY.md)

## Phase 2+ Roadmap
- [ ] GPU rendering with wgpu
- [ ] User interaction (click to spawn)
- [ ] wasm-bindgen web scaffold
- [ ] Canvas2D rendering pipeline
- [ ] Chemical reactions (water + fire = steam/smoke)
- [ ] Temperature-based state changes (ice → water → steam)
