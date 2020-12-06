fn find_my_seat(input: Vec<String>) -> usize {
    let mut ticket_seat_ids: Vec<usize> = input.iter().map(|s| calc_seat_id(s)).collect();
    ticket_seat_ids.sort();

    let max_seat_id = *ticket_seat_ids.last().unwrap();

    let all_seats: Vec<usize> = (0..max_seat_id).collect();

    let missing_seats: Vec<usize> = all_seats
        .iter()
        .filter(|s| !ticket_seat_ids.contains(s))
        .map(|s| *s)
        .collect();

    // Find the missing seat that has tickets for adjacent seats
    let my_seat: Vec<usize> = missing_seats
        .iter()
        // We know seat ID 0 isn't right, so skip it to avoid more logic
        .skip(1)
        .filter(|s| {
            ticket_seat_ids.contains(&(*s - 1 as usize))
                && ticket_seat_ids.contains(&(*s + 1 as usize))
        })
        .map(|s| *s)
        .collect();

    *my_seat
        .get(0)
        .unwrap()
}

fn calc_seat_max_id(input: Vec<String>) -> usize {
    input.iter().map(|s| calc_seat_id(s)).max().unwrap()
}

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

    let f_b = input[0..split_index].to_string();
    let l_r = input[split_index..].to_string();

    (f_b, l_r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_vec_string;
    use colored::Colorize;

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

    #[test]
    fn calc_seat_max_from_input() {
        let expected = 930;

        let seats = load_as_vec_string("day5");
        let actual = calc_seat_max_id(seats);
        println!("{}{}", "Max seat ID: ".green().bold(), actual);

        assert_eq!(actual, expected);
    }

    #[test]
    fn find_my_seat_from_input() {
        let expected = 515;

        let seats = load_as_vec_string("day5");
        let actual = find_my_seat(seats);
        println!("{}{}", "My seat ID: ".green().bold(), actual);

        assert_eq!(actual, expected);
    }
}
