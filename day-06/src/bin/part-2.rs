use std::env;
use std::fs;

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return;
    }
    let input_file = &args[1];
    let _input = fs::read_to_string(input_file).expect("Failed to read input file");
}
