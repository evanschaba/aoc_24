use std::collections::HashMap;

/// Reads the input file provided as a command-line argument.
/// Returns the content of the file as a `String`.
pub fn read_file_from_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    std::fs::read_to_string(&args[1]).expect("Failed to read input file")
}

/// Reads the input file directly using the provided path.
/// Returns the content of the file as a `String`.
pub fn read_file_from_path(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read input file")
}

/// Recursive function to calculate the number of ways to assemble a design.
/// `design_bytes` represents the target design as a byte slice.
/// `available_towel_patterns` is a list of towel patterns as byte slices.
/// `memoization_cache` stores previously computed results to optimize performance.
pub fn count_ways_to_assemble<'a>(
    design_bytes: &'a [u8],
    patterns: &[&[u8]],
    cache: &mut HashMap<&'a [u8], usize>,
) -> usize {
    if design_bytes.is_empty() {
        return 1;
    }

    if let Some(&cached_count) = cache.get(&design_bytes) {
        return cached_count;
    }

    let result = patterns
        .iter()
        .filter(|&pattern| design_bytes.starts_with(pattern))
        .map(|&pattern| count_ways_to_assemble(&design_bytes[pattern.len()..], patterns, cache))
        .sum();

    cache.insert(design_bytes, result);
    result
}

/// count how many designs can be assembled.
pub fn solve(input: &str) -> usize {
    let (towel_patterns, designs) = input.split_once("\n\n").unwrap_or(("", ""));
    let available_towel_patterns = towel_patterns
        .split(", ")
        .map(str::as_bytes)
        .collect::<Vec<_>>();
    let mut cache = HashMap::new();
    let mut result = 0; // possible designs count

    for design in designs.lines() {
        let ways = count_ways_to_assemble(design.as_bytes(), &available_towel_patterns, &mut cache);
        if ways > 0 {
            result += 1;
        }
    }

    result
}

fn main() {
    let input_data = read_file_from_args();

    println!("result: {}", solve(&input_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_empty_input() {
        const EMPTY: &str = "";
        assert_eq!(solve(EMPTY), 0);
    }

    #[test]
    fn test_with_single_pattern() {
        const INPUT: &str = r"b

b";
        assert_eq!(solve(INPUT), 1);
    }

    #[test]
    fn test_with_example_1() {
        const EXAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

        assert_eq!(solve(EXAMPLE), 6);
    }

    #[test]
    fn test_with_input_1() {
        assert_eq!(solve(&read_file_from_path("docs/challenge_1.txt")), 263);
    }
}
