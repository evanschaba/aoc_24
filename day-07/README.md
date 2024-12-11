# Advent of Code - Day 7: Bridge Repair

[**Challenge Details**](docs/challenge.md)

## Approach to Solving the Challenges

### Problem Decomposition

The challenges were tackled by dividing them into distinct, manageable tasks, allowing for modular solutions. This design enabled clear separation of concerns and seamless extensions from [Part 1](src/bin/part-1.rs) to [Part 2](src/bin/part-2.rs).

### Key Steps

1. **Input Parsing**  
   Inputs were parsed into structured data (`Expr` structs), simplifying the handling of complex and varied inputs while ensuring robustness and reusability.

2. **Recursive Operator Exploration**  
   A recursive backtracking algorithm explored all possible combinations of operators (`+`, `*`, and `||`) between the numbers, ensuring every valid solution was evaluated.

3. **Dynamic Operator Evaluation**  
   Operators were evaluated left-to-right to comply with challenge requirements. The addition of the `||` operator in Part 2 was seamlessly incorporated into the existing evaluation logic.

4. **Validation of Results**  
   Each operator combination was validated against the expected equation result. This ensured correctness and enabled early exits when valid solutions were found.

5. **Summing Valid Results**  
   Valid equations were summed to compute the final results for each part of the challenge.

### Benefits of This Approach

- ***Modularity***: Each function handles a single task, making the code more understandable, maintainable, and testable.
- ***Scalability***: Structured design makes adding new operators or adapting logic straightforward.
- ***Reusability***: Recursive solution generation and validation logic are reusable for similar problems.
- **Debugging Ease**: Clear separation of tasks simplifies debugging and identifying performance bottlenecks.
- ***Extensibility***: Transitioning from Part 1 to Part 2 required minimal changes, demonstrating the codebase's flexibility.

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