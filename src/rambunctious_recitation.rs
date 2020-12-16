use std::collections::HashMap;

fn memory_game(starting_numbers: Vec<usize>, final_turn: usize) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut prev_n: usize = 0;

    for i in 0..starting_numbers.len() {
        let n = starting_numbers[i];
        memory.insert(n, i + 1);
    }

    for turn in starting_numbers.len() + 1..final_turn {
        let prev_turn = memory.get_mut(&prev_n);
        if let Some(prev_turn) = prev_turn {
            prev_n = turn - *prev_turn;
            *prev_turn = turn;
        } else {
            memory.insert(prev_n, turn);
            prev_n = 0;
        }
    }

    prev_n
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;

    #[test]
    fn memory_game_2020_example_1() {
        let expected = 436;
        let actual = memory_game(vec![0, 3, 6], 2020);

        assert_eq!(actual, expected);
    }

    #[test]
    fn memory_game_2020_examples() {
        let tests: Vec<(Vec<usize>, usize)> = vec![
            (vec![1, 3, 2], 1),
            (vec![2, 1, 3], 10),
            (vec![1, 2, 3], 27),
            (vec![2, 3, 1], 78),
            (vec![3, 2, 1], 438),
            (vec![3, 1, 2], 1836),
        ];

        for test in tests {
            let expected = test.1;
            let actual = memory_game(test.0, 2020);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn memory_game_2020_from_input() {
        let expected = 614;

        let puzzle_input = vec![14, 3, 1, 0, 9, 5];
        let actual = memory_game(puzzle_input, 2020);
        println!(
            "{}{}",
            "The number spoken on 2020th turn is: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn memory_game_30000000_example_1() {
        let expected = 175594;
        let actual = memory_game(vec![0, 3, 6], 30_000_000);

        assert_eq!(actual, expected);
    }

    #[test]
    fn memory_game_30000000_from_input() {
        let expected = 1065;

        let puzzle_input = vec![14, 3, 1, 0, 9, 5];
        let actual = memory_game(puzzle_input, 30_000_000);
        println!(
            "{}{}",
            "The number spoken on 30000000th turn is: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
