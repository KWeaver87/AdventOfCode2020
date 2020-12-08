fn calc_acc_before_repeat(program: String) -> usize {

    0
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
    // use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

    #[test]
    fn calc_seat_example_1() {
        let expected = 5;
        let actual = calc_acc_before_repeat(EXAMPLE_PROGRAM.to_string());

        assert_eq!(actual, expected);
    }
}
