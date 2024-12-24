use std::collections::{HashMap, HashSet};
use std::iter::once;

/// Reads the input file from command-line arguments
pub fn read_file_from_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    std::fs::read_to_string(&args[1]).expect("Failed to read input file")
}

/// Reads the input file from a specified file path
pub fn read_file_from_path(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read input file")
}

/// Finds the largest fully connected set of computers and returns the password
pub fn solve(input: &str) -> String {
    let (map, _) = build_adj_map(input);

    // To store the largest fully connected set of nodes
    let mut largest_clique = HashSet::new();

    find_largest_clique(
        HashSet::new(),
        map.keys().copied().collect(),
        HashSet::new(),
        &map,
        &mut largest_clique,
    );

    // Convert the largest clique to a sorted, comma-separated password
    let mut pwds: Vec<_> = largest_clique.into_iter().collect();
    pwds.sort_unstable();
    pwds.join(",")
}

/// Recursive function to find the largest fully connected set of nodes
fn find_largest_clique<'a>(
    current_clique: HashSet<&'a str>,
    mut candidates: HashSet<&'a str>,
    mut excluded: HashSet<&'a str>,
    map: &HashMap<&str, HashSet<&'a str>>,
    largest_clique: &mut HashSet<&'a str>,
) {
    if candidates.is_empty() && excluded.is_empty() {
        if current_clique.len() > largest_clique.len() {
            *largest_clique = current_clique;
        }
        return;
    }

    for &node in candidates.clone().iter() {
        let neighbors = &map[node];
        find_largest_clique(
            current_clique.iter().copied().chain(once(node)).collect(),
            candidates.intersection(neighbors).copied().collect(),
            excluded.intersection(neighbors).copied().collect(),
            map,
            largest_clique,
        );
        candidates.remove(node);
        excluded.insert(node);
    }
}

/// Builds the adjacency map from input and identifies all computers whose names start with 't'
fn build_adj_map(input: &str) -> (HashMap<&str, HashSet<&str>>, HashSet<&str>) {
    let mut map = HashMap::<&str, HashSet<&str>>::new();
    let mut t_computers = HashSet::new();

    // Parse and build adjacency map
    for line in input.lines() {
        if let Some((a, b)) = line.split_once('-') {
            map.entry(a).or_default().insert(b);
            map.entry(b).or_default().insert(a);

            // Track computers with names starting with 't'
            if a.starts_with('t') {
                t_computers.insert(a);
            }
            if b.starts_with('t') {
                t_computers.insert(b);
            }
        }
    }

    (map, t_computers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_example() {
        const EXAMPLE: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

        assert_eq!(solve(EXAMPLE), "co,de,ka,ta");
    }

    #[test]
    fn test_with_input() {
        let input = read_file_from_path("docs/challenge_2.txt");
        assert_eq!(solve(&input), "af,aq,ck,ee,fb,it,kg,of,ol,rt,sc,vk,zh");
    }
}

fn main() {
    let input = read_file_from_args();
    println!("result: {}", solve(&input));
}
