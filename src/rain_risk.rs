use std::collections::HashMap;

struct FerryTracker {
    x: isize,
    y: isize,
    dir: char,
    way_x: isize,
    way_y: isize,
}

fn distance_traveled(nav_instructions: Vec<String>) -> usize {
    let parsed_nav: Vec<(char, isize)> = nav_instructions
        .into_iter()
        .map(|n_i| parse_nav_instr(n_i))
        .collect();

    let start_pos = FerryTracker {
        x: 0,
        y: 0,
        dir: 'E',
        way_x: 0,
        way_y: 0,
    };

    let end_pos = navigate(parsed_nav, start_pos);

    (end_pos.x.abs() + end_pos.y.abs()) as usize
}

fn navigate(nav_instructions: Vec<(char, isize)>, position: FerryTracker) -> FerryTracker {
    nav_instructions
        .into_iter()
        .fold(position, |pos, n_i| match n_i.0 {
            'N' | 'S' | 'E' | 'W' => travel_direction(n_i, pos),
            'L' | 'R' => change_heading(n_i, pos),
            'F' => move_forward(n_i, pos),
            _ => panic!("Invalid instruction char"),
        })
}

fn travel_direction(instruction: (char, isize), position: FerryTracker) -> FerryTracker {
    let pos = match instruction.0 {
        'N' => (position.x + instruction.1, position.y),
        'S' => (position.x - instruction.1, position.y),
        'E' => (position.x, position.y + instruction.1),
        'W' => (position.x, position.y - instruction.1),
        _ => panic!("Invalid direction char"),
    };

    FerryTracker {
        x: pos.0,
        y: pos.1,
        ..position
    }
}

fn change_heading(instruction: (char, isize), position: FerryTracker) -> FerryTracker {
    let dir_map: HashMap<char, isize> = [('N', 0), ('E', 90), ('S', 180), ('W', 270)]
        .iter()
        .cloned()
        .collect();

    let dir_n = dir_map.get(&position.dir).unwrap();

    let pos_neg = if instruction.0 == 'L' { -1 } else { 1 };
    let calc_dir = dir_n + (instruction.1 * pos_neg);
    let new_dir_n = ((calc_dir % 360) + 360) % 360;
    let new_dir = dir_map.into_iter().find(|d| d.1 == new_dir_n).unwrap().0;

    FerryTracker {
        dir: new_dir,
        ..position
    }
}

fn move_forward(instruction: (char, isize), position: FerryTracker) -> FerryTracker {
    travel_direction((position.dir, instruction.1), position)
}

fn distance_traveled_waypoint(nav_instructions: Vec<String>) -> usize {
    let parsed_nav: Vec<(char, isize)> = nav_instructions
        .into_iter()
        .map(|n_i| parse_nav_instr(n_i))
        .collect();

    let start_pos = FerryTracker {
        x: 0,
        y: 0,
        dir: 'E',
        way_x: 1,
        way_y: 10,
    };

    let end_pos = navigate_waypoint(parsed_nav, start_pos);

    (end_pos.x.abs() + end_pos.y.abs()) as usize
}

/// Part2
fn navigate_waypoint(nav_instructions: Vec<(char, isize)>, position: FerryTracker) -> FerryTracker {
    nav_instructions
        .into_iter()
        .fold(position, |pos, n_i| match n_i.0 {
            'N' | 'S' | 'E' | 'W' => move_waypoint(n_i, pos),
            'L' | 'R' => rotate_waypoint(n_i, pos),
            'F' => move_to_waypoint(n_i, pos),
            _ => panic!("Invalid instruction char"),
        })
}

fn move_waypoint(instruction: (char, isize), position: FerryTracker) -> FerryTracker {
    let pos = match instruction.0 {
        'N' => (position.way_x + instruction.1, position.way_y),
        'S' => (position.way_x - instruction.1, position.way_y),
        'E' => (position.way_x, position.way_y + instruction.1),
        'W' => (position.way_x, position.way_y - instruction.1),
        _ => panic!("Invalid direction char"),
    };

    FerryTracker {
        way_x: pos.0,
        way_y: pos.1,
        ..position
    }
}

fn rotate_waypoint(instruction: (char, isize), position: FerryTracker) -> FerryTracker {
    let rotater = match instruction.0 {
        'L' => |x, y| (y, 0 - x),
        'R' => |x, y| (0 - y, x),
        _ => panic!("Unknown rotation char"),
    };

    let new_pos = rotater(position.way_x, position.way_y);
    let new_tracker = FerryTracker {
        way_x: new_pos.0,
        way_y: new_pos.1,
        ..position
    };

    let remaining_rotate = instruction.1 - 90;

    if remaining_rotate > 0 {
        rotate_waypoint((instruction.0, remaining_rotate), new_tracker)
    } else {
        new_tracker
    }
}

fn move_to_waypoint(instruction: (char, isize), position: FerryTracker) -> FerryTracker {
    let new_pos = FerryTracker {
        x: position.x + position.way_x,
        y: position.y + position.way_y,
        ..position
    };

    let remaining_moves = instruction.1 - 1;

    if remaining_moves > 0 {
        move_to_waypoint((instruction.0, remaining_moves), new_pos)
    } else {
        new_pos
    }
}

fn parse_nav_instr(nav_instruction: String) -> (char, isize) {
    let (dir, value) = nav_instruction.split_at(1);

    (dir.chars().last().unwrap(), value.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_vec_string;
    use colored::Colorize;

    static EXAMPLE_NAV_INSTRUCTIONS: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn distance_traveled_example() {
        let expected = 25;
        let nav: Vec<String> = EXAMPLE_NAV_INSTRUCTIONS
            .lines()
            .map(|l| l.to_string())
            .collect();
        let actual = distance_traveled(nav);

        assert_eq!(actual, expected);
    }

    /// Part1
    #[test]
    fn distance_traveled_from_input() {
        let expected = 415;

        let nav = load_as_vec_string("day12");
        let actual = distance_traveled(nav);
        println!(
            "{}{}",
            "Manhattan distance traveled: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn distance_traveled_waypoint_example() {
        let expected = 286;
        let nav: Vec<String> = EXAMPLE_NAV_INSTRUCTIONS
            .lines()
            .map(|l| l.to_string())
            .collect();
        let actual = distance_traveled_waypoint(nav);

        assert_eq!(actual, expected);
    }

    /// Part2
    #[test]
    fn distance_traveled_waypoint_from_input() {
        let expected = 29401;

        let nav = load_as_vec_string("day12");
        let actual = distance_traveled_waypoint(nav);
        println!(
            "{}{}",
            "Manhattan distance traveled via waypoint: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
