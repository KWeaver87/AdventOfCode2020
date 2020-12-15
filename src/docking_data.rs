use std::{collections::HashMap, vec};

type BitMask = Vec<char>;
type MemAddress = usize;
type MemValue = u64;

#[derive(PartialEq, Debug)]
enum Instruction {
    Mask(BitMask),
    Mem(MemAddress, MemValue),
}

/// Part1
fn sum_memory(raw_program: Vec<String>) -> u64 {
    let memory = program_into_memory(raw_program, |mask, a, v| apply_bitmask(mask, a, v));

    memory.into_iter().map(|(_, v)| v).sum()
}

fn program_into_memory<F>(raw_program: Vec<String>, mem_map: F) -> HashMap<usize, u64>
where
    F: Fn(&BitMask, MemAddress, MemValue) -> Vec<(MemAddress, MemValue)>,
{
    let program = parse_program(raw_program);

    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask: BitMask = vec![];

    for instr in program {
        match instr {
            Instruction::Mask(new_mask) => mask = new_mask,
            Instruction::Mem(address, value) => {
                let mapped_a_v = mem_map(&mask, address, value);
                for a_v in mapped_a_v {
                    memory.insert(a_v.0, a_v.1);
                }
            }
        }
    }

    memory
}

fn apply_bitmask(
    mask: &BitMask,
    address: MemAddress,
    value: MemValue,
) -> Vec<(MemAddress, MemValue)> {
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
    let a_v = (address, binary_vec_to_int(binary_chars));

    vec![a_v]
}

fn sum_memory_decoder(raw_program: Vec<String>) -> u64 {
    let memory = program_into_memory(raw_program, |mask, a, v| apply_bitmask_decoder(mask, a, v));

    memory.into_iter().map(|(_, v)| v).sum()
}

fn apply_bitmask_decoder(
    mask: &BitMask,
    address: MemAddress,
    value: MemValue,
) -> Vec<(MemAddress, MemValue)> {
    let mut binary_chars: Vec<char> = format!("{:036b}", address).chars().collect();

    for (i, v) in mask.iter().enumerate() {
        match *v {
            '0' => continue,
            _ => binary_chars[i] = *v,
        }
    }
    let binary_address: String = binary_chars.iter().collect();

    get_floating_addresses(binary_address, value)
}

fn get_floating_addresses(binary_address: String, value: u64) -> Vec<(MemAddress, MemValue)> {
    let mut adds_to_check = vec![binary_address];
    let mut finished_a_v: Vec<(MemAddress, MemValue)> = vec![];

    while adds_to_check.len() > 0 {
        let add = adds_to_check.pop().unwrap();
        let add_chars: Vec<char> = add.chars().collect();
        let x_index = add_chars.iter().position(|ch| *ch == 'X').unwrap();

        for n in 0..=1 {
            let mut new_add = add_chars.clone();
            new_add[x_index] = n.to_string().chars().nth(0).unwrap();

            if new_add.contains(&'X') {
                adds_to_check.push(new_add.iter().collect());
            } else {
                let add_finished = binary_vec_to_int(new_add);
                finished_a_v.push((add_finished as MemAddress, value));
            }
        }
    }

    finished_a_v
}

fn binary_vec_to_int(binary_chars: Vec<char>) -> u64 {
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
        let actual = apply_bitmask(&mask, 0, 11)[0].1;

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_memory_example() {
        let expected = 165;
        let prog = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .lines()
            .map(|s| s.to_string())
            .collect();
        let actual = sum_memory(prog);

        assert_eq!(actual, expected);
    }

    #[test]
    // Part1
    fn sum_memory_from_input() {
        let expected = 6386593869035;

        let program = load_as_vec_string("day14");
        let actual = sum_memory(program);
        println!("{}{}", "Sum of memory values using mask against values: ".green().bold(), actual);

        assert_eq!(actual, expected);
    }

    #[test]
    fn get_floating_addresses_test() {
        let expected: Vec<(usize, u64)> = vec![(2, 1), (3, 1)];

        let actual = get_floating_addresses("1X".to_string(), 1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_memory_decoder_example() {
        let expected = 208;
        let program = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .lines()
            .map(|s| s.to_string())
            .collect();

        let actual = sum_memory_decoder(program);
        assert_eq!(actual, expected);
    }

    #[test]
    // Part1
    fn sum_memory_decoder_input() {
        let expected = 4288986482164;

        let program = load_as_vec_string("day14");
        let actual = sum_memory_decoder(program);
        println!(
            "{}{}",
            "Sum of memory values using mask as memory address decoder: "
                .green()
                .bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
