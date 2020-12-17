fn active_cubes_after_boot(initial_cubes: Vec<String>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::input_utils::mutliline_to_vec_string;

    use super::*;
    // use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

    static EXAMPLE_CUBES: &str = ".#.
..#
###";

    #[test]
    fn active_cubes_after_boot_example() {
        let expected = 112;
        let cubes = mutliline_to_vec_string(EXAMPLE_CUBES.to_string());
        let actual = active_cubes_after_boot(cubes);

        assert_eq!(actual, expected);
    }
}
