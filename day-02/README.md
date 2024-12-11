# Day 02: Red-Nosed Reactor Reports

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

The task involves analyzing reports with levels that follow certain patterns of increase or decrease. The levels need to be either all increasing or all decreasing with differences between adjacent levels in the range [-3, -1] or [1, 3].

- [x] **Part 1: Identify Safe Reports**
  - [x] Parse input to extract reports.
  - [x] Implement logic to check if the report's levels are all increasing or decreasing within allowed differences.
  - [x] Count how many reports are safe.

- [x] **Part 2: Handle the Problem Dampener**
  - [x] Modify the logic to allow for a single "bad" level that can be removed from the report.
  - [x] Count how many reports are now safe after considering the dampener.

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