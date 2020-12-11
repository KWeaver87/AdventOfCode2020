#[derive(Clone)]
struct SeatLayout {
    layout: Vec<Vec<char>>,
}

impl SeatLayout {
    fn new(layout: Vec<String>) -> SeatLayout {
        SeatLayout {
            layout: layout.iter().map(|x| x.chars().collect()).collect(),
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.layout[x][y]
    }

    fn get_checked(&self, x: isize, y: isize) -> Option<char> {
        if x < 0 || x >= self.len_x() as _ || y < 0 || y >= self.len_y() as _ {
            return None;
        }

        Some(self.layout[x as usize][y as usize])
    }

    fn get_adjacent(&self, x: usize, y: usize) -> Vec<Option<char>> {
        vec![
            self.get_checked(x as isize - 1, y as isize - 1),
            self.get_checked(x as isize - 1, y as isize),
            self.get_checked(x as isize - 1, y as isize + 1),
            self.get_checked(x as isize, y as isize - 1),
            self.get_checked(x as isize, y as isize + 1),
            self.get_checked(x as isize + 1, y as isize - 1),
            self.get_checked(x as isize + 1, y as isize),
            self.get_checked(x as isize + 1, y as isize + 1),
        ]
    }

    fn count_adjacent_occupied(&self, x: usize, y: usize) -> usize {
        self.get_adjacent(x, y)
            .into_iter()
            .filter(|s| s.is_some())
            .filter(|s| s.unwrap() == '#')
            .count()
    }

    fn count_total_occupied(&self) -> usize {
        self.layout
            .iter()
            .map(|x| {
                x.iter()
                    .map(|&y| if y == '#' { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }

    fn len_x(&self) -> usize {
        self.layout.len()
    }

    fn len_y(&self) -> usize {
        self.layout[0].len()
    }
}

fn stabilized_occupied_seats(layout: Vec<String>) -> usize {
    let seats = SeatLayout::new(layout);
    let stab_seats = run_seat_rules_until_stable(seats);

    stab_seats.count_total_occupied()
}

fn run_seat_rules_until_stable(layout: SeatLayout) -> SeatLayout {
    let mut new_layout = layout.clone();

    for x in 0..layout.len_x() {
        for y in 0..layout.len_y() {
            if layout.get(x, y) == 'L' && layout.count_adjacent_occupied(x, y) == 0 {
                new_layout.layout[x][y] = '#'
            }
            if layout.get(x, y) == '#' && layout.count_adjacent_occupied(x, y) >= 4 {
                new_layout.layout[x][y] = 'L'
            }
        }
    }

    if new_layout.count_total_occupied() == layout.count_total_occupied() {
        new_layout
    } else {
        run_seat_rules_until_stable(new_layout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_vec_string;
    use colored::Colorize;

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

    #[test]
    fn stabilized_occupied_seats_from_input() {
        let expected = 2324;

        let layout = load_as_vec_string("day11");
        let actual = stabilized_occupied_seats(layout);
        println!(
            "{}{}",
            "Total number of occupied seats: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
