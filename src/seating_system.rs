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

    fn get_adjacent_vis(&self, x: usize, y: usize) -> Vec<Option<char>> {
        vec![
            self.get_path(x, y, (-1, -1)),
            self.get_path(x, y, (-1, 0)),
            self.get_path(x, y, (-1, 1)),
            self.get_path(x, y, (0, -1)),
            self.get_path(x, y, (0, 1)),
            self.get_path(x, y, (1, -1)),
            self.get_path(x, y, (1, 0)),
            self.get_path(x, y, (1, 1)),
        ]
    }

    fn get_path(&self, x: usize, y: usize, path: (isize, isize)) -> Option<char> {
        let x2 = x as isize + path.0;
        let y2 = y as isize + path.1;
        let got = self.get_checked(x2, y2);

        match got {
            None => None,
            Some('.') => self.get_path(x2 as usize, y2 as usize, path),
            _ => got,
        }
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

/// Part1
fn stabilized_occupied_seats(layout: Vec<String>) -> usize {
    let seats = SeatLayout::new(layout);
    let adj_getter = |lay: &SeatLayout, x, y| lay.get_adjacent(x, y);
    let stab_seats = run_seat_rules_until_stable(&seats, 4, adj_getter);

    stab_seats.count_total_occupied()
}

fn run_seat_rules_until_stable<F>(
    layout: &SeatLayout,
    occ_seat_limit: usize,
    adj_getter: F,
) -> SeatLayout
where
    F: Fn(&SeatLayout, usize, usize) -> Vec<Option<char>>,
{
    let mut new_layout = layout.clone();

    for x in 0..layout.len_x() {
        for y in 0..layout.len_y() {
            let c = layout.get(x, y);
            let adj = adj_getter(&layout, x, y);
            let occ_count = count_adjacent_occupied(adj);

            if c == 'L' && occ_count == 0 {
                new_layout.layout[x][y] = '#'
            }
            if c == '#' && occ_count >= occ_seat_limit {
                new_layout.layout[x][y] = 'L'
            }
        }
    }

    if new_layout.count_total_occupied() == layout.count_total_occupied() {
        new_layout
    } else {
        run_seat_rules_until_stable(&new_layout, occ_seat_limit, adj_getter)
    }
}

fn count_adjacent_occupied(adjacent: Vec<Option<char>>) -> usize {
    adjacent
        .into_iter()
        .filter(|s| s.is_some())
        .filter(|s| s.unwrap() == '#')
        .count()
}

/// Part2
fn stabilized_occupied_visible_seats(layout: Vec<String>) -> usize {
    let seats = SeatLayout::new(layout);
    let adj_getter = |lay: &SeatLayout, x, y| lay.get_adjacent_vis(x, y);
    let stab_seats = run_seat_rules_until_stable(&seats, 5, adj_getter);

    stab_seats.count_total_occupied()
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

    #[test]
    fn stabilized_occupied_visible_seats_example() {
        let expected = 26;
        let layout = EXAMPLE_LAYOUT.lines().map(|l| l.to_string()).collect();
        let actual = stabilized_occupied_visible_seats(layout);

        assert_eq!(actual, expected);
    }

    #[test]
    fn stabilized_occupied_visible_seats_from_input() {
        let expected = 2068;

        let layout = load_as_vec_string("day11");
        let actual = stabilized_occupied_visible_seats(layout);
        println!(
            "{}{}",
            "Total number of occupied visible seats: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
