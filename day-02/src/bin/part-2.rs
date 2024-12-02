use std::env;
use std::fs::File;
use std::io::{self, BufRead};

// Function to check if the report levels are safe
fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let diffs: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();
    let is_decreasing = diffs.iter().all(|&d| (-3..=-1).contains(&d));
    let is_increasing = diffs.iter().all(|&d| (1..=3).contains(&d));

    is_decreasing || is_increasing
}

// Function to check if a report is safe with one dampener removal
fn is_safe_with_dampener(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified = levels.to_vec();
        modified.remove(i);
        if is_safe(&modified) {
            return true;
        }
    }

    false
}

// Function to count safe reports with a dampener
fn count_safe_reports_with_dampener(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line?;
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if is_safe_with_dampener(&levels) {
            safe_count += 1;
        }
    }

    Ok(safe_count)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin part-2 -- <input_file>");
        std::process::exit(1);
    }

    let input_file = &args[1];

    match count_safe_reports_with_dampener(input_file) {
        Ok(count) => println!("Number of safe reports with dampener: {}", count),
        Err(e) => eprintln!("Error: {}", e),
    }
}
