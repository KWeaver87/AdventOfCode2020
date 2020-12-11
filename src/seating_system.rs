fn stabilized_occupied_seats(layout: Vec<String>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

    static EXAMPLE_LAYOUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn stabilized_occupied_seats_example() {
        let expected = 37;
        let layout = EXAMPLE_LAYOUT.lines().map(|l| l.to_string()).collect();
        let actual = stabilized_occupied_seats(layout);

        assert_eq!(actual, expected);
    }
}
