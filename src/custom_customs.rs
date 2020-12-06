use std::collections::HashSet;

fn sum_group_questions(questions: String) -> usize {
    let groups = split_questions_by_group(questions);

    let counts: Vec<HashSet<char>> = groups
        .iter()
        .map(|group| {
            group.chars()
                .filter(|c| *c >= 'a' && *c <= 'z')
                .collect::<HashSet<char>>()
        })
        .collect();

    counts.iter().map(|set| set.len()).sum()
}

fn split_questions_by_group(questions: String) -> Vec<String> {
    questions.split("\n\n").map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_string;
    use colored::Colorize;

    #[test]
    fn count_group_questions_example() {
        let expected = 11;
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
        let expected = 6809;

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
