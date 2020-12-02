#[derive(PartialEq, Debug)]
struct PasswordPolicy<'a> {
    required: char,
    min: usize,
    max: usize,
    pass: &'a str,
}

/// Count the number of passwords that match their policy
#[allow(dead_code)]
fn count_valid_passwords(passwords: &Vec<PasswordPolicy>) -> usize {
    passwords
        .iter()
        .filter(|pw| {
            let occurances = pw.pass.chars().filter(|ch| ch == &pw.required).count();
            occurances >= pw.min && occurances <= pw.max
        })
        .count()
}

/// Parse a specifically formatted string into PasswordPolicy.
/// The string's format must be (`[MIN]-[MAX] [REQUIRED_SINGLE_CHARACTER]: [PASSWORD]`), e.g. `1-3 a: abcde`
#[allow(dead_code)]
fn parse_password_policy(input: &str) -> PasswordPolicy {
    let elements: Vec<&str> = input.split_ascii_whitespace().collect();

    let min_max: Vec<&str>  = elements[0].split('-').collect();
    let min: usize = min_max[0].parse().unwrap();
    let max: usize = min_max[1].parse().unwrap();
    // Just get first char
    let ch = elements[1].chars().next().unwrap();

    PasswordPolicy {
        required: ch,
        min: min,
        max: max,
        pass: elements[2],
    }
}

#[cfg(test)]
mod tests {
    // use crate::input_utils::load_as_vec_usize;
    // use colored::Colorize;

    use super::*;

    #[test]
    fn parser_test() {
        let actual = parse_password_policy("1-3 a: abcde");
        let expected = PasswordPolicy {
            required: 'a',
            min: 1,
            max: 3,
            pass: "abcde",
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn given_example() {
        let test_passwords = &vec![
            PasswordPolicy {
                required: 'a',
                min: 1,
                max: 3,
                pass: "abcde",
            },
            PasswordPolicy {
                required: 'b',
                min: 1,
                max: 3,
                pass: "cdefg",
            },
            PasswordPolicy {
                required: 'c',
                min: 2,
                max: 9,
                pass: "ccccccccc",
            },
        ];
        assert_eq!(count_valid_passwords(test_passwords), 2);
    }
}
