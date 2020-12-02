/// Find two entries that sum to 2020 and then multiply those two numbers together
#[allow(dead_code)]
fn multiply_2020_from_two(report: &Vec<usize>) -> usize {
    for a in report {
        let found = report.into_iter().find(|&b| a + b == 2020);
        if found.is_some(){
            return a * found.unwrap();
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
        println!("{}{}", "Product of 2020 entries (2): ".green().bold(), actual);

        assert_eq!(actual, expected);
    }
}
