/// Reads the input file from command-line arguments
pub fn read_file_from_args() -> String {
    let args: Vec<String> = std::env::args().collect(); // Collect command-line arguments into a vector
    if args.len() < 2 {
        // Check if the file path is provided
        eprintln!("Usage: {} <input_file>", args[0]); // Print usage if the file argument is missing
        std::process::exit(1); // Exit if the argument is missing
    }

    std::fs::read_to_string(&args[1]).expect("Failed to read input file") // Read file content as string
}

/// Reads the input file from a specified file path
pub fn read_file_from_path(path: &str) -> String {
    std::fs::read_to_string(path).expect("Failed to read input file") // Read file content as string
}

/// Calculate the sum of the 2000th secret number for all initial values
pub fn solve(input: &str) -> isize {
    input
        .lines() // Iterate over each line in the input
        .filter_map(|line| line.parse::<isize>().ok()) // Parse each line into a number, ignoring invalid lines
        .map(|initial_value| calculate_nth_secret(initial_value, 2000)) // Compute the 2000th secret number
        .sum::<isize>() // Sum all computed values
}

/// Computes the nth secret number for a given initial value
fn calculate_nth_secret(initial_value: isize, steps: isize) -> isize {
    let calc_nth_secret = |mut secret_value: isize| -> isize {
        secret_value = ((secret_value * 64) ^ secret_value) % 16_777_216; // Step 1: Multiply, XOR, and prune
        secret_value = ((secret_value / 32) ^ secret_value) % 16_777_216; // Step 2: Divide, XOR, and prune
        ((secret_value * 2048) ^ secret_value) % 16_777_216 // Step 3: Multiply, XOR, and prune
    };
    (0..steps).fold(initial_value, |current, _| calc_nth_secret(current)) // Apply the evolution function repeatedly
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_with_example() {
        let input = r#"1
10
100
2024"#;

        let result = solve(input);

        assert_eq!(result, 37327623);
    }

    #[test]
    fn test_with_input() {
        let input = read_file_from_path("docs/challenge_1.txt"); // Load sample input for Part 1
        assert_eq!(solve(&input), 19241711734); // Verify the result for Part 1
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = &read_file_from_args(); // Include the input data as a string
    println!("result {}", solve(input)); // Compute and print Part 1 result
    Ok(())
}
