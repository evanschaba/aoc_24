# Day 23: Circuit Logic Simulation

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

This problem simulates a [logic circuit](https://en.wikipedia.org/wiki/Logic_gate) where each wire is either a 1 or a 0, and several gates perform logic operations (AND, OR, XOR) on the wires. The task is to calculate values based on the gate operations and wire values, and then detect possible issues with the configuration of the circuit.

### Part 1: Simulating Logic Gates
- **Objective**: Calculate the final value of wires after performing logic operations on them.
- **Approach**:
  - Parse the input data to extract wire values and gates.
  - Simulate the [logic gates' operations](https://en.wikipedia.org/wiki/Logic_gate) (AND, OR, XOR) on the wires.
  - Track the wire states and decode the final output for specific wires.

#### Key Concepts
- **Gate Logic**: [AND](https://en.wikipedia.org/wiki/AND_gate), [OR](https://en.wikipedia.org/wiki/OR_gate), [XOR](https://en.wikipedia.org/wiki/XOR_gate) operations on wire values.
- **Wire Decoding**: Extracting binary values from wire states based on their names.

### Part 2: Detecting Broken Nodes
- **Objective**: Identify any broken nodes in the circuit based on certain predefined patterns.
- **Approach**:
  - Parse the input to build the connections between wires and gates.
  - Detect "broken" nodes by checking patterns like XOR gates connected to non-XOR wires or incorrect input combinations.

#### Key Concepts
- **Broken Node Detection**: Identifying irregularities in node configurations like missing connections or mismatched logic gates.

---

## Steps Taken


### Part 1: Simulating Logic Gates
1. **Parse the Input**:
   - [x] - Extract wire values and gate descriptions.
   - [x] - Map wire names to boolean values.
2. **Simulate the Gates**:
   - [x] - Apply gate logic (AND, OR, XOR) on the wires.
   - [x] - Update wire values based on the operations.
3. **Decode the Result**:
   - [x] - Decode the final output wire values based on the prefix "z".
   - [x] - Return the result for part 1.

### Part 2: Detecting Broken Nodes
1. **Parse the Input**:
   - [x] - Extract wire values and gate descriptions.
   - [x] - Map gates to their input and output connections.
2. **Identify Broken Nodes**:
   - [x] - Check for gates with incorrect configurations.
   - [x] - Mark any problematic nodes as broken.
3. **Return Results**:
   - [x] - Sort and format the broken nodes.
   - [x] - Return the comma-separated list of broken nodes.

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
