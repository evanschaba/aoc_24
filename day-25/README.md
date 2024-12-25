# Day 25: Virtual Lock and Key Challenge [FINAL] ⭐️⭐️ 🎄 ⭐️⭐️

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

The problem revolves around matching locks and keys represented as schematics of five-pin tumbler locks. The locks and keys are defined by their columnar heights, and a valid pair is one where the key fits perfectly into the lock without overlapping. The challenge is divided into two parts:

### Part 1: Counting Valid Lock/Key Pairs
- **Objective**: Determine the number of lock/key pairs that fit together without overlapping.
- **Approach**:
  - Parse the input into schematics, classifying them as either locks or keys.
  - Calculate the column heights for both locks and keys.
  - For each lock/key pair, check if the key's height in each column is less than or equal to the corresponding lock's pin height.
  - Count all valid pairs.

#### Key Concepts
- **Schematic Representation**: Locks and keys are represented as height arrays derived from their schematics.
- **Validation Logic**: A key fits a lock only if no column in the lock is shorter than the corresponding column in the key.

### Part 2: Chronicle Completion
- **Objective**: Combine all historical notes to finalize the chronicle for Santa, ensuring no place is left out.
- **Approach**:
  - Gather all relevant historical notes from The Historians.
  - Cross-check visited places against the chronicle requirements.
  - Compile the notes into the final chronicle document, ready for submission.

#### Key Concepts
- **Chronicle Integration**: Leveraging previously visited locations and historical notes.
- **Validation**: Ensuring completeness before submitting the chronicle.

---

## Steps Taken

### Part 1: Valid Lock/Key Pair Calculation
1. **Parse Input**:
   - [x] Split the input into schematic groups.
   - [x] Identify whether each group is a lock or a key.
   - [x] Compute columnar heights for each schematic.
2. **Validate Lock/Key Pairs**:
   - [x] Generate all combinations of locks and keys.
   - [x] Check if the key fits the lock without overlapping.
   - [x] Count all valid pairs.

### Part 2: Chronicle Compilation
1. **Collect Notes**:
   - [x] Gather all historical notes from The Historians.
2. **Validate Coverage**:
   - [x] Ensure all required places are visited.
   - [x] Check for any missing details in the notes.
3. **Compile Chronicle**:
   - [x] Assemble the final chronicle document.
   - [x] Wrap it up and prepare for submission to Santa.

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