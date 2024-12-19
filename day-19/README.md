# Day 19: Linen Layout

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

In this challenge, you need to arrange towels with specific stripe patterns to match the desired designs. You have unlimited towels of certain patterns, and your goal is to determine which designs are possible (Part 1) and how many ways each design can be created (Part 2).

### Part 1: Determine Possible Designs
- **Objective**: Count the number of designs that can be assembled using the available towel patterns.
- **Approach**:
  - Parse the input to extract towel patterns and designs.
  - Use a recursive function with memoization to check if a design can be formed using the available towel patterns.
  - Count the number of designs that can be successfully assembled.

#### Key Concepts
- **Memoization**: Caching results of recursive calls ensures that overlapping subproblems are not recalculated, optimizing performance for large inputs.

### Part 2: Count All Possible Arrangements
- **Objective**: Calculate the total number of distinct ways each design can be formed.
- **Approach**:
  - Extend the recursive function to count all valid arrangements for each design.
  - Sum up the counts for all designs to get the final result.

#### Key Concepts
- **Dynamic Programming**: The problem of counting arrangements can be seen as a classic dynamic programming task, where each subproblem represents a prefix of the design being matched.

### Fun Notes
- **Pattern Matching in Real Life**: This challenge is reminiscent of problems in **text formatting** or **DNA sequencing**, where patterns need to be matched efficiently.
- **Resource Allocation**: The unlimited supply of towels mirrors scenarios in logistics where certain resources are always available, but careful planning is needed to meet requirements.

---

## Steps Taken

### Part 1
- [x] Parse input to separate towel patterns and designs.
- [x] Implement recursive function with memoization to check design feasibility.
- [x] Count feasible designs and return the result.

### Part 2
- [x] Modify the recursive function to count all arrangements for each design.
- [x] Sum up the counts for all designs.
- [x] Return the total number of arrangements.

---

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
  `cargo run --bin part-1 -- <input_file>`

- **Running Part 2**  
  To run the program for part 2, use:  
  `cargo run --bin part-2 -- <input_file>`

Replace `<input_file>` with the path to your input file.
---