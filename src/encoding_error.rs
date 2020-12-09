fn find_preamble_rule_invalid(xmas: Vec<usize>, preamble_len: usize) -> usize {
    xmas.iter()
        .enumerate()
        .skip(preamble_len)
        .find(|(i, _)| !is_valid_by_preamble_rule(&xmas, *i, preamble_len))
        .map(|(_, &x)| x)
        .unwrap()
}

fn is_valid_by_preamble_rule(xmas: &Vec<usize>, i: usize, preamble_len: usize) -> bool {
    let n = xmas[i];
    let preamble = xmas[i - preamble_len..i].to_vec();

    preamble
        .iter()
        .any(|&x| preamble.iter().any(|&y| x != y && x + y == n))
}

/// Finds the first number that is invalid according to the preamble rule.
/// Then searches for a seqeunce of numbers that sums up to the invalid number.
/// Returns the sum of the min and max values from that sequence.
fn find_preamble_rule_sequence(xmas: Vec<usize>, preamble_len: usize) -> usize {

    0
}

static EXAMPLE_XMAS: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_vec_usize;
    use colored::Colorize;

    #[test]
    fn find_preamble_rule_invalid_example() {
        let expected = 127;
        let example_xmas = EXAMPLE_XMAS.lines().map(|l| l.parse().unwrap()).collect();
        let example_preamble_len = 5;
        let actual = find_preamble_rule_invalid(example_xmas, example_preamble_len);

        assert_eq!(actual, expected);
    }

    #[test]
    fn find_preamble_rule_invalid_from_input() {
        let expected = 217430975;

        let xmas = load_as_vec_usize("day9");
        let preamble_len = 25;
        let actual = find_preamble_rule_invalid(xmas, preamble_len);
        println!(
            "{}{}",
            "First value that does not follow preamble rule: "
                .green()
                .bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn find_preamble_rule_sequence_example() {
        let expected = 62;
        let example_xmas = EXAMPLE_XMAS.lines().map(|l| l.parse().unwrap()).collect();
        let example_preamble_len = 5;

        let actual = find_preamble_rule_sequence(example_xmas, example_preamble_len);

        assert_eq!(actual, expected);
    }
}
