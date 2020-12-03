use colored::Colorize;

#[derive(Debug, Clone, PartialEq, Copy)]
enum GeologyType {
    Open,
    Tree,
}

#[derive(Debug, Clone)]
struct GeologyRow {
    row: Vec<GeologyType>,
}

impl GeologyRow {
    fn get(&self, index: usize) -> GeologyType {
        if index >= self.row.len() {
            self.row[index % self.row.len()]
        } else {
            self.row[index]
        }
    }
}

#[allow(dead_code)]
fn build_geology_row(input: &str) -> GeologyRow {
    let mut row: Vec<GeologyType> = vec![];
    for c in input.chars() {
        row.push(parse_geology_type(c));
    }

    GeologyRow { row: row }
}

#[allow(dead_code)]
fn parse_geology_type(ch: char) -> GeologyType {
    match ch {
        '.' => GeologyType::Open,
        '#' => GeologyType::Tree,
        _ => panic!(format!(
            "{}{}",
            "Unknown character for GeologyType: ".red(),
            ch
        )),
    }
}

#[derive(Debug, Copy, Clone)]
struct TobogganPath {
    start_pos: (usize, usize),
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

#[allow(dead_code)]
fn count_trees_encountered(
    map: Vec<GeologyRow>,
    tob: TobogganPath,
    pos: (usize, usize),
    trees: usize,
) -> usize {
    if pos.0 >= map.len() {
        trees
    } else {
        let tree_here = match map[pos.0].get(pos.1) {
            GeologyType::Open => 0,
            GeologyType::Tree => 1,
        };

        count_trees_encountered(
            map,
            tob,
            (pos.0 - tob.up + tob.down, pos.1 - tob.left + tob.right),
            trees + tree_here,
        )
    }
}

#[allow(dead_code)]
static TEST_GEOLOGY: &[&str] = &[
    "..##.......",
    "#...#...#..",
    ".#....#..#.",
    "..#.#...#.#",
    ".#...##..#.",
    "..#.##.....",
    ".#.#.#....#",
    ".#........#",
    "#.##...#...",
    "#...##....#",
    ".#..#...#.#",
];

#[allow(dead_code)]
static FIRST_TOBOGGAN: TobogganPath = TobogganPath {
    start_pos: (0, 0),
    up: 0,
    down: 1,
    left: 0,
    right: 3,
};

#[cfg(test)]
mod tests {
    use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

    use super::*;

    #[test]
    fn test_row_indexing() {
        let test_row = build_geology_row(TEST_GEOLOGY[0]);

        assert_eq!(test_row.get(0), GeologyType::Open);
        assert_eq!(test_row.get(2), GeologyType::Tree);
        assert_eq!(test_row.get(10), GeologyType::Open);
        assert_eq!(test_row.get(11), GeologyType::Open);
        assert_eq!(test_row.get(13), GeologyType::Tree);
    }

    #[test]
    fn given_example() {
        let mut map: Vec<GeologyRow> = vec![];
        for row in TEST_GEOLOGY {
            map.push(build_geology_row(row));
        }

        assert_eq!(
            count_trees_encountered(map, FIRST_TOBOGGAN, FIRST_TOBOGGAN.start_pos, 0),
            7
        );
    }

    #[test]
    fn run_input() {
        let expected = 195;

        let strings = load_as_vec_string("day3");
        let map = strings
            .iter()
            .map(|r| build_geology_row(r.as_str()))
            .collect();
        let actual = count_trees_encountered(map, FIRST_TOBOGGAN, FIRST_TOBOGGAN.start_pos, 0);
        println!(
            "{}{}",
            "Number of trees encountered: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
