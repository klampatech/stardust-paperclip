# FUL-6: Phase 3 - Full Material System

## Status: SCOPING COMPLETE ✅ - Handoff to CTO

## Issue Information
- **Issue ID**: FUL-6
- **Blocked by**: FUL-2 (Phase 1), FUL-7 (QA) - both resolved
- **Priority**: high
- **Agent**: CEO (requirements & scoping)

## Context

Phase 1 delivered core physics with 6 materials (Air, Sand, Water, Stone, Fire, Smoke).
Phase 2 delivered test coverage (16 tests passing).

Phase 3 should expand the material system with more sophisticated materials, temperature physics, and state transitions.

## Observed Gaps from Phase 1

Based on SPEC.md and implementation review:

### Missing Materials
- [ ] Steam/Vapor (water → steam with heat)
- [ ] Ice (water → ice with cold)
- [ ] Oil (flammable liquid, different from water)
- [ ] Wood (solid, flammable, slower burn)
- [ ] Lava (hot molten rock)
- [ ] Ash (result of burning wood/coal)

### Missing Physics
- [ ] Temperature-based state changes (ice↔water↔steam)
- [ ] Heat transfer between particles
- [ ] Better fire dynamics (flash points, burn rates)
- [ ] Pressure system for water/hydraulics
- [ ] Chemical reactions (water + lava = steam)

### Missing Features
- [ ] Material spawner UI (palette selection)
- [ ] Brush sizes for painting
- [ ] Material-specific colors
- [ ] Particle lifetime controls

## Scoping Decision Needed

Before I delegate to engineering, I need the board to confirm scope for Phase 3:

### Option A: Conservative (Same sprint)
- Add 2-3 new materials (Steam, Ice, Oil)
- Add basic temperature physics
- Update renderer with material colors

### Option B: Moderate (Same sprint)
- Add 4-5 new materials
- Add temperature-based state changes
- Add heat transfer simulation
- Expand test coverage

### Option C: Ambitious (May need more time)
- Full material system with all materials
- Complete temperature/phase system
- Chemical reactions
- Pressure physics

## Recommendation

**Option B** - balanced scope that extends the core without overrunning timeline.

## Questions for Board

1. Which new materials are priority for Phase 3?
2. Should temperature physics be in Phase 3 or Phase 4?
3. Any budget or timeline constraints?

---

*Generated: 2026-05-12*
*Pending board input before delegating to engineering*