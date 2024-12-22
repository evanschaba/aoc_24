# Day 22: Monkey Market

[**Challenge Details**](docs/challenge.md)

## Problem Decomposition

In this challenge, you need to simulate a series of secret number generations from buyers in the Monkey Market, then calculate and predict the prices they offer based on their evolving secret numbers. You'll also analyze changes in prices to maximize the number of bananas you can earn by guiding a monkey to sell at the right time.

### Part 1: Calculate the Sum of the 2000th Secret Number
- **Objective**: For each buyer, calculate the 2000th secret number generated starting from their initial secret number. Then, sum these numbers across all buyers.
- **Approach**:
  - Each buyer starts with an initial secret number and applies a set of transformations to generate subsequent secret numbers.
  - The transformation involves multiplying, dividing, XORing, and pruning the secret number according to a specific algorithm.
  - Repeat the transformation for 2000 steps and calculate the final secret number.
  - Sum all the final secret numbers across all buyers.

#### Key Concepts
- **Secret Number Generation**: Apply a series of mathematical transformations and modular arithmetic to generate the next secret number.
- **Modulo Operation**: Ensure the number is within a specific range after each transformation using modulo `16777216`.

### Part 2: Maximize Banana Earnings by Analyzing Price Changes
- **Objective**: Analyze the changes in prices over time and find the sequence of four consecutive price changes that will yield the maximum number of bananas when detected by a monkey.
- **Approach**:
  - For each buyer, calculate the sequence of prices based on their secret numbers.
  - Calculate the differences between consecutive prices and identify sequences of four price changes.
  - For each sequence, check when it first occurs in the price changes of each buyer.
  - Identify the sequence of changes that will give the highest total banana count when sold by the monkey.

#### Key Concepts
- **Price Analysis**: Convert the secret numbers to their ones digits to derive prices, and compute price changes over time.
- **Sliding Window**: Use a sliding window to track and detect sequences of price changes.

---

## Steps Taken

### Part 1
- [x] Parse input to extract the initial secret numbers for each buyer.
- [x] Simulate the generation of the 2000th secret number for each buyer using a transformation function.
- [x] Sum all the 2000th secret numbers to obtain the final result.

### Part 2
- [x] Convert secret numbers to prices and compute price changes over time.
- [x] Identify all sequences of four consecutive price changes.
- [x] Find the sequence that will maximize the total number of bananas when sold.
- [x] Compute the result by summing the prices at the points where the sequence occurs.

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