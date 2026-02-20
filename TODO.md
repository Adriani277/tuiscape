# Tuiscape — RuneScape-like TUI Game

A terminal-based RPG inspired by RuneScape, built in Rust. The project is split into
two clear layers so the UI backend can be swapped without touching game logic.

---

## Architecture

```
src/
  core/        # Pure game logic — no UI dependencies
  tui/         # Ratatui-based terminal UI — no game logic
  main.rs      # Wires TUI to core, handles the event loop
```

### `core` — Game Engine

All gameplay mechanics live here. No ratatui, no crossterm, no I/O.
The core exposes a clean API that any frontend can drive.

- **Skills** — each skill has a type, level (1–99), and XP total
  - XP-to-level curve matching RuneScape (or a simplified version)
  - Level-up events emitted so the UI can react
- **Actions / Methods** — things the player can do to train a skill
  - Each action specifies XP rate, requirements (level gate), and a tick duration
  - Examples: Fishing → fly fishing, lobster pots; Woodcutting → normal logs, oak, willow
- **Game loop tick** — core advances by discrete ticks; UI calls `tick()` and gets back events
- **Inventory / Resources** — items collected while training (fish, logs, ores, bars …)
- **Player** — owns a collection of `Skill`s and an `Inventory`
- **Persistence** — save/load player state (JSON or binary) so progress survives restarts

### `tui` — Terminal UI (ratatui)

Everything visual lives here. Receives game state snapshots; sends input actions back to core.
Swapping this layer for a different UI (e.g. a web frontend, an egui desktop app) should not
require touching `core`.

- **Layout** — RuneScape-inspired panels:
  - Skills panel (grid of skill icons + level numbers)
  - Action/method panel (list of trainable methods for the selected skill)
  - Inventory panel (items collected)
  - Activity log / chat (XP drops, level-ups, messages)
  - Status bar (current action, ticks remaining, XP/hr)
- **Input** — vim-style navigation (hjkl / arrows), Enter to select, Esc/q to back out
- **Widgets** — reusable ratatui widgets:
  - `SkillWidget` — skill name + level + XP progress bar
  - `ActionWidget` — method list with requirement indicators
  - `InventoryWidget` — item grid
  - `XpDropWidget` — animated XP gain overlay
- **Focus / screen state** — tracks which panel is active; modal dialogs for level-ups

---

## TODO

### Foundation
- [ ] Restructure `src/` into `core/` and `tui/` modules
- [ ] Move `Skill` and `SkillType` into `core`
- [ ] Define `Player` struct in `core` (owns all skills + inventory)
- [ ] Define `Action` / `Method` trait in `core`
- [ ] Implement XP curve (RuneScape table or formula)
- [ ] Implement game tick (`core::tick(action) -> Vec<GameEvent>`)

### Skills & Actions
- [ ] Woodcutting — logs (1), oak (15), willow (30), maple (45), yew (60), magic (75)
- [ ] Fishing — shrimps (1), trout (20), salmon (30), lobster (40), swordfish (50)
- [ ] Firemaking — uses logs from inventory; XP scales with log type
- [ ] Cooking — cook raw fish/meat; burn chance decreases with level

### TUI Layer
- [ ] Replace monolithic `App` render impl with dedicated `tui` module
- [ ] Skills panel — grid layout matching RuneScape's 3-column skill grid
- [ ] Action/method panel — shown when a skill is selected
- [ ] Inventory panel — 28-slot grid (RuneScape standard)
- [ ] Activity log — scrollable list of recent XP drops and events
- [ ] Level-up modal — flash message when a level is gained

### Quality of Life
- [ ] Save/load player progress to `~/.local/share/tuiscape/save.json`
- [ ] Config file for keybindings
- [ ] Colour theme matching RuneScape's brown/gold palette
