# Day 5: Print Queue

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

The problem revolves around ensuring that pages for a safety manual are printed in the correct order based on certain rules. For each set of updates, we need to validate if the order of pages follows the rules. If it does, we calculate the middle page number of the valid updates.

### Part 1
- Read and parse the input to get the page ordering rules and updates.
- Validate each update to check if the page numbers are in the correct order according to the rules.
- Calculate the sum of middle page numbers for valid updates.

### Part 2
- For updates that are not in the correct order, reorder the pages according to the rules.
- Calculate the middle page number for each corrected update and sum them.

- [x] Parse input
- [x] Implement validation for update order
- [x] Identify correct updates (Part 1)
- [x] Sort incorrectly ordered updates (Part 2)
- [x] Calculate sum of middle numbers for valid and corrected updates

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
