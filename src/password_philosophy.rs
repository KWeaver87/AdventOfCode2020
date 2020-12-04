#[derive(PartialEq, Debug, Clone)]
struct PasswordPolicy<'a> {
    required: char,
    digit1: usize,
    digit2: usize,
    pass: &'a str,
}

/// Count the number of passwords that match policy, using `digit1` as minimum
/// amount of `required` and `digit2` as maximum allowed `required`.
fn count_valid_passwords_min_max(passwords: Vec<PasswordPolicy>) -> usize {
    passwords
        .iter()
        .filter(|pw| {
            let occurances = pw.pass.chars().filter(|ch| ch == &pw.required).count();
            occurances >= pw.digit1 && occurances <= pw.digit2
        })
        .count()
}

/// Count the number of passwords that match policy, using `digit1` as first
/// position to check and `digit2` as second position to check. Only one
/// position may contain `required`.
fn count_valid_passwords_position(passwords: Vec<PasswordPolicy>) -> usize {
    passwords
        .iter()
        .filter(|pw| {
            (pw.pass.chars().nth(pw.digit1 - 1).unwrap() == pw.required)
                != (pw.pass.chars().nth(pw.digit2 - 1).unwrap() == pw.required)
        })
        .count()
}

/// Parse a specifically formatted string into PasswordPolicy.
/// The string's format must be (`[DIGIT1]-[DIGIT2] [REQUIRED_SINGLE_CHARACTER]: [PASSWORD]`), e.g. `1-3 a: abcde`
fn parse_password_policy(input: &str) -> PasswordPolicy {
    let elements: Vec<&str> = input.split_ascii_whitespace().collect();
    let digits: Vec<&str> = elements[0].split('-').collect();

    PasswordPolicy {
        // Just get first char
        required: elements[1].chars().next().unwrap(),
        digit1: digits[0].parse().unwrap(),
        digit2: digits[1].parse().unwrap(),
        pass: elements[2],
    }
}

static TEST_PASSWORDS: &[PasswordPolicy; 3] = &[
    PasswordPolicy {
        required: 'a',
        digit1: 1,
        digit2: 3,
        pass: "abcde",
    },
    PasswordPolicy {
        required: 'b',
        digit1: 1,
        digit2: 3,
        pass: "cdefg",
    },
    PasswordPolicy {
        required: 'c',
        digit1: 2,
        digit2: 9,
        pass: "ccccccccc",
    },
];

#[cfg(test)]
mod tests {
    use crate::input_utils::load_as_vec_string;
    use colored::Colorize;

    use super::*;

    #[test]
    fn parser_test() {
        let actual = parse_password_policy("1-3 a: abcde");
        let expected = PasswordPolicy {
            required: 'a',
            digit1: 1,
            digit2: 3,
            pass: "abcde",
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn given_example_min_max() {
        assert_eq!(count_valid_passwords_min_max(TEST_PASSWORDS.to_vec()), 2);
    }

    #[test]
    fn run_input_min_max() {
        let expected = 556;

        let strings = load_as_vec_string("day2");
        let passwords = strings
            .iter()
            .map(|p| parse_password_policy(p.as_str()))
            .collect();
        let actual = count_valid_passwords_min_max(passwords);
        println!(
            "{}{}",
            "Number of valid passwords by min max: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn given_example_position() {
        assert_eq!(count_valid_passwords_position(TEST_PASSWORDS.to_vec()), 1);
    }

    #[test]
    fn run_input_position() {
        let expected = 605;

        let strings = load_as_vec_string("day2");
        let passwords = strings
            .iter()
            .map(|p| parse_password_policy(p.as_str()))
            .collect();
        let actual = count_valid_passwords_position(passwords);
        println!(
            "{}{}",
            "Number of valid passwords by position: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
