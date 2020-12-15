use std::{collections::HashMap, vec};

type BitMask = Vec<char>;
type MemAddress = usize;
type MemValue = u64;

#[derive(PartialEq, Debug)]
enum Instruction {
    Mask(BitMask),
    Mem(MemAddress, MemValue),
}

fn sum_memory(raw_program: Vec<String>) -> u64 {
    let program = parse_program(raw_program);

    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask: BitMask = vec![];

    for instr in program {
        match instr {
            Instruction::Mask(new_mask) => mask = new_mask,
            Instruction::Mem(address, value) => {
                memory.insert(address, apply_bitmask(&mask, value));
            }
        }
    }

    memory.into_iter().map(|(_, v)| v).sum()
}

fn apply_bitmask(mask: &BitMask, value: u64) -> u64 {
    let mut binary_chars: Vec<char> = format!("{:036b}", value).chars().collect();
    let mask_digits: Vec<(usize, char)> = mask
        .iter()
        .enumerate()
        .filter_map(|(i, v)| match *v {
            'X' => None,
            _ => Some((i, *v)),
        })
        .collect();

    for (i, v) in mask_digits {
        binary_chars[i] = v
    }
    let binary: String = binary_chars.iter().collect();

    u64::from_str_radix(binary.as_str(), 2).unwrap()
}

fn parse_program(raw_program: Vec<String>) -> Vec<Instruction> {
    raw_program
        .into_iter()
        .map(|instr| {
            let first_three: String = instr.chars().take(3).collect();
            match first_three.as_str() {
                "mas" => parse_mask(instr),
                "mem" => parse_mem(instr),
                _ => panic!("Invalid instruction"),
            }
        })
        .collect()
}

fn parse_mem(instr: String) -> Instruction {
    let start = instr.find('[').unwrap() + 1;
    let end = instr.find(']').unwrap();

    let split_i = instr.find("=").unwrap() + 2;
    let (_, mem_val) = instr.split_at(split_i);

    Instruction::Mem(instr[start..end].parse().unwrap(), mem_val.parse().unwrap())
}

fn parse_mask(instr: String) -> Instruction {
    let split_i = instr.find("=").unwrap() + 2;
    let (_, instr_mask) = instr.split_at(split_i);
    let mask: Vec<char> = instr_mask.chars().collect();

    Instruction::Mask(mask)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_vec_string;
    use colored::Colorize;

    #[test]
    fn parse_mask_test() {
        let expected = Instruction::Mask(vec!['X', '1', 'X', 'X', '0', 'X', 'X']);
        let mask = "mask = X1XX0XX".to_string();
        let actual = parse_mask(mask);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_mem_test() {
        let expected = Instruction::Mem(8, 11);
        let mem = "mem[8] = 11".to_string();
        let actual = parse_mem(mem);

        assert_eq!(actual, expected);
    }

    #[test]
    fn apply_bitmask_test() {
        let expected = 73;
        let mask = vec![
            'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
            'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', '1', 'X', 'X', 'X', 'X',
            '0', 'X',
        ];
        let actual = apply_bitmask(&mask, 11);

        assert_eq!(actual, expected);
    }

    static EXAMPLE_PROGRAM: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn sum_memory_example() {
        let expected = 165;
        let prog = EXAMPLE_PROGRAM.lines().map(|s| s.to_string()).collect();
        let actual = sum_memory(prog);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_memory_from_input() {
        let expected = 6386593869035;

        let program = load_as_vec_string("day14");
        let actual = sum_memory(program);
        println!("{}{}", "Sum of memory values: ".green().bold(), actual);

        assert_eq!(actual, expected);
    }
}
