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


## Usage Instructions

```zsh
    # Lint, Format & apply auto-fixes:
    cargo fmt 
    cargo clippy --fix --allow-dirty --allow-staged;

    # testing with RUST_BACKTRACE:
    RUST_BACKTRACE=1 cargo test --bin part-1
    RUST_BACKTRACE=1 cargo test --bin part-2

    # Running the Program:**
    cargo run --bin part-1 -- challenge_1.txt
    cargo run --bin part-2 -- challenge_2.txt
```