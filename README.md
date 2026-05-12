# Anchor 🪝

A lightweight, always-on-top Pomodoro timer for Windows. Lives as a floating, translucent window that stays out of your way until you need it.

> **Status:** Work in progress — phases 1–5 implemented (except settings UI)

---

## Features

- **Floating & translucent** — sits on top of all windows at low opacity; barely noticeable while you work
- **Fully interactive** — clicks work anywhere on the window; click-through has been removed for simpler interaction
- **Controls on window** — close button, nav buttons, and play/pause accessible directly on the window
- **Right-click menu** — additional controls accessible in two steps or fewer
- **Two profiles** — Classic (with long break) and No Long Break
- **Visual flash** — window pulses on phase transitions so you notice without being interrupted
- **Audio notifications** — distinct sounds for work end, short break end, and long break end, embedded in the binary
- **Draggable** — drag from anywhere on the window when interaction mode is active
- **Portable** — single `.exe`, no installation, config saved next to the executable

---

## Usage

Right-click the timer to open the context menu:

| Action | Menu item |
|---|---|
| Start / Pause | Start / Pause |
| Restart phase | Restart |
| Skip phase | Skip phase |
| Switch profile | Profile → Classic / No Long Break |
| Open settings | Settings |
| Quit | Close |

The window is mostly transparent and click-through by default. Click the central timer area to enter interaction mode (full opacity, draggable, right-click menu accessible).

---

## Profiles

| Profile | Work | Short break | Long break | Cycles until long break |
|---|---|---|---|---|
| Classic | 25 min | 5 min | 15 min | 4 |
| No Long Break | 25 min | 5 min | — | — |

---

## Window sizes

| Size | Dimensions | Typical use |
|---|---|---|
| S | ~120 × 60 px | Large monitors, corner placement |
| M | ~180 × 90 px | General use (default) |
| L | ~260 × 130 px | Small monitors or reduced vision |

---

## Building

Requires Rust stable and Windows.

```bash
git clone https://github.com/ibelchi/Anchor.git
cd Anchor
cargo build --release
```

The binary will be at `target/release/anchor.exe`. Copy it anywhere and run it — no installation needed.

---

## Stack

| Component | Technology |
|---|---|
| Language | Rust |
| UI | egui / eframe |
| Audio | rodio |
| Window management | winit + Win32 API |
| Config | serde + TOML |

---

## Roadmap

- [x] Phase 1 — Core timer & Pomodoro cycle
- [x] Phase 2 — Interaction (context menu, drag, profiles)
- [x] Phase 3 — Window behaviour (opacity, click-through, sizes)
- [x] Phase 4.1 — Audio notifications
- [x] Phase 4.2 — Visual flash on phase change
- [x] Phase 4.3 — Cycle counter UI
- [/] Phase 5 — Config persistence (Settings UI removed)
- [ ] Phase 6 — Polish, DPI testing, release build

---

## License

MIT
