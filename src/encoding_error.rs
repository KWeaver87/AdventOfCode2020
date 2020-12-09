fn find_preamble_rule_invalid(xmas: Vec<usize>, preamble_len: usize) -> usize {

    xmas
        .iter()
        .skip(preamble_len)
        .enumerate()
        .find(|(i, _)| !is_valid_by_preamble_rule(&xmas, *i, preamble_len))
        .map(|(_, &x)| x)
        .unwrap()
}

fn is_valid_by_preamble_rule(xmas: &Vec<usize>, i: usize, preamble_len: usize) -> bool {

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_vec_string;
    use colored::Colorize;

    #[test]
    fn find_preamble_rule_invalid_example() {
        let expected = 127;
        let example_xmas = "35
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
576"
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
        let example_preamble_len = 5;
        let actual = find_preamble_rule_invalid(example_xmas, example_preamble_len);

        assert_eq!(actual, expected);
    }
}
