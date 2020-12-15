    fn memory_game(starting_numbers: Vec<u32>) -> u32 {


        todo!()
    }

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

    #[test]
    fn calc_seat_example_1() {
        let expected = 436;
        let actual = memory_game(vec![0,3,6]);

        assert_eq!(actual, expected);
    }
}
