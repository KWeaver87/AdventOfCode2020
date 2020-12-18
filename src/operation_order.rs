fn sum_homework(homework: Vec<String>) -> isize {
    homework.iter().map(|l| eval_line(l)).sum()
}

fn eval_line(l: &String) -> isize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_homework_example_1() {
        let expected = 71;
        let homework = vec!["1 + 2 * 3 + 4 * 5 + 6".to_string()];
        let actual = sum_homework(homework);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_homework_example_2() {
        let expected = 51;
        let homework = vec!["1 + (2 * 3) + (4 * (5 + 6))".to_string()];
        let actual = sum_homework(homework);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_homework_other_examples() {
        let examples = vec![
            (26, "2 * 3 + (4 * 5)"),
            (437, "5 + (8 * 3 + 9 + 3 * 4 * 3)"),
            (12240, "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            (13632, "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        ];

        for (expected, example) in examples {
            let actual = sum_homework(vec![example.to_string()]);

            assert_eq!(actual, expected);
        }
    }
}
