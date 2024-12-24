use std::collections::{HashMap, HashSet};

/// Enum representing the logic gates used in the system.
#[derive(Clone, Copy, PartialEq, Eq)]
enum GateLogic {
    And, // AND operation
    Or,  // OR operation
    Xor, // XOR operation
}

/// Reads the input file specified in the command line arguments.
pub fn read_file_from_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    std::fs::read_to_string(&args[1]).expect("Failed to read input file")
}

/// Reads the input file from the provided path.
pub fn read_file_by_path(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read input file")
}

/// Solves the problem by analyzing the system of gates and wire connections.
fn solve(input: &str) -> String {
    // Split the input into wire data and gate data
    let (wire_data, gate_data) = input.split_once("\n\n").unwrap();

    // Create a map of wire states (x, y) to boolean values
    let _ = wire_data
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").unwrap();
            (parts.0, parts.1 == "1")
        })
        .collect::<HashMap<_, _>>();

    // Parse the gate instructions into a vector of gate data
    let gates = gate_data
        .lines()
        .map(|line| {
            let (left, output) = line.split_once(" -> ").unwrap();
            let segments = left.split_whitespace().collect::<Vec<_>>();
            let logic = match segments[1] {
                "AND" => GateLogic::And,
                "OR" => GateLogic::Or,
                "XOR" => GateLogic::Xor,
                _ => panic!("Invalid gate: {}", left),
            };
            (segments[0], segments[2], output, logic)
        })
        .collect::<Vec<_>>();

    // Create a map to track connections for each wire
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for gate in &gates {
        connections.entry(gate.0).or_default().push(gate.2);
        connections.entry(gate.1).or_default().push(gate.2);
    }

    let mut broken_nodes = HashSet::new();

    // Check each gate for broken node patterns
    for gate in &gates {
        // z nodes must be XOR (except for z45)
        if gate.2.starts_with("z") && gate.2 != "z45" && gate.3 != GateLogic::Xor {
            println!("Broken node detected: {}", gate.2); // Debugging
            broken_nodes.insert(gate.2);
        }

        // z nodes must not be inputs of other nodes
        if gate.0.starts_with("z") {
            broken_nodes.insert(gate.0);
        }
        if gate.1.starts_with("z") {
            broken_nodes.insert(gate.1);
        }

        // Inputs of XOR nodes must be x and y nodes, excluding z nodes
        if gate.3 == GateLogic::Xor
            && !gate.2.starts_with("z")
            && !((gate.0.starts_with("x") && gate.1.starts_with("y"))
                || (gate.0.starts_with("y") && gate.1.starts_with("x")))
        {
            broken_nodes.insert(gate.2);
        }

        // XOR nodes must have exactly two inputs
        if gate.3 == GateLogic::Xor && !gate.2.starts_with("z") && connections[gate.2].len() != 2 {
            broken_nodes.insert(gate.2);
        }

        // AND nodes must have exactly one input, except for specific cases
        if gate.3 == GateLogic::And
            && !gate.2.starts_with("z")
            && connections[gate.2].len() != 1
            && !((gate.0 == "x00" && gate.1 == "y00") || (gate.0 == "y00" && gate.1 == "x00"))
        {
            broken_nodes.insert(gate.2);
        }
    }

    // Sort the broken nodes alphabetically and return as a comma-separated string
    let mut broken_nodes = broken_nodes.into_iter().collect::<Vec<_>>();
    broken_nodes.sort();
    broken_nodes.join(",")
}

fn main() {
    let input = read_file_from_args();

    println!("result: {}", solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simulating the gates and producing the expected output on the z wires
    #[test]
    fn test_part_example() {
        let input = r#"x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00"#;

        // Expected result after fixing swapped wires
        let expected_swap_wires = "z00,z01,z02,z03,z04,z05";
        let result = solve(input);
        assert_eq!(
            result, expected_swap_wires,
            "Expected: {}, Got: {}",
            expected_swap_wires, result
        );
    }

    #[test]
    fn test_with_input() {
        let input = read_file_by_path("docs/challenge_2.txt");
        assert_eq!(solve(&input), "ctg,dmh,dvq,rpb,rpv,z11,z31,z38");
    }
}
