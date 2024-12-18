use regex::Regex;

#[derive(Debug)]
struct ClawMachine {
    a: (i64, i64),     // Coordinates for button A's movement vector
    b: (i64, i64),     // Coordinates for button B's movement vector
    prize: (i64, i64), // Coordinates of the prize
}

/// Parse the input string into a vector of Configuration structs.
fn parse(input: &str) -> Vec<ClawMachine> {
    // Regex pattern to match the input format
    let pattern =
        r"Button A: X\+(\d+), Y\+(\d+)\s*Button B: X\+(\d+), Y\+(\d+)\s*Prize: X=(\d+), Y=(\d+)";
    let rx = Regex::new(pattern).unwrap(); // Compile the regex

    rx.captures_iter(input) // Iterate over all matches in the input string
        .map(|c| {
            // Parse the captured values into i64 and create a Configuration struct
            ClawMachine {
                a: (c[1].parse::<i64>().unwrap(), c[2].parse::<i64>().unwrap()),
                b: (c[3].parse::<i64>().unwrap(), c[4].parse::<i64>().unwrap()),
                prize: (c[5].parse::<i64>().unwrap(), c[6].parse::<i64>().unwrap()),
            } // Return the parsed Configuration
        })
        .collect() // Collect all configurations into a vector
}

/// Part 2: Adjust prize coordinates and calculate the total cost.
fn solve(cms: &[ClawMachine]) -> i64 {
    cms.iter()
        .map(|cm| {
            // Adjust prize coordinates with a large offset
            let cm_input = ClawMachine {
                prize: (
                    cm.prize.0 + 10000000000000, // Adjust x-coordinate of the prize
                    cm.prize.1 + 10000000000000, // Adjust y-coordinate of the prize
                ),
                ..*cm // Keep the other fields the same as in the original configuration
            };
            let (n, m) = handle_presses(&cm_input); // Get the new number of presses
            n * 3 + m // Cost calculation: 3 tokens for A and 1 token for B
        })
        .sum() // Return the sum of costs for all configurations
}

/// Solve the claw machine problem for a single configuration.
/// Returns the number of presses for buttons A and B.
fn handle_presses(cm: &ClawMachine) -> (i64, i64) {
    // Calculate the determinant for the system of equations (A * n + B * m = prize)
    let det = (cm.a.0 * cm.b.1) - (cm.a.1 * cm.b.0);
    if det == 0 {
        panic!("det == 0"); // If determinant is zero, no valid solution
    }

    // Solve for the number of presses using Cramer's rule
    let n = (cm.b.1 * cm.prize.0 - cm.b.0 * cm.prize.1) / det;
    let m = (cm.a.0 * cm.prize.1 - cm.a.1 * cm.prize.0) / det;

    // Verify if the calculated presses give the correct prize coordinates
    // Return the solution if it's correct, otherwise return (0, 0)
    if (n * cm.a.0 + m * cm.b.0 == cm.prize.0) && (n * cm.a.1 + m * cm.b.1 == cm.prize.1) {
        (n, m) // Valid solution
    } else {
        (0, 0) // Invalid solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to generate sample input for testing
    fn gen_sample() -> String {
        [
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
            "Button A: X+26, Y+66",
            "Button B: X+67, Y+21",
            "Prize: X=12748, Y=12176",
            "Button A: X+17, Y+86",
            "Button B: X+84, Y+37",
            "Prize: X=7870, Y=6450",
            "Button A: X+69, Y+23",
            "Button B: X+27, Y+71",
            "Prize: X=18641, Y=10279",
        ]
        .join("\n") // Join the parts to form a single input string
    }

    // Test case using the generated sample input
    #[test]
    fn test_with_sample() {
        let sample: &String = &gen_sample();
        assert_eq!(solve(&parse(sample)), 875318608908); // Check part 2 result
    }

    // Test case using a file-based input
    #[test]
    fn test_with_input() {
        let input = include_str!("../../docs/challenge_2.txt"); // Read input from file
        assert_eq!(solve(&parse(&input)), 101406661266314); // Check part 2 result
    }
}

fn main() {
    // Collect command-line arguments, expecting the first argument to be the input file name.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1); // Exit if no input file is provided
    }

    // Read the content of the input file.
    let input = std::fs::read_to_string(&args[1]).expect("Failed to read input file");

    let data = parse(&input); // Parse the input into configurations

    // Print the results
    println!("result: {}", solve(&data));
}
