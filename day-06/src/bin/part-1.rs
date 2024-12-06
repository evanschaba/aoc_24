use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

fn parse_map(input: &str) -> (Vec<Vec<char>>, Position, Direction) {
    let mut map = Vec::new();
    let mut guard_position = Position(0, 0);
    let mut guard_direction = Direction::Up;

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '^' || c == '>' || c == 'v' || c == '<' {
                guard_position = Position(x as i32, y as i32);
                guard_direction = match c {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    _ => unreachable!(),
                };
                row.push('.'); // Replace the guard with an empty space
            } else {
                row.push(c);
            }
        }
        map.push(row);
    }
    println!(
        "Parsed map with guard at position {:?} facing {:?}",
        guard_position, guard_direction
    );
    (map, guard_position, guard_direction)
}

fn move_guard(
    map: &mut [Vec<char>],
    position: &mut Position,
    direction: &mut Direction,
    visited: &mut HashSet<Position>,
) {
    loop {
        // Log current position and visit it if not already visited
        println!("Current position: {:?}", position);
        if visited.insert(*position) {
            println!("Visiting new position: {:?}", position);
        } else {
            println!("Already visited position: {:?}", position);
        }

        // Calculate the next position based on the current direction
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };
        let new_position = Position(position.0 + dx, position.1 + dy);

        // Check if the new position is out of bounds
        if new_position.0 < 0
            || new_position.1 < 0
            || new_position.1 >= map.len() as i32
            || new_position.0 >= map[0].len() as i32
        {
            println!(
                "New position {:?} is out of bounds. Exiting loop.",
                new_position
            );
            break;
        }

        // Check for obstacles in the new position
        if let Some(row) = map.get(new_position.1 as usize) {
            if let Some(&cell) = row.get(new_position.0 as usize) {
                if cell == '#' {
                    println!(
                        "Obstacle detected at {:?}. Turning direction.",
                        new_position
                    );
                    // Turn 90 degrees to the right if there's an obstacle
                    *direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    };
                    println!("New direction after turn: {:?}", direction);
                    continue; // Skip moving into the obstacle
                }
            }
        }

        // Move to the new position if it's valid and not an obstacle
        println!("Moving to new position: {:?}", new_position);
        *position = new_position;
    }
} 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_map() {
        let input = vec![
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ];
        let input_string = input.join("\n");
        let (mut map, mut guard_position, mut guard_direction) = parse_map(&input_string);
        let mut visited = HashSet::new();
        move_guard(
            &mut map,
            &mut guard_position,
            &mut guard_direction,
            &mut visited,
        );
        assert_eq!(visited.len(), 41);
    }

    #[test]
    fn test_guard_turns_right_when_obstacle_detected() {
        let input = vec![
            "....#.....",
            "....#.....",
            "....#.....",
            "....#.....",
            "....#.....",
            "....#.....",
            "....^.....",
            "....#.....",
            "....#.....",
            "....#.....",
        ];
        let input_string = input.join("\n");
        let (mut map, mut guard_position, mut guard_direction) = parse_map(&input_string);
        let mut visited = HashSet::new();
        move_guard(
            &mut map,
            &mut guard_position,
            &mut guard_direction,
            &mut visited,
        );
        assert!(visited.contains(&Position(4, 6))); // Check that the guard has turned at the obstacle
        assert!(visited.contains(&Position(5, 6))); // The guard should have moved to the next position
    }

    #[test]
    fn test_guard_exit_boundaries() {
        let input = vec![
            "...........",
            "...........",
            "...........",
            "...........",
            "...........",
            "...........",
            "....^......",
            "...........",
            "...........",
            "...........",
        ];
        let input_string = input.join("\n");
        let (mut map, mut guard_position, mut guard_direction) = parse_map(&input_string);
        let mut visited = HashSet::new();
        move_guard(
            &mut map,
            &mut guard_position,
            &mut guard_direction,
            &mut visited,
        );
        assert!(visited.len() > 0);
        assert!(!visited.contains(&Position(6, 6))); // Ensure the guard moves out of bounds and stops
    }
    #[test]
    fn test_guard_moves_without_obstacles() {
        let input = vec![
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
            "....^.....",
            "..........",
            "..........",
            "..........",
        ];
        let input_string = input.join("\n");
        let (mut map, mut guard_position, mut guard_direction) = parse_map(&input_string);
        let mut visited = HashSet::new();
        move_guard(
            &mut map,
            &mut guard_position,
            &mut guard_direction,
            &mut visited,
        );
        assert!(visited.len() > 0); // Ensure the guard moves
    }
    #[test]
    fn test_guard_starts_at_edge() {
        let input = vec![
            "##########",
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
            "....^.....",
            "..........",
            "..........",
            "..........",
        ];
        let input_string = input.join("\n");
        let (mut map, mut guard_position, mut guard_direction) = parse_map(&input_string);
        let mut visited = HashSet::new();
        move_guard(
            &mut map,
            &mut guard_position,
            &mut guard_direction,
            &mut visited,
        );
        assert!(visited.len() > 0); // Ensure guard can move within the map
        assert!(!visited.contains(&Position(0, 6))); // Guard should not move outside the map's bounds
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return;
    }
    let input_file = &args[1];
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    let (mut map, mut guard_position, mut guard_direction) = parse_map(&input);
    let mut visited = HashSet::new();

    move_guard(
        &mut map,
        &mut guard_position,
        &mut guard_direction,
        &mut visited,
    );

    println!("Visited positions: {:?}", visited.len());
}
