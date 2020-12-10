use std::collections::BTreeSet;

fn product_jolt_differences(adapters: Vec<usize>) -> usize {
    let adapts: BTreeSet<usize> = adapters.into_iter().collect();

    let joltages = adapts
        .iter()
        .fold((0, 0, 0), |(prev, j1, j3), &cur| match cur - prev {
            1 => (cur, j1 + 1, j3),
            3 => (cur, j1, j3 + 1),
            _ => (cur, j1, j3),
        });

    joltages.1 * (joltages.2 + 1)
}

/// Part2
fn total_distinct_arrangements(adapters: Vec<usize>) -> u64 {
    let adapts: BTreeSet<usize> = adapters.clone().into_iter().collect();
    let max = *adapters.iter().max().unwrap();
    let mut tracker: Vec<u64> = vec![0; max + 4];
    tracker[max + 3] = 1;

    for x in adapts.into_iter().rev() {
        let slice = &tracker[x + 1..=x + 3];
        tracker[x] = slice.iter().sum();
    }

    tracker[1..=3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::load_as_vec_usize;
    use colored::Colorize;

    static EXAMPLE_ADAPTER_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    static EXAMPLE_ADAPTER_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn product_jolt_differences_example_1() {
        let expected = 35;
        let example_adapters = EXAMPLE_ADAPTER_1
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        let actual = product_jolt_differences(example_adapters);

        assert_eq!(actual, expected);
    }

    #[test]
    fn product_jolt_differences_example_2() {
        let expected = 220;
        let example_adapters = EXAMPLE_ADAPTER_2
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        let actual = product_jolt_differences(example_adapters);

        assert_eq!(actual, expected);
    }

    #[test]
    fn product_jolt_differences_from_input() {
        let expected = 2277;

        let adapters = load_as_vec_usize("day10");
        let actual = product_jolt_differences(adapters);
        println!(
            "{}{}",
            "Product of 1 jolt and 3 jolt differences: ".green().bold(),
            actual
        );

        assert_eq!(actual, expected);
    }

    // Part2
    #[test]
    fn total_distinct_arrangements_example_1() {
        let expected = 8;
        let example_adapters = EXAMPLE_ADAPTER_1
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        let actual = total_distinct_arrangements(example_adapters);

        assert_eq!(actual, expected);
    }

    #[test]
    fn total_distinct_arrangements_example_2() {
        let expected = 19208;
        let example_adapters = EXAMPLE_ADAPTER_2
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        let actual = total_distinct_arrangements(example_adapters);

        assert_eq!(actual, expected);
    }

    #[test]
    fn total_distinct_arrangements_from_input() {
        let expected = 37024595836928;

        let adapters = load_as_vec_usize("day10");
        let actual = total_distinct_arrangements(adapters);
        println!(
            "{}{}",
            "Total number of distinct value arrangements: "
                .green()
                .bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
