fn calc_seat_id(input: &str) -> usize {


    0
}

#[cfg(test)]
mod tests {
    // use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;
    use super::*;

    #[test]
    fn calc_seat_example_1() {
        let expected = 357;
        let actual = calc_seat_id("FBFBBFFRLR");

        assert_eq!(actual, expected);
    }

    #[test]
    fn calc_seat_example_2() {
        let expected = 567;
        let actual = calc_seat_id("BFFFBBFRRR");

        assert_eq!(actual, expected);
    }

    #[test]
    fn calc_seat_example_3() {
        let expected = 119;
        let actual = calc_seat_id("FFFBBBFRRR");

        assert_eq!(actual, expected);
    }

    #[test]
    fn calc_seat_example_4() {
        let expected = 820;
        let actual = calc_seat_id("BBFFBBFRLL");

        assert_eq!(actual, expected);
    }
}
