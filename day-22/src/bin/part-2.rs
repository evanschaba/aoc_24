use std::{
    collections::HashMap,
    iter::once,
    sync::{Arc, Mutex},
    thread,
};

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

/// Find the sequence of price changes to maximize sales (faster than the original single threaded fn)
pub fn solve(input: &str) -> Result<isize, Box<dyn std::error::Error>> {
    let lines = input
        .lines() // Split input into lines
        .map(|line| line.to_owned()) // Clone each line into a string
        .collect::<Vec<String>>(); // Collect lines into a vector
    let shared_counts = Arc::new(Mutex::new(HashMap::new())); // Shared hashmap to store sequences and counts

    let num_threads = std::thread::available_parallelism()?.get().max(1); // Determine the number of threads
    let chunk_size = (lines.len() / num_threads).max(1); // Calculate the chunk size, ensuring it is at least 1
    let chunks = lines.chunks(chunk_size); // Split the input lines into chunks

    // Spawn threads to process each chunk
    let thread_handles: Vec<_> = chunks
        .map(|chunk| {
            let chunk_clone = chunk.to_owned(); // Clone the chunk for thread processing
            let shared_counts_clone = Arc::clone(&shared_counts); // Clone the shared hashmap reference
            thread::spawn(move || {
                let mut local_counts = HashMap::new(); // Local hashmap for this thread
                for line in chunk_clone {
                    if let Ok(initial_value) = line.parse::<isize>() {
                        for (sequence, value) in compute_sequences(initial_value, 2000) {
                            *local_counts.entry(sequence).or_insert(0) += value; // Update local counts
                        }
                    }
                }
                let mut global_counts = shared_counts_clone.lock().unwrap(); // Lock the shared hashmap
                for (sequence, value) in local_counts {
                    *global_counts.entry(sequence).or_insert(0) += value; // Merge local counts into shared hashmap
                }
            })
        })
        .collect();

    for handle in thread_handles {
        handle.join().unwrap(); // Wait for all threads to complete
    }

    let counts = shared_counts.lock().unwrap(); // Lock the shared hashmap
    let max_value = counts.values().max().ok_or("No maximum value found")?; // Find the maximum value in the hashmap
    Ok(*max_value) // Return the maximum value
}

/// Computes sequences of price changes over n steps and their corresponding values
fn compute_sequences(initial_value: isize, steps: isize) -> HashMap<Vec<isize>, isize> {
    let step = |mut s| -> isize {
        s = ((s * 64) ^ s) % 16_777_216; // Step 1: Multiply, XOR, and prune
        s = ((s / 32) ^ s) % 16_777_216; // Step 2: Divide, XOR, and prune
        ((s * 2048) ^ s) % 16_777_216 // Step 3: Multiply, XOR, and prune
    };

    let prices = once(initial_value % 10) // Start with the initial price
        .chain((0..steps).scan(initial_value, |state, _| {
            let next_value = step(*state); // Compute the next secret number
            *state = next_value; // Update the state
            Some(next_value % 10) // Compute the next price
        }))
        .collect::<Vec<isize>>(); // Collect all prices into a vector

    let price_differences = prices
        .windows(2) // Create sliding windows of size 2
        .map(|window| window[1] - window[0]) // Calculate the difference between consecutive prices
        .collect::<Vec<isize>>(); // Collect differences into a vector

    price_differences
        .windows(4) // Create sliding windows of size 4
        .enumerate() // Enumerate the windows with their indices
        .fold(HashMap::new(), |mut sequences, (index, window)| {
            let sequence_key = window.to_vec(); // Convert the window into a vector
            sequences.entry(sequence_key).or_insert(prices[index + 4]);
            sequences
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_with_example() {
        let input = r#"1
2
3
2024"#;

        let result = solve(input).unwrap();

        assert_eq!(result, 23);
    }

    #[test]
    fn test_with_input() {
        let input = read_file_from_path("docs/challenge_2.txt"); // Load sample input for Part 2
        assert_eq!(solve(&input).unwrap(), 2058); // Verify the result for Part 2
    }
}

fn main() {
    let input = &read_file_from_args(); // Include the input data as a string
    println!("result: {}", solve(input).unwrap()); // Compute and print Part 2 result
}
