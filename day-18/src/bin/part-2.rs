use std::collections::{HashSet, VecDeque};

type Coordinate = (i32, i32);
type MemoryGrid = Vec<Coordinate>;

/// Reads the input file provided as a command-line argument.
/// Returns the content of the file as a `String`.
pub fn read_input_from_arg() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    std::fs::read_to_string(&args[1]).expect("Failed to read input file")
}

/// Reads the input file directly using the provided path.
/// Returns the content of the file as a `String`.
pub fn read_input_from_path(path: &str) -> String {
    std::fs::read_to_string(path).expect("Failed to read input file")
}

/// Parses the input data into a vector of grid coordinates (bytes).
/// Each line represents a coordinate in the form `x,y`.
fn parse_input(input: &str) -> MemoryGrid {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.trim().parse().unwrap(), y.trim().parse().unwrap())
        })
        .collect()
}

/// Returns the neighboring coordinates for a given grid coordinate.
fn get_neighbors(coord: &Coordinate) -> Vec<Coordinate> {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .map(|(dx, dy)| (coord.0 + dx, coord.1 + dy))
        .collect()
}

/// Simulates the grid at a given time step and calculates the shortest path.
/// Returns the minimum steps to the exit if reachable; otherwise, `None`.
fn simulate_memory_grid(grid: &MemoryGrid, time: usize) -> Option<u32> {
    let (width, height) = if grid.len() > 1000 { (71, 71) } else { (7, 7) };

    let mut space: HashSet<Coordinate> = (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .collect();

    for coord in grid.iter().take(time) {
        space.remove(coord);
    }

    let mut to_explore = VecDeque::from([((0, 0), 0)]);
    let mut visited = HashSet::new();

    while let Some((current, distance)) = to_explore.pop_back() {
        if !visited.insert(current) {
            continue;
        }

        if current == (width - 1, height - 1) {
            return Some(distance);
        }

        for neighbor in get_neighbors(&current) {
            if !visited.contains(&neighbor) && space.contains(&neighbor) {
                to_explore.push_front((neighbor, distance + 1));
            }
        }
    }

    None
}

/// Part 2: Finds the first byte that blocks the path to the exit.
fn solve(grid: &MemoryGrid) -> String {
    let mut reachable = 0;
    let mut unreachable = grid.len() - 1;

    while unreachable > (reachable + 1) {
        let midpoint = (reachable + unreachable) / 2;
        if simulate_memory_grid(grid, midpoint).is_some() {
            reachable = midpoint;
        } else {
            unreachable = midpoint;
        }
    }

    format!("{},{}", grid[reachable].0, grid[reachable].1)
}

fn main() {
    let input = read_input_from_arg();
    let grid = parse_input(&input);
    let result = solve(&grid);

    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_example() {
        let input = r#"5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0"#;

        let grid = parse_input(input.trim());
        // The byte at 6,1 is the first that prevents the exit from being reachable.
        assert_eq!(solve(&grid), "6,1");
    }

    #[test]
    fn test_with_input() {
        let input = read_input_from_path("docs/challenge_2.txt");
        let grid = parse_input(&input);
        let result = solve(&grid);

        assert_eq!(result, "30,12")
    }
}
