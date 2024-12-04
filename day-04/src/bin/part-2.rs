use std::env;
use std::fs;

fn main() {
    // Get the input file path from command line arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin part-2 -- <input_file>");
        return;
    }

    let input_file = &args[1];
    let content = fs::read_to_string(input_file).expect("Failed to read input file");

    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let n = grid.len();
    let m = grid[0].len();
    let mut count = 0;

    // Check for X-MAS patterns centered at (i, j).
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if has_xmas(&grid, i, j) {
                count += 1;
            }
        }
    }

    println!("Number of X-MAS patterns found: {}", count);
}

fn has_xmas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if grid[i][j] != 'A' {
        return false;
    }

    // Check both diagonal directions for the X-MAS pattern.
    let diag1 = (grid[i - 1][j - 1], grid[i + 1][j + 1]);
    let diag2 = (grid[i - 1][j + 1], grid[i + 1][j - 1]);

    (diag1 == ('M', 'S') || diag1 == ('S', 'M')) && (diag2 == ('M', 'S') || diag2 == ('S', 'M'))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_grid() -> Vec<Vec<char>> {
        vec![
            vec!['M', 'M', 'M', 'S', 'X', 'M', 'M', 'S'],
            vec!['M', 'S', 'A', 'M', 'S', 'M', 'X', 'M'],
            vec!['M', 'M', 'A', 'M', 'S', 'A', 'M', 'S'],
            vec!['X', 'M', 'S', 'M', 'M', 'M', 'S', 'A'],
            vec!['S', 'A', 'M', 'X', 'M', 'S', 'A', 'S'],
            vec!['M', 'S', 'M', 'M', 'X', 'S', 'M', 'M'],
        ]
    }

    #[test]
    fn test_has_xmas() {
        let grid = create_test_grid();
        assert!(has_xmas(&grid, 1, 2)); // Should be true for this position
        assert!(!has_xmas(&grid, 0, 0)); // Should be false, no X-MAS pattern
        assert!(has_xmas(&grid, 4, 3)); // Another true case
        assert!(!has_xmas(&grid, 2, 4)); // False case
    }

    #[test]
    fn test_count_xmas_patterns() {
        let grid = create_test_grid();
        let n = grid.len();
        let m = grid[0].len();
        let mut count = 0;
        for i in 1..n - 1 {
            for j in 1..m - 1 {
                if has_xmas(&grid, i, j) {
                    count += 1;
                }
            }
        }
        assert_eq!(count, 3); // Expected number of X-MAS patterns in this test grid
    }
}
