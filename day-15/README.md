# Day 15: Warehouse Woes

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

In this challenge, we simulate a robot in a warehouse where it moves around and pushes boxes. The robot's movement is guided by a sequence of commands, and we need to account for the robot's interactions with boxes, walls, and the boundaries of the warehouse.

**Key Steps:**
- Parse the warehouse layout and movement instructions.
- Simulate the robot’s movement and its effect on boxes (`O`), ensuring no box or robot moves through walls (`#`).
- Track the GPS coordinates of all boxes after all moves are processed, where the GPS value is calculated as `100 * row + col` for each box.

**Fun Elements:**
- The robot’s behavior is dynamic, where sometimes the box-pushing logic can cause cascading effects as boxes shift.
- For Part 2, the warehouse is expanded, and the robot now interacts with larger boxes, requiring us to adjust our calculations accordingly.

### Checklist for Solving Part 1:
- [x] Parse the initial warehouse layout and movement instructions.
- [x] Implement robot movement logic with respect to box-pushing and wall collisions.
- [x] Update the positions of the robot and boxes as per the movement instructions.
- [x] Calculate the GPS coordinates of all boxes after the robot completes all its moves.
- [x] Compute the sum of all GPS coordinates.

### Checklist for Solving Part 2:
- [x] Expand the warehouse layout to double its width, adjusting tiles accordingly.
- [x] Implement the same robot movement logic, now with wider boxes and new interactions.
- [x] Calculate the new GPS coordinates for the expanded warehouse.
- [x] Compute the sum of all new GPS coordinates.

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