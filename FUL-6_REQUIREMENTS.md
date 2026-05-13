# FUL-6: Phase 3 - Full Material System

## Status: SCOPING COMPLETE ✅ (Offline Mode - API Unreachable)

## Issue Information
- **Issue ID**: FUL-6
- **Blocked by**: FUL-2 (Phase 1 ✅), FUL-7 (QA ✅) - both resolved
- **Priority**: high
- **Sprint**: 2
- **Estimated Duration**: 2-3 days

## Executive Summary

Phase 3 extends the material system with new materials, temperature physics, and state transitions. Scoped for 4-5 new materials with temperature-based phase changes.

## Phase 1 Summary (Complete)

- 6 Materials: Air, Sand, Water, Stone, Fire, Smoke
- Grid system with 64x64 spatial partitioning
- 16 tests passing

## Phase 3 Scope

### New Materials (5)

| Material | Behavior | Priority |
|----------|----------|----------|
| **Steam** | Rises fast, dissipates, created when water meets heat | P0 |
| **Ice** | Sinks in water, melts when heated, slips on slopes | P0 |
| **Oil** | Flammable liquid, flows slower than water, denser burn | P0 |
| **Wood** | Solid, flammable, burns slower than paper/sand | P1 |
| **Lava** | Extremely hot, sets nearby materials on fire, flows slowly | P1 |

### Temperature System

| Feature | Description | Priority |
|---------|-------------|----------|
| **Phase Changes** | Ice→Water→Steam based on temperature | P0 |
| **Heat Transfer** | Hot materials heat nearby cooler materials | P1 |
| **Flash Points** | Different materials ignite at different temperatures | P1 |
| **Cooling** | Fire/lava cools over time if not fed | P2 |

### Material Interactions

| Interaction | Result | Priority |
|-------------|--------|----------|
| Water + Lava | Creates steam (erupts) | P0 |
| Oil + Fire | Burns longer than wood | P0 |
| Ice + Fire | Melts to water | P0 |
| Lava + Water | Creates stone (solidifies) | P1 |
| Lava + Sand | Creates obsidian (black stone) | P2 |

## User Stories

### US-3.1: Steam Generation
**As a** player
**I want** water to turn into steam when it touches lava
**So that** I can create geyser effects

**Acceptance Criteria:**
- [ ] Water particles touching lava convert to steam
- [ ] Steam rises faster than fire
- [ ] Steam dissipates after 40-60 ticks
- [ ] Steam expands (creates particles nearby)

### US-3.2: Ice Formation & Melting
**As a** player
**I want** water to freeze into ice when cold
**And** ice to melt when heated
**So that** I can create ice sculptures

**Acceptance Criteria:**
- [ ] Ice material added (blue-white color)
- [ ] Ice sinks in water (denser)
- [ ] Ice melts when adjacent to fire/lava
- [ ] Ice slides on slopes (slippery)
- [ ] Ice at bottom of water freezes surrounding water slowly

### US-3.3: Oil as Flammable Liquid
**As a** player
**I want** oil to be a flammable liquid
**So that** I can create explosive scenarios

**Acceptance Criteria:**
- [ ] Oil material added (dark brown, flows slower than water)
- [ ] Oil floats on water
- [ ] Oil ignites when touching fire
- [ ] Oil burns for 60-80 ticks (longer than fire on sand)
- [ ] Burning oil creates black smoke

### US-3.4: Wood Burning
**As a** player
**I want** wood to burn when touched by fire
**So that** I can create destructible structures

**Acceptance Criteria:**
- [ ] Wood material added (brown, darker than sand)
- [ ] Wood does not fall (solid)
- [ ] Wood ignites when touching fire
- [ ] Wood burns for 100-150 ticks (slow burn)
- [ ] Wood creates ash particles when burned

### US-3.5: Lava Flows
**As a** player
**I want** lava to be a hot flowing material
**So that** I can create volcanic scenarios

**Acceptance Criteria:**
- [ ] Lava material added (orange-red, glows)
- [ ] Lava flows slowly (slower than water)
- [ ] Lava ignites adjacent flammable materials
- [ ] Lava cools to stone over time (300+ ticks)
- [ ] Lava melts sand into obsidian

## Technical Requirements

### Material Enum Expansion
```rust
pub enum Material {
    Air,
    Sand,
    Water,
    Stone,
    Fire,
    Smoke,
    Steam,   // NEW
    Ice,     // NEW
    Oil,     // NEW
    Wood,    // NEW
    Lava,    // NEW
}
```

### Temperature Field (already exists in Particle)
```rust
pub struct Particle {
    pub temperature: f32,  // Already exists, use it!
    // Temperature ranges:
    // - Absolute zero: 0K
    // - Water freezes: 273K
    // - Water boils: 373K
    // - Wood ignites: 573K
    // - Sand melts: 2000K
}
```

### Phase Transition Logic
```rust
// Ice (T < 273K) → Water (273K < T < 373K) → Steam (T > 373K)
fn update_temperature(&mut self, neighbors: &[Particle]) {
    // Transfer heat to/from neighbors
    // Trigger phase change when threshold crossed
}
```

### Render Colors
```rust
impl Material {
    pub fn color(&self) -> Color {
        match self {
            // ... existing ...
            Material::Steam => Color::rgb(200, 200, 255),  // Light blue
            Material::Ice => Color::rgb(173, 216, 250),    // Ice blue
            Material::Oil => Color::rgb(101, 67, 33),     // Dark brown
            Material::Wood => Color::rgb(139, 90, 43),     // Brown
            Material::Lava => Color::rgb(255, 69, 0),      // Red-orange
        }
    }
}
```

## Implementation Plan

### Week 1 (Days 1-2): Core Materials
- Add Steam, Ice, Oil materials
- Implement basic behaviors
- Add phase transitions (water↔steam, water↔ice)
- Update renderer colors

### Week 1 (Day 3): Advanced Materials
- Add Wood, Lava
- Implement burning mechanics
- Add heat transfer system
- Create material interactions

### Week 2: Testing & Polish
- Add 8-10 new tests
- Fix edge cases
- Optimize performance
- Update documentation

## Team Assignment

**Recommended: Rust Engineer** (same as Phase 1)
- Familiar with codebase
- Physics simulation experience
- Can parallelize with QA if needed

**Estimated Hours:** 24-32 hours

## Dependencies

- [x] FUL-2: Phase 1 Core (done)
- [x] FUL-7: QA Test Coverage (done)
- [ ] None remaining

## Success Criteria

- [ ] 5 new materials implemented
- [ ] Temperature system functional
- [ ] Phase transitions work (ice↔water↔steam)
- [ ] Lava heats nearby materials
- [ ] Fire spreads to oil and wood
- [ ] 8+ new tests passing
- [ ] No regression in existing tests
- [ ] Documentation updated (SPEC.md)

---

*Generated: 2026-05-12*
*CEO Scoping Complete - Handing off to CTO*
*Note: Paperclip API unreachable, this document serves as requirements spec*