extern crate regex;

use regex::Regex;

// Function to calculate the safety factor for the robots' positions after 100 seconds
fn solve(input: &str, lim_x: i32, lim_y: i32) -> u32 {
    let rx = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    // Function to parse integer from string
    let parse_int = |s: &str| -> i32 { s.parse().unwrap() };

    // Iterate through each line, extract robot position and velocity, calculate the robot's quadrant, and count the robots in each quadrant
    input
        .lines()
        .fold([0u32; 4], |mut acc, line| {
            if let Some(caps) = rx.captures(line) {
                let x0 = parse_int(&caps[1]);
                let y0 = parse_int(&caps[2]);
                let vx = parse_int(&caps[3]);
                let vy = parse_int(&caps[4]);

                // Calculate the robot's position after 100 seconds with wrapping
                let x1 = (x0 + 100 * vx).rem_euclid(lim_x);
                let y1 = (y0 + 100 * vy).rem_euclid(lim_y);

                // Skip robots that end up on the horizontal or vertical center
                if x1 == lim_x / 2 || y1 == lim_y / 2 {
                    return acc;
                }

                // Determine the quadrant based on robot's final position
                let in_upper_half = y1 < lim_y / 2;
                let in_left_half = x1 < lim_x / 2;

                let qua = (in_upper_half as usize) << 1 | in_left_half as usize;
                acc[qua] += 1;
            }
            acc
        })
        // Calculate the safety factor by multiplying the number of robots in each quadrant
        .iter()
        .product()
}

fn main() {
    // Collect command-line arguments, expecting the first argument to be the input file name
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1); // Exit if no input file is provided
    }

    // Read the content of the input file
    let input = std::fs::read_to_string(&args[1]).expect("Failed to read input file");

    // Print the safety factor after 100 seconds
    println!("result: {}", solve(&input, 101, 103));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    fn get_input(file_name: &str) -> String {
        // Read the content of the input file and return it as a String
        std::fs::read_to_string(file_name).expect("Failed to read input file")
    }

    #[test]
    fn test_example() {
        // Test safety factor calculation with example input
        assert_eq!(solve(EXAMPLE, 11, 7), 12);
    }

    #[test]
    fn test_with_input() {
        // Test safety factor with input from a file
        let input = &get_input("./docs/challenge_1.txt");
        assert_eq!(solve(input, 101, 103), 214400550);
    }
}
