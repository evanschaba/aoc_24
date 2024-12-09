/// Represents an item on the disk, either a file or free space.
#[derive(Copy, Clone, Debug)]
enum DiskItem {
    /// A file, identified by its ID and size.
    File(i64, i64),
    /// Free space of a certain size.
    FreeSpace(i64),
}

/// Parses the input string into a vector of `DiskItem` enums.
fn parse(input: &str) -> Result<Vec<DiskItem>, String> {
    let mut file_id = 0;
    let mut items = Vec::new();

    for (index, character) in input.trim().chars().enumerate() {
        match character.to_digit(10) {
            Some(size) => {
                if index % 2 == 0 {
                    let file = DiskItem::File(file_id, size as i64);
                    file_id += 1;
                    items.push(file);
                } else {
                    items.push(DiskItem::FreeSpace(size as i64));
                }
            }
            None => return Err(format!("Invalid character at index {}: {}", index, character)),
        }
    }

    Ok(items)
}

/// Compacts files by moving them into free spaces on the disk.
fn compact_files(disk_items: &mut Vec<DiskItem>) -> Result<Vec<DiskItem>, String> {
    if disk_items.is_empty() {
        return Ok(vec![]);
    }

    let mut items = vec![];
    let mut left = 0;
    let mut right = disk_items.len().saturating_sub(1);

    while left <= right {
        match disk_items[left] {
            DiskItem::File(_, _) => {
                items.push(disk_items[left]);
                left += 1;
            }
            DiskItem::FreeSpace(free_space_left) => {
                if right == 0 {
                    break; // Prevent index underflow
                }
                match &disk_items[right] {
                    DiskItem::FreeSpace(_) => {
                        right = right.saturating_sub(1);
                    }
                    DiskItem::File(file_id, file_size) => {
                        match free_space_left.cmp(file_size) {
                            std::cmp::Ordering::Less => {
                                items.push(DiskItem::File(*file_id, free_space_left));
                                disk_items[right] = DiskItem::File(*file_id, file_size - free_space_left);
                                left += 1;
                            }
                            std::cmp::Ordering::Greater => {
                                items.push(DiskItem::File(*file_id, *file_size));
                                disk_items[left] = DiskItem::FreeSpace(free_space_left - *file_size);
                                right = right.saturating_sub(1);
                            }
                            std::cmp::Ordering::Equal => {
                                items.push(DiskItem::File(*file_id, *file_size));
                                left += 1;
                                right = right.saturating_sub(1);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(items)
}

/// Calculates the checksum based on the compacted disk items.
fn calculate_checksum(compacted_items: &[DiskItem]) -> Result<(i32, i64), String> {
    let mut checksum = 0;
    let mut position = 0;
    let mut offset = 0;

    for item in compacted_items {
        match item {
            DiskItem::File(file_id, file_size) => {
                let partial_sum = (file_size - 1) * file_size / 2;
                checksum += file_id * (offset * file_size + partial_sum);
                position += 1;
                offset += file_size;
            }
            DiskItem::FreeSpace(_) => {
                return Err("Invalid state: Free space in compacted items".into());
            }
        }
    }

    Ok((position, checksum))
}

/// Solves the problem by parsing the input, compacting the disk items, and calculating the checksum.
fn solve(input: &str) -> i64 {
    match parse(input) {
        Ok(mut disk_items) => {
            match compact_files(&mut disk_items) {
                Ok(compacted_items) => {
                    match calculate_checksum(&compacted_items) {
                        Ok((_, checksum)) => checksum,
                        Err(err) => {
                            eprintln!("Error calculating checksum: {}", err);
                            0
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error compacting files: {}", err);
                    0
                }
            }
        }
        Err(err) => {
            eprintln!("Error parsing input: {}", err);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_empty_input() {
        let input = "";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn test_single_file() {
        let input = "1";
        assert_eq!(solve(input), 0); // Checksum for a single file with no space.
    }

    #[test]
    fn test_sample() {
        let input = "12345";
        assert_eq!(solve(input), 60); // Example checksum for the given input
    }

    #[test]
    fn test_example() {
        let input = "2333133121414131402";
        assert_eq!(solve(input), 1928);
    }

    #[test]
    fn test_with_file_input() {
        // Read the contents of the input file
        let content = fs::read_to_string("docs/challenge_1.txt")
            .expect("Failed to read input file");

        // Call the algorithm and check the output
        assert_eq!(solve(&content.trim()), 6323641412437);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let content = std::fs::read_to_string(&args[1]).expect("Failed to read input file");
    let result = solve(&content.trim());
    println!("Result: {}", result);
}
