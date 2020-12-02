/// Find two entries that sum to 2020 and then multiply those two numbers together
#[allow(dead_code)]
fn multiply_2020_from_two(report: &Vec<usize>) -> usize {
    for a in report {
        let maybe_b = report.into_iter().find(|&b| a + b == 2020);
        if maybe_b.is_some() {
            return a * maybe_b.unwrap();
        }
    }

    panic!("Could not find 2020.");
}

/// Find three entries that sum to 2020 and then multiply those two numbers together
#[allow(dead_code)]
fn multiply_2020_from_three(report: &Vec<usize>) -> usize {
    let length = report.len() - 1;
    for a_index in 0..=length {
        let a = report[a_index];
        for b_index in a_index + 1..=length {
            let b = report[b_index];
            let maybe_c = report.into_iter().find(|&c| a + b + c == 2020);
            if maybe_c.is_some() {
                return a * b * maybe_c.unwrap();
            }
        }
    }

    panic!("Could not find 2020.");
}

#[cfg(test)]
mod tests {
    use crate::input_utils::load_as_vec_usize;
    use colored::Colorize;

    use super::*;

    #[test]
    fn given_example_for_two() {
        let test_report = &vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(multiply_2020_from_two(test_report), 514579);
    }

    #[test]
    fn run_input_for_two() {
        let expected = 712075;

        let report = &load_as_vec_usize("day1-1");
        let actual = multiply_2020_from_two(report);
        println!(
            "{}{}",
            "Product of two 2020 entries: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn given_example_for_three() {
        let test_report = &vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(multiply_2020_from_three(test_report), 241861950);
    }

    #[test]
    fn run_input_for_three() {
        let expected = 145245270;

        let report = &load_as_vec_usize("day1-1");
        let actual = multiply_2020_from_three(report);
        println!(
            "{}{}",
            "Product of three 2020 entries: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
