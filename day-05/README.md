### Advent of Code - Day 5: Print Queue

[**Challenge details**](docs/challenge.md)

### Approach to solving the queue challenge

#### Problem Summary
The task is to determine if updates to a manual, defined as sequences of page numbers, adhere to a set of ordering rules. For updates that are in the correct order, identify the "middle page number" and sum them up.

#### Preferred Approach
1. **Parse Input**:
   - Split the input into two sections: the rules and the updates.
   - Represent the rules as directed relationships (e.g., a graph or adjacency list).

2. **Validate Order**:
   - For each update, check if the sequence respects all relevant rules.
   - Ignore rules for pages not present in the update.

3. **Find Middle Pages**:
   - Identify correctly-ordered updates.
   - Extract the middle element from each update.

4. **Compute the Result**:
   - Sum the middle page numbers of all valid updates.

#### Benefits of This Approach
- **Efficiency**: Using graph traversal (e.g., topological sort or constraints checking) ensures that even larger inputs can be processed quickly.
- **Scalability**: Isolating rules and updates simplifies validation and reduces redundant checks.
- **Modularity**: The parsing, validation, and summation steps can be tested and refined independently.

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