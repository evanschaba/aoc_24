use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

// Struct to represent the reindeer, including its curr position, direction, score, and prev steps
#[derive(Debug, Clone)]
struct Map {
    position: (usize, usize),  // curr position of the reindeer
    facing: Direction,         // curr direction the reindeer is facing
    score: usize,              // curr score of the reindeer (based on distance traveled)
    prev: Vec<(usize, usize)>, // List of prev positions (steps taken)
}

impl Map {
    fn new() -> Self {
        Map {
            position: (0, 0),
            facing: Direction::East,
            score: 0,
            prev: vec![],
        }
    }

    // Returns a new Map instance with updated position and direction
    fn next_step(&self, new_pos: Option<(usize, usize)>, new_direction: Option<Direction>) -> Self {
        let mut score = self.score;
        let mut pos = self.position;
        let mut face = self.facing;
        let mut prev = self.prev.clone();

        // If a new position is provided, update the position and increase score
        if let Some(position) = new_pos {
            pos = position;
            prev.push(self.position);
            score += 1;
        }

        // If a new direction is provided, update the direction and increase score
        if let Some(dir) = new_direction {
            face = dir;
            score += 1_000;
        }

        Map {
            position: pos,
            facing: face,
            score,
            prev,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

// Collect the command-line argument for input file
fn get_arg_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1); // Exit if no input file is provided
    }

    std::fs::read_to_string(&args[1]).expect("Failed to read input file")
}

// Parse input to identify walls, start position, and end position
fn parse_input(input: &str) -> (HashSet<(usize, usize)>, Map, (usize, usize)) {
    let mut walls = HashSet::new();
    let mut start = Map::new();
    let mut end = (0, 0);
    for (line_no, line) in input.trim().lines().enumerate() {
        for (col_no, elem) in line.char_indices() {
            match elem {
                '#' => {
                    walls.insert((line_no, col_no)); // Mark walls
                }
                'S' => {
                    start.position = (line_no, col_no); // Mark start position
                }
                'E' => {
                    end = (line_no, col_no); // Mark end position
                }
                '.' => {}
                _ => unreachable!(),
            }
        }
    }
    (walls, start, end)
}

// Solves the maze and calculates the lowest score to reach the end
fn solve(data: (HashSet<(usize, usize)>, Map, (usize, usize))) -> Result<usize, Box<dyn Error>> {
    let (walls, start_map, end_pos) = data;
    let mut queue = vec![start_map];
    let mut reached: HashMap<(usize, usize, Direction), usize> = HashMap::new();

    while let Some(curr) = queue.pop() {
        // If the current position matches the end, return the score
        if curr.position == end_pos {
            println!("The lowest score the map can get is: {}", curr.score);
            return Ok(curr.score);
        }

        // Skip positions that have already been reached with a lower score
        let prev = reached
            .entry((curr.position.0, curr.position.1, curr.facing))
            .or_insert(usize::MAX);

        if *prev < curr.score {
            continue;
        } else {
            *prev = curr.score;
        }

        // Get the next possible positions and directions
        let (forward_pos, left_neighbor, right_neighbor) = match curr.facing {
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
        };

        let mut added = false;

        // Add valid steps to the queue
        if !walls.contains(&forward_pos) {
            queue.push(curr.next_step(Some(forward_pos), None));
            added = true;
        }
        if !walls.contains(&(left_neighbor.0, left_neighbor.1)) {
            queue.push(curr.next_step(None, Some(left_neighbor.2)));
            added = true;
        }
        if !walls.contains(&(right_neighbor.0, right_neighbor.1)) {
            queue.push(curr.next_step(None, Some(right_neighbor.2)));
            added = true;
        }

        // If no new positions were added, return early
        if !added && queue.is_empty() {
            return Err("No path found!".into());
        }

        // Sort the queue by score (higher score first)
        queue.sort_by(|a, b| b.score.cmp(&a.score));
    }

    Err("No path found!".into())
}

fn main() {
    let input = get_arg_input();
    let data = parse_input(&input);

    match solve(data) {
        Ok(score) => println!("result: {}", score),
        Err(e) => eprintln!("error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Collect the content using input file name
    fn get_file_input(file_path: &str) -> String {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            eprintln!("Usage: {} <input_file>", args[0]);
            std::process::exit(1);
        }

        std::fs::read_to_string(file_path).expect("Failed to read input file")
    }

    #[test]
    fn test_with_example() {
        let input = r#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;

        let data = parse_input(input);
        let result = solve(data).unwrap();
        assert_eq!(result, 7036);
    }

    #[test]
    fn test_with_input() {
        let file_path = "docs/challenge_1.txt";
        let input = get_file_input(file_path);
        let data = parse_input(&input);
        let result = solve(data).unwrap();
        assert_eq!(result, 95476);
    }
}
