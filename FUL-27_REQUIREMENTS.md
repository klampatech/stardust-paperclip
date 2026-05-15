# FUL-27: Leverage Stardust Engine - Requirements

## Objective
Determine how the Stardust engine can power a space-based black hole game and create a comprehensive recommendation for game implementation.

## Project Context
The Stardust project is a falling sand particle simulation engine with:
- 13 material types (Sand, Water, Stone, Fire, Smoke, BlackHole, Steam, Ice, Oil, Wood, Lava, Ash)
- Temperature-based physics and phase transitions
- Black hole physics with gravitational pull and event horizon capture
- Post-processing pipeline (bloom, chromatic aberration, space distortion)
- Canvas2D rendering via React editor
- WebAssembly support scaffolded

## Requirements from Issue

### R1: Pixel Rendering Architecture
Determine how the Stardust engine renders pixel graphics for a space game.

**Acceptance Criteria:**
- [ ] Document current Canvas2D rendering pipeline
- [ ] Analyze performance characteristics (60fps target)
- [ ] Identify missing rendering features for space game
- [ ] Recommend optimizations for real-time gameplay

### R2: Web-Native HTML Canvas Architecture
Build the game using HTML Canvas for web-native deployment.

**Acceptance Criteria:**
- [ ] Document React editor architecture (src/editor/)
- [ ] Analyze Vite build setup for web deployment
- [ ] Recommend game-mode vs editor-mode architecture
- [ ] Identify WASM integration opportunities
- [ ] Propose optimal build targets

### R3: Black Hole Effect Improvements
Current implementation lacks dramatic gravitational effects.

**Acceptance Criteria:**
- [ ] Stronger gravitational suction (currently too weak)
- [ ] Spaghettification effect for particles near event horizon
- [ ] Accretion disk visual enhancement
- [ ] Performance considerations for high-particle scenarios

### R4: Space Game Object Models
Design consistent models for rendered pixels as game entities.

**Acceptance Criteria:**
- [ ] Entity hierarchy: asteroids, stars, planets, aliens
- [ ] Map entities to Material variants or new entity types
- [ ] Define collision/boundary behaviors
- [ ] Entity spawning/despawning mechanics
- [ ] Missing physics identification (orbital mechanics)

## Sub-Issues

| ID | Task | Owner | Status |
|----|------|-------|--------|
| FUL-28 | Pixel rendering architecture analysis | Rust Engineer | todo |
| FUL-29 | HTML Canvas web-native architecture | React Engineer | todo |
| FUL-30 | Black hole effect improvements | Rust Engineer | todo |
| FUL-31 | Space game object models | Game Developer | todo |
| FUL-32 | Compile findings for CTO | TBD | todo |

## Success Criteria

### Phase 1: Requirements & Analysis
- [x] Sub-issues created and assigned
- [x] Specialists working on analysis
- [ ] Analysis complete for all 4 analysis tasks
- [ ] FUL-32 synthesis complete

### Phase 2: Game Implementation (Future - FUL-33)
- [ ] CTO approval of recommendation
- [ ] Team assembled for implementation
- [ ] Core space entities implemented
- [ ] Black hole effects enhanced
- [ ] Editor extended with game mechanics

## Timeline Estimate
- Requirements analysis: 1-2 days (current)
- Game implementation: 1-2 weeks (pending CTO approval)

## Risks
1. **Performance**: Particle-heavy scenes may exceed 60fps targets
2. **WASM Integration**: Complex binding between Rust simulation and JS rendering
3. **Black Hole Physics**: Spaghettification may require significant physics changes
4. **Game Design**: Entity model may require fundamental architecture changes

## Dependencies
- FUL-28, FUL-29, FUL-30, FUL-31 must complete before FUL-32
- FUL-32 must complete before CTO handoff

## References
- [FUL-27](/PAP/issues/FUL-27) - Parent issue
- [FUL-3](/PAP/issues/FUL-3) - Original black hole physics implementation
- [FUL-10](/PAP/issues/FUL-10) - GPU compute shader pipeline