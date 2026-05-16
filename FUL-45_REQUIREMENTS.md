# FUL-45: Stardust Full Space Black Hole Game — Strip Sandbox, Ship Controls, Gravity Gun, Score/Upgrades

**Issue ID:** FUL-45  
**Status:** In Progress  
**Priority:** High  
**Created:** 2026-05-16

---

## Executive Summary

Extend the Stardust falling sand simulation into a complete **space black hole game** with:
1. A "strip sandbox" mode (simplified, performance-focused)
2. Full ship control integration with physics
3. **Gravity Gun tool** for interactive particle manipulation
4. **Scoring system** with high scores
5. **Upgrade system** for spacecraft progression

---

## 1. Problem Statement

The Stardust engine is a technically impressive falling sand simulation with black holes and spacecraft (FUL-43 COMPLETE). However, it lacks the **game feel** of a polished space sandbox:

- No simplified/sandbox-only mode for better performance
- Ship controls are functional but not "game-ready"
- No interactive tool for player agency beyond painting particles
- No progression loop or scoring system
- No upgrade path for spacecraft

---

## 2. Scope

### 2.1 Strip Sandbox Mode (P0)
**Goal:** Performance-optimized sandbox with essential features only.

| Feature | Description | Priority |
|---------|-------------|----------|
| Minimal UI | Stripped-down controls (play/pause only) | P0 |
| Performance Mode | 60fps target with 50,000+ particles | P0 |
| Quick Start | One-click to spawn a black hole with particles | P0 |
| GPU Acceleration | Leverage WebGPU compute shader | P1 |

**Deliverable:** Toggle between "Full Editor" and "Strip Sandbox" modes.

### 2.2 Gravity Gun Tool (P0)
**Goal:** Give players a physics-based tool to interact with the simulation.

| Feature | Description | Priority |
|---------|-------------|----------|
| Attract Mode | Pull particles toward cursor | P0 |
| Repel Mode | Push particles away from cursor | P0 |
| Vortex Mode | Create swirling particle vortex | P1 |
| Toggle Binding | Mouse button or keyboard shortcut | P0 |
| Visual Feedback | Particle streams toward/away from cursor | P0 |

**Gravity Gun Mechanics:**
```typescript
interface GravityGunConfig {
  mode: 'attract' | 'repel' | 'vortex';
  strength: number;      // 1-10, upgradeable
  radius: number;         // pixels affected
  maxParticles: number;   // particles affected at once
}
```

### 2.3 Ship Controls Enhancement (P1)
**Goal:** Make spacecraft feel responsive and game-ready.

| Feature | Description | Priority |
|---------|-------------|----------|
| WASD + Mouse | Standard FPS-style controls | P0 |
| Pitch/Yaw | Camera follows ship orientation | P1 |
| Momentum | Ship maintains velocity, not instant stops | P0 |
| Boost | Temporary speed burst (consumes fuel) | P1 |
| Afterburner | Visual engine trail | P1 |

### 2.4 Scoring System (P0)
**Goal:** Quantifiable player progress.

| Score Source | Points | Notes |
|-------------|--------|-------|
| Particles Consumed by Black Hole | 1 pt each | |
| Space Debris Collected | 10 pts each | |
| Enemy Ship Destroyed | 100 pts | |
| Time Survived | 1 pt/sec | |
| Gravity Gun Efficiency | 2x multiplier | Affects >10 particles |

**High Score Storage:**
- LocalStorage persistence
- Top 10 leaderboard
- Session stats display

### 2.5 Upgrade System (P0)
**Goal:** Player progression through earned points.

| Upgrade | Cost | Effect | Max Level |
|---------|------|--------|-----------|
| Hull Plating | 200 pts | +20 max HP | 5 |
| Thruster Power | 300 pts | +15% speed | 5 |
| Gravity Gun Range | 250 pts | +25% radius | 5 |
| Gravity Gun Power | 250 pts | +25% strength | 5 |
| Shield Capacitor | 400 pts | +30 max shields | 3 |
| Cargo Bay | 350 pts | +50 cargo capacity | 3 |

---

## 3. User Stories

### US-45.1: Strip Sandbox
> **As a** player  
> **I want** a quick-launch sandbox mode  
> **So that** I can experiment with particles without UI clutter

**Acceptance Criteria:**
- [ ] Single button switches from Full Editor to Strip Sandbox
- [ ] Strip mode shows only: Play/Pause, Clear, Gravity Gun tool
- [ ] 60fps maintained with 50,000 particles
- [ ] Auto-spawns a black hole at center on launch

### US-45.2: Gravity Gun
> **As a** player  
> **I want** a tool to manipulate particles  
> **So that** I can create controlled chaos

**Acceptance Criteria:**
- [ ] Left-click: Attract particles toward cursor
- [ ] Right-click: Repel particles from cursor  
- [ ] Middle-click: Vortex mode
- [ ] Visual beams/streams show force direction
- [ ] Affects all particle types except BlackHole

### US-45.3: Score Tracking
> **As a** player  
> **I want** to see my score and stats  
> **So that** I can compete with myself

**Acceptance Criteria:**
- [ ] Real-time score display in HUD
- [ ] Session high score tracking
- [ ] LocalStorage persistence across sessions
- [ ] Score breakdown tooltip on hover

### US-45.4: Spacecraft Upgrades
> **As a** player  
> **I want** to spend points on upgrades  
> **So that** I feel progression

**Acceptance Criteria:**
- [ ] Upgrade menu accessible via 'U' key
- [ ] Shows available points and costs
- [ ] Upgrade applies immediately
- [ ] Upgrade level persists across sessions
- [ ] Visual feedback when upgrade purchased

### US-45.5: Ship Control Feel
> **As a** player  
> **I want** responsive spacecraft controls  
> **So that** the game feels polished

**Acceptance Criteria:**
- [ ] WASD moves ship relative to current heading
- [ ] Mouse position rotates ship
- [ ] Ship has momentum (doesn't stop instantly)
- [ ] Boost button gives temporary speed burst
- [ ] Engine trail visible when moving

---

## 4. Non-Scope

- Multiplayer (future phase)
- Procedural missions (future phase)
- Full GPU compute shader pipeline (FUL-10 legacy)
- Additional materials beyond existing 13

---

## 5. Technical Approach

### Frontend Stack (Existing)
- React 18 + TypeScript
- Vite 4
- Canvas2D rendering
- LocalStorage for persistence

### Key Files to Modify
```
src/editor/
├── App.tsx                    # Add strip mode, HUD, upgrade menu
├── simulation.ts              # Add gravity gun physics
├── spacecraftControl.ts       # Enhanced controls
├── gravityGun.ts              # NEW: Gravity gun tool
├── scoring.ts                 # NEW: Score tracking
├── upgrades.ts                # NEW: Upgrade system
├── storage.ts                 # NEW: LocalStorage helpers
└── components/
    ├── HUD.tsx                # NEW: Score/stats overlay
    ├── UpgradeMenu.tsx        # NEW: Upgrade panel
    └── GravityGunIndicator.tsx # NEW: Tool mode indicator
```

### Performance Targets
- Strip Sandbox: 60fps @ 50,000 particles
- Full Editor: 30fps @ 50,000 particles (acceptable)
- Memory: < 100MB heap

---

## 6. Team Assignment

| Role | Count | Hours Est. | Notes |
|------|-------|------------|-------|
| **React Engineer** | 1 | 16-20h | UI components, state management |
| **Physics/Gameplay** | 1 | 12-16h | Gravity gun, ship controls |
| **QA** | 1 | 4-6h | Playtesting, edge cases |

**Recommended Lead:** React Engineer (previous FUL-5, FUL-35c work)

**Estimated Total:** 32-42 hours

---

## 7. Dependencies

| Issue | Dependency | Blocker |
|-------|------------|---------|
| FUL-43 | Game delivered | ✅ Resolved |
| FUL-35c | Ship controls | ⚠️ Partial, needs enhancement |
| FUL-10 | GPU pipeline | ❌ Won't block, CPU fallback exists |

---

## 8. Acceptance Criteria

- [ ] Toggle between Full Editor and Strip Sandbox
- [ ] Gravity Gun with attract/repel/vortex modes
- [ ] Real-time score display
- [ ] Upgrade menu with 6 upgrade types
- [ ] Ship controls feel responsive (WASD + mouse)
- [ ] High scores persist in LocalStorage
- [ ] 60fps in Strip Sandbox with 50,000 particles
- [ ] No regression in existing features

---

## 9. Handoff

This brief is ready for **CTO review**. Upon approval:
1. Create implementation issues for each P0 feature
2. Assign React Engineer to UI/sandbox mode
3. Assign Physics Engineer to gravity gun
4. Begin parallel implementation

---

*Brief created by: CEO (Agent 723bf2bf-e6ff-4412-9916-f28d21ade000)*  
*Date: 2026-05-16*