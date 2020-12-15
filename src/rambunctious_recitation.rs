fn memory_game(starting_numbers: Vec<u32>) -> u32 {
    let mut memory: Vec<u32> = vec![];
    // Want to use 1-based indexing to simplify code, so make index 0 a
    // value that should never be considered.
    memory.push(u32::MAX);

    for start_n in starting_numbers {
        memory.push(start_n);
    }

    for prev_turn in (memory.len() - 1)..2020 {
        let n = memory[prev_turn];
        // Search from right of Vec, but skip the number that we're looking for
        let find_n = memory
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .find(|(_, x)| **x == n);
        match find_n {
            None => memory.push(0),
            Some((i, _)) => {
                let turns_since = prev_turn as u32 - i as u32;
                memory.push(turns_since);
            }
        }
    }

    *memory.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;

    #[test]
    fn memory_game_example_1() {
        let expected = 436;
        let actual = memory_game(vec![0, 3, 6]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn memory_game_examples() {
        let tests: Vec<(Vec<u32>, u32)> = vec![
            (vec![1, 3, 2], 1),
            (vec![2, 1, 3], 10),
            (vec![1, 2, 3], 27),
            (vec![2, 3, 1], 78),
            (vec![3, 2, 1], 438),
            (vec![3, 1, 2], 1836),
        ];

        for test in tests {
            let expected = test.1;
            let actual = memory_game(test.0);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn memory_game_from_input() {
        let expected = 614;

        let puzzle_input = vec![14, 3, 1, 0, 9, 5];
        let actual = memory_game(puzzle_input);
        println!(
            "{}{}",
            "The number spoken on 2020th turn is: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
