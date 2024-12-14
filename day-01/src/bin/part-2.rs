use std::collections::HashMap;
use std::env;
use std::fs;

// cargo run --bin part-2 -- input_2.txt 23387399 ✅

fn main() {
    // Get input file from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <input.txt>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let content = fs::read_to_string(input_file).expect("Failed to read input file");

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    // Parse input file into two separate lists
    for line in content.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() != 2 {
            eprintln!("Invalid input format");
            std::process::exit(1);
        }

        let left = numbers[0]
            .parse::<i64>()
            .expect("Invalid number in left column");
        let right = numbers[1]
            .parse::<i64>()
            .expect("Invalid number in right column");

        left_list.push(left);
        right_list.push(right);
    }

    // Count occurrences of each number in the right list
    let mut right_count: HashMap<i64, i64> = HashMap::new();
    for num in right_list {
        *right_count.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    let mut similarity_score = 0;
    for num in left_list {
        if let Some(&count) = right_count.get(&num) {
            similarity_score += num * count;
        }
    }

    println!("Similarity Score: {}", similarity_score);
}
