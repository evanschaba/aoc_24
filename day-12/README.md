# Day 12: Garden Groups

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

The problem involves calculating the cost of fencing various garden regions defined by a grid map:

- Part 1: Use the **area** and **perimeter** of regions to calculate fencing costs.
- Part 2: Use the **area** and **number of sides** instead for fencing costs.

### Part 1
- [x] Parse the grid input into a structured map.
- [x] Identify connected regions of the same plant type.
  - [x] Use BFS to group connected cells.
- [x] Calculate region area and perimeter.
- [x] Compute the total fencing cost.

### Part 2
- [x] Adapt the calculation to use region area and the number of sides.
- [x] Ensure proper counting of all boundary sides, including internal boundaries.
- [x] Compute the updated total fencing cost.

### Fun Aspects
- **Spatial Thinking**: Mapping connected regions encourages thinking about grids and connectivity in 2D space.
- **Real-World Relevance**: The challenge mirrors real-world land management problems where calculating accurate costs for irregular shapes is essential.

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