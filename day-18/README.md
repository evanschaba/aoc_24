# Day 18: RAM Run

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

In this challenge, you need to simulate the process of bytes falling into a 2D grid representing memory space. Each byte falls in at specific coordinates and corrupts the memory at those locations. The challenge requires determining the shortest path to the exit while avoiding corrupted memory, which becomes increasingly more difficult as bytes fall into place. In Part 1, you'll find the shortest path to the exit after simulating the first 1024 bytes. In Part 2, you'll determine the first byte that blocks the path to the exit.

### Part 1: Shortest Path to the Exit
- **Objective**: Calculate the minimum number of steps required to travel from the top-left to the bottom-right of the grid, avoiding corrupted coordinates.
- **Approach**:
  - Parse the input to extract the coordinates where the bytes will fall.
  - Simulate the memory grid by marking locations as corrupted based on the given byte coordinates.
  - Use **Breadth-First Search (BFS)** to explore the grid and find the shortest path from the start `(0, 0)` to the exit `(width-1, height-1)`.
  - Return the number of steps taken in the shortest valid path.

#### Key Concepts
- **Breadth-First Search (BFS)**: BFS is particularly useful here because it guarantees that the first time we reach the exit, we have taken the shortest possible path. BFS explores all neighbors level by level, ensuring the shortest path is found in unweighted graphs.
  
### Part 2: First Byte That Blocks the Path
- **Objective**: Identify the first byte that blocks the path to the exit after it is placed.
- **Approach**:
  - Instead of finding the shortest path directly, simulate bytes falling into the memory grid and check whether the exit becomes unreachable after each byte.
  - **Binary Search** is used to optimize the process by halving the search space. This reduces the number of simulations needed to find the exact byte that blocks the path.

#### Key Concepts
- **Binary Search**: By adjusting the number of bytes simulated using binary search, we reduce the number of checks, making the process more efficient than simulating each byte individually.
  
### Fun Notes
- **Pushdown Automata**: This concept is a type of computational model used to process context-free languages and perform stack-based memory operations. In real-world applications, pushdown automata are often used for **compiler design**, especially for syntax and grammar checking. Compilers rely heavily on concepts like **context-free grammars** to parse programming languages correctly, which is why automata play a crucial role in making sure code is syntactically valid.
  
- **Memory Corruption**: In the real world, memory corruption is a serious issue in computing, leading to bugs, crashes, and security vulnerabilities. For example, **buffer overflow** vulnerabilities occur when more data is written to a memory buffer than it can hold, causing corruption and potentially allowing for exploits. In fact, some types of **malware** exploit memory corruption to gain control of systems.

- **Routing Algorithms**: The idea of avoiding corrupted locations and finding the shortest path can be related to **robotics**, where pathfinding algorithms (like BFS or A*) are used in environments with obstacles (like moving robots avoiding walls). This also has applications in **network routing**, where packets of data need to find optimal paths through networks while avoiding bottlenecks or faulty nodes.

- **Optimization Problems**: The problem involves techniques that are often used in optimizing **traffic flow** or **logistics** in real-world scenarios. For example, in **supply chain management**, you might have to simulate the movement of goods through various facilities (similar to bytes falling into a grid), and find the most efficient path while avoiding areas of congestion or failure.

### Steps Taken

#### Part 1: Shortest Path to the Exit
- [x] Parse the input file into grid coordinates of corrupted memory.
- [x] Simulate the memory grid, marking coordinates as corrupted based on the falling bytes.
- [x] Use BFS to explore the grid and find the shortest path from `(0, 0)` to `(width-1, height-1)`.
- [x] Return the minimum number of steps required to reach the exit.

#### Part 2: Finding the First Blocking Byte
- [x] Use binary search to minimize the number of bytes simulated.
- [x] After each simulation, check if the exit is still reachable.
- [x] Return the coordinates of the first byte that blocks the path to the exit.

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