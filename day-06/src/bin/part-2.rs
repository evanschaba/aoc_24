use std::collections::HashSet;
use std::env;
use std::fs;

/// Main function that reads an input file, parses it into a grid representation, and calculates the number of valid positions for an obstruction.
fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return;
    }

    // Extract the input file path from command line arguments
    let file_path = &args[1];

    // Read the file content into a string, handling errors if reading fails
    let content = fs::read_to_string(file_path).expect("Failed to read input file");

    // Convert the content into a grid representation where each character is a cell in the grid
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    // Print the output of the `calculate_path` function, which returns the number of positions that create a loop
    println!("Result: {}", calculate_path(&grid));
}

/// Calculates the number of positions where adding an obstruction would cause the guard to get stuck in a loop.
///
/// # Parameters
/// - `grid`: A 2D vector representing the map, where each character indicates an obstacle (`#`), open path (`.`), or starting point (`^`).
///
/// # Returns
/// - The number of valid positions where an obstruction can be added to create a loop.
fn calculate_path(grid: &[Vec<char>]) -> usize {
    // Find the starting position of the guard in the grid
    let start_pos = find_start_position(grid);

    // Traverse the path of the guard and count valid positions that would cause a loop
    trace_path(grid)
        .iter()
        .filter(|&&pos| {
            // Exclude the starting position and check if placing an obstruction here forms a loop
            pos != start_pos && will_form_loop(grid, start_pos, pos)
        })
        .count()
}

/// Determines if placing an obstruction at a given position will cause the guard to get stuck in a loop.
///
/// # Parameters
/// - `grid`: The 2D grid representing the lab layout.
/// - `start_pos`: The starting position of the guard.
/// - `current_pos`: The position where an obstruction is being tested.
///
/// # Returns
/// - `true` if placing an obstruction at `current_pos` forms a loop, `false` otherwise.
fn will_form_loop(
    grid: &[Vec<char>],
    start_pos: (usize, usize),
    current_pos: (usize, usize),
) -> bool {
    // Set to track turns the guard has made to detect loops
    let mut visited_turns = HashSet::new();
    let mut pos = start_pos;
    let mut direction = (-1, 0); // Initial direction facing up

    'outer_loop: loop {
        'inner_loop: for _ in 0..2 {
            // Attempt to move twice at each step (simulating the guard's path)
            if let Some(next_pos) = move_position(grid, pos, direction) {
                if grid[next_pos.0][next_pos.1] == '#' || next_pos == current_pos {
                    // Record the turn if an obstruction or edge is encountered
                    let turn_record = (pos, direction);
                    if visited_turns.contains(&turn_record) {
                        return true; // Loop detected
                    }
                    visited_turns.insert(turn_record);

                    // Rotate direction clockwise and retry
                    direction = rotate_direction(direction);
                    continue 'inner_loop;
                } else {
                    pos = next_pos;
                }
            } else {
                // Stop if the position is out of bounds
                break 'outer_loop;
            }
        }
    }

    false // No loop detected
}

/// Traverses the grid to record all positions visited by the guard.
///
/// # Parameters
/// - `grid`: The 2D grid representing the lab layout.
///
/// # Returns
/// - A set of positions that the guard visits.
fn trace_path(grid: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut visited_positions = HashSet::new();
    let start_pos = find_start_position(grid);
    visited_positions.insert(start_pos);

    let mut current_pos = start_pos;
    let mut direction = (-1, 0); // Initial direction facing up

    'path_loop: loop {
        for _ in 0..2 {
            if let Some(next_pos) = move_position(grid, current_pos, direction) {
                if grid[next_pos.0][next_pos.1] == '#' {
                    // Change direction if an obstacle is encountered
                    direction = rotate_direction(direction);
                } else {
                    current_pos = next_pos;
                    visited_positions.insert(current_pos);
                }
            } else {
                // Stop if out of bounds
                break 'path_loop;
            }
        }
    }

    visited_positions
}

/// Finds the starting position of the guard in the grid.
///
/// # Parameters
/// - `grid`: The 2D grid to search for the starting position.
///
/// # Returns
/// - A tuple representing the row and column of the starting position.
///
/// # Panics
/// - Panics if the starting position is not found.
fn find_start_position(grid: &[Vec<char>]) -> (usize, usize) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '^' {
                return (row, col);
            }
        }
    }

    unreachable!("Start position not found in the grid.");
}

/// Rotates a direction 90 degrees clockwise.
///
/// # Parameters
/// - `direction`: A tuple representing the current direction.
///
/// # Returns
/// - A tuple representing the rotated direction.
fn rotate_direction(direction: (i32, i32)) -> (i32, i32) {
    (direction.1, -direction.0)
}

/// Moves the guard's position based on the current direction and checks for grid boundaries.
///
/// # Parameters
/// - `grid`: The 2D grid representing the map.
/// - `pos`: The current position of the guard.
/// - `direction`: The direction the guard is moving in.
///
/// # Returns
/// - `Some(new_pos)` if the move is valid.
/// - `None` if the move goes out of bounds.
fn move_position(
    grid: &[Vec<char>],
    pos: (usize, usize),
    direction: (i32, i32),
) -> Option<(usize, usize)> {
    if pos.0 == 0 && direction.0 < 0 {
        return None;
    }
    if pos.1 == 0 && direction.1 < 0 {
        return None;
    }

    let new_pos = (
        ((pos.0 as i32) + direction.0) as usize,
        ((pos.1 as i32) + direction.1) as usize,
    );

    if new_pos.0 >= grid.len() || new_pos.1 >= grid[0].len() {
        return None;
    }

    Some(new_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to read the content of a file.
    fn read_test_input(file_path: &str) -> String {
        fs::read_to_string(file_path).expect("Failed to read test input file")
    }

    /// Helper function to parse the input string into a grid.
    fn parse_input(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }

    #[test]
    fn test_challenge_2_input() {
        let input = read_test_input("docs/challenge_2.txt");
        assert_eq!(calculate_path(&parse_input(&input)), 1972);
    }
}
