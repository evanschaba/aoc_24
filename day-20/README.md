# Day 24: Race Condition

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

In this challenge, you are tasked with navigating a race track represented by a grid of tiles, some of which are blocked by walls. The goal is to find the shortest path from the start (`S`) to the end (`E`), and then count the number of possible "cheats" that can save at least a specified number of picoseconds by skipping over obstacles.

### Part 1: Count Cheats Saving at Least 100 Picoseconds
- **Objective**: Calculate the number of cheats that save at least 100 picoseconds. A cheat allows bypassing walls for a limited time (2 picoseconds).
- **Approach**:
  - Parse the input grid to identify start (`S`), end (`E`), and obstacles (`#`).
  - Use **Breadth-First Search (BFS)** to find the shortest path from the start to every other point on the grid. This will help in determining the possible routes.
  - Track all reachable points and check if a shortcut (cheat) can be applied, reducing the time by skipping obstacles.
  - Count how many "cheats" save at least 100 picoseconds by comparing the paths where obstacles are skipped.

#### Key Concepts
- **Breadth-First Search (BFS)**: This algorithm finds the shortest path in an unweighted grid and is optimal for exploring all reachable nodes from a source node.
- **Cheating**: A "cheat" is defined as a shortcut that skips over walls for 2 picoseconds, offering a time-saving shortcut between two points.

### Part 2: Count Cheats Saving at Least 100 Picoseconds with Extended Cheat Duration
- **Objective**: Extend the cheat duration to 20 picoseconds and calculate how many cheats save at least 100 picoseconds.
- **Approach**:
  - Similar to Part 1 but with the extended cheat duration.
  - Modify the BFS to consider the new, longer cheat duration, which allows you to bypass walls over a greater distance.
  - Track and count valid cheats with the new time-saving threshold.

#### Key Concepts
- **Extended Cheat Duration**: The cheat duration is now 20 picoseconds, allowing more substantial shortcuts compared to Part 1, enabling more complex pathfinding optimizations.

### Fun Notes
- **Grid Navigation**: This challenge involves navigating a grid with obstacles, a classic problem in robotics, pathfinding, and navigation algorithms.
- **Cheating in Games**: This problem plays on the concept of "cheats" in video games, where shortcuts or exploits can be used to gain advantages, speeding up gameplay.

---

## Steps Taken

### Part 1
- [x] Parse the input to identify start (`S`), end (`E`), and obstacles (`#`).
- [x] Implement BFS to compute the shortest path from the start position.
- [x] Identify all reachable points and check which can be "cheated" (i.e., can be bypassed).
- [x] Count the number of valid cheats that save at least 100 picoseconds.
- [x] Return the total number of cheats that meet the time-saving condition.

### Part 2
- [x] Modify the cheat duration to 20 picoseconds.
- [x] Update the BFS logic to account for the extended cheat duration.
- [x] Track and count valid cheats with the extended cheat duration that save at least 100 picoseconds.
- [x] Return the result, including the total number of extended cheats.

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