use std::collections::{HashMap, VecDeque}; // Import HashMap and VecDeque from the standard library
use std::rc::Rc; // Import Rc for reference-counted pointers

pub const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)]; // Define movement directions (right, down, left, up)

type CursorT = Vec<u8>;
type KeypadT = [[u8; 3]];
type PathsT = Vec<Vec<u8>>;
type PathsCacheT = Rc<Vec<Vec<u8>>>;
type PositionT = (usize, usize);

const NUMERIC_KEYPAD: [[u8; 3]; 4] = [
    // Numeric keypad layout (3x4 grid)
    [b'7', b'8', b'9'],
    [b'4', b'5', b'6'],
    [b'1', b'2', b'3'],
    [b' ', b'0', b'A'],
];

const DIRECTIONAL_KEYPAD: [[u8; 3]; 2] = [
    // Directional keypad layout (2x3 grid)
    [b' ', b'^', b'A'],
    [b'<', b'v', b'>'],
];

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

/// Reads the input file from a given file path
pub fn read_file_from_path(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read input file") // Read file content as string
}

/// Finds positions of the start and end keys in the keypad
fn find_key_positions(keypad: &KeypadT, start_key: u8, end_key: u8) -> (PositionT, PositionT) {
    let mut start_pos = (0, 0); // Initialize start position
    let mut end_pos = (0, 0); // Initialize end position

    for (y, row) in keypad.iter().enumerate() {
        // Iterate over keypad rows
        for (x, &key) in row.iter().enumerate() {
            // Iterate over keys in each row
            if key == start_key {
                // If the current key matches the start key
                start_pos = (x, y); // Save the start key's position
            }
            if key == end_key {
                // If the current key matches the end key
                end_pos = (x, y); // Save the end key's position
            }
        }
    }
    (start_pos, end_pos) // Return both positions as a tuple
}

/// Performs a BFS to find the shortest paths between two keys
fn bfs_find_paths(keypad: &KeypadT, start_pos: PositionT, end_pos: PositionT) -> PathsT {
    let mut distances = vec![vec![usize::MAX; 3]; keypad.len()]; // Initialize distance matrix with MAX distances
    let mut queue = VecDeque::new(); // Create a queue for BFS
    queue.push_back((start_pos.0, start_pos.1, 0)); // Add the starting position to the queue

    while let Some((x, y, steps)) = queue.pop_front() {
        // Process the next position from the queue
        if distances[y][x] == usize::MAX {
            // If the position has not been visited
            distances[y][x] = steps; // Set the distance to the number of steps taken
        }

        for &(dx, dy) in &DIRECTIONS {
            // Check all possible directions
            let nx = (x as i32 + dx) as usize; // Calculate new x position
            let ny = (y as i32 + dy) as usize; // Calculate new y position
            if nx < 3
                && ny < keypad.len()
                && keypad[ny][nx] != b' '
                && distances[ny][nx] == usize::MAX
            {
                // If within bounds and not a wall
                queue.push_back((nx, ny, steps + 1)); // Add the new position to the queue
            }
        }
    }

    let mut paths = Vec::new() as PathsT; // Vector to store all possible paths
    let mut stack = Vec::new(); // Stack to backtrack through the BFS results
    stack.push((end_pos.0, end_pos.1, vec![b'A'])); // Start backtracking from the end key

    while let Some((x, y, path)) = stack.pop() {
        // Process the stack while it's not empty
        if x == start_pos.0 && y == start_pos.1 {
            // If the start position is reached
            paths.push(path); // Add the current path to the list of paths
            continue; // Continue to the next iteration
        }

        for (i, &(dx, dy)) in DIRECTIONS.iter().enumerate() {
            // Check all possible directions
            let nx = (x as i32 + dx) as usize; // Calculate new x position
            let ny = (y as i32 + dy) as usize; // Calculate new y position
            if nx < 3 && ny < keypad.len() && distances[ny][nx] < distances[y][x] {
                // If the position is closer to the start
                let direction = match i {
                    // Determine the direction based on index
                    0 => b'<', // Left
                    1 => b'^', // Up
                    2 => b'>', // Right
                    3 => b'v', // Down
                    _ => unreachable!(),
                };
                let mut new_path = vec![direction]; // Create a new path with the direction
                new_path.extend(&path); // Add the current path to the new path
                stack.push((nx, ny, new_path)); // Add the new position and path to the stack
            }
        }
    }

    paths as PathsT // Return all found paths
}

/// Returns the shortest paths between two keys with caching
fn find_shortest_paths(
    keypad: &KeypadT,
    start_key: u8,
    end_key: u8,
    path_cache: &mut HashMap<(u8, u8), PathsCacheT>,
) -> PathsCacheT {
    if let Some(cached) = path_cache.get(&(start_key, end_key)) {
        // Check if the result is cached
        return cached.clone(); // Return the cached result
    }

    let (start_pos, end_pos) = find_key_positions(keypad, start_key, end_key); // Find positions of start and end keys
    let paths = bfs_find_paths(keypad, start_pos, end_pos); // Find the shortest paths using BFS

    let result = Rc::new(paths); // Create a reference-counted result
    path_cache.insert((start_key, end_key), result.clone()); // Cache the result
    result // Return the result
}

/// Computes the complexity for a sequence of codes
fn calculate_sequence_complexity(
    code: &[u8],
    depth: usize,
    is_numeric: bool,
    cursors: &mut CursorT,
    sequence_cache: &mut HashMap<(Vec<u8>, usize, u8), usize>,
    path_cache: &mut HashMap<(u8, u8), PathsCacheT>,
) -> usize {
    let cache_key = (code.to_vec(), depth, cursors[depth]); // Generate a cache key based on the code, depth, and cursor position
    if let Some(&cached) = sequence_cache.get(&cache_key) {
        // If the result is cached
        return cached; // Return the cached result
    }

    let mut result = 0; // Initialize the result to 0
    for &key in code {
        // Iterate over the code
        let paths = find_shortest_paths(
            if is_numeric {
                &NUMERIC_KEYPAD
            } else {
                &DIRECTIONAL_KEYPAD
            }, // Select the appropriate keypad based on whether it's numeric
            cursors[depth], // Current cursor position
            key,            // Target key
            path_cache,     // Path cache
        );

        if depth == 0 {
            // If at the top depth level
            result += paths[0].len(); // Add the length of the first path
        } else {
            result += paths
                .iter()
                .map(|p| {
                    // For each path, calculate the minimum sequence complexity
                    calculate_sequence_complexity(
                        p,              // Path
                        depth - 1,      // Decrease depth
                        false,   // Set to false since we're no longer working with numeric keys
                        cursors, // Cursor positions
                        sequence_cache, // Sequence cache
                        path_cache, // Path cache
                    )
                })
                .min()
                .unwrap(); // Get the minimum value and add it to the result
        }

        cursors[depth] = key; // Update the cursor position
    }

    sequence_cache.insert(cache_key, result); // Cache the result
    result // Return the result
}

/// Computes the sum of complexities for all lines in the input
fn compute_complexity_sum(input: &str, max_depth: usize) -> usize {
    let lines = input.lines().collect::<Vec<_>>(); // Split input into lines
    let mut sequence_cache = HashMap::new(); // Initialize sequence cache
    let mut cache = HashMap::new(); // Initialize path cache

    let mut result = 0; // Initialize the result
    for line in &lines {
        // Iterate over each line
        let mut cursors = vec![b'A'; max_depth + 1]; // Initialize cursors for all depths
        let sequence_length = calculate_sequence_complexity(
            // Calculate the sequence length
            line.as_bytes(),     // Convert the line to bytes
            max_depth,           // Maximum depth
            true,                // Use numeric keypad
            &mut cursors,        // Pass cursors for depth tracking
            &mut sequence_cache, // Pass the sequence cache
            &mut cache,          // Pass the path cache
        );

        let numeric_part = line[0..3].parse::<usize>().unwrap(); // Parse the numeric part of the line
        result += numeric_part * sequence_length; // Add the complexity to the result
    }

    result // Return the total complexity
}

fn solve(input: &str) -> usize {
    compute_complexity_sum(input, 25)
}

fn main() {
    let input = read_file_from_args(); // Read the input file
    println!("result: {}", solve(&input));
}

#[cfg(test)] // Tests are only included when the module is compiled for testing
mod tests {
    use super::*; // Import all functions from the main code

    #[test]
    fn test_with_example_2() {
        const EXAMPLE: &str = r#"029A
980A
179A
456A
379A"#;

        let result = solve(EXAMPLE); // Compute complexity for part 2
        assert_eq!(result, 154115708116294, "Example test case failed");
    }

    #[test]
    fn test_with_file_input_2() {
        let input = read_file_from_path("docs/challenge_2.txt"); // Read input from file
        let result = solve(&input); // Compute complexity for part 2
        assert_eq!(result, 242337182910752); // Assert the result matches the expected value
    }
}
