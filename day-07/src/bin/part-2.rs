// Define constants for the operators: addition, concatenation, and multiplication.
const ADD: char = '+';
const JOIN: char = '|';
const MULTIPLY: char = '*';

// Include all three operators (+, *, ||) in a constant array.
const OP: [char; 3] = [ADD, MULTIPLY, JOIN];

// A structure to represent an equation consisting of numbers and a result.
#[derive(Debug)]
struct Expr {
    nums: Vec<i64>, // List of numbers in the equation.
    result: i64,    // Expected result of the equation.
}

// Entry function to solve all equations provided in the input data.
fn solve(data: &str) -> i64 {
    let items = parse(data); // Parse the input into a list of equations.
    // For each equation, calculate its result and sum them up.
    items.iter().map(|item| calibrate(item, &OP)).sum()
}

// Attempt to calibrate the given equation by finding operator combinations.
fn calibrate(expr: &Expr, ops: &[char]) -> i64 {
    let mut result: i64 = 0;
    let size = expr.nums.len() - 1; // Number of operators required.
    let mut current: Vec<char> = vec![ADD; size]; // Start with all addition operators.
    gen_inner_op(expr, &mut result, size, ops, &mut current); // Generate combinations recursively.
    result
}

// Recursive function to generate operator combinations and check results.
fn gen_inner_op(expr: &Expr, result: &mut i64, size: usize, ops: &[char], current: &mut Vec<char>) {
    if size == 0 {
        // Base case: when all operators are set.
        // Check if the combination of operators produces the expected result.
        if solve_expr(&expr.nums, current) == expr.result {
            *result = expr.result; // Update the result if it's valid.
        }
        return;
    }

    // Iterate through available operators and set each one at the current position.
    for ch in ops.iter() {
        current[size - 1] = *ch;
        gen_inner_op(expr, result, size - 1, ops, current); // Recurse to set the next operator.

        if *result > 0 {
            // Stop early if a valid result is found.
            return;
        }
    }
}

// Evaluate the equation using the given numbers and operator sequence.
fn solve_expr(nums: &[i64], ops: &[char]) -> i64 {
    let mut result = nums[0]; // Start with the first number.

    for (i, op) in ops.iter().enumerate() {
        let right = nums[i + 1]; // Get the next number.
        if op == &ADD {
            result += right; // Apply addition.
        } else if op == &MULTIPLY {
            result *= right; // Apply multiplication.
        } else if op == &JOIN {
            // Concatenate the current result and the next number.
            let n_str = format!("{}{}", result, right);
            result = n_str.parse().unwrap();
        }
    }
    result
}

// Parse the input data into a list of Expr structures.
fn parse(data: &str) -> Vec<Expr> {
    data.lines() // Split input into lines.
        .filter_map(|line| parse_expr_line(line).ok()) // Parse each line into an Expr.
        .collect()
}

// Parse a single line into an Expr structure.
fn parse_expr_line(input_str: &str) -> Result<Expr, String> {
    let mut data = input_str.split(':');

    // Parse the expected result.
    let result = data
        .next()
        .ok_or("Missing part".to_string())?
        .trim()
        .parse()
        .map_err(|_| "Failed to parse result".to_string())?;

    // Parse the list of numbers.
    let nums = data
        .next()
        .ok_or("Missing part".to_string())?
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Failed to parse nums".to_string())?;

    let expr = Expr { result, nums }; // Construct the Expr.
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_expr() {
        let nums: Vec<i64> = vec![15, 15, 30, 40];
        assert_eq!(100, solve_expr(&nums, &vec![ADD, ADD, ADD]));
    }

    #[test]
    fn test_solve_expr_a() {
        let nums: Vec<i64> = vec![15, 6];
        assert_eq!(156, solve_expr(&nums, &vec![JOIN]));
    }

    #[test]
    fn test_solve_expr_b() {
        let nums: Vec<i64> = vec![6, 8, 6, 15];
        assert_eq!(7290, solve_expr(&nums, &vec![MULTIPLY, JOIN, MULTIPLY]));
    }

    #[test]
    fn test_solve_expr_c() {
        let nums: Vec<i64> = vec![17, 8, 14];
        assert_eq!(192, solve_expr(&nums, &vec![JOIN, ADD]));
    }

    #[test]
    fn test_solve_expr_d() {
        let nums: Vec<i64> = vec![11, 6, 16, 20];
        assert_eq!(292, solve_expr(&nums, &vec![ADD, MULTIPLY, ADD]));
    }

    #[test]
    fn test_with_str() {
        let input = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";
        let result = solve(input);
        assert_eq!(result, 11387);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).expect("Failed to read input file");

    let result = solve(&input);

    println!("result: {}", result);
}
