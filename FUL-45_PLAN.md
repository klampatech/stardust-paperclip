# FUL-45 Implementation Plan

## Overview
Extend Stardust falling sand simulation into a complete space black hole game with: Strip Sandbox mode, Gravity Gun tool, enhanced Ship Controls, Scoring system, and Upgrade system.

## Scope Summary

| Feature | Priority | Hours Est. |
|---------|---------|------------|
| Strip Sandbox Mode | P0 | 6-8h |
| Gravity Gun Tool | P0 | 8-10h |
| Scoring System | P0 | 4-6h |
| Upgrade System | P0 | 6-8h |
| Ship Controls Enhancement | P1 | 4-6h |
| High Score Persistence | P0 | 2h |

**Total Estimated:** 30-38 hours

## Implementation Tasks

### Phase 1: Strip Sandbox Mode
- [ ] Add mode toggle state in App.tsx
- [ ] Create simplified StripSandbox component
- [ ] Implement quick-start auto-spawn black hole
- [ ] Add GPU acceleration path (use WebGPU when available)

### Phase 2: Gravity Gun Tool
- [ ] Create `src/editor/gravityGun.ts`
- [ ] Implement attract/repel/vortex physics
- [ ] Add mouse button handlers (left/right/middle click)
- [ ] Add visual beam/stream rendering
- [ ] Integrate into simulation tick loop

### Phase 3: Scoring System
- [ ] Create `src/editor/scoring.ts`
- [ ] Track particles consumed, enemies destroyed, time survived
- [ ] Create HUD component for real-time score display
- [ ] Implement LocalStorage persistence
- [ ] Create high score leaderboard

### Phase 4: Upgrade System
- [ ] Create `src/editor/upgrades.ts`
- [ ] Define upgrade definitions (6 types, 5 levels each)
- [ ] Create UpgradeMenu component
- [ ] Integrate upgrade effects into ship stats
- [ ] Persist upgrades in LocalStorage

### Phase 5: Ship Controls Enhancement
- [ ] Refine WASD + mouse control scheme
- [ ] Add momentum/acceleration physics
- [ ] Implement boost with fuel consumption
- [ ] Add visual engine trail

## File Changes

```
src/editor/
├── App.tsx                    # Add mode toggle, HUD, upgrade menu
├── simulation.ts              # Add gravity gun physics
├── gravityGun.ts              # NEW
├── scoring.ts                 # NEW
├── upgrades.ts                # NEW
├── storage.ts                 # NEW
└── components/
    ├── HUD.tsx                # NEW
    ├── UpgradeMenu.tsx        # NEW
    └── GravityGunIndicator.tsx # NEW
```

## Dependencies
- FUL-43: Game delivered ✅
- FUL-35c: Ship controls (partial) ⚠️

## Risk Assessment
- **Medium Risk**: Gravity Gun physics may impact performance at 50k+ particles
- **Mitigation**: Limit affected particles per tick, use spatial optimization

## Next Steps
1. Confirm this plan
2. CTO creates implementation subtasks
3. Parallel development by React + Physics engineers
4. QA playtesting and edge case validation