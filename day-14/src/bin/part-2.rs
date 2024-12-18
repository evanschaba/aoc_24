extern crate regex;

use regex::Regex;
use std::collections::HashSet;

type Robots = Vec<((i32, i32), (i32, i32))>;

// Simulate the positions of robots after `t` seconds with wrapping around
fn simulate_robots(robots: &Robots, t: i32, max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    robots
        .iter()
        .map(|&((rx, ry), (vx, vy))| {
            (
                (rx + t * vx).rem_euclid(max_x), // Wrap x-coordinate
                (ry + t * vy).rem_euclid(max_y), // Wrap y-coordinate
            )
        })
        .collect()
}

// Count how many robots are touching each other (in formation)
fn count_robots(robots: &HashSet<(i32, i32)>) -> usize {
    // Directions to check for adjacent robots: up, down, left, right
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut adj_count = 0;
    for &(rx, ry) in robots {
        for (dx, dy) in directions.iter() {
            // Check if an adjacent robot exists
            if robots.contains(&(rx + dx, ry + dy)) {
                adj_count += 1;
                break; // Stop checking further if one adjacent robot is found
            }
        }
    }
    adj_count
}

// Solve the puzzle by simulating robot movements and checking for formation
fn solve(input: &str, max_x: i32, max_y: i32) -> i32 {
    // Regular expression to parse the input lines
    let rx = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    // Function to parse integer from string
    let parse_int = |s: &str| -> i32 { s.parse().unwrap() };

    // Parse robot data: positions and velocities
    let robots: Vec<((i32, i32), (i32, i32))> = input
        .lines()
        .filter_map(|line| {
            if let Some(caps) = rx.captures(line) {
                let x0 = parse_int(&caps[1]);
                let y0 = parse_int(&caps[2]);
                let vx = parse_int(&caps[3]);
                let vy = parse_int(&caps[4]);
                Some(((x0, y0), (vx, vy))) // Store the position and velocity as a tuple
            } else {
                None
            }
        })
        .collect();

    // Iterate over time steps to check when robots form a pattern
    for t in 1.. {
        let positions: HashSet<(i32, i32)> = simulate_robots(&robots, t, max_x, max_y)
            .into_iter()
            .collect();
        let in_formation = count_robots(&positions);

        // If half or more of the robots are in formation, return the time `t`
        if in_formation >= positions.len() / 2 {
            return t;
        }
    }

    unreachable!() // This should never be reached since we expect a formation
}

fn main() {
    // Collect command-line arguments, expecting the first argument to be the input file name.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1); // Exit if no input file is provided
    }

    // Read the content of the input file.
    let input = std::fs::read_to_string(&args[1]).expect("Failed to read input file");

    println!("result: {}", solve(&input, 101, 103));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // Example input for testing
    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    // Helper function to read the content of an input file
    fn get_input(file_name: &str) -> String {
        // Read the content of the input file and return it as a String.
        std::fs::read_to_string(file_name).expect("Failed to read input file")
    }

    // Test the robot simulation after `t` seconds
    #[test]
    fn test_simulate_robots() {
        let robots = vec![((0, 4), (3, -3)), ((6, 3), (-1, -3)), ((10, 3), (-1, 2))];
        let expected_positions = vec![(3, 1), (5, 0), (9, 5)];
        assert_eq!(simulate_robots(&robots, 1, 101, 103), expected_positions);
    }

    // Test the counting of robots in formation (touching each other)
    #[test]
    fn test_count_in_formation() {
        let positions: HashSet<(i32, i32)> = [(1, 1), (2, 1), (1, 2), (2, 2), (3, 1)]
            .into_iter()
            .collect();
        assert_eq!(count_robots(&positions), 5);
    }

    // Test the solution with example input
    #[test]
    fn test_part_2_example() {
        assert_eq!(solve(EXAMPLE, 11, 7), 1);
    }

    // Test the solution with real input (challenge_2.txt)
    #[test]
    fn test_with_input_2() {
        let input = &get_input("./docs/challenge_2.txt");
        assert_eq!(solve(input, 101, 103), 8149);
    }
}
