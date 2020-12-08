use std::collections::HashSet;

struct ProgramRunner {
    instructions: Vec<(String, i32)>,
    instruction_index: i32,
    accumulator: i32,
    seen_instructions: HashSet<usize>,
}

impl ProgramRunner {
    fn new(instructions: Vec<String>) -> ProgramRunner {
        let parsed_instructions = instructions
            .into_iter()
            .map(|l| parse_instruction_line(l.to_string()))
            .collect();

        ProgramRunner {
            instructions: parsed_instructions,
            instruction_index: 0,
            accumulator: 0,
            seen_instructions: HashSet::new(),
        }
    }

    fn run_instructions(&mut self) {
        self.run_instruction(&self.instructions[0].clone());
    }

    fn run_instruction(&mut self, instruction: &(String, i32)) {
        if !self.seen_instructions.insert(self.instruction_index as usize) {
            return;
        }
        let op = instruction.0.as_str();
        match op {
            "acc" => {
                self.accumulator += instruction.1;
                self.instruction_index += 1;
            }
            "jmp" => {
                self.instruction_index += instruction.1;
            }
            "nop" => {
                self.instruction_index += 1;
            }
            _ => panic!(format!("Invalid operation: {}", op)),
        };

        self.run_instruction(&self.instructions[self.instruction_index as usize].clone());
    }
}

fn parse_instruction_line(line: String) -> (String, i32) {
    let splits: Vec<&str> = line.split_whitespace().collect();
    let op = splits.get(0).unwrap().to_string();
    let arg: i32 = splits.get(1).unwrap().parse().unwrap();

    (op, arg)
}

fn calc_acc_before_repeat(program: Vec<String>) -> i32 {
    let mut prog = ProgramRunner::new(program);

    prog.run_instructions();

    prog.accumulator
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
    fn calc_acc_before_repeat_example_1() {
        let expected = 5;
        let actual = calc_acc_before_repeat(EXAMPLE_PROGRAM.lines().map(|l| l.to_string()).collect());

        assert_eq!(actual, expected);
    }

    #[test]
    fn calc_acc_before_repeat_from_input() {
        let expected = 1586;

        let program = load_as_vec_string("day8");
        let actual = calc_acc_before_repeat(program);
        println!("{}{}", "Accumulator before an instruction repeats: ".green().bold(), actual);

        assert_eq!(actual, expected);
    }
}
