# Day 11: Plutonian Pebbles

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

The solution revolves around simulating the evolution of stones based on specific transformation rules applied over multiple iterations ("blinks"). To efficiently handle the potentially exponential growth in the number of stones, memoization is employed to store and reuse results for previously computed states. The program is structured to:

- [x] Parse the input, which is a list of numbers representing the initial arrangement of stones.
- [x] Use recursive logic to evolve each stone based on the rules:
- [x] Stones marked 0 transform to 1.
- [x] Stones with even-digit numbers split into two stones.
- [x] Other stones are multiplied by 2024.
- [x] Aggregate the total number of stones after a given number of blinks.
- [x] Include separate computation logic for 25 and 75 blinks as required for parts 1 and 2 of the problem.
- [x] The implementation is optimized for clarity and efficiency, leveraging memoization to minimize redundant computations.

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