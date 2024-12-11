# Day 10: 

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

The solution for Day 10 involves finding and analyzing hiking trails on a topographic map based on specific rules. 

The project focuses on two distinct computations for the trailheads:

- Trailhead Scores: Calculating the sum of scores for all trailheads, where a trailhead's score is defined as the number of 9s reachable from that trailhead by valid hiking trails.

- Trailhead Ratings: Determining the sum of ratings for all trailheads, where a trailhead's rating is the count of distinct hiking trails starting from that trailhead.


Input Parsing:
- [x] Convert the topographic map into a 2D grid of integers for processing.
- [x] Identify trailheads as positions marked with height 0.

Trail Traversal:

- [x] For each trailhead, use depth-first search (DFS) to explore valid hiking trails.
- [x] Ensure that trails follow the constraints:
    - [x] Height increases by exactly 1 at each step.
    - [x] Movement is restricted to adjacent cells (no diagonals).
- [x] Part 1 (Trailhead Scores):
    - [x] Use DFS to find all positions with height 9 reachable from each trailhead.
    - [x] Aggregate the scores across all trailheads.

- [x] Part 2 (Trailhead Ratings):
    - [x] Count the distinct hiking trails starting from each trailhead.
    - [x] Use DFS to track each unique trail path and sum the ratings across all trailheads.

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