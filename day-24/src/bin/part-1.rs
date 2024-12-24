use std::collections::{HashMap, VecDeque};

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

/// Decodes the wires starting with a given prefix into a decimal number.
/// The wires are sorted by name and their boolean values are treated as bits.
fn decode_wires(wires: &HashMap<&str, bool>, prefix: &str) -> u64 {
    let mut selected_wires = wires
        .iter()
        .filter(|(name, _)| name.starts_with(prefix))
        .collect::<Vec<_>>();

    selected_wires.sort(); // Sort wires by name
    selected_wires.reverse(); // Reverse to handle bits in big-endian order

    let mut result = 0u64;
    for (_, &value) in selected_wires {
        result <<= 1; // Shift left to make space for the next bit
        if value {
            result += 1; // Set the bit if the value is true
        }
    }

    result
}

/// Computes the output by processing gates sequentially. The gates are applied
/// until all values are calculated.
fn compute_result<'a>(
    wires: &HashMap<&'a str, bool>,
    gates: &[(&'a str, &'a str, &'a str, GateLogic)],
    renamed_wires: &HashMap<&'a str, &'a str>,
) -> u64 {
    let mut wires = wires.clone(); // Clone the wire data to manipulate it
    let mut queue = VecDeque::new(); // Queue to handle gates in order

    // Push gates into the queue for processing
    for (input1, input2, output, logic) in gates {
        queue.push_back((input1, input2, output, *logic)); // Store gates with logic operations
    }

    // Process each gate in the queue
    while let Some((input1, input2, output, logic)) = queue.pop_front() {
        let output = renamed_wires.get(output).unwrap_or(output); // Resolve renamed wires

        // Check if both inputs are available
        let Some(&input1_val) = wires.get(input1) else {
            queue.push_back((input1, input2, output, logic)); // Requeue gate if input1 is not yet available
            continue;
        };
        let Some(&input2_val) = wires.get(input2) else {
            queue.push_back((input1, input2, output, logic)); // Requeue gate if input2 is not yet available
            continue;
        };

        // Apply the gate logic based on the operation
        let result = match logic {
            GateLogic::And => input1_val && input2_val,
            GateLogic::Or => input1_val || input2_val,
            GateLogic::Xor => input1_val != input2_val,
        };

        // Store the computed result
        wires.insert(output, result);
    }

    // Decode the final result from the wires starting with 'z'
    decode_wires(&wires, "z")
}

/// Simulates the gates and computes the output from wires starting with `z`.
/// This function processes the wire and gate data and computes the result.
fn solve(input: &str) -> u64 {
    let (wire_data, gate_data) = input.split_once("\n\n").unwrap(); // Split input into wire and gate sections

    // Parse wire values (initial state)
    let wires = wire_data
        .lines()
        .map(|line| {
            let parts = line.split_once(": ").unwrap();
            (parts.0, parts.1 == "1") // Parse wire name and boolean value
        })
        .collect::<HashMap<_, _>>();

    // Parse gates (logic operations)
    let gates = gate_data
        .lines()
        .map(|line| {
            let (left, output) = line.split_once(" -> ").unwrap();
            let segments = left.split_whitespace().collect::<Vec<_>>();
            let logic = match segments[1] {
                "AND" => GateLogic::And,
                "OR" => GateLogic::Or,
                "XOR" => GateLogic::Xor,
                _ => panic!("Invalid gate: {}", left), // Catch invalid gate operations
            };
            (segments[0], segments[2], output, logic)
        })
        .collect::<Vec<_>>();

    // Compute the result based on wire and gate data
    compute_result(&wires, &gates, &HashMap::new())
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
    fn test_with_example() {
        let input = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"#;

        // Expected result after processing the gates and wires
        let expected_result: u64 = 2024;
        let result = solve(input);
        assert_eq!(
            result, 2024,
            "Expected: {}, Got: {}",
            expected_result, result
        );
    }

    #[test]
    fn test_with_input() {
        let input = read_file_by_path("docs/challenge_1.txt");
        assert_eq!(solve(&input), 59336987801432);
    }
}
