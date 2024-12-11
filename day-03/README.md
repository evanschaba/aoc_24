# Day 03: Mull It Over

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

The challenge involves parsing a corrupted memory sequence to sum the results of valid multiplication (`mul`) operations. The sequence contains malformed instructions that must be ignored, while valid instructions need to be parsed and summed.

- **Part 1**: Parse valid `mul(x, y)` instructions, ignore invalid characters, and sum the multiplication results.
- **Part 2**: Introduce new commands (`do()` and `don't()`) to enable/disable `mul` operations. Only enabled multiplications should contribute to the sum.

### Steps Taken to Solve the Challenge

- [x] **Part 1**: 
  - Used a regular expression (`mul(x, y)`) to find valid multiplication instructions.
  - Ignored all invalid characters and instructions.
  - Parsed the valid instructions, performed the multiplications, and summed the results.

- [x] **Part 2**: 
  - Implemented logic to handle `do()` and `don't()` instructions that enable/disable future multiplications.
  - Updated the regular expression to also match `do()` and `don't()` instructions.
  - Ensured that only enabled `mul` operations were considered in the final sum.

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