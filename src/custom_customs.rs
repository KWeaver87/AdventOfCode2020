use std::collections::HashSet;

/// Counts the number of unique questions answered by each group, and then
/// sums those counts.
fn sum_group_questions_anyone(questions: String) -> usize {
    let groups = split_questions_by_group(questions);

    let counts: Vec<HashSet<char>> = groups
        .iter()
        .map(|group| list_all_questions(group.to_owned()))
        .collect();

    counts.iter().map(|set| set.len()).sum()
}

/// Counts the number of unique questions answered by everyone in each group,
/// and then sums those counts.
fn sum_group_questions_everyone(questions: String) -> usize {
    let groups = split_questions_by_group(questions);

    let counts: Vec<Vec<char>> = groups
        .iter()
        .map(|group| {
            list_all_questions(group.to_owned())
                .iter()
                .filter(|c| group.lines().all(|l| l.contains(**c)))
                .map(|c| *c)
                .collect::<Vec<char>>()
        })
        .collect();

    counts.iter().map(|set| set.len()).sum()
}

fn split_questions_by_group(questions: String) -> Vec<String> {
    questions.split("\n\n").map(|s| s.to_string()).collect()
}

fn list_all_questions(input: String) -> HashSet<char> {
    input.chars().filter(|c| c.is_alphabetic()).collect()
}

static TEST_LIST: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_string;
    use colored::Colorize;

    #[test]
    fn sum_group_questions_anyone_example() {
        let expected = 11;
        let actual = sum_group_questions_anyone(TEST_LIST.to_string());

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_group_questions_anyone_from_input() {
        let expected = 6809;

        let questions = load_as_string("day6");
        let actual = sum_group_questions_anyone(questions);
        println!(
            "{}{}",
            "Sum of groups where anyone answered yes: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_group_questions_everyone_example() {
        let expected = 6;
        let actual = sum_group_questions_everyone(TEST_LIST.to_string());

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_group_questions_everyone_from_input() {
        let expected = 3394;

        let questions = load_as_string("day6");
        let actual = sum_group_questions_everyone(questions);
        println!(
            "{}{}",
            "Sum of groups where everyone answered yes: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
