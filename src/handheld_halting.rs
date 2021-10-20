struct ProgramTracker {
    instruction_index: i32,
    accumulator: i32,
    seen_instructions: Vec<usize>,
    is_infinite_loop: bool,
}

/// Part1
fn calc_acc_before_repeat(program: Vec<String>) -> i32 {
    let parsed_instructions = program
        .into_iter()
        .map(|l| parse_instruction_line(l.to_string()))
        .collect();

    let init_tracker = ProgramTracker {
        accumulator: 0,
        instruction_index: 0,
        is_infinite_loop: false,
        seen_instructions: vec![],
    };

    let run = run_instruction(parsed_instructions, init_tracker);

    run.accumulator
}

/// Part2
fn calc_acc_reversing_nop_jmp(program: Vec<String>) -> i32 {
    let parsed_instructions: Vec<(String, i32)> = program
        .into_iter()
        .map(|l| parse_instruction_line(l.to_string()))
        .collect();

    let indices_nop_jmp: Vec<usize> = parsed_instructions
        .iter()
        .enumerate()
        .filter_map(|(i, instr)| match instr.0.as_str() {
            "nop" | "jmp" => Some(i),
            _ => None,
        })
        .collect();

    indices_nop_jmp
        .into_iter()
        .find_map(|i| {
            let mut instrs = parsed_instructions.clone();
            instrs[i].0 = match instrs[i].0.as_str() {
                "nop" => "jmp".to_string(),
                "jmp" => "nop".to_string(),
                _ => panic!("Should be only nop or jmp here"),
            };

            let init_tracker = ProgramTracker {
                accumulator: 0,
                instruction_index: 0,
                is_infinite_loop: false,
                seen_instructions: vec![],
            };

            let run = run_instruction(instrs, init_tracker);

            if run.is_infinite_loop {
                None
            } else {
                Some(run.accumulator)
            }
        })
        .unwrap()
}

fn run_instruction(instructions: Vec<(String, i32)>, tracker: ProgramTracker) -> ProgramTracker {
    let index = tracker.instruction_index as usize;
    if tracker.seen_instructions.contains(&(index)) {
        return ProgramTracker {
            is_infinite_loop: true,
            ..tracker
        };
    }
    if index >= instructions.len() {
        return tracker;
    }

    let next_run = determine_next_run(&instructions, tracker);

    run_instruction(instructions, next_run)
}

fn determine_next_run(
    instructions: &Vec<(String, i32)>,
    prev_run: ProgramTracker,
) -> ProgramTracker {
    let index = prev_run.instruction_index as usize;

    let instruction = instructions[index].clone();
    let op = instruction.0.as_str();
    let next_seen_instr = prev_run
        .seen_instructions
        .iter()
        .chain([index].iter())
        .map(|i| *i)
        .collect();

    match op {
        "acc" => ProgramTracker {
            instruction_index: prev_run.instruction_index + 1,
            accumulator: prev_run.accumulator + instruction.1,
            seen_instructions: next_seen_instr,
            ..prev_run
        },
        "jmp" => ProgramTracker {
            instruction_index: prev_run.instruction_index + instruction.1,
            seen_instructions: next_seen_instr,
            ..prev_run
        },
        "nop" => ProgramTracker {
            instruction_index: prev_run.instruction_index + 1,
            seen_instructions: next_seen_instr,
            ..prev_run
        },
        _ => panic!("Invalid operation: {}", op),
    }
}

fn parse_instruction_line(line: String) -> (String, i32) {
    let splits: Vec<&str> = line.split_whitespace().collect();
    let op = splits.get(0).unwrap().to_string();
    let arg: i32 = splits.get(1).unwrap().parse().unwrap();

    (op, arg)
}

static EXAMPLE_PROGRAM: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_vec_string;
    use colored::Colorize;

    #[test]
    fn calc_acc_before_repeat_example() {
        let expected = 5;

        let program = EXAMPLE_PROGRAM.lines().map(|l| l.to_string()).collect();
        let actual = calc_acc_before_repeat(program);

        assert_eq!(actual, expected);
    }

    #[test]
    fn calc_acc_before_repeat_from_input() {
        let expected = 1586;

        let program = load_as_vec_string("day8");
        let actual = calc_acc_before_repeat(program);
        println!(
            "{}{}",
            "Accumulator before an instruction repeats: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn calc_acc_reversing_nop_jmp_example() {
        let expected = 8;

        let program = EXAMPLE_PROGRAM.lines().map(|l| l.to_string()).collect();
        let actual = calc_acc_reversing_nop_jmp(program);

        assert_eq!(actual, expected);
    }

    #[test]
    fn calc_acc_reversing_nop_jmp_from_input() {
        let expected = 703;

        let program = load_as_vec_string("day8");
        let actual = calc_acc_reversing_nop_jmp(program);
        println!(
            "{}{}",
            "Accumulator after reversing nop and jmp: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
