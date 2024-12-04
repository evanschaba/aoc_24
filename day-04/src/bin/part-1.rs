use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin part-1 -- <input_file>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let grid = read_input(filename)?;
    let word = "XMAS";
    let count = count_word_occurrences(&grid, word);

    println!("Answer: {}", count);

    Ok(())
}

fn read_input(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map_while(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    Ok(grid)
}

fn count_word_occurrences(grid: &[Vec<char>], word: &str) -> usize {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (0, -1),  // Left
        (-1, 0),  // Up
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
    ];

    let mut count = 0;
    let word_chars: Vec<char> = word.chars().collect();
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                if matches_word(grid, &word_chars, row, col, dr, dc) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn matches_word(
    grid: &[Vec<char>],
    word: &[char],
    start_row: usize,
    start_col: usize,
    dr: isize,
    dc: isize,
) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    for (i, &ch) in word.iter().enumerate() {
        let row = start_row as isize + i as isize * dr;
        let col = start_col as isize + i as isize * dc;

        if row < 0 || row >= rows as isize || col < 0 || col >= cols as isize {
            return false;
        }

        if grid[row as usize][col as usize] != ch {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_word_occurrences() {
        let grid = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];

        assert_eq!(count_word_occurrences(&grid, "XMAS"), 18);
    }
}
