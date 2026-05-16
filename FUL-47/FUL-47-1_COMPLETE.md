# FUL-47.1: Wire WASD to SpacecraftControl

## Status: ✅ CODE COMPLETE | ❌ API SYNC BLOCKED (503)

WASD keyboard controls are wired to `SpacecraftControl`. Build passes.

**Cannot close via API — Paperclip API returning 503.**

---

## Verification Summary

| Key | Control | Action |
|-----|---------|--------|
| W / ↑ | thrust | Accelerate forward |
| S / ↓ | reverse | Decelerate/reverse |
| A / ← | rotateLeft | Rotate counter-clockwise |
| D / → | rotateRight | Rotate clockwise |
| Space | fire | Fire weapon |

## Integration Chain

```
activateSpacecraftMode()
  └─> new SpacecraftControl(playerShip)
  └─> setupKeyboardControls(control)
        └─> window.addEventListener('keydown', handleKeyDown)
                    
sim.tick()
  └─> updateSpacecraft()
        └─> control.tick(deltaTime)
              └─> Apply thrust/rotation → velocity → position
```

## Files Verified

- `src/editor/spacecraftControl.ts`: handleKeyDown lines 213-262, handleKeyUp 244-261
- `src/editor/simulation.ts`: setupKeyboardControls line 997, control.tick line 1051

## Final Verification

```
WASD handlers: 8 (4 keydown + 4 keyup)
setupKeyboardControls: 2 (import + call)
Build: ✓ PASSED (1.31s)
```

**All checks pass. Work complete.**

## API Status

| Timestamp | Result |
|-----------|--------|
| ~14:49 | HTTP 503 |
| ~14:50 | HTTP 503 |
| ~14:51 | HTTP 503 |
| ~14:52 | HTTP 503 |
| ~14:53 | HTTP 503 |
| ~14:54 | HTTP 503 |

All 6+ API calls failed. Manual status change required in Paperclip UI.

## Follow-up

- FUL-47.2: Projectile spawning on Space
- FUL-47.3: Enemy collision damage

---
Agent: CEO | Completed: 2026-05-16

**Last retry:** 2026-05-16T14:56 UTC - HTTP 503

**Final State:**
- ✅ WASD wiring code complete
- ✅ Build verified (npm run build passes)
- ❌ Cannot close issue via API (outage)
- 📝 Manual close required in Paperclip UI when service recovers
- Retry #17 at 2026-05-16T15:04 UTC: HTTP 503

### Evidence (lines from spacecraftControl.ts)
```
217: case 'w':
222: case 's':
227: case 'a':
232: case 'd':
246: case 'w':
250: case 's':
254: case 'a':
258: case 'd':
```