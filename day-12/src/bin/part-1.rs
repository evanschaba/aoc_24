use std::collections::{HashSet, VecDeque};

/// Movement directions (up, right, down, left).
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

/// Represents the map grid of garden plots.
pub struct GardenPlot {
    grid: Vec<Vec<char>>, // 2D grid of garden plots with plant types.
    rows: usize,          // Number of rows in the grid.
    cols: usize,          // Number of columns in the grid.
}

/// Represents a region of connected garden plots with the same plant type.
#[derive(Debug)]
pub struct Region {
    plant_type: char,                     // Type of plant in this region.
    coordinates: HashSet<(usize, usize)>, // Set of coordinates belonging to this region.
    area: usize,                          // Total number of plots in this region.
}

/// Generates regions from the grid based on connected garden plots.
fn gen_regions(grid: &Vec<Vec<char>>, rows: usize, cols: usize) -> Vec<Region> {
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    // Traverse the grid to identify regions.
    for row in 0..rows {
        for col in 0..cols {
            // Skip already visited cells.
            if visited.contains(&(row, col)) {
                continue;
            }

            // Initialize region data for the current plant type.
            let plant = grid[row][col];
            let mut coords = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((row, col));

            // Perform BFS to identify all connected plots in the region.
            while let Some((current_row, current_col)) = queue.pop_front() {
                // Skip already visited cells.
                if visited.contains(&(current_row, current_col)) {
                    continue;
                }

                visited.insert((current_row, current_col));
                coords.insert((current_row, current_col));

                // Check neighboring cells for connectivity using DIRECTIONS.
                for &(row_offset, col_offset) in &DIRECTIONS {
                    let n_row = (current_row as isize + row_offset as isize) as usize;
                    let n_col = (current_col as isize + col_offset as isize) as usize;

                    if n_row < rows
                        && n_col < cols
                        && grid[n_row][n_col] == plant
                        && !visited.contains(&(n_row, n_col))
                    {
                        queue.push_back((n_row, n_col));
                    }
                }
            }

            regions.push(Region {
                plant_type: plant,
                area: coords.len(),
                coordinates: coords,
            });
        }
    }

    regions
}

/// Computes the total fencing cost for part 1 using area and perimeter.
pub fn compute((regions, map): &(Vec<Region>, GardenPlot)) -> usize {
    let mut result = 0;

    for region in regions {
        let mut perimeter = 0;

        for &(row, col) in &region.coordinates {
            for &(row_offset, col_offset) in &DIRECTIONS {
                let neighbor_row = (row as isize + row_offset as isize) as usize;
                let neighbor_col = (col as isize + col_offset as isize) as usize;

                if neighbor_row >= map.rows
                    || neighbor_col >= map.cols
                    || map.grid[neighbor_row][neighbor_col] != region.plant_type
                {
                    perimeter += 1;
                }
            }
        }

        result += region.area * perimeter;
    }

    result
}

/// Parses input to generate regions and the corresponding garden map.
pub fn solve(input: &str) -> usize {
    let cols = input.lines().next().unwrap().len();
    let mut rows = 0;

    // Construct the grid from input.
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            rows += 1;
            line.chars().collect()
        })
        .collect();

    let data = (gen_regions(&grid, rows, cols), GardenPlot {
        grid,
        rows,
        cols,
    });

    compute(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_sample() -> String {
        ["AAAA", "BBCD", "BBCC", "EEEC"].join("\n")
    }

    fn gen_sample_2() -> String {
        ["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"].join("\n")
    }

    #[test]
    fn test_with_samples() {
        assert_eq!(solve(&gen_sample()), 140);
        assert_eq!(solve(&gen_sample_2()), 772);
    }
}

fn main() {
    // Collect command-line arguments, expecting the first argument to be the input file name.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    // Read the content of the input file.
    let input = std::fs::read_to_string(&args[1]).expect("Failed to read input file");

    println!("result: {}", solve(&input));
}
