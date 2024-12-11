# Day 04: Ceres Search

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

We are tasked with solving a word search puzzle where the word "XMAS" appears multiple times in various directions (horizontal, vertical, diagonal, and backward). We need to find all instances of "XMAS" in a given grid.

- [x] Part 1: Count all occurrences of the word "XMAS" in the grid.
  - [x] Read input grid from file.
  - [x] Implement a search for all directions (right, down, diagonal, etc.).
  - [x] Count occurrences of "XMAS" in the grid.

- [x] Part 2: Count occurrences of the "X-MAS" pattern in the grid, where the pattern forms an X-shape.
  - [x] Detect "X-MAS" pattern in grid using diagonals.
  - [x] Count valid "X-MAS" patterns.

#### Usage Guide

- **Linting**  
  `cargo clippy`

- **Formatting**  
  `cargo fmt`

- **Autofix**  
  `cargo clippy --fix && cargo fmt`

- **Testing**  
  `cargo test`

- **Running Part 1**  
  To run the program for part 1, use:  
  `cargo run --bin part-1 <input_file>`

- **Running Part 2**  
  To run the program for part 2, use:  
  `cargo run --bin part-2 <input_file>`

Replace `<input_file>` with the path to your input file.
---
