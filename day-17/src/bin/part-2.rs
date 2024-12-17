type RegisterA = u64; // Represents Register A
type RegisterB = u64; // Represents Register B
type RegisterC = u64; // Represents Register C

type Program = Vec<u64>; // Represents the program instructions
type Output = Vec<u64>; // Represents the result output

type ProgramData = (RegisterA, RegisterB, RegisterC, Program); // Full program state

/// Reads command-line arguments and retrieves the input file content.
fn read_input_file() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    std::fs::read_to_string(&args[1]).expect("Failed to read input file")
}

// Collect the content using input file name
pub fn read_input_from_name(name: &str) -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    std::fs::read_to_string(name).expect("Failed to read input file")
}
/// Parses the input data into initial register states and the program instructions.
fn parse(input: &str) -> ProgramData {
    let lines = input.lines().collect::<Vec<_>>();

    let register_a = lines[0][12..].parse::<RegisterA>().unwrap();
    let register_b = lines[1][12..].parse::<RegisterB>().unwrap();
    let register_c = lines[2][12..].parse::<RegisterC>().unwrap();

    let program = lines[4][9..]
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    (register_a, register_b, register_c, program)
}

/// Executes the program based on the given registers and program instructions.
/// Returns the output as a vector of u64 values.
fn execute_program(
    mut reg_a: RegisterA,
    mut reg_b: RegisterB,
    mut reg_c: RegisterC,
    program: &[u64],
) -> Output {
    let mut instruction_pointer = 0; // Tracks the current position in the program
    let mut output = Vec::new(); // Stores the output values

    // Execute instructions until the end of the program
    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];

        // Evaluate combo operand based on its value
        let combo_value = match operand {
            0..=3 => operand, // Literal values 0-3
            4 => reg_a,       // Operand refers to Register A
            5 => reg_b,       // Operand refers to Register B
            6 => reg_c,       // Operand refers to Register C
            _ => operand,     // Default invalid case
        };

        // Execute instruction based on opcode
        match opcode {
            0 => reg_a /= 2u64.pow(combo_value as u32), // adv: Divide Register A
            1 => reg_b ^= operand,                      // bxl: Bitwise XOR with literal
            2 => reg_b = combo_value % 8,               // bst: Modulo 8 operation on combo operand
            3 => {
                // jnz: Jump if Register A is not zero
                if reg_a != 0 {
                    instruction_pointer = operand as usize;
                    continue;
                }
            }
            4 => reg_b ^= reg_c, // bxc: Bitwise XOR of Register B and C
            5 => output.push(combo_value % 8), // out: Output combo value % 8
            6 => reg_b = reg_a / 2u64.pow(combo_value as u32), // bdv: Divide Register A and store in B
            7 => reg_c = reg_a / 2u64.pow(combo_value as u32), // cdv: Divide Register A and store in C
            _ => panic!("Invalid opcode encountered"),         // Handle invalid opcode
        };

        instruction_pointer += 2; // Move to the next instruction
    }

    output
}

/// Solves Part 2: Finds the lowest value for Register A that outputs a copy of the program.
fn solve(input: &str) -> u64 {
    let (_, register_b, register_c, program) = parse(input);
    let mut register_a_candidates = vec![0; program.len()];

    loop {
        let mut candidate_a = 0;

        // Generate Register A candidate value
        for (i, &factor) in register_a_candidates.iter().enumerate() {
            candidate_a += 8u64.pow(i as u32) * factor;
        }

        // Check if the candidate produces the required program output
        let output = execute_program(candidate_a, register_b, register_c, &program);
        if output == program {
            return candidate_a; // Found the correct value
        }

        // Update candidates for next iteration
        for i in (0..program.len()).rev() {
            if output.len() <= i || output[i] != program[i] {
                register_a_candidates[i] += 1;
                break;
            }
        }
    }
}

fn main() {
    let input = read_input_file();

    println!("result: {}", solve(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_example() {
        const EXAMPLE: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
        assert_eq!(solve(EXAMPLE), 117440);
    }

    #[test]
    fn test_with_input() {
        let input = read_input_from_name("docs/challenge_2.txt");
        assert_eq!(solve(&input), 107413700225434);
    }
}
