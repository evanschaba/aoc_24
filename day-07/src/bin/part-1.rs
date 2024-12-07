// Constants for operators
const ADD: char = '+';
const JOIN: char = '|'; // Operator to concatenate numbers
const MULTIPLY: char = '*';

// Array of possible operators
const OP: [char; 2] = [ADD, MULTIPLY];

// Struct to represent an expression with numbers and the expected result
#[derive(Debug)]
struct Expr {
    nums: Vec<i64>, // Numbers in the equation
    result: i64,    // Target result of the equation
}

// Solves the entire input by parsing and summing up the valid results
fn solve(data: &str) -> i64 {
    let items = parse(data); // Parse input data into expressions
    items.iter().map(|item| calibrate(item, &OP)).sum() // Sum results of valid equations
}

// Calibrates a single expression to check if the result matches by trying all operator combinations
fn calibrate(eq: &Expr, ops: &[char]) -> i64 {
    let mut result: i64 = 0; // Initialize result
    let size = eq.nums.len() - 1; // Number of operator positions
    let mut current: Vec<char> = vec![ADD; size]; // Start with all '+' operators
    gen_inner_op(eq, &mut result, size, ops, &mut current); // Generate operator combinations recursively
    result
}

// Recursively generates all possible operator combinations and checks if they solve the equation
fn gen_inner_op(eq: &Expr, result: &mut i64, size: usize, ops: &[char], current: &mut Vec<char>) {
    if size == 0 {
        // Base case: check if current operator combination solves the equation
        if solve_expr(&eq.nums, current) == eq.result {
            *result = eq.result; // Set the result if solved
        }
        return;
    }

    // Try all operators at the current position
    for c in ops.iter() {
        current[size - 1] = *c; // Set the operator
        gen_inner_op(eq, result, size - 1, ops, current); // Recurse to the next position

        if *result > 0 {
            return; // Stop recursion early if result is found
        }
    }
}

// Evaluates the expression using the given numbers and operators
fn solve_expr(nums: &Vec<i64>, ops: &Vec<char>) -> i64 {
    let mut result = nums[0]; // Start with the first number

    for (i, op) in ops.iter().enumerate() {
        let right = nums[i + 1]; // Get the next number
        if op == &ADD {
            result += right; // Perform addition
        } else if op == &MULTIPLY {
            result *= right; // Perform multiplication
        } else if op == &JOIN {
            // Concatenate numbers as strings and parse back to i64
            let str_num = format!("{}{}", result, right);
            result = str_num.parse().unwrap();
        }
    }
    result
}

// Parses the input data into a vector of expressions
fn parse(data: &str) -> Vec<Expr> {
    data.lines()
        .filter_map(|line| parse_expr_line(line).ok()) // Parse each line and filter out errors
        .collect()
}

// Parses a single line into an Expr struct
fn parse_expr_line(input_str: &str) -> Result<Expr, String> {
    let mut data = input_str.split(':'); // Split the line into result and numbers

    let result = data
        .next() // Get the result part
        .ok_or("Missing part".to_string())?
        .trim()
        .parse()
        .map_err(|_| "Failed to parse result".to_string())?;

    let nums = data
        .next() // Get the numbers part
        .ok_or("Missing part".to_string())?
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>() // Parse numbers into a vector
        .map_err(|_| "Failed to parse nums".to_string())?;

    let expr = Expr { result, nums }; // Create the expression struct

    Ok(expr)
}

// Unit tests for different functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_expr_a() {
        let nums: Vec<i64> = vec![10, 20, 30, 40];
        assert_eq!(100, solve_expr(&nums, &vec![ADD, ADD, ADD]));
    }

    #[test]
    fn test_solve_expr_b() {
        let nums: Vec<i64> = vec![11, 6, 16, 20];
        assert_eq!(292, solve_expr(&nums, &vec![ADD, MULTIPLY, ADD]));
    }

    #[test]
    fn test_solve_expr_c() {
        let nums: Vec<i64> = vec![81, 40, 27];
        assert_eq!(3267, solve_expr(&nums, &vec![ADD, MULTIPLY]));
    }

    #[test]
    fn test_solve_expr_d() {
        let nums: Vec<i64> = vec![10, 19];
        assert_eq!(190, solve_expr(&nums, &vec![MULTIPLY]));
    }

    #[test]
    fn test_with_str() {
        let input = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";
        let result = solve(input);
        assert_eq!(result, 3749);
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
