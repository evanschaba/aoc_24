# Advent of Code 2024: Historian Hysteria

--- Day 1: Historian Hysteria ---

The Chief Historian is always present for the big Christmas sleigh launch, but nobody has seen him in months! Last anyone heard, he was visiting locations historically significant to the North Pole. A group of Senior Historians has asked you to join them in checking the places he was most likely to visit.

### Goal
Help the historians collect **50 stars** before Santa's takeoff on **December 25th** by solving puzzles. Each day, two puzzles are unlocked. Completing the first unlocks the second, and each puzzle awards one star.

---

### Problem Description

The historians' list of locations to check is empty! While searching the Chief Historian's office, they find two lists of **location IDs**. However, these lists are inconsistent. Your task is to help them reconcile the two lists by calculating the **total distance** between them.

#### Example Input
```
List 1: [3, 4, 2, 1, 3, 3]
List 2: [4, 3, 5, 3, 9, 3]
```

#### Matching Procedure:
1. Sort both lists.
2. Pair elements in sorted order.
3. Compute the absolute difference for each pair.
4. Sum all differences.

#### Example Calculation
| Pair | Difference |
|------|------------|
| (1, 3) | 2 |
| (2, 3) | 1 |
| (3, 3) | 0 |
| (3, 4) | 1 |
| (3, 5) | 2 |
| (4, 9) | 5 |

**Total Distance**: `2 + 1 + 0 + 1 + 2 + 5 = 11`

---

### Usage

#### Running the Solutions
Use the following commands to execute the solutions for Part 1 and Part 2:

```bash
cargo run --bin part-1 -- input.txt
cargo run --bin part-2 -- input_2.txt
```

#### Input Format
Each input file should contain two lists of integers, one per line. For example:
```txt
3, 4, 2, 1, 3, 3
4, 3, 5, 3, 9, 3
```

---

### Notes
- Ensure your input files are formatted correctly.
- Replace `input.txt` and `input_2.txt` with the appropriate file paths.

Good luck, and may the stars guide you!
