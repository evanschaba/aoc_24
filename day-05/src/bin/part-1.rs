use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn main() {
    // Read command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin part-1 <input-file>");
        process::exit(1);
    }
    let input_path = &args[1];

    // Read and parse the input file
    let content = fs::read_to_string(input_path).expect("Failed to read input file");
    let (raw_rules, updates_section) = content
        .trim()
        .split_once("\n\n")
        .expect("Invalid input format: Input should contain rules and updates separated by a double newline.");

    // Parse the rules with detailed error handling
    let rules: Vec<(i32, i32)> = raw_rules
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line.split('|')
                .map(|x| x.trim().parse().expect("Invalid number in rules"))
                .collect();
            if parts.len() != 2 {
                panic!("Invalid rule format, expected two numbers separated by '|': {}", line);
            }
            (parts[0], parts[1])
        })
        .collect();

    // Parse the updates with error handling
    let updates: Vec<Vec<i32>> = updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.trim().parse().expect("Invalid number in update"))
                .collect()
        })
        .collect();

    // Calculate the sum of middle numbers for valid updates
    let ans: i32 = updates
        .into_iter()
        .filter_map(|update| {
            let (valid, mid) = follows_rules(&update, &rules);
            if valid { Some(mid) } else { None }
        })
        .sum();

    // Print the final answer
    println!("{}", ans);
}

// Function to check if an update follows the rules
fn follows_rules(update: &[i32], rules: &[(i32, i32)]) -> (bool, i32) {
    let idx: HashMap<_, _> = update.iter().enumerate().map(|(i, &num)| (num, i)).collect();

    for (a, b) in rules {
        if let (Some(&pos_a), Some(&pos_b)) = (idx.get(a), idx.get(b)) {
            if pos_a >= pos_b {
                return (false, 0);
            }
        }
    }

    let mid = update[update.len() / 2];
    (true, mid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_abstract_example() {
        // Simple example rules
        let rules = vec![
            (1, 2), // 1 must come before 2
            (2, 3), // 2 must come before 3
        ];

        // Simple updates
        let updates = vec![
            vec![1, 2, 3], // Correct order
            vec![2, 1, 3], // Incorrect order (1 should be before 2)
            vec![1, 3, 2], // Incorrect order (3 should be after 2)
        ];

        let ans: i32 = updates
            .into_iter()
            .filter_map(|update| {
                let (valid, mid) = follows_rules(&update, &rules);
                if valid { Some(mid) } else { None }
            })
            .sum();

        assert_eq!(ans, 2);
    }

    #[test]
    fn test_aoc_example_1() {
        // Sample rules from the problem statement
        let rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];

        // Example updates
        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];

        let ans: i32 = updates
            .into_iter()
            .filter_map(|update| {
                let (valid, mid) = follows_rules(&update, &rules);
                if valid { Some(mid) } else { None }
            })
            .sum();

        assert_eq!(ans, 143);
    }
}