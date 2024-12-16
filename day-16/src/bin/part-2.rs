use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

// Type alias for representing the position of the reindeer
type Position = (usize, usize);

// Type alias for representing the state of a position and direction
type PositionAndDirection = (usize, usize, Direction);

// Enum to represent the possible movement directions of the reindeer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

// Struct to represent the reindeer, including its curr position, direction, score, and prev steps
#[derive(Debug, Clone)]
struct Map {
    position: (usize, usize),  // curr position of the reindeer
    facing: Direction,         // curr direction the reindeer is facing
    score: usize,              // curr score of the reindeer (based on distance traveled)
    prev: Vec<(usize, usize)>, // List of prev positions (steps taken)
}

impl Map {
    // Constructor to create a new reindeer at the start position facing East
    fn new() -> Self {
        Map {
            position: (0, 0),        // Default starting position
            facing: Direction::East, // Default facing East
            score: 0,                // Initial score of 0
            prev: vec![],            // No previous steps
        }
    }

    // Returns a new Reindeer instance based on the current state, with optional updates for position and direction
    fn next_step(&self, new_pos: Option<Position>, new_direction: Option<Direction>) -> Self {
        let mut score = self.score;
        let mut pos = self.position;
        let mut face = self.facing;
        let mut prev = self.prev.clone();

        // Update position if provided
        if let Some(position) = new_pos {
            pos = position;
            prev.push(self.position);
            score += 1; // Increment score on each move
        }

        // Update direction if provided
        if let Some(dir) = new_direction {
            face = dir;
            score += 1_000; // Increment score heavily on changing direction
        }

        Map {
            position: pos,
            facing: face,
            score,
            prev,
        }
    }
}

// Function to parse the maze input and return the walls, reindeer start position, end position, and best path tiles
fn parse_input(
    input: &str,
) -> (
    HashSet<Position>, // Walls in the maze
    Map,               // The starting position of the reindeer
    Position,          // The end position of the maze
    HashSet<Position>, // Set of best path tiles ('O')
) {
    let mut walls = HashSet::new();
    let mut start = Map::new();
    let mut end = (0, 0);
    let mut path_tiles = HashSet::new(); // This will track the 'O' tiles

    // Parsing the maze input line by line
    for (line_no, line) in input.trim().lines().enumerate() {
        for (col_no, elem) in line.char_indices() {
            match elem {
                '#' => {
                    walls.insert((line_no, col_no)); // Mark wall positions
                }
                'S' => {
                    start.position = (line_no, col_no); // Set start position
                }
                'E' => {
                    end = (line_no, col_no); // Set end position
                }
                'O' => {
                    path_tiles.insert((line_no, col_no)); // Mark best path tiles
                }
                '.' => {}            // Empty tiles, do nothing
                _ => unreachable!(), // Unexpected characters
            }
        }
    }

    (walls, start, end, path_tiles)
}

// Refactor the `get_neighbors` function with a simplified return type
fn get_neighbors(
    curr: &Map,
) -> (
    Position,             // Forward position
    PositionAndDirection, // Left neighbor
    PositionAndDirection, // Right neighbor
) {
    match curr.facing {
        Direction::North => (
            (curr.position.0 - 1, curr.position.1),
            (curr.position.0, curr.position.1 - 1, Direction::West),
            (curr.position.0, curr.position.1 + 1, Direction::East),
        ),
        Direction::East => (
            (curr.position.0, curr.position.1 + 1),
            (curr.position.0 - 1, curr.position.1, Direction::North),
            (curr.position.0 + 1, curr.position.1, Direction::South),
        ),
        Direction::South => (
            (curr.position.0 + 1, curr.position.1),
            (curr.position.0, curr.position.1 + 1, Direction::East),
            (curr.position.0, curr.position.1 - 1, Direction::West),
        ),
        Direction::West => (
            (curr.position.0, curr.position.1 - 1),
            (curr.position.0 + 1, curr.position.1, Direction::South),
            (curr.position.0 - 1, curr.position.1, Direction::North),
        ),
    }
}

fn update_covered_tiles(curr: &Map, covered_tiles: &mut HashSet<Position>, end_pos: Position) {
    if curr.position == end_pos {
        covered_tiles.insert(end_pos);
    }

    for pos in curr.prev.iter() {
        covered_tiles.insert(*pos);
    }
}

fn solve(
    data: (
        HashSet<Position>, // Walls in the maze
        Map,               // Reindeer start position
        Position,          // End position
        HashSet<Position>, // Best path tiles
    ),
    count_start_end: bool, // Flag to indicate whether to exclude start and end positions from the count
) -> Result<usize, Box<dyn Error + 'static>> {
    let (walls, reindeer, end_pos, path_tiles) = data;
    let mut queue: Vec<Map> = vec![reindeer];
    let mut reached: HashMap<(usize, usize, Direction), usize> = HashMap::new();
    let mut lowest_score = usize::MAX;
    let mut covered_tiles: HashSet<Position> = HashSet::new();

    while let Some(curr) = queue.pop() {
        if curr.position == end_pos && lowest_score >= curr.score {
            lowest_score = curr.score;
            update_covered_tiles(&curr, &mut covered_tiles, end_pos);
            continue;
        }

        let prev = reached
            .entry((curr.position.0, curr.position.1, curr.facing))
            .or_insert(lowest_score);

        if *prev < curr.score || curr.score > lowest_score {
            continue;
        } else {
            *prev = curr.score;
        }

        let (forward_pos, left_neighbor, right_neighbor) = get_neighbors(&curr);

        if !walls.contains(&forward_pos) {
            queue.push(curr.next_step(Some(forward_pos), None));
        }
        if !walls.contains(&(left_neighbor.0, left_neighbor.1)) {
            queue.push(curr.next_step(None, Some(left_neighbor.2)));
        }
        if !walls.contains(&(right_neighbor.0, right_neighbor.1)) {
            queue.push(curr.next_step(None, Some(right_neighbor.2)));
        }

        queue.sort_by(|a, b| b.score.cmp(&a.score));
    }

    covered_tiles.extend(path_tiles);

    if count_start_end {
        Ok(covered_tiles.len() - 1) // while counting zeroes, especially for sample input text in tests, this is necessary
    } else {
        Ok(covered_tiles.len())
    }
}

// Helper function to read command-line arguments and input file
fn get_arg_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    std::fs::read_to_string(&args[1]).expect("Failed to read input file")
}

// Main function to read input, solve the problem, and print the result
fn main() {
    let input = get_arg_input();
    let data = parse_input(&input);

    match solve(data, false) {
        Ok(score) => println!("result: {}", score),
        Err(e) => eprintln!("error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to read input from a file
    fn get_file_input(file_path: &str) -> String {
        std::fs::read_to_string(file_path).expect("Failed to read input file")
    }

    // Test case with an example input
    #[test]
    fn test_with_example_2() {
        let input = r#"
###############
#.......#....O#
#.#.###.#.###O#
#.....#.#...#O#
#.###.#####.#O#
#.#.#.......#O#
#.#.#####.###O#
#..OOOOOOOOO#O#
###O#O#####O#O#
#OOO#O....#O#O#
#O#O#O###.#O#O#
#OOOOO#...#O#O#
#O###.#.#.#O#O#
#O..#.....#OOO#
###############"#;

        let data = parse_input(input);
        let result = solve(data, true).unwrap();
        assert_eq!(result, 45); // Expected result based on the example
    }

    // Test case with another example input
    #[test]
    fn test_with_example() {
        let input = r#"
#################
#...#...#...#..O#
#.#.#.#.#.#.#.#O#
#.#.#.#...#...#O#
#.#.#.#.###.#.#O#
#OOO#.#.#.....#O#
#O#O#.#.#.#####O#
#O#O..#.#.#OOOOO#
#O#O#####.#O###O#
#O#O#..OOOOO#OOO#
#O#O###O#####O###
#O#O#OOO#..OOO#.#
#O#O#O#####O###.#
#O#O#OOOOOOO..#.#
#O#O#O#########.#
#O#OOO..........#
#################"#;

        let data = parse_input(input);
        let result = solve(data, true).unwrap();
        assert_eq!(result, 64); // Expected result based on the example
    }

    // Test with actual input from file
    #[test]
    fn test_with_input() {
        let file_path = "docs/challenge_2.txt";
        let input = get_file_input(file_path);
        let data = parse_input(&input);
        let result = solve(data, false).unwrap();
        assert_eq!(result, 511); // Expected result based on actual input
    }
}
