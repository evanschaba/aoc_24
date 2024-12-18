use std::{collections::HashMap, env, fs};

fn main() {
    // Collect command-line arguments, expecting the first argument to be the input file name.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    // Read the content of the input file.
    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");

    // Parse numbers from the input file.
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    println!("Part 1: {}", calculate_total_stones(&stones, 25));
}

// Calculates the total number of stones after a given number of blinks.
fn calculate_total_stones(stones: &[u64], blinks: u8) -> u64 {
    let mut memoization_table: HashMap<(u64, u8), u64> = HashMap::new();

    stones
        .iter()
        .map(|&stone| evolve_stone(stone, blinks, &mut memoization_table))
        .sum()
}

// Computes the number of stones resulting from evolving a single stone.
fn evolve_stone(stone: u64, blinks: u8, memoization_table: &mut HashMap<(u64, u8), u64>) -> u64 {
    // Check for cached results.
    if let Some(&cached_result) = memoization_table.get(&(stone, blinks)) {
        return cached_result;
    }

    // Base case: no more blinks left.
    if blinks == 0 {
        return 1;
    }

    let result = if stone == 0 {
        // Rule 1: Replace a stone marked 0 with a stone marked 1.
        evolve_stone(1, blinks - 1, memoization_table)
    } else {
        let digit_count = ((stone as f64).log10() + 1.0) as u32;

        if digit_count % 2 == 0 {
            // Rule 2: Split the stone into two halves if it has an even number of digits.
            let left_half = stone / 10u64.pow(digit_count / 2);
            let right_half = stone % 10u64.pow(digit_count / 2);

            evolve_stone(left_half, blinks - 1, memoization_table)
                + evolve_stone(right_half, blinks - 1, memoization_table)
        } else {
            // Rule 3: Multiply the stone by 2024 if no other rules apply.
            evolve_stone(stone * 2024, blinks - 1, memoization_table)
        }
    };

    // Cache the result for this stone and blink count.
    memoization_table.insert((stone, blinks), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_stones_sample() {
        let stones: Vec<u64> = "125 17"
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();

        assert_eq!(calculate_total_stones(&stones, 25), 55312);
    }

    #[test]
    fn test_calculate_total_stones_challenge() {
        let challenge_input: &str = include_str!("../../docs/challenge_1.txt");
        let stones: Vec<u64> = challenge_input
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect();

        assert_eq!(calculate_total_stones(&stones, 25), 209412);
    }
}
