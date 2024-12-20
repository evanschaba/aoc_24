use std::collections::{HashMap, HashSet, VecDeque};

type Coord = (i32, i32);  // Represents a coordinate (x, y) on the grid

const DURATION: i32 = 2;  // Duration of the cheat period in picoseconds

#[derive(Debug)]
pub struct Race {
   pub obstacles: HashSet<Coord>,  // Set of coordinates representing obstacles (walls)
   pub start: Coord,  // Starting point of the race
   pub end: Coord,    // Ending point of the race
}

/// Reads the input file from the command-line arguments.
/// Returns the content of the file as a `String`.
pub fn read_file_from_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    std::fs::read_to_string(&args[1]).expect("Failed to read input file")
}

/// Reads the input file directly from the provided path.
/// Returns the content of the file as a `String`.
pub fn read_file_from_path(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read input file")
}

/// Parses the input map into a `Race` structure containing obstacles, start, and end points.
fn parse_race_map(input: &str) -> Race {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();

    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, tile)| match tile {
            '#' => {
                obstacles.insert((x as i32, y as i32));  // Add wall coordinates to obstacles set
            }
            'S' => {
                start = (x as i32, y as i32);  // Mark the starting point
            }
            'E' => {
                end = (x as i32, y as i32);  // Mark the ending point
            }
            _ => (),  // Ignore empty spaces and other tiles
        })
    });

    Race {
        obstacles,
        start,
        end,
    }
}

/// Performs a Breadth-First Search (BFS) to compute the shortest path distances from the start position.
fn bfs_shortest_path(race: &Race) -> HashMap<Coord, i32> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    
    // Initialize the queue with the start position and distance 0
    queue.push_back((race.start, 0));
    distances.insert(race.start, 0);

    // Perform BFS to calculate the shortest distance to each point
    while let Some((coord, cost)) = queue.pop_front() {
        for next_coord in [
            (coord.0 - 1, coord.1),  // Move up
            (coord.0 + 1, coord.1),  // Move down
            (coord.0, coord.1 - 1),  // Move left
            (coord.0, coord.1 + 1),  // Move right
        ] {
            // If the coordinate is not visited and not an obstacle, add it to the queue
            if !distances.contains_key(&next_coord) && !race.obstacles.contains(&next_coord) {
                distances.insert(next_coord, cost + 1);  // Update distance to the new point
                queue.push_back((next_coord, cost + 1));  // Add new position to queue
            }
        }
    }

    distances
}

/// Counts the number of deprecated "cheats" that save at least the specified number of picoseconds.
fn count_cheats_saving_at_least(race: &Race, saved: i32) -> usize {
    let path = bfs_shortest_path(race);
    let mut result = 0;

    // For each coordinate in the shortest path, check if cheating can save the specified number of picoseconds
    for (coord, cost) in path.iter() {
        for next_coord in [
            (coord.0 - 2, coord.1),  // Check 2 steps left
            (coord.0 + 2, coord.1),  // Check 2 steps right
            (coord.0, coord.1 - 2),  // Check 2 steps up
            (coord.0, coord.1 + 2),  // Check 2 steps down
        ] {
            if let Some(next_cost) = path.get(&next_coord) {
                if next_cost - cost >= saved + DURATION {
                    result += 1;  // If the savings are sufficient, count it as a valid cheat
                }
            }
        }
    }

    result
}

/// Solves the problem by finding the number of cheats that save at least the specified number of picoseconds.
fn solve(input: &str) -> usize {
    let race = &parse_race_map(input);
    count_cheats_saving_at_least(race, 100)  // Looking for cheats that save at least 100 picoseconds
}

fn main() {
    let input = read_file_from_args();  // Read input file from command-line argument

    let result = solve(&input);  // Solve the puzzle
    println!("result: {}", result);  // Print the result
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_with_example() {
        // Test case with example map
        assert_eq!(count_cheats_saving_at_least(&parse_race_map(EXAMPLE), 64), 1);
    }

    #[test]
    fn test_with_input() {
        let input = read_file_from_path("docs/challenge_1.txt");  // Read input file
        assert_eq!(count_cheats_saving_at_least(&parse_race_map(&input), 100), 1381);
    }
}
