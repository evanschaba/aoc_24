# Day 17: Chronospatial Computer

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

The challenge revolves around simulating a minimal `3-bit` computer with instructions, registers, and an **instruction pointer**. The system processes an input program that uses fundamental operations like [division](https://en.wikipedia.org/wiki/Division_(mathematics)), [bitwise XOR](https://en.wikipedia.org/wiki/Bitwise_operation#XOR), [modulus](https://en.wikipedia.org/wiki/Modulo_operation), and conditional jumps. Each instruction manipulates one of three registers (`A`, `B`, `C`) or outputs a value.

The task is split into two parts:

- **Part 1**: Run a program using a given register state and collect all output values.
- **Part 2**: Identify the lowest possible initial value of Register `A` such that the program outputs a copy of itself.

### Key Insights:
- **Combo Operands**: Interpret the operand as either a literal value (`0–3`) or as the content of registers (`A`, `B`, `C`).
- **Instruction Execution**: Map the given program into actions (like division and XOR) that update registers or jump within the program.
- **Output Validation**: Compare program output for correctness, especially in Part 2 where the program "self-replicates".

---

## Checklist to Solve the Challenge

### Part 1
- [x] Parse the input to extract:
  - Initial values for registers `A`, `B`, `C`
  - Program instructions
- [x] Implement a virtual machine (VM) to:
  - Read opcodes and operands
  - Execute instructions (division, bitwise XOR, jumps, etc.)
  - Collect output when encountering the `out` instruction
- [x] Join the collected output values into a comma-separated string for the final result.

### Part 2
- [x] Identify the minimum value of Register `A` that causes the program to output itself:
  - [x] Incrementally test candidate values of `A`
  - [x] Validate that the program's output matches the original instructions
  - [x] Stop when a match is found.

---

## Fun Notes and Real-World Uses

1. **Bit Manipulation**: Operations like [bitwise XOR](https://en.wikipedia.org/wiki/Bitwise_operation#XOR) are critical in fields such as cryptography, error detection (checksums), and efficient algorithms for switching data values without temporary variables.

2. **Self-Replicating Programs**: Part 2 involves creating programs that output their own source code—akin to **[quine programs](https://en.wikipedia.org/wiki/Quine_computing)**. This concept appears in **debuggers**, **compilers**, and even malicious code like [computer worms](https://en.wikipedia.org/wiki/Computer_worm).

3. **Virtual Machines (VMs)**: Simulating a CPU with registers and opcodes is foundational for real-world systems like **Java Virtual Machine (JVM)** and emulators for retro systems.

4. **Instruction Pointer Jumps**: Handling conditional jumps (like `jnz`) mimics how modern CPUs perform branching and optimize program execution.

This challenge is a fun blend of computer architecture and algorithm design, making it a perfect opportunity to explore low-level programming concepts!

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