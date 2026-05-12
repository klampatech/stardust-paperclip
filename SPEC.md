# Phase 1: Core Falling Sand Prototype - SPEC

## Overview
This is the foundational implementation of a falling sand particle simulation engine with fire, smoke, and black hole physics, optimized for 50,000+ particles using spatial partitioning.

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

### Black Holes (Phase 2)
1. **Gravitational Pull**: Particles within influence radius are attracted toward black holes
2. **Inverse-square law**: Force = G × mass / distance²
3. **Event Horizon**: Particles crossing the event horizon are removed
4. **Hawking Radiation**: Small particles (Fire/Smoke) are emitted periodically
5. **Mass Growth**: Black hole mass increases slightly when consuming particles (optional)

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
- [x] Canvas2D rendering pipeline
- [x] Black hole physics (FUL-3)
  - [x] `BlackHole` material variant with properties
  - [x] Gravitational pull (inverse-square law, F = G/r²)
  - [x] Event horizon capture (particles consumed when dist < event_horizon_radius)
  - [x] Hawking radiation emission (Fire/Smoke particles)
  - [x] Tidal forces and spaghettification near event horizon
  - [x] Camera shake trigger on consumption
  - [x] Velocity-based movement for gravity effects
  - [x] Particle mass affecting gravity response
- [ ] GPU rendering with wgpu
- [ ] User interaction (click to spawn)
- [ ] wasm-bindgen web scaffold
- [ ] Chemical reactions (water + fire = steam/smoke)
- [ ] Temperature-based state changes (ice → water → steam)
- [ ] Accretion disk physics (stretch goal)

---

# Phase 3: Full Material System - SPEC

## Overview
Phase 3 extends the material system with 5 new materials and temperature-based physics.

## New Materials (5)

| Material | Behavior | Priority |
|----------|----------|----------|
| **Steam** | Rises fast, dissipates, created when water meets heat | P0 |
| **Ice** | Sinks in water, melts when heated, slips on slopes | P0 |
| **Oil** | Flammable liquid, flows slower than water, denser burn | P0 |
| **Wood** | Solid, flammable, burns slower than paper/sand | P1 |
| **Lava** | Extremely hot, sets nearby materials on fire, flows slowly | P1 |

## Temperature System

| Feature | Description | Priority |
|---------|-------------|----------|
| **Phase Changes** | Ice→Water→Steam based on temperature | P0 |
| **Heat Transfer** | Hot materials heat nearby cooler materials | P1 |
| **Flash Points** | Different materials ignite at different temperatures | P1 |

## Material Interactions

| Interaction | Result | Priority |
|-------------|--------|----------|
| Water + Lava | Creates steam (erupts) | P0 |
| Oil + Fire | Burns longer than wood | P0 |
| Ice + Fire | Melts to water | P0 |
| Lava + Water | Creates stone (solidifies) | P1 |

## Expanded Material Enum

```rust
pub enum Material {
    Air,
    Sand,
    Water,
    Stone,
    Fire,
    Smoke,
    Steam,   // NEW - rises fast, dissipates
    Ice,     // NEW - sinks, melts when heated
    Oil,     // NEW - flammable liquid, slow flow
    Wood,    // NEW - solid, slow burn
    Lava,    // NEW - hot, flows slowly, ignites nearby
}
```

## Temperature Ranges (Kelvin)

- Water freezes: 273K
- Water boils: 373K
- Wood ignites: 573K
- Sand melts: 2000K

## Success Criteria (Phase 3)

- [x] 5 new materials implemented (Steam, Ice, Oil, Wood, Lava)
- [x] Temperature system functional
- [x] Phase transitions work (ice↔water↔steam)
- [x] Lava heats nearby materials
- [x] Fire spreads to oil and wood
- [x] 10+ new tests added
- [x] No regression in existing tests (16 existing)

**Total Materials:** 12 (Air, Sand, Water, Stone, Fire, Smoke, BlackHole, Steam, Ice, Oil, Wood, Lava)

## Team Assignment

**Recommended: Rust Engineer** (same as Phase 1)

**Estimated Duration:** 2-3 days (24-32 hours)
