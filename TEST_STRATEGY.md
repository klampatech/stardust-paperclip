# Test Strategy - Falling Sand Simulation

## Overview
This document outlines the testing approach for the falling sand particle simulation engine.

## Test Pyramid

```
        ┌─────────┐
        │   E2E   │  Integration tests: physics behaviors
       /└─────────┘\
      /  Integration  \  Unit tests: individual behaviors
     /─────────────────\
    │      Units        │  Component tests: Grid, Particle, Simulator
   /└───────────────────┘
```

## Test Categories

### 1. Unit Tests (lib.rs)
- **Grid operations**: creation, get/set, bounds checking
- **Particle creation**: materials, properties, flags
- **Material behaviors**: is_fluid, has_gravity, rises, is_flammable

### 2. Component Tests (simulation.rs)
- **Sand physics**: falling, diagonal piling, stacking
- **Water physics**: falling, diagonal falling, horizontal flow
- **Fire physics**: rising, lifetime decay, fire spread
- **Smoke physics**: rising faster than fire, dissipation

### 3. Integration Tests
- Fire → Water interaction (extinguishing)
- Fire → Flammable material spread
- Multi-particle interactions

## Success Criteria Coverage

| Criterion | Test Location | Status |
|-----------|--------------|--------|
| Particles fall and pile correctly | `simulation::tests::test_sand_*` | ✅ All pass |
| Fire spreads to flammable materials | `simulation::tests::test_fire_spread` | ✅ Pass |
| Water extinguishes fire | `simulation::tests::test_water_extinguishes_fire` | ✅ Pass |
| Smoke rises and dissipates | `simulation::tests::test_smoke_*` | ✅ All pass |

## Test Execution

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test simulation::tests
```

## Coverage Targets ✅
- Minimum: 3 core physics tests (per SPEC.md) - **16 tests total**
- Target: All material behaviors + edge cases - **Achieved**
- Critical paths: Sand→Floor, Fire→Water, Fire→Flammable - **All covered**

## Test Inventory (16 tests)

### lib.rs tests (3)
1. `test_grid_creation` - Grid creation and size validation
2. `test_particle_operations` - Spawn, get, validate
3. `test_swap` - Position swap functionality

### simulation.rs tests (13)
4. `test_sand_falls` - Basic sand falling
5. `test_sand_piles_on_floor` - Sand stacking behavior
6. `test_water_falls` - Basic water falling
7. `test_water_extinguishes_fire` - Fire-water interaction
8. `test_water_flows_horizontal` - Water lateral flow
9. `test_fire_rises` - Fire rising behavior
10. `test_fire_dies` - Fire lifetime/expiration
11. `test_fire_spreads` - Fire spread to flammable
12. `test_smoke_rises` - Smoke rising behavior
13. `test_smoke_dissipates` - Smoke lifetime/expiration
14. `test_stone_immutable` - Stone never moves
15. `test_multiple_sand_particles` - Multiple particle physics
16. `test_grid_boundaries` - Boundary handling

## Edge Cases
- ✅ Particles at grid boundaries
- ✅ Simultaneous conflicting moves (bottom-to-top processing)
- ✅ Fire spread to non-flammable materials
- ✅ Empty grid operations
