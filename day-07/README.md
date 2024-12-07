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

- **Modularity**: Each function handles a single task, making the code more understandable, maintainable, and testable.
- **Scalability**: Structured design makes adding new operators or adapting logic straightforward.
- **Reusability**: Recursive solution generation and validation logic are reusable for similar problems.
- **Debugging Ease**: Clear separation of tasks simplifies debugging and identifying performance bottlenecks.
- **Extensibility**: Transitioning from Part 1 to Part 2 required minimal changes, demonstrating the codebase's flexibility.

## Usage Instructions

Follow these steps to work with this Rust project:

```zsh
# Format the Code
# Ensure the code adheres to Rust's standard formatting:
cargo fmt --all

# Check & Auto-Fix Warnings
# Use Clippy to lint and improve code quality:
cargo clippy --all-targets --all-features -- -D warnings

# Automatically Apply Fixes
cargo fix --allow-dirty --allow-staged

# Run the Code
# Execute the program with debugging enabled:
RUST_BACKTRACE=1 cargo run --bin part-1 -- docs/challenge_1.txt
RUST_BACKTRACE=1 cargo run --bin part-2 -- docs/challenge_2.txt

# Test the Code
# Run all unit tests to verify correctness:
RUST_BACKTRACE=1 cargo test --bin part-1
RUST_BACKTRACE=1 cargo test --bin part-2
