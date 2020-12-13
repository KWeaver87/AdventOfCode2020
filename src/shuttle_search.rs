fn multiply_earliest_bus(EXAMPLE_SCHEDULE: String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

    static EXAMPLE_SCHEDULE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn multiply_earliest_bus_example() {
        let expected = 205;
        let actual = multiply_earliest_bus(EXAMPLE_SCHEDULE.to_string());

        assert_eq!(actual, expected);
    }
}
