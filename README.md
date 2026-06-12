# Keyguard Trainer

[![Rust](https://img.shields.io/badge/Rust-2021-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-purple)](./LICENSE)

⌨️ **A terminal typing trainer that feels like a game.** Words fall from the top of the screen. Type them before they hit the bottom. Like Typing of the Dead meets Space Invaders.

## 🎮 Why

All typing trainers are boring web apps. This one runs in your terminal and feels like a game. Challenge yourself to type faster while defending against falling words!

## ✨ Features

- **Falling words** — Words descend from the top; type them before they reach the bottom
- **Dynamic difficulty** — Speed increases as you type faster (WPM-based progression)
- **Explosion animations** — Correctly typed words explode satisfyingly
- **Damage system** — Missed words stack up as "damage" — game over at critical damage
- **Score tracking** — Real-time WPM, accuracy, and streak counters
- **3 difficulty levels** — Easy, Medium, Hard
- **Cross-platform** — Works on macOS, Linux, Windows (any terminal with UTF-8)

## 🚀 Quick Start

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and run
git clone https://github.com/zhang-forever/keyguard-trainer.git
cd keyguard-trainer
cargo run
```

## 🎯 How to Play

1. Launch the game in your terminal
2. Words fall from the top of the screen
3. Type the word before it hits the bottom
4. Correct typing = explosion + points
5. Missed words = damage
6. Game ends when damage reaches critical level

## 🛠️ Tech Stack

- **Language:** Rust 2021
- **TUI:** ratatui (terminal UI framework)
- **Input:** crossterm (cross-platform terminal handling)
- **Rendering:** Real-time terminal rendering at 60fps

## 📦 Build

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run optimized version
./target/release/keyguard-trainer
```

## 🎮 Controls

| Key | Action |
|-----|--------|
| Type letters | Destroy falling words |
| `Esc` | Quit game |
| `R` | Restart after game over |

## 📊 Difficulty Levels

| Level | Starting Speed | Max Speed | Damage Multiplier |
|-------|----------------|-----------|-------------------|
| Easy | Slow | Medium | 1x |
| Medium | Medium | Fast | 1.5x |
| Hard | Fast | Very Fast | 2x |

## 📄 License

MIT
