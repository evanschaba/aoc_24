use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin part-2 <input-file>");
        process::exit(1);
    }
    let input_path = &args[1];
    let content = fs::read_to_string(input_path).expect("Failed to read input file");
    let (raw_rules, updates_section) = content
        .trim()
        .split_once("\n\n")
        .expect("Invalid input format: Input should contain rules and updates separated by a double newline.");

    let rules: Vec<(i32, i32)> = raw_rules
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line
                .split('|')
                .map(|x| x.trim().parse().expect("Invalid number in rules"))
                .collect();
            if parts.len() != 2 {
                panic!(
                    "Invalid rule format, expected two numbers separated by '|': {}",
                    line
                );
            }
            (parts[0], parts[1])
        })
        .collect();

    let updates: Vec<Vec<i32>> = updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.trim().parse().expect("Invalid number in update"))
                .collect()
        })
        .collect();

    let mut ans = 0;

    for update in &updates {
        if follows_rules(update, &rules).0 {
            continue;
        }
        let sorted_update = sort_correctly(update, &rules);
        ans += sorted_update[sorted_update.len() / 2];
    }

    println!("Answer: {}", ans);
}

fn follows_rules(update: &[i32], rules: &[(i32, i32)]) -> (bool, i32) {
    let idx_set: HashSet<i32> = update.iter().cloned().collect();

    for (a, b) in rules {
        if idx_set.contains(a) && idx_set.contains(b) {
            if let (Some(pos_a), Some(pos_b)) = (
                update.iter().position(|&x| x == *a),
                update.iter().position(|&x| x == *b),
            ) {
                if pos_a >= pos_b {
                    return (false, 0);
                }
            }
        }
    }

    let mid = update[update.len() / 2];
    (true, mid)
}

fn sort_correctly(update: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
    let mut dependency_count = HashMap::new();

    // Count dependencies for each element
    for (_a, b) in rules
        .iter()
        .filter(|&&(a, b)| update.contains(&a) && update.contains(&b))
    {
        *dependency_count.entry(b).or_insert(0) += 1;
    }

    let mut sorted = Vec::new();
    while sorted.len() < update.len() {
        for &item in update {
            if !sorted.contains(&item) && dependency_count.get(&item).unwrap_or(&0) == &0 {
                sorted.push(item);
                // Decrease the dependency count for items that depend on the current item
                for (a, b) in rules {
                    if *a == item {
                        if let Some(count) = dependency_count.get_mut(&b) {
                            *count -= 1;
                        }
                    }
                }
            }
        }
    }

    sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_with_input(rules: Vec<(i32, i32)>, updates: Vec<Vec<i32>>, expected: i32) {
        let mut ans = 0;
        for update in &updates {
            if follows_rules(update, &rules).0 {
                continue;
            }
            let sorted_update = sort_correctly(update, &rules);
            ans += sorted_update[sorted_update.len() / 2];
        }
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_part_2_example() {
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

        let updates = vec![vec![75, 97, 47, 61, 53], vec![61, 13, 29], vec![
            97, 13, 75, 29, 47,
        ]];

        test_with_input(rules, updates, 123);
    }

    #[test]
    fn test_part_2_additional() {
        let rules = vec![(1, 2), (2, 3), (3, 4), (4, 5), (1, 5)];

        let updates = vec![vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]];

        test_with_input(rules, updates, 3); // Expected result, adjust as needed.
    }
}
