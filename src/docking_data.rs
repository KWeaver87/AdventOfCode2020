fn sum_memory(raw_program: Vec<String>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

    static EXAMPLE_PROGRAM: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn sum_memory_example() {
        let expected = 0;
        let prog = EXAMPLE_PROGRAM.lines().map(|s| s.to_string()).collect();
        let actual = sum_memory(prog);

        assert_eq!(actual, expected);
    }
}
