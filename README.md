# Tetris CLI

A terminal-based Tetris game written in Rust using Crossterm and Ratatui.

## Features
- Classic Tetris gameplay
- Colorful tetrominoes
- Ghost piece
- 3-piece preview with selection
- Keyboard controls
- Score tracking

## Controls
← → / A D  - Move left/right
↓ / S    - Soft drop
Space    - Hard drop
R / W    - Rotate
G        - Toggle ghost
1 2 3    - Select next piece
Esc      - Exit

## How to Run
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository
3. Build and run:
   ```sh
   cargo run --release
   ```

## Dependencies
- [crossterm](https://crates.io/crates/crossterm)
- [ratatui](https://crates.io/crates/ratatui)
- [rand](https://crates.io/crates/rand)

---
Enjoy playing Tetris in your terminal!
