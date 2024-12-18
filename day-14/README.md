# Day 14: Restroom Redoubt

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

This problem involves simulating the movement of robots in a bounded space, with positions and velocities provided in the input. The robots move according to their velocities, and we need to predict their positions at a given time, checking for any interesting formations they might form (like a Christmas tree). The problem is divided into two parts: Part 1 involves calculating the safety factor by analyzing robot positions after 100 seconds, while Part 2 asks for the fewest seconds until the robots form a specific pattern.

### Part 1 - Safety Factor Calculation
- [x] **Parse the input** to extract robot positions and velocities.
  - [x] Implement a regular expression to capture robot data in the format `p=x,y v=vx,vy`.
  - [x] Parse the positions and velocities into integers for processing.
  
- [x] **Simulate robot movement** after 100 seconds in a space of fixed width and height (101 x 103).
  - [x] Implement a function to calculate each robot's new position after 100 seconds, considering the wrapping behavior of the space (i.e., robots that hit the edges "teleport" to the opposite side).
  
- [x] **Determine the safety factor** by counting the robots in each quadrant after 100 seconds.
  - [x] Exclude robots that are exactly in the center (horizontally or vertically) from the count.
  - [x] Implement logic to count robots in each of the four quadrants.
  - [x] Multiply the robot counts in all quadrants to compute the safety factor.

### Part 2 - Robot Formation
- [x] **Simulate robot movement over time** to detect when the robots form a recognizable pattern.
  - [x] Continue simulating robot positions after each second.
  - [x] Track robot positions at each time step and store them in a set to avoid counting duplicates.
  
- [x] **Detect robot formations** by checking when a significant number of robots are positioned adjacent to each other.
  - [x] Implement a function to check the number of adjacent robots (robots that are "touching").
  - [x] Track the number of robots that are touching each other in each step.
  - [x] Stop the simulation when the robots form the desired pattern (e.g., a Christmas tree).

- [x] **Return the time when the robots form the pattern**. The earliest time when at least half the robots are in formation is the result.

#### Fun Observations:
- Robots wrap around the edges of the space, which means their movement is not just linear but "cyclic".
- Part 1 involves calculating a safety factor based on robot distribution, which adds a spatial complexity to the problem.
- Part 2 requires detecting an interesting formation (like a Christmas tree) based on robot proximity, which involves simulating the positions for multiple steps and checking for patterns.

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