# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run Commands

- **Run**: `cargo run` (or `cargo r`)
- **Build**: `cargo build`
- **Test all**: `cargo test` (or `cargo t`)
- **Test single**: `cargo test <test_name>` (e.g., `cargo test test_board_move_up`)
- **Release build**: `cargo build --release` (optimized with LTO, stripped)

## Architecture

Terminal-based 2048 game built with [Ratatui](https://ratatui.rs) (TUI framework) and Crossterm (terminal backend). Rust edition 2024.

### Module Structure

- **`main.rs`** — App struct and event loop. Handles keyboard input (hjkl/arrows for movement, r to restart, q to quit), dispatches `AppEvent`s, and renders the layout (board on the left, score + controls on the right).
- **`game.rs`** — `Board` struct containing a `[Row; 4]` grid where `Row = [Option<i32>; 4]`. Implements tile movement/merging logic (`move_board`), random cell spawning, game-over detection (`is_board_movable`), and the Ratatui `Widget` trait for rendering. All unit tests live here.
- **`score.rs`** — `Score` widget that renders the current score using Ratatui's `Widget` trait.

### Key Design Details

- Board state uses `state[x][y]` where x=row, y=column (top-left is `[0][0]`).
- `move_board` returns `bool` indicating whether the board changed, which determines if a new random cell spawns.
- Score is the sum of all tile values on the board, recalculated after each move.
- New tiles are 2 (90% chance) or 4 (10% chance).
