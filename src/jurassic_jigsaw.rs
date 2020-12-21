fn product_corner_tiles(tiles: String) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::input_utils::load_as_string;

    use super::*;

    #[test]
    fn calc_seat_example_1() {
        let expected = 20899048083289;
        let tiles = load_as_string("day20ex");
        let actual = product_corner_tiles(tiles);

        assert_eq!(actual, expected);
    }
}
