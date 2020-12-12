fn distance_traveled(nav_instructions: Vec<String>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

    static NAV_INSTRUCTIONS: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn distance_traveled_example() {
        let expected = 25;
        let nav: Vec<String> = NAV_INSTRUCTIONS.lines().map(|l| l.to_string()).collect();
        let actual = distance_traveled(nav);

        assert_eq!(actual, expected);
    }
}
