# Keyguard Trainer

A terminal typing trainer. Words fall from the top of the screen. Type them before they hit the bottom. Like Typing of the Dead meets Space Invaders.

## Why
All typing trainers are boring web apps. This one runs in your terminal and feels like a game.

## Tech
- Rust + ratatui (terminal UI)
- crossterm for keyboard input
- Real-time WPM tracking
- Difficulty progression (speed increases)

## Features (MVP)
- Words fall from top, speed increases with WPM
- Correct typing destroys the word (explosion animation)
- Missed words stack up as "damage"
- Score tracking: WPM, accuracy, streak
- 3 difficulty levels

## Dev
```bash
cargo run
```

## Build
```bash
cargo build --release
```
