# Advent of Code 2024 - Day 2: Red-Nosed Reports

## Problem Overview

### Part 1: Safe Reports Analysis

The engineers at the Red-Nosed Reindeer nuclear fusion/fission plant have requested your help in analyzing unusual data reports from the reactor. Each report is a list of numbers, called levels, separated by spaces. Your task is to identify reports that meet the following criteria for safety:

1. The levels are either strictly increasing or strictly decreasing.
2. The difference between any two adjacent levels is at least 1 and at most 3.

**Example input:**
7 6 4 2 1 1 2 7 8 9 9 7 6 2 1 1 3 2 4 5 8 6 4 4 1 1 3 6 7 9

csharp
Copy code

**Output for Part 1:**
In the given example, 2 reports are safe.

### Part 2: Problem Dampener

The engineers have a "Problem Dampener," which allows the reactor safety systems to tolerate one level that would otherwise render the report unsafe. If removing a single level from an unsafe report would make it safe, the report should be considered safe.

**Example input with Part 2 rules:**
7 6 4 2 1: Safe without removing any level. 1 2 7 8 9: Unsafe regardless of which level is removed. 9 7 6 2 1: Unsafe regardless of which level is removed. 1 3 2 4 5: Safe by removing the second level, 3. 8 6 4 4 1: Safe by removing the third level, 4. 1 3 6 7 9: Safe without removing any level.

markdown
Copy code

**Output for Part 2:**
In this updated scenario, 4 reports are now safe.

## Code Quality and Validation

This project includes automated code quality checks, ensuring the highest level of maintainability and production readiness. The code adheres to best practices and includes detailed documentation. The project uses continuous integration (CI) to validate code with tests and format checks.

## usage

```bash
# Clone
git clone https://github.com/username/aoc_2024.git
cd aoc_2024/day-02

# Run the project in debug mode:
cargo run --bin part-2 -- input_2.txt

#Lint and format the code:
cargo test
cargo clippy
cargo fmt
```

### License

This project is licensed under the Unlicense. You are free to use, modify, distribute, or do whatever you like with this code.

