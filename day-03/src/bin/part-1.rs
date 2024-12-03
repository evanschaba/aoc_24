use regex::Regex;
use std::env;
use std::fs;

// cargo fmt && cargo clippy --fix --allow-dirty --allow-staged
// cargo run --bin day-3 -- input.txt
// Example: Total sum of valid mul operations: 155955228 ✅

fn main() {
    // Get the input file from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin part-1 -- <input_file>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let input = fs::read_to_string(input_file).expect("Could not read input file");

    let total_sum = calculate_mul_sum(&input);

    println!("Total sum of valid mul operations: {}", total_sum);
}

fn calculate_mul_sum(input: &str) -> i32 {
    // Regular expression to match valid `mul(X,Y)` instructions
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let x: i32 = caps[1].parse().unwrap();
            let y: i32 = caps[2].parse().unwrap();
            x * y
        })
        .sum()
}
