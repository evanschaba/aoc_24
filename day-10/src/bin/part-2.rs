// Function to parse the input string and return the coordinates of trailheads.
fn parse(input: &str) -> Vec<(usize, usize)> {
    let grid = build_grid(input); // Build a grid from the input string.
    extract_trailheads(&grid) // Extract the trailhead coordinates from the grid.
}

// Function to convert the input string into a 2D vector of integers representing the map.
fn build_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .trim() // Trim any leading or trailing whitespace.
        .lines() // Split the input into lines.
        .map(|line| {
            line.trim() // Trim each line to remove extra spaces.
                .chars() // Convert each line into characters.
                .map(|c| c.to_digit(10).unwrap()) // Convert each character to a digit (u32).
                .collect() // Collect the characters into a vector.
        })
        .collect() // Collect the rows into a 2D vector.
}

// Function to extract trailheads from the grid. A trailhead is defined as any '0' in the grid.
fn extract_trailheads(grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    grid
        .iter() // Iterate over each row of the grid.
        .enumerate() // Enumerate to get the row index.
        .flat_map(|(row_id, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, &col)| {
                if col == 0 { // Check if the current cell is a trailhead (value 0).
                    Some((row_id, col_idx)) // Return the coordinates of the trailhead.
                } else {
                    None // Return None if the current cell is not a trailhead.
                }
            })
        })
        .collect() // Collect the trailhead coordinates into a vector.
}

// Function to solve the problem by calculating the sum of ratings for all trailheads.
fn solve(input: &str) -> Option<u32> {
    let grid = build_grid(input); // Build the grid from input.
    let trailheads = parse(input); // Get the trailhead coordinates.
    calculate_ratings(&grid, &trailheads) // Calculate the sum of ratings.
}

// Function to calculate the sum of ratings for each trailhead.
fn calculate_ratings(grid: &Vec<Vec<u32>>, trailheads: &[(usize, usize)]) -> Option<u32> {
    let mut res = 0; // Initialize the result to 0.

    for &(x, y) in trailheads {
        res += walk(grid, x, y); // Accumulate the count of trails for each trailhead.
    }

    Some(res) // Return the total sum of ratings.
}

// Function to perform a depth-first search (DFS) from a given starting point to count trails.
fn walk(grid: &Vec<Vec<u32>>, start_x: usize, start_y: usize) -> u32 {
    let mut stack = vec![(start_x, start_y)]; // Stack for DFS, initialized with the starting position.
    let mut count = 0; // Counter for the number of trails.

    // DFS loop to traverse the grid.
    while let Some((cur_x, cur_y)) = stack.pop() {
        let cur_val = grid[cur_x][cur_y]; // Current value at the position.
        if cur_val == 9 { // If the value is 9, we found a trail endpoint.
            count += 1; // Increment the count.
            continue; // Continue to the next iteration.
        }

        // Check and add neighboring cells to the stack if they have the next incrementing value.
        if cur_x > 0 && grid[cur_x - 1][cur_y] == cur_val + 1 {
            stack.push((cur_x - 1, cur_y));
        }
        if cur_y > 0 && grid[cur_x][cur_y - 1] == cur_val + 1 {
            stack.push((cur_x, cur_y - 1));
        }
        if cur_x + 1 < grid.len() && grid[cur_x + 1][cur_y] == cur_val + 1 {
            stack.push((cur_x + 1, cur_y));
        }
        if cur_y + 1 < grid[0].len() && grid[cur_x][cur_y + 1] == cur_val + 1 {
            stack.push((cur_x, cur_y + 1));
        }
    }

    count // Return the total number of trails found starting from the given position.
}

// Unit tests for the solution.
#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to read the input from a file.
    fn get_input(file_name: &str) -> String {
        let input = std::fs::read_to_string(file_name).expect("Failed to read input file");
        input
    }

    // Test case for the example provided in the problem statement.
    #[test]
    fn test_example() {
        let txt = vec![
            "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
            "10456732",
        ]
        .join("\n");
        assert_eq!((solve(&txt)), Some(81)); // Assert that the result is 81.
    }

    // Test case with input read from an external file.
    #[test]
    fn test_with_input() {
        let result = solve(&get_input("docs/challenge_2.txt")); // Read input from a file.
        assert_eq!(result, Some(1340)); // Assert that the result matches the expected value.
    }
}

// Main function for running the program.
fn main() {
    let args: Vec<String> = std::env::args().collect(); // Collect command-line arguments.
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]); // Print usage message if input file is missing.
        std::process::exit(1); // Exit with an error code.
    }

    // Read input from the specified file.
    let input = std::fs::read_to_string(&args[1]).expect("Failed to read input file");

    // Solve the problem and print the result.
    let result = solve(&input).expect("invalid result");

    println!("result: {}", result); // Print the final result.
}
