use std::collections::{HashMap, HashSet};

// Directions for robot movement
const DIRECTION_LEFT: char = '<';
const DIRECTION_RIGHT: char = '>';
const DIRECTION_UP: char = '^';
const DIRECTION_DOWN: char = 'v';

// Map elements
const TILE_WALL: char = '#';
const TILE_BOX: char = 'O';
const TILE_ROBOT: char = '@';
const TILE_EMPTY: char = '.';

const TILE_BOX_LEFT: char = '[';
const TILE_BOX_RIGHT: char = ']';

#[derive(Clone)]
struct WareHouse {
    layout: Vec<Vec<char>>, // The 2D representation of the warehouse map
    robot_x: usize,         // Robot's x-coordinate
    robot_y: usize,         // Robot's y-coordinate
    map_width: usize,       // Width of the map
    map_height: usize,      // Height of the map
}

impl WareHouse {
    // Print the current state of the map
    fn write_map(&self) {
        for y in 0..self.map_height {
            for x in 0..self.map_width {
                if self.robot_x != x || self.robot_y != y {
                    print!("{}", self.layout[y][x]);
                } else {
                    print!("{}", TILE_ROBOT);
                }
            }
            //    println!("");
        }
    }

    // Calculate the next position given a direction
    fn robot_check_position(
        x: usize,
        y: usize,
        direction_x: isize,
        direction_y: isize,
    ) -> (usize, usize) {
        let next_x = (x as isize + direction_x) as usize;
        let next_y = (y as isize + direction_y) as usize;
        (next_x, next_y)
    }

    // Move the robot in the specified direction
    fn robot_move(&mut self, direction: char) {
        let (direction_x, direction_y) = match direction {
            DIRECTION_LEFT => (-1, 0),
            DIRECTION_RIGHT => (1, 0),
            DIRECTION_UP => (0, -1),
            DIRECTION_DOWN => (0, 1),
            _ => unreachable!(),
        };

        let mut affected_boxes = HashSet::new();
        self.boxes_collect(
            self.robot_x,
            self.robot_y,
            direction_x,
            direction_y,
            &mut affected_boxes,
        );

        let (next_x, next_y) =
            Self::robot_check_position(self.robot_x, self.robot_y, direction_x, direction_y);

        if affected_boxes.is_empty() {
            if self.layout[next_y][next_x] == TILE_WALL {
                return; // Robot cannot move into a wall
            }
        } else {
            if !self.boxes_check_movable(&affected_boxes, direction_x, direction_y) {
                return; // Boxes cannot move further
            }
            self.boxes_move(&affected_boxes, direction_x, direction_y);
        }

        // Update the robot's position
        self.robot_x = next_x;
        self.robot_y = next_y;
    }

    // Shift the boxes in the specified direction
    fn boxes_move(
        &mut self,
        boxes: &HashSet<(usize, usize)>,
        direction_x: isize,
        direction_y: isize,
    ) {
        let mut current_positions = HashMap::new();
        for &(box_x, box_y) in boxes {
            current_positions.insert((box_x, box_y), self.layout[box_y][box_x]);
        }

        for &(box_x, box_y) in boxes {
            let (next_x, next_y) =
                Self::robot_check_position(box_x, box_y, direction_x, direction_y);
            self.layout[next_y][next_x] = current_positions[&(box_x, box_y)];

            let (prev_x, prev_y) =
                Self::robot_check_position(box_x, box_y, -direction_x, -direction_y);
            if current_positions.contains_key(&(prev_x, prev_y)) {
                self.layout[box_y][box_x] = current_positions[&(prev_x, prev_y)];
            } else {
                self.layout[box_y][box_x] = TILE_EMPTY;
            }
        }
    }

    // Check if the boxes can move in the given direction
    fn boxes_check_movable(
        &self,
        boxes: &HashSet<(usize, usize)>,
        direction_x: isize,
        direction_y: isize,
    ) -> bool {
        for &(box_x, box_y) in boxes {
            let (next_x, next_y) =
                Self::robot_check_position(box_x, box_y, direction_x, direction_y);
            if self.layout[next_y][next_x] == TILE_WALL {
                return false; // A box cannot move into a wall
            }
        }
        true
    }

    // Collect all boxes that are affected by the robot's movement
    fn boxes_collect(
        &self,
        x: usize,
        y: usize,
        direction_x: isize,
        direction_y: isize,
        collected_boxes: &mut HashSet<(usize, usize)>,
    ) {
        let (adjacent_x, adjacent_y) = Self::robot_check_position(x, y, direction_x, direction_y);

        let touching_positions = match self.layout[adjacent_y][adjacent_x] {
            TILE_BOX => vec![(adjacent_x, adjacent_y)],
            TILE_BOX_RIGHT => vec![(adjacent_x, adjacent_y), (adjacent_x - 1, adjacent_y)],
            TILE_BOX_LEFT => vec![(adjacent_x, adjacent_y), (adjacent_x + 1, adjacent_y)],
            _ => Vec::new(),
        };

        for position in touching_positions {
            if !collected_boxes.contains(&position) {
                collected_boxes.insert(position);
                self.boxes_collect(
                    position.0,
                    position.1,
                    direction_x,
                    direction_y,
                    collected_boxes,
                );
            }
        }
    }
}
// Parse the map from a textual representation
fn parse_input(input: &str) -> WareHouse {
    let mut layout: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let map_height = layout.len();
    let map_width = layout[0].len();

    // Find the robot's initial position and replace it with an empty tile
    for y in 0..map_height {
        for x in 0..map_width {
            if layout[y][x] == TILE_ROBOT {
                layout[y][x] = TILE_EMPTY;
                return WareHouse {
                    layout,
                    map_width,
                    map_height,
                    robot_x: x,
                    robot_y: y,
                };
            }
        }
    }
    panic!("Robot's starting position not found on the map");
}

// Compute the GPS sum for all boxes on the map
fn calc_gps_coord_boxes_sum(wh: &WareHouse) -> usize {
    let mut gps_sum: usize = 0;
    for y in 0..wh.map_height {
        for x in 0..wh.map_width {
            if [TILE_BOX, TILE_BOX_LEFT].contains(&wh.layout[y][x]) {
                gps_sum += (100 * y) + x;
            }
        }
    }
    gps_sum
}

// Simulate part one of the puzzle
fn simulate(mut wh: WareHouse, moves: &Vec<char>) -> usize {
    for &movement in moves {
        wh.robot_move(movement);
    }

    wh.write_map();

    calc_gps_coord_boxes_sum(&wh)
}

fn get_simulation_input(input: &str) -> (WareHouse, Vec<char>) {
    // Find the first occurrence of a blank line in the input (two consecutive newlines)
    let blank_line = input.trim().find("\n\n").unwrap();

    // Parse the warehouse map from the input before the blank line
    let wh = parse_input(&input[..blank_line]);

    // Get the movement instructions from the part of the input after the blank line
    let movements = input[blank_line..]
        .trim() // Remove leading and trailing whitespace
        .chars() // Convert the string into an iterator of characters
        .filter(|c| *c != '\n') // Remove newline characters
        .collect::<Vec<char>>(); // Collect the remaining characters into a vector

    (wh, movements)
}

fn solve(input: &str) -> usize {
    let (wh, movements) = get_simulation_input(input);
    simulate(wh.clone(), &movements)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input(file_name: &str) -> String {
        // Read the content of the input file and return it as a String
        std::fs::read_to_string(file_name).expect("Failed to read input file")
    }

    #[test]
    fn test_with_input() {
        assert_eq!(solve(&get_input("./docs/challenge_1.txt")), 1509074)
    }

    #[test]
    fn test_with_example() {
        const EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        assert_eq!(solve(&EXAMPLE), 10092);
    }

    #[test]
    fn test_with_example_2() {
        const EXAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        assert_eq!(solve(&EXAMPLE), 2028);
    }

    #[test]
    fn test_with_gps_coords() {
        let map = "##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########";
        let wh = parse_input(map);
        assert_eq!(calc_gps_coord_boxes_sum(&wh), 10092);
    }
}

fn main() {
    // Collect command-line arguments into a vector, expecting the first argument to be the input file name
    let args: Vec<String> = std::env::args().collect();

    // Check if there are less than 2 arguments (program name + input file name)
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]); // Print usage message
        std::process::exit(1); // Exit with error code 1 if no input file is provided
    }

    // Read the content of the input file specified as the first argument
    let input = std::fs::read_to_string(&args[1]).expect("Failed to read input file");

    // Call the solve function with the parsed map and the movement instructions
    let result = solve(&input);

    println!("result: {}", result);
}
