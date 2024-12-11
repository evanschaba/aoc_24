# Advent of Code - Day 6: Guard Gallivant

[**Challenge details**](docs/challenge.md)

## Challenge Overview

The goal of this challenge is to predict the patrol path of a guard following a strict protocol through a lab environment. Using the provided map, which includes obstacles and the guard's initial position and direction, we need to determine how many distinct positions the guard will visit before exiting the mapped area.

## Approach to Solving the Challenge

- **Input Parsing**:
  - Read and interpret the map to understand the guard's starting position and direction (`^`, `>`, `v`, `<`).
  - Identify obstacles (`#`) on the map.

- **Guard Movement Protocol**:
  - Understand the two rules:
    1. If there is an obstacle directly in front of the guard, turn 90° to the right.
    2. Otherwise, move one step forward in the current direction.
  - Repeat the rules iteratively until the guard exits the map boundaries.

- **Tracking Visited Positions**:
  - Keep a record of all unique positions visited by the guard, including the starting position.
  - Ensure positions are stored efficiently to avoid duplicates.

- **Boundary Check**:
  - Detect when the guard moves beyond the map boundaries, signaling the end of the patrol.

- ***Simulation***:
  - Implement a step-by-step simulation of the guard's movement, applying the patrol rules and updating the visited positions at each step.

## Insights and Learnings

- **Pattern Recognition**:
  - The challenge teaches us to break down movement rules into discrete, predictable actions that can be iteratively simulated.

- **State Tracking**:
  - Keeping track of the guard’s position, direction, and visited states highlights the importance of state management in simulations.

- **Boundary and Edge Case Handling**:
  - Managing edge cases, such as starting near a boundary or encountering consecutive obstacles, is crucial for robust solutions.

- ***Optimization***:
  - Efficient storage and retrieval of visited positions (e.g., using sets) minimizes computational overhead.

- **Real-World Analogy**:
  - This challenge mirrors problems in robotics and path planning, where understanding and predicting movement based on constraints is critical.

## Key Takeaways
- The value of iterative rule application for simulating behavior.
- Importance of mapping and tracking states in a confined space.
- Skills in parsing input, managing state transitions, and handling boundaries translate well to both computational and physical-world problem-solving.

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