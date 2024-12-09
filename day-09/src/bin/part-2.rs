/// Represents an item on the disk, either a file or free space.
#[derive(Copy, Clone, Debug)]
enum DiskItem {
    File(i64, i64), // File with its ID and size.
    FreeSpace(i64), // Free space with its size.
}

/// Parses the input string into a vector of `DiskItem`s.
fn parse(input: &str) -> Vec<DiskItem> {
    let mut file_id = 0;
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(index, char)| {
            let size: i64 = char.to_digit(10).unwrap() as i64;
            if index % 2 == 0 {
                assert!(size > 0, "File size must be greater than 0");
                let item = DiskItem::File(file_id, size);
                file_id += 1;
                item
            } else {
                DiskItem::FreeSpace(size)
            }
        })
        .collect()
}

/// Consolidates consecutive free spaces into a single free space item.
fn consolidate_items(items: &mut Vec<DiskItem>, temp_storage: &mut Vec<DiskItem>) {
    // Move items from the main vector to temporary storage in reverse order.
    while let Some(item) = items.pop() {
        temp_storage.push(item);
    }

    // Merge items back from the temporary storage into the main vector.
    while let Some(item) = temp_storage.pop() {
        if items.is_empty() {
            items.push(item);
            continue;
        }

        match item {
            DiskItem::File(_, _) => items.push(item),
            DiskItem::FreeSpace(size) => {
                if let Some(DiskItem::FreeSpace(existing_size)) = items.last_mut() {
                    *existing_size += size; // Merge with the previous free space.
                } else {
                    items.push(DiskItem::FreeSpace(size));
                }
            }
        }
    }
}

/// Solves the problem by processing input to find the checksum of the compacted disk.
fn solve(input: &str) -> i64 {
    let mut items: Vec<DiskItem> = parse(input);
    let mut temp_storage: Vec<DiskItem> = Vec::with_capacity(items.len());
    let item_count = items.len();

    // Iterate through items from right to left to move files.
    for index in (0..item_count).rev() {
        match items[index] {
            DiskItem::FreeSpace(_) => continue, // Skip free space.
            DiskItem::File(_, size) => {
                // Search for a suitable free space to the left.
                let mut slot_index = 0;
                let mut found = false;
                let mut empty_space_size = 0;

                for j in 0..index {
                    if let DiskItem::FreeSpace(space) = items[j] {
                        if space >= size {
                            slot_index = j;
                            found = true;
                            empty_space_size = space;
                            break;
                        }
                    }
                }

                // If a suitable slot was found, move the file to it.
                if found {
                    if empty_space_size == size {
                        items[slot_index] = items[index];
                        items[index] = DiskItem::FreeSpace(size);
                    } else {
                        // Insert the file at the found slot and split the free space.
                        items.insert(slot_index, items[index]);
                        items[slot_index + 1] = DiskItem::FreeSpace(empty_space_size - size);
                        items[index + 1] = DiskItem::FreeSpace(size);
                    }
                }
            }
        }

        // Consolidate consecutive free spaces after processing each item.
        consolidate_items(&mut items, &mut temp_storage);
    }

    // Calculate the checksum of the final state.
    let (_, checksum) = calculate_checksum(&items);

    checksum
}

/// Calculates the checksum of the items on the disk.
fn calculate_checksum(items: &[DiskItem]) -> (i32, i64) {
    let mut checksum = 0;
    let mut position = 0;
    let mut offset = 0;

    for item in items {
        match item {
            DiskItem::FreeSpace(space) => {
                position += 1;
                offset += space;
            }
            DiskItem::File(id, size) => {
                // Calculate partial sum for the file size.
                let partial_sum = (size - 1) * size / 2;
                checksum += id * (offset * size + partial_sum);
                position += 1;
                offset += size;
            }
        }
    }

    (position, checksum)
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
        assert_eq!(solve(input), 0);
    }
    
    #[test]
    fn test_sample() {
        let input = "12345";
        assert_eq!(solve(input), 132);
    }

    #[test]
    fn test_example() {
        let input = "2333133121414131402";
        assert_eq!(2858, solve(input));
    }

    #[test]
    fn test_with_file_input() {
        let content = fs::read_to_string("docs/challenge_2.txt").expect("Failed to read input file");
        assert_eq!(solve(&content.trim()), 6351801932670);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let content = std::fs::read_to_string(&args[1]).expect("Failed to read input file");
    let result = solve(content.trim());
    println!("Result: {}", result);
}
