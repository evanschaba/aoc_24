# Advent of Code - Day 8: Resonant Collinearity

[**Challenge Details**](docs/challenge.md)

##  Problem Decomposition
   
For Day 8, the task was to identify antinodes created by antennas based on their frequency. An antinode is a position in line with two antennas of the same frequency, with one antenna being twice the distance from the other.

### Key Steps

- Read the input grid from a file, which contains antennas marked by characters.
- Parse the grid and store antenna positions grouped by their frequencies.
- Iterate through pairs of antennas of the same frequency and calculate potential antinode positions.
- Check if these positions are valid (within the grid and meeting the distance criteria).
- Track and count unique antinode locations.

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