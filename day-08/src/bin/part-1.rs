use std::fs;

type Positions = [Vec<(i32, i32)>; 128];

// Reads the input file and returns a tuple with the grid, its width, height, and antenna positions.
fn read_input_file(input_file: &str) -> (Vec<u8>, usize, usize, Positions) {
    // Read the entire content of the input file as a string.
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    // Split the input into lines and determine the grid's dimensions.
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();

    // Convert the lines into a flat vector of bytes (grid representation).
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    // Initialize an array to hold the positions of each frequency (0-127).
    let mut positions = [(); 128].map(|_| Vec::new());
    for y in 0..height {
        for x in 0..width {
            let c = grid[y * width + x];
            // If the character is not a '.', it represents an antenna.
            if c != b'.' {
                positions[c as usize].push((x as i32, y as i32));
            }
        }
    }

    // Return the grid, its dimensions, and the positions of antennas by frequency.
    (grid, width, height, positions)
}

// Calculates the number of unique antinode locations on the grid.
fn calculate_antinodes(
    grid: &mut [u8],
    width: usize,
    height: usize,
    positions: &[Vec<(i32, i32)>; 128],
) -> i32 {
    let mut antinodes = 0; // Counter for unique antinodes.

    // Iterate over each set of antennas by frequency.
    for antennas in positions.iter() {
        for i in 0..antennas.len() {
            for j in 0..antennas.len() {
                if i == j {
                    continue; // Skip comparison with itself.
                }

                // Calculate the directional vector from antenna i to antenna j.
                let dx = antennas[j].0 - antennas[i].0;
                let dy = antennas[j].1 - antennas[i].1;

                // Iterate through potential antinode positions along the line.
                for n in 1..i32::MAX {
                    let nx = antennas[i].0 + dx * n;
                    let ny = antennas[i].1 + dy * n;

                    // Check if the new coordinates are within the grid boundaries.
                    if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                        let ni = ny as usize * width + nx as usize;

                        // If the position is occupied by an antenna of a different frequency, mark it.
                        if grid[ni] > 2 {
                            grid[ni] = 2;
                        }

                        // Check if this is the second antinode at distance 2.
                        if n == 2 && grid[ni] != 1 {
                            antinodes += 1; // Increment the antinode counter.
                            grid[ni] = 1; // Mark the position as having an antinode.
                        }
                    } else {
                        break; // Stop if the position is out of bounds.
                    }
                }
            }
        }
    }
    antinodes // Return the total number of unique antinodes found.
}

fn main() {
    // Collect command-line arguments and verify the input file is passed.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return;
    }

    // Read input from the file and extract grid and antenna positions.
    let input_file = &args[1];
    let (mut grid, width, height, positions) = read_input_file(input_file);

    // Calculate the number of unique antinode locations.
    let antinodes = calculate_antinodes(&mut grid, width, height, &positions);

    // Print the result.
    println!("Result: {}", antinodes);
}

// Unit tests for the `read_input_file` and `calculate_antinodes` functions.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_antinodes_empty() {
        // Test input with no antennas or antinodes.
        let lines: Vec<&str> = vec![
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
        ];

        let width = lines[0].len();
        let height = lines.len();
        let grid = lines
            .iter()
            .flat_map(|&line| line.as_bytes())
            .copied()
            .collect::<Vec<u8>>();

        let positions = [(); 128].map(|_| Vec::new());

        let mut grid_ref = grid.clone();
        let antinodes = calculate_antinodes(&mut grid_ref, width, height, &positions);

        assert_eq!(antinodes, 0); // No antinodes expected in an empty grid.
    }

    #[test]
    fn test_calculate_antinodes_from_file() {
        // Simulate the file path and expected output
        let input_file = "docs/challenge_1.txt";
        let (mut grid, width, height, positions) = read_input_file(input_file);

        // Call the function to get the number of antinodes
        let antinodes = calculate_antinodes(&mut grid, width, height, &positions);

        // Assert that the result matches the expected answer
        assert_eq!(antinodes, 426);
    }
}
