use std::fs;
use std::io::{self, BufRead};

//cargo run < input.txt # solution: 1197984 ✅

/// Computes the total distance between two lists after sorting.
fn calculate_total_distance(mut left: Vec<i32>, mut right: Vec<i32>) -> Result<i32, &'static str> {
    if left.len() != right.len() {
        return Err("Lists must have the same length");
    }

    left.sort_unstable();
    right.sort_unstable();

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum())
}

/// Parses input into two vectors of integers from a given string.
fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>), &'static str> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().map_err(|_| "Failed to parse a number"))
            .collect::<Result<_, _>>()?;

        if nums.len() != 2 {
            return Err("Each line must contain exactly two numbers");
        }

        left.push(nums[0]);
        right.push(nums[1]);
    }

    Ok((left, right))
}

fn main() {
    // Read from either `input.txt` or stdin.
    let input = fs::read_to_string("input.txt").unwrap_or_else(|_| {
        eprintln!("input.txt not found, falling back to stdin. Enter data below:");
        io::stdin()
            .lock()
            .lines()
            .collect::<Result<String, _>>()
            .unwrap()
    });

    match parse_input(&input) {
        Ok((left, right)) => match calculate_total_distance(left, right) {
            Ok(distance) => println!("Total Distance: {}", distance),
            Err(err) => eprintln!("Error calculating total distance: {}", err),
        },
        Err(err) => eprintln!("Error parsing input: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_distance() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(calculate_total_distance(left, right).unwrap(), 11);
    }

    #[test]
    fn test_parse_input_valid() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        let (left, right) = parse_input(input).unwrap();
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_parse_input_invalid_line() {
        let input = "3 4\n4";
        assert!(parse_input(input).is_err());
    }

    #[test]
    fn test_calculate_total_distance_length_mismatch() {
        let left = vec![3, 4, 2];
        let right = vec![4, 3];
        assert!(calculate_total_distance(left, right).is_err());
    }

    #[test]
    fn test_integration() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
        let (left, right) = parse_input(input).unwrap();
        let distance = calculate_total_distance(left, right).unwrap();
        assert_eq!(distance, 11);
    }
}
