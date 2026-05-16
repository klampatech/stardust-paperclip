# FUL-48: Title Screen + Kill Sandbox — Pure Game

## Scope Summary
Transform the sandbox editor into a pure game with title screen, no sandbox controls.

## To Remove
- [ ] MaterialPalette component
- [ ] BrushSelector component  
- [ ] ControlBar component (sandbox mode)
- [ ] "Structures" button
- [ ] Mode cycling with 'M' key
- [ ] Attract mode (materials palette behind game UI)
- [ ] Sandbox keyboard shortcuts (1-0 for materials, C for clear with context)
- [ ] Ship class selector (ship is always active)
- [ ] "Strip Sandbox"/"Full Editor" buttons
- [ ] All sandbox appMode states ('full', 'strip-sandbox')

## To Build

### 1. Game State Machine (src/editor/App.tsx)
```typescript
type GameState = 'title' | 'playing' | 'paused' | 'gameOver';
```
- Replace `appMode` with `gameState`
- ESC toggles pause when playing
- Ship spawns automatically on PLAY

### 2. TitleScreen Component (src/editor/components/TitleScreen.tsx)
- "STARDUST" title with tagline "A Space Black Hole Game"
- "PLAY" button → transitions to 'playing'
- "CONTROLS" button → shows controls overlay
- Animated starfield background (reuse existing starfield)

### 3. PauseMenu Component (src/editor/components/PauseMenu.tsx)
- "RESUME" → back to playing
- "RESTART" → restart game
- "CONTROLS" → show controls
- "MAIN MENU" → back to title

### 4. GameOverScreen Component (src/editor/components/GameOverScreen.tsx)
- Final score, survival time, objects consumed
- "PLAY AGAIN" → restart
- "MAIN MENU" → title screen

### 5. Clean HUD (src/editor/components/HUD.tsx)
- Keep: Score, hull health, gravity gun energy, upgrade levels
- Remove: Any sandbox hints or controls
- Show ESC hint in corner

### 6. Controls Overlay
- Show on title screen "CONTROLS" or pause menu "CONTROLS"
- WASD = Move, LMB = Attract, RMB = Repel, MMB = Vortex, ESC = Pause, U = Upgrades

## Files to Modify
1. `src/editor/App.tsx` - Game state machine, remove sandbox mode
2. `src/editor/components/TitleScreen.tsx` - NEW
3. `src/editor/components/PauseMenu.tsx` - NEW
4. `src/editor/components/GameOverScreen.tsx` - NEW
5. `src/editor/components/HUD.tsx` - Clean up, remove sandbox hints
6. `src/editor/App.tsx` - Remove imports for deleted components

## Files to Delete
1. `src/editor/components/MaterialPalette.tsx`
2. `src/editor/components/BrushSelector.tsx`
3. `src/editor/components/ControlBar.tsx`
4. `src/editor/styles/editor.css` - May need partial cleanup

## Verification
1. Game loads to styled title screen (not editor)
2. PLAY starts game with ship, black hole, spawning objects
3. WASD moves ship, LMB/RMB/MMB for gravity gun
4. ESC opens pause menu with correct options
5. Game over shows score + restart/menu options
6. NO sandbox UI appears anywhere
7. `npm run build` passes
