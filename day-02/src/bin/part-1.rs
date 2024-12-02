use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// cargo run -- input.txt 
// Number of safe reports: 306 ✅

fn main() {
    // Get the input file from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin part-1 -- <input_file>");

        std::process::exit(1);
    }

    let input_file = &args[1];
    let lines = read_lines(input_file).expect("Could not read input file");

    let mut safe_count = 0;

    for line in lines.flatten() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if is_safe(&levels) {
            safe_count += 1;
        }
    }

    println!("Number of safe reports: {}", safe_count);
}

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let diffs: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();

    // Check if all differences are within [-3, -1] (decreasing) or [1, 3] (increasing)
    let is_decreasing = diffs.iter().all(|&d| (-3..=-1).contains(&d));
    let is_increasing = diffs.iter().all(|&d| (1..=3).contains(&d));

    is_decreasing || is_increasing
}

// Utility to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
