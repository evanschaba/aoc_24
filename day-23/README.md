# Day 23: LAN Party

[**Challenge Details**](docs/challenge_1.txt)

## Problem Decomposition

This challenge involves analyzing a network of computers to identify specific groups of fully connected nodes. The task is divided into two parts:

### Part 1: Finding Triangles
- **Objective**: Identify all sets of three computers (triangles) where each computer is directly connected to the other two. Further, filter these triangles to include only those where at least one computer's name starts with the letter `t`.
- **Approach**:
  - Parse the network connections into an adjacency map.
  - Search for sets of three computers that form a triangle by checking mutual connectivity.
  - Filter triangles to retain only those containing a computer whose name starts with `t`.
  - Count the resulting triangles.

#### Key Concepts
- **Graph Representation**: The network is represented as an undirected graph using an adjacency map.
- **Triangle Detection**: Efficiently finding fully connected sets of three nodes is critical.
- **Filtering**: Post-process triangles to filter based on naming constraints.

### Part 2: Largest Fully Connected Set
- **Objective**: Find the largest group of computers that are all connected to each other (a clique). Return the clique's computers sorted alphabetically and joined with commas as the password.
- **Approach**:
  - Use a recursive algorithm to find the largest clique in the graph.
  - Ensure that the resulting set of nodes is fully connected.
  - Format the solution as a sorted, comma-separated string.

#### Key Concepts
- **Clique Finding**: A computationally challenging problem that requires pruning candidates efficiently.
- **Backtracking Search**: Recursive exploration of cliques with pruning to find the largest set.

---

## Steps Taken

### Part 1: Finding Triangles
1. **Input Parsing**:
   - [x] - Read the network connections and build an adjacency map.
   - [x] - Track all computers whose names start with `t`.
1. **Triangle Detection**:
   - [x] - Iterate through all nodes and their neighbors to find triangles.
   - [x] - Use a hash set to ensure each triangle is counted only once (in sorted order).
2. **Filtering**:
   - [x] - Retain only triangles that include at least one computer whose name starts with `t`.
3. **Output**:
   - [x] - Count and return the number of filtered triangles.

### Part 2: Largest Clique
1. **Input Parsing**:
   - [x] - Build the adjacency map from the input.
2. **Clique Detection**:
   - [x] - Use a recursive backtracking algorithm to find the largest fully connected set of nodes.
   - [x] - Prune candidates that are not fully connected to all nodes in the current clique.
3. **Formatting**:
   - [x] - Sort the nodes in the largest clique alphabetically.
   - [x] - Join them with commas to generate the password.

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