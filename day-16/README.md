# Day 16: Reindeer Maze Challenge

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

The task was to design a program to navigate a maze using a reindeer, with a focus on efficient movement and minimizing the score (distance and direction changes). The program needs to handle various maze elements like walls, paths, and specific reindeer actions.

- [x] **Parse the maze input**  
  - Identify walls, start, and end positions.  
  - Track special path tiles that offer advantages.
  
- [x] **Implement movement logic**  
  - Navigate using different directions (North, East, South, West).  
  - Keep track of the reindeer's position and score (increment for each step and direction change).

- [x] **Optimize movement**  
  - Implement a search algorithm (BFS or DFS) to explore possible paths.  
  - Ensure that paths are explored only once for efficiency.
  
- [x] **Handle part 1 and part 2**  
  - In part 1, focus on the shortest path to the end.  
  - In part 2, also count the number of tiles covered, excluding start and end.

#### Fun Insights

- The reindeer’s movement required combining both position and direction updates to keep the score and path accurate.
- Part 2 added a fun twist by introducing special path tiles ('O') and the need to optimize not just for reaching the end, but for maximizing the tiles covered.

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