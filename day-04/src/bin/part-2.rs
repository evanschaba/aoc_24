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
