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
    fn sum_homework_example_3() {
        let expected = 26;
        let homework = vec!["2 * 3 + (4 * 5)".to_string()];
        let actual = sum_homework(homework);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_homework_example_4() {
        let expected = 437;
        let homework = vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()];
        let actual = sum_homework(homework);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_homework_example_5() {
        let expected = 12240;
        let homework = vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()];
        let actual = sum_homework(homework);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_homework_example_6() {
        let expected = 13632;
        let homework = vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()];
        let actual = sum_homework(homework);

        assert_eq!(actual, expected);
    }
}
