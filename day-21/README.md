# Day 21: Keypad Conundrum

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

In this challenge, you need to guide a robot to type specific door codes on a numeric keypad using a complex directional keypad. The challenge is to calculate the shortest sequence of button presses on a directional keypad, which will then cause a robot to type the correct sequence on a numeric keypad.

### Part 1: Calculate the Shortest Sequence
- **Objective**: Calculate the shortest sequence of directional keypad presses that will allow a robot to type a specific code on a numeric keypad.
- **Approach**:
  - The robot's arm starts at the 'A' position on the directional keypad.
  - Use directional buttons (up, down, left, right) to move the arm across the keypad and the 'A' button to press the corresponding button on the numeric keypad.
  - Compute the shortest sequence for each door code by using an optimal pathfinding algorithm.

#### Key Concepts
- **Pathfinding**: The problem involves calculating the shortest path on a grid, ensuring that no invalid positions (gaps) are ever visited.
- **Search Algorithms**: Implementing a search algorithm like BFS or DFS to explore the shortest path.

### Part 2: Calculate Complexity of Each Code
- **Objective**: For each code, calculate the complexity by multiplying the length of the shortest sequence by the numeric part of the code.
- **Approach**:
  - For each code, compute the sequence of button presses.
  - Multiply the length of this sequence by the numeric value of the code (ignoring leading zeros).
  - Sum the complexities for all codes to obtain the final result.

#### Key Concepts
- **Multiplicative Complexity**: Each code's complexity is determined by the product of its sequence length and numeric value.

### Fun Notes
- **Grid Navigation**: This challenge is a good exercise in **grid navigation** algorithms and **pathfinding** problems, often seen in robotics or game development.
- **Optimized Search**: The challenge also involves optimizing the search for the shortest path in an environment where some nodes (gaps) are not allowed.

---

## Steps Taken

### Part 1
- [x] Parse input to extract the door codes.
- [x] Implement pathfinding to determine the shortest sequence of directional button presses.
- [x] Return the shortest sequence for each code.

### Part 2
- [x] Calculate the complexity for each code by multiplying the sequence length by the numeric value.
- [x] Sum the complexities to get the final result.

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