use regex::Regex;
use std::env;
use std::fs;

// cargo fmt && cargo clippy --fix --allow-dirty --allow-staged;
// cargo run --bin part-2 -- input_2.txt
// 100189366✅

fn parse_and_sum(input: &str) -> i64 {
    let mut total_sum = 0;
    let mut enable_mul = true; // Multiplication starts enabled

    // Regex to match 'do()', 'don't()', or 'mul(x, y)'
    let pattern = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();

    for capture in pattern.captures_iter(input) {
        match capture.get(1).map(|m| m.as_str()) {
            Some("do()") => {
                // Enable multiplications
                enable_mul = true;
            }
            Some("don't()") => {
                // Disable multiplications
                enable_mul = false;
            }
            Some(_) => {
                // Check if it's a mul() call and parse x, y
                if let (Some(x_str), Some(y_str)) = (capture.get(2), capture.get(3)) {
                    if let (Ok(x), Ok(y)) =
                        (x_str.as_str().parse::<i64>(), y_str.as_str().parse::<i64>())
                    {
                        if enable_mul {
                            total_sum += x * y;
                        }
                    }
                }
            }
            None => {}
        }
    }

    total_sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin part-2 -- <input_file>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let input = fs::read_to_string(input_file)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", input_file));

    let result = parse_and_sum(&input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(parse_and_sum(input), 48);
    }
}
