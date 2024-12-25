/// Enum representing either a Lock or a Key with their respective heights.
#[derive(Debug)]
enum Schematic {
    Key([usize; 5]),
    Lock([usize; 5]),
}

/// Reads the input file specified in the command line arguments.
pub fn read_input_file() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    std::fs::read_to_string(&args[1]).expect("Failed to read input file")
}

/// Reads the input file from the provided file path.
pub fn read_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read input file")
}

/// Solves the problem by counting all valid lock/key pairs.
fn count_valid_pairs(input: &str) -> usize {
    let (locks, keys) = parse_schematics(input);

    // Check all lock/key combinations for compatibility
    locks
        .iter()
        .flat_map(|lock| keys.iter().map(move |key| (lock, key)))
        .filter(|(lock, key)| match (lock, key) {
            (Schematic::Lock(lock_heights), Schematic::Key(key_heights)) => {
                for i in 0..5 {
                    // A key must fit in every column
                    if lock_heights[i] < key_heights[i] {
                        return false;
                    }
                }
                true
            }
            _ => unreachable!(),
        })
        .count()
}

/// Parses the input into locks and keys, represented as height arrays.
fn parse_schematics(input: &str) -> (Vec<Schematic>, Vec<Schematic>) {
    let mut locks = vec![];
    let mut keys = vec![];

    // Split the input into groups (blocks of schematics)
    let schematic_groups = input.split("\n\n");

    for group in schematic_groups {
        let mut heights = [0, 0, 0, 0, 0];
        let rows: Vec<&str> = group.lines().collect();
        let is_lock = rows[0] == "#####"; // Identify if the group is a lock or a key
        let sentinel = if is_lock { '.' } else { '#' };

        // Calculate the heights for each column
        for (i, row) in group.lines().enumerate() {
            let height = 7 - i; // Height decreases from top to bottom
            for (j, c) in row.char_indices() {
                if c == sentinel && height > heights[j] {
                    heights[j] = height;
                }
            }
        }

        // Store as either a Lock or Key based on the schematic type
        if is_lock {
            locks.push(Schematic::Lock(heights));
        } else {
            keys.push(Schematic::Key(heights));
        }
    }

    (locks, keys)
}

/// Entry point of the program.
fn main() {
    let input = read_input_file();
    println!("Valid lock/key pairs: {}", count_valid_pairs(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        const EXAMPLE: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;
        assert_eq!(count_valid_pairs(EXAMPLE), 3);
    }

    #[test]
    fn test_with_actual_input() {
        let input = read_file("docs/challenge_1.txt");
        assert_eq!(count_valid_pairs(&input), 3690);
    }
}
