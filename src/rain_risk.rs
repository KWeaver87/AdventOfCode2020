use std::collections::HashMap;

struct FerryTraker {
    x: isize,
    y: isize,
    dir: char,
}

fn distance_traveled(nav_instructions: Vec<String>) -> usize {
    let parsed_nav: Vec<(char, isize)> = nav_instructions
        .into_iter()
        .map(|n_i| parse_nav_instr(n_i))
        .collect();

    let start_pos = FerryTraker {
        x: 0,
        y: 0,
        dir: 'E',
    };

    let end_pos = navigate(parsed_nav, start_pos);

    (end_pos.x.abs() + end_pos.y.abs()) as usize
}

fn navigate(nav_instructions: Vec<(char, isize)>, position: FerryTraker) -> FerryTraker {
    nav_instructions
        .into_iter()
        .fold(position, |pos, n_i| match n_i.0 {
            'N' | 'S' | 'E' | 'W' => travel_direction(n_i, pos),
            'L' | 'R' => change_heading(n_i, pos),
            'F' => move_forward(n_i, pos),
            _ => panic!("Invalid instruction char"),
        })
}

fn travel_direction(instruction: (char, isize), position: FerryTraker) -> FerryTraker {
    let pos = match instruction.0 {
        'N' => (position.x + instruction.1, position.y),
        'S' => (position.x - instruction.1, position.y),
        'E' => (position.x, position.y + instruction.1),
        'W' => (position.x, position.y - instruction.1),
        _ => panic!("Invalid direction char"),
    };

    FerryTraker {
        x: pos.0,
        y: pos.1,
        ..position
    }
}

fn change_heading(instruction: (char, isize), position: FerryTraker) -> FerryTraker {
    let dir_map: HashMap<char, isize> = [('N', 0), ('E', 90), ('S', 180), ('W', 270)]
        .iter()
        .cloned()
        .collect();

    let dir_n = dir_map.get(&position.dir).unwrap();

    let pos_neg = if instruction.0 == 'L' { -1 } else { 1 };
    let calc_dir = dir_n + (instruction.1 * pos_neg);
    let new_dir_n = ((calc_dir % 360) + 360) % 360;
    let new_dir = dir_map.into_iter().find(|d| d.1 == new_dir_n).unwrap().0;

    FerryTraker {
        dir: new_dir,
        ..position
    }
}

fn move_forward(instruction: (char, isize), position: FerryTraker) -> FerryTraker {
    travel_direction((position.dir, instruction.1), position)
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
}
