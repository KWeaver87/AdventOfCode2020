use colored::Colorize;

/// Wrapper struct to add custom getter
#[derive(Debug, Clone)]
struct GeologyRow {
    row: Vec<usize>,
}

impl GeologyRow {
    /// The map can repeat to infinity, so this `get` will loop back to the
    /// original array when the index exceeds the length.
    fn get(&self, index: usize) -> usize {
        if index >= self.row.len() {
            self.row[index % self.row.len()]
        } else {
            self.row[index]
        }
    }
}

#[allow(dead_code)]
fn build_geology_row(input: &str) -> GeologyRow {
    GeologyRow {
        row: input.chars().map(|c| parse_geology_type(c)).collect(),
    }
}

#[allow(dead_code)]
fn parse_geology_type(ch: char) -> usize {
    match ch {
        '.' => 0,
        '#' => 1,
        _ => panic!(format!(
            "{}{}",
            "Unknown character for GeologyType: ".red(),
            ch
        )),
    }
}

/// Starting at position (0, 0), count the number of trees encountered in
/// given map along given toboggan path (right, down)
#[allow(dead_code)]
fn count_trees_encountered_start(map: &Vec<GeologyRow>, tob_path: (usize, usize)) -> usize {
    count_trees_encountered(map, tob_path, (0, 0), 0)
}

/// Recursively count the trees
#[allow(dead_code)]
fn count_trees_encountered(
    map: &Vec<GeologyRow>,
    tob_path: (usize, usize),
    pos: (usize, usize),
    trees: usize,
) -> usize {
    if pos.0 >= map.len() {
        trees
    } else {
        let tree_here = map[pos.0].get(pos.1);
        count_trees_encountered(
            map,
            tob_path,
            // Add right movement to x coord, and add down movement to y coord
            (pos.0 + tob_path.1, pos.1 + tob_path.0),
            trees + tree_here,
        )
    }
}

#[allow(dead_code)]
fn product_trees_encountered_multiple_toboggans(
    map: &Vec<GeologyRow>,
    tob_paths: Vec<(usize, usize)>,
) -> usize {
    tob_paths.iter().map(|t| count_trees_encountered_start(map, *t)).product()
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

#[cfg(test)]
mod tests {
    use crate::input_utils::load_as_vec_string;
    // use colored::Colorize;

    use super::*;

    #[test]
    fn test_row_indexing() {
        let test_row = build_geology_row(TEST_GEOLOGY[0]);

        assert_eq!(test_row.get(0), 0);
        assert_eq!(test_row.get(2), 1);
        assert_eq!(test_row.get(10), 0);
        assert_eq!(test_row.get(11), 0);
        assert_eq!(test_row.get(13), 1);
    }

    #[test]
    fn given_example_single_tob() {
        let mut map: Vec<GeologyRow> = vec![];
        for row in TEST_GEOLOGY {
            map.push(build_geology_row(row));
        }

        assert_eq!(count_trees_encountered_start(&map, (3, 1)), 7);
    }

    #[test]
    fn run_input_single_tob() {
        let expected = 195;

        let file_input = load_as_vec_string("day3");
        let map = file_input
            .iter()
            .map(|r| build_geology_row(r.as_str()))
            .collect();
        let actual = count_trees_encountered_start(&map, (3, 1));
        println!(
            "{}{}",
            "Number of trees encountered: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn given_example_multi_tob() {
        let mut map: Vec<GeologyRow> = vec![];
        for row in TEST_GEOLOGY {
            map.push(build_geology_row(row));
        }

        let toboggans = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let actual = product_trees_encountered_multiple_toboggans(&map, toboggans);

        assert_eq!(actual, 336);
    }

    #[test]
    fn run_input_multi_tob() {
        let expected = 3772314000;

        let file_input = load_as_vec_string("day3");
        let map = file_input
            .iter()
            .map(|r| build_geology_row(r.as_str()))
            .collect();

        let toboggans = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let actual = product_trees_encountered_multiple_toboggans(&map, toboggans);
        println!(
            "{}{}",
            "Product of trees encountered: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
