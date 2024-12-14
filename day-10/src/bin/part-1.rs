use std::collections::HashMap;

/// Solves the hiking trail problem.
///
/// # Arguments
/// * `input` - A string representing the topographic map.
///
/// # Returns
/// * `Option<u32>` - The sum of the scores of all trailheads.
pub fn calculate_trailhead_scores(input: &str) -> Option<u32> {
    // Convert the input string into a grid of integers representing the topographic map.
    let grid: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap()) // Convert each character to a digit.
                .collect()
        })
        .collect();

    // Identify all trailheads (positions with height 0).
    let trailheads: Vec<(usize, usize)> = grid
        .iter()
        .enumerate() // Enumerate to get the row index.
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, &value)| {
                if value == 0 {
                    // Check if the current position has a height of 0.
                    Some((row_idx, col_idx)) // Return the coordinates if it's a trailhead.
                } else {
                    None
                }
            })
        })
        .collect();

    // Initialize the total score for all trailheads.
    let mut total_score = 0;

    // Traverse from each trailhead and calculate scores.
    for (start_x, start_y) in trailheads {
        // Stack to help with depth-first traversal (DFS) of the map.
        let mut stack: Vec<(usize, usize)> = vec![(start_x, start_y)];
        // HashMap to track visited positions to prevent revisiting.
        let mut visited_positions: HashMap<(usize, usize), bool> = HashMap::new();

        // Traverse the map using DFS.
        while let Some((x, y)) = stack.pop() {
            let current_height = grid[x][y];

            // If the current position's height is 9, mark it as visited and continue.
            if current_height == 9 {
                visited_positions.insert((x, y), true);
                continue;
            }

            // Check adjacent positions and push them to the stack if they are one height step up.
            if x > 0 && grid[x - 1][y] == current_height + 1 {
                stack.push((x - 1, y));
            }
            if y > 0 && grid[x][y - 1] == current_height + 1 {
                stack.push((x, y - 1));
            }
            if x + 1 < grid.len() && grid[x + 1][y] == current_height + 1 {
                stack.push((x + 1, y));
            }
            if y + 1 < grid[0].len() && grid[x][y + 1] == current_height + 1 {
                stack.push((x, y + 1));
            }
        }

        // Count the number of visited positions that have a height of 9 and add to the total score.
        total_score += visited_positions.len() as u32;
    }

    // Return the total score as an Option<u32>.
    Some(total_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reads input from a file for testing purposes.
    fn read_input(file_name: &str) -> String {
        std::fs::read_to_string(file_name).expect("Failed to read input file")
    }

    #[test]
    fn test_example() {
        // Test input provided as part of the problem example.
        let input = vec![
            "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
            "10456732",
        ]
        .join("\n");

        // Check if the function calculates the correct score.
        assert_eq!(calculate_trailhead_scores(&input), Some(36));
    }

    #[test]
    fn test_with_input_file() {
        // Read input from an actual file for testing.
        let input = read_input("docs/challenge_1.txt");
        // Assert that the function returns the correct result based on the file.
        assert_eq!(calculate_trailhead_scores(&input), Some(587)); // Update the expected value if needed.
    }
}

fn main() {
    // Collect command-line arguments, expecting the first argument to be the input file name.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    // Read the content of the input file.
    let input = std::fs::read_to_string(&args[1]).expect("Failed to read input file");

    // Call the main function and handle the result.
    match calculate_trailhead_scores(&input) {
        Some(result) => println!("Result: {}", result),
        None => eprintln!("No valid result computed"),
    }
}
