use std::collections::{HashMap, HashSet, VecDeque};

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

/// Builds the adjacency map and identifies all computers whose names start with 't'
fn build_adj_map(input: &str) -> (HashMap<&str, HashSet<&str>>, HashSet<&str>) {
    let mut map = HashMap::<&str, HashSet<&str>>::new();
    let mut t_computers = HashSet::new();

    // Parse connections and build adjacency map
    for line in input.lines() {
        if let Some((a, b)) = line.split_once('-') {
            // Add both directions to the adjacency map
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

/// Finds the count of all triangles (fully connected sets of 3 nodes)
/// where at least one computer's name starts with 't'
pub fn solve(input: &str) -> usize {
    let (map, t_computers) = build_adj_map(input);

    // To track unique triangles (sorted to avoid duplicate representations)
    let mut unique_triangles = HashSet::new();

    // Start BFS for all 't' computers
    let mut bfs_queue: VecDeque<_> = t_computers
        .into_iter()
        .map(|node| (node, vec![node]))
        .collect();

    while let Some((current, mut path)) = bfs_queue.pop_front() {
        if path.len() == 3 {
            // Ensure the triangle is added in a unique sorted order
            path.sort_unstable();
            unique_triangles.insert(path);
            continue;
        }
        for &neighbor in map[&current].iter() {
            // Check if neighbor forms a triangle with the starting node
            if !path.contains(&neighbor) && map[&neighbor].contains(&path[0]) {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                bfs_queue.push_back((neighbor, new_path));
            }
        }
    }

    unique_triangles.len()
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

        assert_eq!(solve(EXAMPLE), 7);
    }

    #[test]
    fn test_with_input() {
        let input = read_file_from_path("docs/challenge_1.txt");
        assert_eq!(solve(&input), 1419);
    }
}

fn main() {
    let input = read_file_from_args();
    println!("result: {}", solve(&input));
}
