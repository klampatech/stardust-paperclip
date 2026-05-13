# FUL-8a: ISSUE CLOSED

**Issue ID:** FUL-9 / 06bff5a7-afc3-47da-9ec8-6c0f016efdb9
**Status:** COMPLETE (locally) - Paperclip API unreachable
**Closed:** 2026-05-13T03:21:13Z

## Issue Status: DONE ✅

All 4 deliverables for FUL-8a Spatial Hashing & Dirty-Chunk Tracking complete:

1. ✅ **Spatial hash with configurable cell size**
   - `src/chunk.rs` - `SpatialHash` struct with O(1) insert/lookup
   - Configurable cell size (default 16px)
   - 8-directional neighbor queries
   - 3 tests: `test_spatial_hash_insert`, `test_spatial_hash_neighbors`, `test_spatial_hash_clear`

2. ✅ **Dirty-chunk tracking with incremental invalidation**
   - `ChunkedGrid::dirty_chunks` HashSet
   - `set()`, `spawn()`, `swap()` mark chunks dirty
   - `mark_dirty()`, `clear_dirty()` for control
   - 2 tests: `test_dirty_chunk_tracking`, `test_dirty_adjacent_chunks`

3. ✅ **Sleeping particle list with wake-on-neighbor-change**
   - `sleeping_particles` / `woken_particles` HashSets
   - `mark_sleeping()`, `wake_particle()`, `is_sleeping()`
   - `wake_neighbor_chunks()` - wakes adjacent to dirty
   - 3 tests: `test_sleeping_particles`, `test_wake_neighbor_chunks`, etc.

4. ✅ **Benchmark harness for spatial queries**
   - `src/benchmark.rs` - `SpatialBenchmark`, `SpatialBenchmarkConfig`
   - 68 tests total
   - `examples/spatial_integration_test.rs`

## Integration Evidence

- `tick_chunked()` builds SpatialHash each tick (simulation.rs:95-99)
- Fire/Lava use `get_neighbors()` for O(1) lookup (simulation.rs:189, 209)
- Sleeping particles skip physics (simulation.rs:124)
- `wake_neighbor_chunks()` called each tick (simulation.rs:133)

## Files

- `src/chunk.rs` - 707 lines
- `src/benchmark.rs` - 242 lines (NEW)
- `src/simulation.rs` - +80 lines (integration)
- `examples/ful_8a_demo.rs` - 166 lines (NEW)
- `examples/spatial_integration_test.rs` - 166 lines (NEW)

## Note

Paperclip API at http://100.83.52.32:3100 unreachable (connection timeout).
Issue status could not be updated via API.
This document serves as local confirmation of issue completion.
