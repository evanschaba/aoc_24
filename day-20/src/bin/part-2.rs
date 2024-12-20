use std::collections::{HashSet, VecDeque};

type Coord = (i32, i32);

const DURATION: i32 = 20;  // Maximum duration allowed for a cheat in picoseconds.

#[derive(Debug)]
pub struct Race {
    pub obstacles: HashSet<Coord>,
    pub start: Coord,
    pub end: Coord,
}

/// Pathfinding GOLD 🔱 SO cool 😎: https://docs.rs/pathfinding/latest/pathfinding/ 

/// Reads the input file provided as a command-line argument.
/// Returns the content of the file as a `String`.
pub fn read_file_from_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    std::fs::read_to_string(&args[1]).expect("Failed to read input file")
}

/// Reads the input file directly using the provided path.
/// Returns the content of the file as a `String`.
pub fn read_file_from_path(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read input file")
}

fn parse_race_map(input: &str) -> Race {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();

    // Parse the race map and identify the positions of obstacles, start, and end.
    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, tile)| match tile {
            '#' => {
                obstacles.insert((x as i32, y as i32));
            }
            'S' => {
                start = (x as i32, y as i32);
            }
            'E' => {
                end = (x as i32, y as i32);
            }
            _ => (),
        })
    });

    Race {
        obstacles,
        start,
        end,
    }
}

/// Performs a Breadth-First Search (BFS) to find the shortest paths and distances from the start.
fn bfs_find_distances(race: &Race) -> HashSet<(Coord, i32)> {
    let mut queue = VecDeque::from([(race.start, 0)]);
    let mut visited = HashSet::new();
    visited.insert(race.start);

    let mut distances = HashSet::new();

    // Process each position in the queue and calculate the distance to neighboring positions.
    while let Some((coord, cost)) = queue.pop_front() {
        distances.insert((coord, cost));

        // Check adjacent coordinates (up, down, left, right)
        for next_coord in [
            (coord.0 - 1, coord.1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1 - 1),
            (coord.0, coord.1 + 1),
        ] {
            // Only visit unvisited positions that are not obstacles.
            if !visited.contains(&next_coord) && !race.obstacles.contains(&next_coord) {
                visited.insert(next_coord);
                queue.push_back((next_coord, cost + 1));
            }
        }
    }

    distances
}

/// Counts the number of valid shortcuts that save at least a certain number of steps.
/// A valid shortcut is one where the distance between two points is less than or equal to MAX_CHEAT_DURATION,
/// and the cost difference is greater than or equal to the minimum saved steps.
fn count_valid_shortcuts(race: &Race, min_saved_steps: i32) -> usize {
    let distances = bfs_find_distances(race);
    let mut shortcut_count = 0;

    // Check each pair of points (start and end) to see if a valid shortcut can be made.
    for (start_coord, start_cost) in &distances {
        for (end_coord, end_cost) in &distances {
            let distance = (end_coord.0 - start_coord.0).abs() + (end_coord.1 - start_coord.1).abs();

            // Check if the shortcut is valid: distance should be within the allowed limit, and
            // the cost difference should be enough to save the required number of steps.
            if distance <= DURATION && end_cost - start_cost >= min_saved_steps + distance {
                shortcut_count += 1;
            }
        }
    }

    shortcut_count
}

/// Solves the puzzle by parsing the input, calculating the number of valid shortcuts that save at least
/// a certain number of steps, and returning the result.
fn solve(input: &str) -> usize {
    let race = &parse_race_map(input);
    count_valid_shortcuts(race, 100)
}

/// Main function that reads the input file, solves the puzzle, and prints the result.
fn main() {
    let input = read_file_from_args();

    let result = solve(&input);
    println!("result: {}", result);
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
        assert_eq!(count_valid_shortcuts(&parse_race_map(EXAMPLE), 50), 285);
    }

    #[test]
    fn test_with_input() {
        let input = read_file_from_path("docs/challenge_2.txt");
        assert_eq!(count_valid_shortcuts(&parse_race_map(&input), 100), 982124);
    }
}
