# FUL-8c: Optimization Plan

**Issue**: FUL-11 / cc15ab62-1b24-4595-8a2d-a052c834e66f
**Status**: In Progress (Optimization Required)
**Date**: 2026-05-13

## Current Performance Gap

| Particles | Build (ms) | Query (ms) | Target Budget | Gap |
|-----------|------------|------------|---------------|-----|
| 1K | 0.30 | 0.31 | 16.67ms | ✓ +16ms |
| 10K | 2.40 | 8.59 | 16.67ms | ✓ +8ms |
| 50K | 11.87 | 164.61 | 16.67ms | ✗ -159ms |
| 100K | 26.30 | 705.79 | 33.33ms | ✗ -699ms |

**Target**: 500K @ 60fps (16.67ms budget)

## Root Cause

Query time scales with neighbor count per particle. The SpatialHash uses `HashMap` with O(1) average insert but O(n) iteration for `get_neighbors()`.

## Optimization Roadmap

### P0: Critical (100K @ 30fps target)

1. **Replace HashMap with Grid-based Spatial Hash**
   - Pre-allocate flat array: `Vec<Vec<(x,y)>>` sized by cell count
   - O(1) cell lookup instead of hash computation
   - Reduces memory allocations per frame

2. **Reduce Cell Size**
   - Change from 16px to 8px cell size
   - More cells (4x), fewer particles per cell
   - Reduces neighbor iteration count

3. **Incremental Hash Updates**
   - Track dirty cells, only update changed regions
   - Avoid full rebuild each tick
   - Use dirty chunk tracking from FUL-8a

### P1: High (500K @ 60fps target)

4. **GPU Offload for Static Particles**
   - Transfer sleeping/stable particles to GPU
   - Skip CPU processing entirely
   - GPU compute shader handles physics

5. **Material Batching (SIMD-friendly)**
   - Group particles by material type
   - Process similar materials together
   - Better cache locality

6. **LOD System**
   - Reduce simulation fidelity at distance
   - Merge particles at low detail levels
   - Only simulate active region at full detail

### P2: Stretch (1M particles)

7. **Multi-threaded Chunk Processing**
   - Parallel physics across chunks
   - Reduce single-thread bottleneck

8. **Spatial Index Hierarchy**
   - Multi-level spatial structure
   - Coarse grid + fine grid
   - Skip irrelevant regions early

## Implementation Progress

### Phase 1: Grid-based Spatial Hash ✓ IMPLEMENTED

```rust
// GridSpatialHash now in src/chunk.rs
pub struct GridSpatialHash {
    cell_size: usize,
    grid_width: usize,
    grid_height: usize,
    cells: Vec<Vec<(usize, usize)>>,  // Pre-allocated flat array
}
```

**Files modified:**
- `src/chunk.rs` - Added `GridSpatialHash` struct with flat array
- `src/lib.rs` - Exported `GridSpatialHash`
- `examples/grid_spatial_benchmark.rs` - NEW: Comparison benchmark

**Tests added:**
- `grid_spatial_hash_tests` (4 tests) - Insert, neighbors, clear, consistency with SpatialHash

### Phase 2: Incremental Updates (Planned)
- Track dirty cells for selective rebuild
- Use existing dirty chunk tracking from ChunkedGrid

### Phase 3: GPU Pipeline (Planned)
- `src/gpu/compute.rs` exists but needs integration
- Wire up buffer sync between CPU and GPU

## Verification

```bash
# Run grid spatial hash benchmark (compares HashMap vs flat array)
cargo run --example grid_spatial_benchmark

# Run spatial integration test
cargo run --example spatial_integration_test
```

## Next Steps

1. [x] Implement grid-based spatial hash (Phase 1) - **DONE**
2. [ ] Add incremental updates (Phase 2)
3. [ ] Verify 100K @ 30fps target with GridSpatialHash
4. [ ] Integrate GPU pipeline (Phase 3)
5. [ ] Verify 500K @ 60fps target

## Resources

- `src/chunk.rs` - SpatialHash and GridSpatialHash
- `src/lib.rs` - Public exports
- `examples/grid_spatial_benchmark.rs` - Comparison benchmark
- `examples/spatial_integration_test.rs` - Integration test

---

**Updated**: 2026-05-13
**Status**: Phase 1 complete, Phase 2-3 pending