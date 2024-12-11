# Day 01: Historian Hysteria

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition


The task requires comparing two lists of location IDs and calculating the total distance between them in Part 1 and a similarity score in Part 2.

**Part 1**:
- Pair up the smallest numbers in both lists and calculate their absolute distance.
- Sum up the distances to get the total.

**Part 2**:
- Count how many times each number from the left list appears in the right list.
- Multiply each number in the left list by the number of times it appears in the right list, then sum these products to get the similarity score.


- [x] Parse the input into two lists of numbers.
- [x] Sort both lists.
- [x] Calculate the total distance between corresponding pairs in the two lists (Part 1).
- [x] Count occurrences of each number from the left list in the right list (Part 2).
- [x] Compute the similarity score by multiplying and summing these counts.

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