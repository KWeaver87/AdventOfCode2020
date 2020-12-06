fn calc_seat_id(input: &str) -> usize {
    let rows: Vec<u8> = (0..128).collect();
    let columns: Vec<u8> = (0..8).collect();
    let (f_b, l_r) = split_input(input.to_string());

    let seat_partitioner = |a: Vec<u8>, c: char| match c {
        'F' | 'L' => a[0..(a.len() / 2)].to_vec(),
        'B' | 'R' => a[(a.len() / 2)..].to_vec(),
        _ => panic!("Invalid char"),
    };

    // Using temp variables to prevent values from being dropped
    let row_temp = f_b.chars().fold(rows, seat_partitioner);
    let row = *row_temp.get(0).unwrap() as usize;
    let col_temp = l_r.chars().fold(columns, seat_partitioner);
    let col = *col_temp.get(0).unwrap() as usize;

    row * 8 + col
}

fn split_input(input: String) -> (String, String) {
    let finder: &[_] = &['L', 'R'];
    let split_index = input.find(finder).unwrap();

    let fb = input[0..split_index - 1].to_string();
    let lr = input[split_index..].to_string();

    (fb, lr)
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
