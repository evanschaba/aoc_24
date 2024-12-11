### Advent of Code - Day 4: Ceres Search

##### Intro

Day 4 of Advent of Code 2024 presents a challenging word search puzzle. The story begins at the Ceres monitoring station, where you help an Elf find the word "XMAS" in a grid of characters. This challenge involves not just finding the word in a straightforward manner, but also counting all possible instances of it in various orientations and positions.

##### Problem Description

**Part 1: Simple XMAS Search**

Given a grid of characters, your task is to find how many times the word "XMAS" appears. The word can appear:

- [x] - Horizontally

- [x] - Vertically

- [x] - Diagonally

- [x] - Backwards

- [x] - Without Overlapping with words

Example grid:

```
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
```

In this example, `"XMAS"` appears a total of 18 times.

**Part 2: X-MAS Pattern Search**

For part 2, you need to find instances of the `"X-MAS"` pattern, which consists of two occurrences of `"MAS"` arranged in an X shape:
```
M.S
.A.
M.S
```
Each occurrence of `"MAS"` can be oriented forwards or backwards within the X.

Example grid (with the `X-MAS` marked):
```
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.SS.
.A.A.A.A..
M.M.M.M.M.
..........
```
In this example, the `X-MAS` pattern appears 9 times.

##### Approach and Solution

Steps

Read and parse the input grid: Convert the input text file into a 2D array.

Implement a search algorithm: Iterate through the grid and check for instances of "XMAS" or the `X-'MAS'` pattern.

Count occurrences: Implement logic to count each valid instance, with valid ranes, without off by 1 misses and without overlaps.

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