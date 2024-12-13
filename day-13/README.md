# Day 00: Claw Contraption

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

In this challenge, we are tasked with determining the fewest tokens to win the most prizes from claw machines, each with specific button movements and prize locations.

1. **Understand the configuration of each claw machine:**
   - Each machine has two buttons (A and B) that move the claw along the X and Y axes.
   - The goal is to position the claw exactly over the prize using the least number of presses.

2. **Set up the problem mathematically:**
   - For each machine, calculate the number of presses required for buttons A and B to align the claw's position to the prize's coordinates using the equation system.

3. **Implement the calculation:**
   - Solve the system of linear equations using the determinant method (Cramer's rule).
   - Check for valid solutions where the claw reaches the exact coordinates of the prize.

4. **Optimize token usage:**
   - Ensure that the total token cost (based on the number of presses) is minimized for each machine configuration.

5. **Handle multiple machines:**
   - Parse multiple machines' configurations and compute the cost for each, summing up the total tokens needed for all valid machines.

6. **For Part 2:**
   - Adjust the prize coordinates by adding a large constant to both X and Y coordinates due to an error in measurements.
   - Recalculate the number of presses needed for each adjusted prize and find the minimal cost again.

### Steps to Solve Part 1:
- [x] Parse the input data and extract button configurations and prize locations.
- [x] Calculate the total cost to win as many prizes as possible.
  - [x] For each machine, solve for button presses using the system of equations.
  - [x] Calculate the number of tokens spent for valid solutions.
  - [x] Sum the costs for all valid machines.

### Steps to Solve Part 2:
- [x] Adjust the prize coordinates by adding a large value to both the X and Y coordinates.
- [x] Re-run the calculations for each machine configuration with the updated prize locations.
- [x] Determine the total cost to win as many prizes as possible with the corrected coordinates.

#### Fun Aspects:
- It's like solving a system of equations but with a real-world twist — claw machines!
- Using Cramer's rule to solve for button presses is a neat application of linear algebra, even if the machines are fictional!

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