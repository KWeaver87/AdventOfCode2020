fn sum_group_questions(questions: String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_string;
    use colored::Colorize;

    #[test]
    fn count_group_questions_example() {
        let expected = 0;
        let actual = sum_group_questions(
            "abc

a
b
c

ab
ac

a
a
a
a

b"
            .to_string(),
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn count_group_questions_from_input() {
        let expected = 0;

        let questions = load_as_string("day6");
        let actual = sum_group_questions(questions);
        println!(
            "{}{}",
            "Sum of groups questions answered yes: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
